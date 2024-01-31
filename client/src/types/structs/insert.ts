import type { Value } from "./common.ts";

export interface InsertData {
    columns: { [key: string]: Value };
    ifNotExists?: boolean | null;
}

export interface InsertResponse {
    success: boolean;
    error?: string | null;
}
