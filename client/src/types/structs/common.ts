import type { ConnectData } from "./connect.ts";
import type { InsertData, InsertResponse } from "./insert.ts";
import type { SelectData } from "./select.ts";

export type Value = string | number | boolean | null | Array<Value> | { [key: string]: Value };

// export type CommandData = SelectData | InsertData | ConnectData | QueryResult | InsertResponse;

export interface Command {
    hash: string;
    command: string;
    table: string | null;
    keyspace: string | null;
    // data: CommandData;
    length: number;
    nonce: string | null;
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

export type Commands = InsertCommand | SelectCommand | ConnectCommand | RawCommand;