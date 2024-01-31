import type { Value } from "./common.ts";

export interface SelectData {
    where: { [key: string]: Value };
    columns: string[];
    limit: number;
}
