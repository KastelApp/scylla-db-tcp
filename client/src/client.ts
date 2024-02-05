import fs from "fs/promises";
import { join } from "path";
import { $, SHA512 } from "bun";
import { downloadRelease } from "./util/downloader.ts";
import { WebSocket, type RawData } from "ws";
import type { Commands } from "./types/structs/common.ts";
import generateUUIDv4 from "./util/uuid.ts";
import { baseCommands } from "./util/baseCommands.ts";
import type { IndexResult, Result, TypeResult } from "./types/responses.ts";
import { parser } from "./util/parser.ts";
interface ClientOptions {
    contactPoints?: string[];
    localDataCenter?: string;
    keyspace?: string;
    credentials?: {
        username: string;
        password: string;
    };
    durableWrites?: boolean;
    networkTopologyStrategy?: {
        [key: string]: number;
    };
    scyllatcp?: {
        host: string;
        port: number;
        /**
         * @description Whether the client should start a pre-downloaded Scylla-TCP server locally. (may not be the latest version)
         */
        startLocally: boolean;
        /**
         * @description Whether the client should pull the latest Scylla-TCP server from GitHub, requires internet connection.
         */
        pullLatest?: boolean;
    };
}

// ? people should not listen to raw_message since we use it internally
type events = "connect" | "error" | "close" | "raw_message";

type values = string | number | boolean | null | values[];

interface Client {
    on(event: "connect", callback: () => void): void;
    on(event: "error", callback: () => void): void;
    on(event: "close", callback: () => void): void;
    on(event: "raw_message", callback: (data: Commands) => void): void;
    emit(event: events, data?: unknown): void;
}

class Client {

    public nonces = new Map<string, (data: any, client: Client) => void>();

    public contactPoints: string[];

    public localDataCenter: string;

    public keyspace: string;

    #credentials: {
        username: string;
        password: string;
    };

    public get credentials() {
        return this.#credentials;
    }

    public durableWrites: boolean;

    public networkTopologyStrategy: {
        [key: string]: number;
    };

    public scyllatcp: {
        host: string;
        port: number;
        /**
         * @description Whether the client should start a pre-downloaded Scylla-TCP server locally. (may not be the latest version)
         */
        startLocally: boolean;
        /**
         * @description Whether the client should pull the latest Scylla-TCP server from GitHub, requires internet connection.
         */
        pullLatest?: boolean;
    };

    public connected = false;

    #ws!: WebSocket;

    #events = new Map<string, ((data: unknown) => void)[]>();

    public data: {
        tables: Result[] | null;
        types: TypeResult[] | null;
        indexes: IndexResult[] | null;
    } = {
            indexes: null,
            tables: null,
            types: null
        };

    public parsedData: Map<string, {
        primaryKeys: string[]; // ? the keys you are required to provide for selecting
        indexKeys: string[][]; // ? The keys you can provide when indexing
    }> = new Map();

    public constructor(options: ClientOptions) {
        this.contactPoints = options.contactPoints ?? ["localhost:9042"];

        this.localDataCenter = options.localDataCenter ?? "datacenter1";

        this.keyspace = options.keyspace ?? "test";

        this.#credentials = options.credentials ?? {
            username: "cassandra",
            password: "cassandra"
        };

        this.durableWrites = options.durableWrites ?? true;

        this.networkTopologyStrategy = options.networkTopologyStrategy ?? {
            "datacenter1": 1
        };

        this.scyllatcp = options.scyllatcp ?? {
            host: "localhost",
            port: 9042,
            startLocally: false
        };
    }

    public on(event: string, callback: unknown) {
        if (!this.#events.has(event)) {
            this.#events.set(event, []);
        }

        if (typeof callback !== "function") throw new TypeError("Callback must be a function");

        this.#events.get(event)!.push(callback as (data: unknown) => void);
    }

    public emit(event: string, data?: unknown) {
        this.#events.get(event)?.forEach((callback) => callback(data));
    }

    public async connect(): Promise<void> {
        if (this.connected) return;

        if (this.scyllatcp.startLocally) {
            if (this.scyllatcp.pullLatest) {
                await downloadRelease();
            }

            this.startTcp(); // ? this is a promise, which we do not want to resolve due to the shell commands never completing due to them needing to stay open
        }

        const ws = new WebSocket(`ws://${this.scyllatcp.host}:${this.scyllatcp.port}`);

        ws.on("open", () => this.handleConnect());
        ws.on("error", () => this.handleError());
        ws.on("close", () => this.handleClose());
        ws.on("message", (data) => this.handleMessage(data));

        this.#ws = ws;

        return new Promise((resolve, reject) => {
            this.on("connect", () => {
                resolve();
            });

            setTimeout(() => {
                reject("Connection timed out");
            }, 15_000);
        });
    }

    public async startTcp() {
        const platform = process.platform === "win32" ? "windows" : process.platform === "darwin" ? "macos" : "linux";

        if (platform === "macos") {
            throw new Error("Sorry, MacOs was not able to be built, so for now its not supported.");
        }

        let filePath = join(import.meta.dirname, "releases", "latest-scyllatcp-" + platform);

        // check if it exists
        try {
            if (platform === "windows") filePath += ".exe";

            await fs.access(filePath);
        } catch {
            //  ? if it doesn't exist then we default to the base version (scyllatcp-platform)
            filePath = join(import.meta.dirname, "releases", "scyllatcp-" + platform);

            if (platform === "windows") filePath += ".exe";
        }

        await $`chmod +x ${filePath}`;
        await $`${filePath} ${this.scyllatcp.host} ${this.scyllatcp.port} false`;
    }

    private handleError() {
        console.log("Error occurred, reconnecting...");
    }

    private handleConnect() {
        const cmd: Commands = {
            hash: "",
            command: "connect",
            data: {
                contactPoints: this.contactPoints,
                localDataCenter: this.localDataCenter,
                credentials: this.credentials,
                keyspace: this.keyspace,
            },
            length: 0,
            keyspace: null,
            nonce: null,
            table: null,
            type: null
        };

        cmd.length = cmd.command.length + JSON.stringify(cmd.data).length;

        cmd.hash = this.generateHash(cmd.command + cmd.length + JSON.stringify(cmd.data));

        this.#ws.send(JSON.stringify(cmd));
    }

    private handleMessage(data: RawData) {
        try {
            const parsed = JSON.parse(data.toString()) as Commands;

            if (parsed.nonce) {
                if (this.nonces.has(parsed.nonce)) this.nonces.get(parsed.nonce)!({
                    command: parsed.command,
                    data: parsed.data,
                    hashVerified: parsed.hash === this.generateHash(parsed.command + parsed.length + JSON.stringify(parsed.data)) // ? if this is false, the command is unsafe, and should be discarded
                }, this);
            }

            if (!this.connected && parsed.command === "connect") {
                this.connected = true;

                for (const cmd of baseCommands) {
                    cmd.cmd.length = cmd.cmd.command.length + JSON.stringify(cmd.cmd.data).length;

                    cmd.cmd.data.where.keyspace_name = this.keyspace;

                    cmd.cmd.hash = this.generateHash(cmd.cmd.command + cmd.cmd.length + JSON.stringify(cmd.cmd.data));

                    cmd.cmd.nonce = generateUUIDv4();

                    this.nonces.set(cmd.cmd.nonce, cmd.func);

                    this.#ws.send(JSON.stringify(cmd.cmd));
                }
            }

            this.emit("raw_message", parsed);
        } catch {
            console.log(data.toString());
        }
    }

    private handleClose() {
        this.connected = false;

        console.log("Connection closed, reconnecting...");

        this.reconnect();
    }

    private reconnect() {
        this.#ws = new WebSocket(`ws://${this.scyllatcp.host}:${this.scyllatcp.port}`);

        this.#ws.on("open", () => this.handleConnect());
        this.#ws.on("error", () => this.handleError());
        this.#ws.on("close", () => this.handleClose());
        this.#ws.on("message", (data) => this.handleMessage(data));
    }

    private generateHash(str: string) {
        const hash = new SHA512;

        hash.update(str);

        return hash.digest("hex");
    }

    public attemptParsing() {
        console.log(this.data);

        if (!this.data.tables || !this.data.types || !this.data.indexes) return;

        const parse = parser(this.data.tables, this.data.indexes);

        for (const table of parse.tables) {
            if (!this.parsedData.has(table)) {
                this.parsedData.set(table, {
                    primaryKeys: [],
                    indexKeys: []
                });
            }
        }

        for (const primary of parse.primaryKeys) {
            if (!this.parsedData.has(primary.table)) {
                this.parsedData.set(primary.table, {
                    primaryKeys: [primary.column],
                    indexKeys: []
                });

                continue;
            }

            this.parsedData.get(primary.table)!.primaryKeys.push(primary.column);
        }

        for (const index of parse.indexKeys) {
            if (!this.parsedData.has(index.for_table)) {
                this.parsedData.set(index.for_table, {
                    primaryKeys: [],
                    indexKeys: [index.keys.filter((a) => a !== "idx_token")]
                });

                continue;
            }

            this.parsedData.get(index.for_table)!.indexKeys.push(index.keys.filter((a) => a !== "idx_token"));
        }

        this.emit("connect");
    }

    public execute(query: string, values?: values[], options?: {
        table?: string;
        keyspace?: string;
        limit?: number;
    }): Promise<unknown[] | null> {
        return new Promise((resolve, reject) => {
            const command: Commands = {
                command: "raw",
                hash: "",
                table: options?.table ?? null,
                keyspace: options?.keyspace ?? null,
                nonce: generateUUIDv4(),
                length: 0,
                data: {
                    query,
                    values: values ?? [],
                    limit: options?.limit ?? 0
                },
                type: null
            };

            command.length = command.command.length + JSON.stringify(command.data).length;

            command.hash = this.generateHash(command.command + command.length + JSON.stringify(command.data));

            this.nonces.set(command.nonce!, (data) => {
                if (!data.hashVerified) {
                    reject(`Hashes do not match, expected ${command.hash}, received ${data.hash}`);
                }

                if (data.data.error) {
                    resolve(null);
                }

                resolve(data.data.result);
            });

            this.#ws.send(JSON.stringify(command));

            setTimeout(() => {
                if (this.nonces.has(command.nonce!)) {
                    resolve(null);

                    this.nonces.delete(command.nonce!);
                }
            }, 15_000);
        });
    }

    public handleCommad(data: Commands) {
        data.length = data.command.length + JSON.stringify(data.data).length;

        data.hash = this.generateHash(data.command + data.length + JSON.stringify(data.data));

        console.log(Bun.inspect(data, { colors: true, depth: 20 }));

        this.#ws.send(JSON.stringify(data));
    }
}

export default Client;

export { Client, type ClientOptions };