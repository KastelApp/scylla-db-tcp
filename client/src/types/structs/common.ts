import type { ConnectData } from "./connect.ts";
import type { InsertData, InsertResponse } from "./insert.ts";
import type { SelectData } from "./select.ts";

export type Value = string | number | boolean | null | Array<Value> | { [key: string]: Value };

export interface Command {
    hash: string;
    command: string;
    table: string | null;
    keyspace: string | null;
    length: number;
    nonce: string | null;
    type: string | null
}

export interface QueryResult {
    result: Array<{ [key: string]: Value }>;
    error: string | null;
}

export interface InsertCommand extends Command {
    command: "insert";
    data: InsertData | InsertResponse;
}

export interface SelectCommand extends Command {
    command: "select";
    data: SelectData | QueryResult;
}

export interface ConnectCommand extends Command {
    command: "connect";
    data: ConnectData;
}

export interface RawCommand extends Command {
    command: "raw";
    data: {
        query: string;
        values: Value[];
        limit: number;
    }
}

export interface ShutdownCommand extends Command {
    command: "shutdown";
    data: {};
}

export interface UpdateCommand extends Command {
    command: "update";
    data: {
        values: { [key: string]: Value };
        primary: { [key: string]: Value };
    };
}

export interface DeleteCommand extends Command {
    command: "delete";
    data: {
        whereData: { [key: string]: Value };
    };

}

export type Commands = InsertCommand | SelectCommand | ConnectCommand | RawCommand | ShutdownCommand | UpdateCommand | DeleteCommand;