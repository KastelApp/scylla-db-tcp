import { Client, type ClientOptions } from "@/client.ts";
import mapping, {
    DefaultMapping,
    Mapper,
    ModelMapper,
    UnderScoreCqlToCamelCaseMappings,
    UnderscoreCqlToPascalCaseMappings
} from "@/mapping/index.ts";
import type {
    IndexResult,
    Result,
    TypeResult
} from '@/types/responses.ts'
import type {
    Command,
    Commands,
    ConnectCommand,
    InsertCommand,
    QueryResult,
    RawCommand,
    SelectCommand,
    Value
} from '@/types/structs/common.ts'
import type {
    ConnectData
} from '@/types/structs/connect.ts'
import type {
    InsertData,
    InsertResponse
} from '@/types/structs/insert.ts'
import type {
    SelectData
} from '@/types/structs/select.ts'
import { baseCommands } from "./util/baseCommands.ts";


export {
    Client,
    mapping,
    baseCommands,
    DefaultMapping,
    Mapper,
    ModelMapper,
    UnderScoreCqlToCamelCaseMappings,
    UnderscoreCqlToPascalCaseMappings
};

export default Client;

export type {
    IndexResult,
    Result,
    TypeResult,
    Command,
    Commands,
    ConnectCommand,
    InsertCommand,
    QueryResult,
    RawCommand,
    SelectCommand,
    Value,
    ConnectData,
    InsertData,
    InsertResponse,
    SelectData,
    ClientOptions
};