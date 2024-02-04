import type Client from "@/client.ts";
import type { Model } from "./Mapper.ts";
import type { Commands } from "@/types/structs/common.ts";
import generateUUIDv4 from "@/util/uuid.ts";
import Result from "./Result.ts";

interface docFields<T> {
    fields?: (keyof T)[];
    allowFiltering?: boolean;
}

interface FindDocInfo<T> {
    fields?: (keyof T)[];
    orderBy?: { [key: string]: string; };
    limit?: number;
    allowFiltering?: boolean;
};

interface InsertDocInfo<T> {
    fields?: (keyof T)[];
    ifNotExists?: boolean;
}

type UpdateDocInfo<T> = {
    fields?: (keyof T)[];
    ifExists?: boolean;
    orderBy?: { [key: string]: string; };
    limit?: number;
    deleteOnlyColumns?: boolean;
};

type RemoveDocInfo<T> = {
    fields?: (keyof T)[];
    ifExists?: boolean;
    deleteOnlyColumns?: boolean;
};

class ModelMapper<T = any> {
    private client: Client;

    private keyspace: string;

    private model: Model;

    public constructor(
        client: Client,
        keyspace: string,
        model: Model
    ) {
        this.client = client;
        this.keyspace = keyspace;
        this.model = model;
    }

    public get(doc: Partial<T>, docInfo?: docFields<T>): Promise<T | null> {
        return new Promise((resolve, reject) => {

            if (!this.model.tables.some((table) => this.client.parsedData.has(table))) {
                reject("Table not found");
            }

            const table = this.client.parsedData.get(this.model.tables[0])!; // ? unsure how cassandra-driver handles multiple tables, I only care about the first one :shrug:

            const providedDocKeys = Object.keys(doc).map((key) => this.model.mappings.getColumnName(key));

            // ? first check if they match primary keys (i.e all the primary keys are provided)
            if (!table.primaryKeys.every((key) => providedDocKeys.includes(key))) {
                // ? ok so it doesn't, that may be fine, now we check if it matches the index keys, those are arrays of arrays of strings, they must match / include the index keys
                if (!table.indexKeys.every((index) => index.every((key) => providedDocKeys.includes(key)))) {
                    // ? So they don't match, we throw an error else scylla will not accept the query
                    reject(`Missing required keys: ${table.primaryKeys.join(", ")} or ${table.indexKeys.map((index) => index.join(", ")).join(" or ")}`);
                }
            }

            const command: Commands = {
                command: "select",
                table: this.model.tables[0],
                keyspace: this.keyspace,
                data: {
                    where: Object.fromEntries(Object.entries(doc).map(([key, value]) => [this.model.mappings.getColumnName(key), value])) as { [key: string]: string; },
                    columns: docInfo?.fields ? docInfo.fields.map((field) => this.model.mappings.getColumnName(field as string)) : [],
                    limit: 0
                },
                hash: "",
                length: 0,
                nonce: generateUUIDv4(),
            };

            this.client.nonces.set(command.nonce!, (cmd, client) => {
                if (cmd.data.error) {
                    reject(cmd.data.error);
                }

                if (!cmd.hashVerified) {
                    reject("Hash not verified");
                }

                resolve(cmd.data.result.map((row: any) => this.model.mappings.objectToNormalCasing(row) as T)[0] ?? null);
            });

            this.client.handleCommad(command);

            setTimeout(() => {
                if (this.client.nonces.has(command.nonce!)) {
                    resolve(null);

                    this.client.nonces.delete(command.nonce!);
                }
            }, 15_000);
        });
    };

    public find(doc: Partial<T>, docInfo?: docFields<T>): Promise<Result<T>> {
        return new Promise((resolve, reject) => {
            if (!this.model.tables.some((table) => this.client.parsedData.has(table))) {
                reject("Table not found");
            }

            const table = this.client.parsedData.get(this.model.tables[0])!; // ? unsure how cassandra-driver handles multiple tables, I only care about the first one :shrug:

            const providedDocKeys = Object.keys(doc).map((key) => this.model.mappings.getColumnName(key));

            // ? first check if they match primary keys (i.e all the primary keys are provided)
            if (!table.primaryKeys.every((key) => providedDocKeys.includes(key))) {
                // ? ok so it doesn't, that may be fine, now we check if it matches the index keys, those are arrays of arrays of strings, they must match / include the index keys
                if (!table.indexKeys.every((index) => index.every((key) => providedDocKeys.includes(key)))) {
                    // ? So they don't match, we throw an error else scylla will not accept the query
                    reject(`Missing required keys: ${table.primaryKeys.join(", ")} or ${table.indexKeys.map((index) => index.join(", ")).join(" or ")}`);
                }
            }

            const command: Commands = {
                command: "select",
                table: this.model.tables[0],
                keyspace: this.keyspace,
                data: {
                    where: Object.fromEntries(Object.entries(doc).map(([key, value]) => [this.model.mappings.getColumnName(key), value])) as { [key: string]: string; },
                    columns: docInfo?.fields ? docInfo.fields.map((field) => this.model.mappings.getColumnName(field as string)) : [],
                    limit: 0
                },
                hash: "",
                length: 0,
                nonce: generateUUIDv4(),
            };

            this.client.nonces.set(command.nonce!, (cmd, client) => {
                if (cmd.data.error) {
                    reject(cmd.data.error);
                }

                if (!cmd.hashVerified) {
                    reject("Hash not verified");
                }

                const result = new Result<T>(cmd.data.result, this.model.mappings);

                resolve(result);
            });

            this.client.handleCommad(command);

            setTimeout(() => {
                if (this.client.nonces.has(command.nonce!)) {
                    const result = new Result<T>([], this.model.mappings);

                    resolve(result);
                }

                this.client.nonces.delete(command.nonce!);
            }, 15_000);
        });
    };

    /**
     * @description To be Honestly, I have no clue what this is supposed to do compared to "find", so yeah I'm just leaving it empty someone PR if you want to fix this
     */
    public findAll(doc: FindDocInfo<T>): T[] {
        return [];
    };

    public insert(doc: Partial<T>, docInfo?: InsertDocInfo<T>): Promise<T | null> {
        return new Promise((resolve, reject) => {
            if (!this.model.tables.some((table) => this.client.parsedData.has(table))) {
                reject("Table not found");
            }

            const command: Commands = {
                command: "insert",
                table: this.model.tables[0],
                keyspace: this.keyspace,
                data: {
                    columns: this.model.mappings.objectToSnakeCasing(doc),
                    ifNotExists: docInfo?.ifNotExists ?? false,
                },
                hash: "",
                length: 0,
                nonce: generateUUIDv4(),
            };

            this.client.nonces.set(command.nonce!, (cmd, client) => {
                if (cmd.data.error) {
                    reject(cmd.data.error);
                }

                if (!cmd.hashVerified) {
                    reject("Hash not verified");
                }

                resolve(this.model.mappings.objectToNormalCasing(doc) as T);
            });

            this.client.handleCommad(command);

            setTimeout(() => {
                if (this.client.nonces.has(command.nonce!)) {
                    resolve(null);

                    this.client.nonces.delete(command.nonce!);
                }
            }, 15_000);
        });
    };

    public update(doc: Partial<T>, docInfo?: UpdateDocInfo<T>): T | null {
        return null;
    };

    public remove(doc: Partial<T>, docInfo?: RemoveDocInfo<T>): boolean {
        return false;
    };
}

export {
    ModelMapper,
    type docFields,
    type FindDocInfo,
    type InsertDocInfo,
    type UpdateDocInfo,
    type RemoveDocInfo
};