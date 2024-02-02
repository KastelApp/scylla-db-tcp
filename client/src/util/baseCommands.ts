import type Client from "@/client.ts";
import type { Command, Commands } from "@/types/structs/common.ts";
import type { IndexResult, Result, TypeResult } from "@/types/responses.ts";

interface cmd<T = any> {
    command: "select";
    data: {
        result: T[],
        error: string | null,
    };
    hashVerified: boolean;
}

interface BaseCommand extends Command {
    data: {
        where: {
            keyspace_name: string;
        },
        columns: string[],
        limit: number,
    }
}

export const baseCommands: { cmd: BaseCommand, func: (data: cmd, client: Client) => void }[] = [
    {
        cmd: { // fetches all the tables
            command: "select",
            data: {
                where: {
                    keyspace_name: ""
                },
                columns: [],
                limit: 0,
            },
            hash: "",
            length: 0,
            keyspace: "system_schema",
            nonce: null,
            table: "columns"
        },
        func: (data: cmd<Result>, client) => {
            client.data.tables = data.data.result;

            client.attemptParsing();
        }
    },
    {
        cmd: { // fetches all the types
            command: "select",
            data: {
                where: {
                    keyspace_name: ""
                },
                columns: [],
                limit: 0,
            },
            hash: "",
            length: 0,
            keyspace: "system_schema",
            nonce: null,
            table: "types"
        },
        func: (data: cmd<TypeResult>, client) => {
            client.data.types = data.data.result;

            client.attemptParsing();
        }
    },
    {
        cmd: { // fetches all the indexes
            command: "select",
            data: {
                where: {
                    keyspace_name: ""
                },
                columns: [],
                limit: 0,
            },
            hash: "",
            length: 0,
            keyspace: "system_schema",
            nonce: null,
            table: "indexes"
        },
        func: (data: cmd<IndexResult>, client) => {
            client.data.indexes = data.data.result;

            client.attemptParsing();
        }
    }
];