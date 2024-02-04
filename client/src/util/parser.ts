import type { IndexResult, Result } from "@/types/responses.ts";

const regex = /frozen<([^<>]+)>/;

const defaultTypes = {
    "timestamp": "date",
    "text": "string",
    "boolean": "boolean",
    "bigint": "bigint",
    "int": "number",
};

const getTypes = (types: string[]) => {
    const returntype: {
        default: string[],
        custom: string[];
    } = {
        default: [],
        custom: []
    };

    for (const type of types) {
        if (defaultTypes[type.replace("[]", "") as keyof typeof defaultTypes]) {
            returntype.default.push(defaultTypes[type.replace("[]", "") as keyof typeof defaultTypes] + (type.endsWith("[]") ? "[]" : ""));
        } else {
            returntype.custom.push(type);
        }
    }

    return returntype;
};

const parser = (results: Result[], indexes: IndexResult[]) => {
    const mappedTypes = Array.from(new Set(results.map((result) => result.type)));

    const realTypes = mappedTypes.map((type) => {
        const matched = regex.exec(type);

        if (matched) {
            return matched[1] + "[]";
        }

        if (type.startsWith("list<")) {
            return type.replace("list<", "").replace(">", "") + "[]";
        }

        return type;
    });

    const types = getTypes(realTypes);

    const primaryKeys = results.filter((a) => a.kind === "partition_key" && !a.table_name.endsWith("_index")).map((a) => ({ table: a.table_name, column: a.column_name }));
    const indexKeys = results.filter((a) => {

        if (a.table_name.endsWith("_index")) {
            return primaryKeys.some((b) => b.table === a.table_name.slice(0, -6) && b.column === a.column_name);
        }

        return a.table_name.endsWith("_index") && indexes.some((b) => b.index_name === a.table_name.slice(0, -6));
    })
        .map((a) => ({ table: a.table_name, column: a.column_name })).reduce((acc, cur) => {
            const index = acc.findIndex((a) => a.index === cur.table);

            if (index === -1) {
                acc.push({ index: cur.table, keys: [cur.column], for_table: indexes.find((a) => a.index_name === cur.table.slice(0, -6))?.table_name ?? "" });
            } else {
                acc[index].keys.push(cur.column);
            }

            return acc;
        }, [] as { index: string, keys: string[]; for_table: string; }[]);

    const tables = Array.from(new Set(results.map((a) => a.table_name).filter((a) => !a.endsWith("_index"))));
    const indextypes = Array.from(new Set(results.map((a) => a.table_name).filter((a) => a.endsWith("_index"))));

    return {
        types,
        primaryKeys,
        indexKeys,
        tables,
        indextypes
    }
};

export {
    parser,
    getTypes
}