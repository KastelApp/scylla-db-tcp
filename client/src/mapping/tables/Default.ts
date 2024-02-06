class DefaultMapping {
    public reservedWords = [
        'ADD', 'ALLOW', 'ALTER', 'AND', 'APPLY', 'ASC', 'AUTHORIZE', 'BATCH', 'BEGIN', 'BY', 'COLUMNFAMILY', 'CREATE', 'DELETE', 'DESC', 'DESCRIBE', 'DROP', 'ENTRIES', 'EXECUTE', 'FROM', 'FULL', 'GRANT', 'IF', 'IN', 'INDEX', 'INFINITY', 'INSERT', 'INTO', 'KEYSPACE', 'LIMIT', 'MODIFY', 'NAN', 'NORECURSIVE', 'NOT', 'NULL', 'OF', 'ON', 'OR', 'ORDER', 'PRIMARY', 'RENAME', 'REPLACE', 'REVOKE', 'SCHEMA', 'SELECT', 'SET', 'TABLE', 'TO', 'TOKEN', 'TRUNCATE', 'UNLOGGED', 'UPDATE', 'USE', 'USING', 'VIEW', 'WHERE', 'WITH'
    ].map((word) => word.toLowerCase());

    /**
     * @description Converts the string to snake casing (i.e userId to user_id)
     */
    public getColumnName(column: string): string {
        return column;
    }

    /**
     * @description Converts the string to normal casing (i.e user_id to userId)
     */
    public getPropertyName(column: string): string {
        return column;
    }

    public objectToNormalCasing<T>(obj: T): any {
        if (typeof obj !== "object" || obj === null || obj === undefined) return obj;

        if (!Array.isArray(obj)) {
            const newObj: { [key: string]: any } = {};

            for (const [key, value] of Object.entries(obj)) {
                newObj[this.getPropertyName(key)] = this.objectToNormalCasing(value);
            }

            return newObj;
        } else if (Array.isArray(obj)) {
            return obj.map((value) => this.objectToNormalCasing(value));
        }

        return obj;
    }

    public objectToSnakeCasing<T>(obj: T): any {
        if (typeof obj !== "object" || obj === null || obj === undefined) return obj;
        if (obj instanceof Date) return obj.toISOString();

        if (!Array.isArray(obj)) {
            const newObj: { [key: string]: any } = {};

            for (const [key, value] of Object.entries(obj)) {
                newObj[this.getColumnName(key)] = this.objectToSnakeCasing(value);
            }

            return newObj;
        } else if (Array.isArray(obj)) {
            return obj.map((value) => this.objectToSnakeCasing(value));
        }

        return obj;
    }
}

export default DefaultMapping;