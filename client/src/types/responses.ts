export interface Result {
    keyspace_name: string;
    table_name: string;
    column_name: string;
    clustering_order: "NONE";
    kind: "regular" | "partition_key" | "clustering_column";
    poistion: number;
    type: string | "timestamp" | "text" | "boolean" | "bigint" | "int";
}

export interface IndexResult {
    keyspace_name: string;
    table_name: string;
    index_name: string;
    kind: "COMPOSITES";
    options: null;
}

export interface TypeResult {
    keyspace_name: string;
    type_name: string;
    field_names: string[];
    field_types: string[];
}

export interface KeyspaceResult {
    keyspace_name: string;
    durable_writes: boolean;
    replication: [string, string][];
}