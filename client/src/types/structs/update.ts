import type { Value } from "./common.ts";

export interface UpdateData {
    primary: { [key: string]: Value };
    values: { [key: string]: Value };
}
