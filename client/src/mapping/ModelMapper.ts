import type Client from "@/client.ts";
import type { Model } from "./Mapper.ts";
import type { Commands } from "@/types/structs/common.ts";
import generateUUIDv4 from "@/util/uuid.ts";
import Result from "./Result.ts";

interface docFields<T> {
    fields?: (keyof T)[];
    allowFiltering?: boolean;
    limit?: number;
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
    where?: {
        [K in keyof T]: T[K];
    };
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

    private shouldIncludeType: boolean;

    public constructor(
        client: Client,
        keyspace: string,
        model: Model,
        shouldIncludeType = false
    ) {
        this.client = client;
        this.keyspace = keyspace;
        this.model = model;
        this.shouldIncludeType = shouldIncludeType;
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
                type: null
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
                    limit: docInfo?.limit ?? 0,
                },
                hash: "",
                length: 0,
                nonce: generateUUIDv4(),
                type: "raw"
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
                type: null
            };

            if (Object.values(doc).some((value) => Array.isArray(value) && value.some((v) => typeof v === "object"))) {
                command.type = this.model.tables[0];
            }

            if (this.shouldIncludeType) {
                command.type = this.model.tables[0];
            }

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

    public update(doc: Partial<T>, docInfo?: UpdateDocInfo<T>): Promise<T | null> {
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

            // ? we get the keys that are primary or index keys, these will be the "where" part of the query if docInfo.where is not provided
            let whereKeys = docInfo?.where;

            if (!whereKeys) {
                if (table.primaryKeys.every((key) => providedDocKeys.includes(key))) {
                    // we only want to provide the primary keys nothing else
                    whereKeys = Object.fromEntries(table.primaryKeys.map((key) => [key, doc[this.model.mappings.getPropertyName(key) as keyof T]])) as { [k in keyof T]: T[k] };
                } else if (table.indexKeys.every((index) => index.every((key) => providedDocKeys.includes(key))) && !whereKeys) {
                    // ? now we only want ONE set of index keys whichever one is provided
                    whereKeys = Object.fromEntries(table.indexKeys.map((index) => index.map((key) => [key, doc[this.model.mappings.getPropertyName(key) as keyof T]]).filter(([, value]) => value !== undefined))[0]) as { [k in keyof T]: T[k] };
                }

                if (!whereKeys) {
                    reject(`Missing required keys: ${table.primaryKeys.join(", ")} or ${table.indexKeys.map((index) => index.join(", ")).join(" or ")}`);
                }
            }

            let data = doc;

            // remove the primary keys from the data (or index keys if theres no primary keys)
            for (const key of table.primaryKeys) {
                delete data[this.model.mappings.getPropertyName(key) as keyof T];
            }

            if (!table.primaryKeys.every((key) => providedDocKeys.includes(key))) {
                for (const index of table.indexKeys) {
                    for (const key of index) {
                        delete data[this.model.mappings.getPropertyName(key) as keyof T];
                    }
                }
            }

            const command: Commands = {
                command: "update",
                data: {
                    primary: this.model.mappings.objectToSnakeCasing(whereKeys!),
                    values: this.model.mappings.objectToSnakeCasing(doc),
                },
                table: this.model.tables[0],
                keyspace: this.keyspace,
                hash: "",
                length: 0,
                nonce: generateUUIDv4(),
                type: null
            };

            if (this.shouldIncludeType) {
                command.type = this.model.tables[0];
            }

            this.client.nonces.set(command.nonce!, (cmd, client) => {
                if (cmd.data.error) {
                    reject(cmd.data.error);
                }

                if (!cmd.hashVerified) {
                    reject("Hash not verified");
                }

                if (cmd.data.succses) {
                    resolve(this.model.mappings.objectToNormalCasing(doc) as T);
                }

                resolve(null);
            });

            this.client.handleCommad(command);
        });
    };

    public remove(doc: Partial<T>, docInfo?: RemoveDocInfo<T>): Promise<boolean> {
        return new Promise((resolve, reject) => {
            if (!this.model.tables.some((table) => this.client.parsedData.has(table))) {
                reject("Table not found");
            }

            const table = this.client.parsedData.get(this.model.tables[0])!; // ? unsure how cassandra-driver handles multiple tables, I only care about the first one :shrug:

            const providedDocKeys = Object.keys(doc).map((key) => this.model.mappings.getColumnName(key));

            // ? first check if they match primary keys (i.e all the primary keys are provided)
            if (!table.primaryKeys.every((key) => providedDocKeys.includes(key))) {
                // ? ok so it doesn't, that may be fine, now we check if it matches the index keys, those are arrays of arrays of strings, they must match / include the index keys
                if (!table.indexKeys.every((index) => index.every((key) => providedDocKeys.includes(key))) && !docInfo?.deleteOnlyColumns) {
                    // ? So they don't match, we throw an error else scylla will not accept the query
                    reject(`Missing required keys: ${table.primaryKeys.join(", ")} or ${table.indexKeys.map((index) => index.join(", ")).join(" or ")}`);
                }
            }

            // ? we get the keys that are primary or index keys, these will be the "where" part of the query if docInfo.where is not provided
            let whereKeys = doc;;

            if (!whereKeys) {
                if (table.primaryKeys.every((key) => providedDocKeys.includes(key))) {
                    // we only want to provide the primary keys nothing else
                    whereKeys = Object.fromEntries(table.primaryKeys.map((key) => [key, doc[this.model.mappings.getPropertyName(key) as keyof T]])) as { [k in keyof T]: T[k] };
                } else if (table.indexKeys.every((index) => index.every((key) => providedDocKeys.includes(key)) && !whereKeys)) {
                    // ? now we only want ONE set of index keys whichever one is provided
                    whereKeys = Object.fromEntries(table.indexKeys.map((index) => index.map((key) => [key, doc[this.model.mappings.getPropertyName(key) as keyof T]]).filter(([, value]) => value !== undefined))[0]) as { [k in keyof T]: T[k] };
                }
            }

            if (!whereKeys) {
                reject(`Missing required keys: ${table.primaryKeys.join(", ")} or ${table.indexKeys.map((index) => index.join(", ")).join(" or ")}`);
            }

            const command: Commands = {
                command: "delete",
                data: {
                    whereData: this.model.mappings.objectToSnakeCasing(whereKeys),
                },
                table: this.model.tables[0],
                keyspace: this.keyspace,
                hash: "",
                length: 0,
                nonce: generateUUIDv4(),
                type: null
            };

            if (this.shouldIncludeType) {
                command.type = this.model.tables[0];
            }

            this.client.nonces.set(command.nonce!, (cmd, client) => {
                if (cmd.data.error) {
                    reject(cmd.data.error);
                }

                if (!cmd.hashVerified) {
                    reject("Hash not verified");
                }

                if (cmd.data.succses) {
                    resolve(true);
                }

                resolve(false);
            });

            this.client.handleCommad(command);

            setTimeout(() => {
                if (this.client.nonces.has(command.nonce!)) {
                    resolve(false);

                    this.client.nonces.delete(command.nonce!);
                }
            }, 15_000);
        });
    }
}

export {
    ModelMapper,
    type docFields,
    type FindDocInfo,
    type InsertDocInfo,
    type UpdateDocInfo,
    type RemoveDocInfo
};