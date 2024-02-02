class DefaultMapping {
    public reservedWords = [
        'ADD', 'ALLOW', 'ALTER', 'AND', 'APPLY', 'ASC', 'AUTHORIZE', 'BATCH', 'BEGIN', 'BY', 'COLUMNFAMILY', 'CREATE', 'DELETE', 'DESC', 'DESCRIBE', 'DROP', 'ENTRIES', 'EXECUTE', 'FROM', 'FULL', 'GRANT', 'IF', 'IN', 'INDEX', 'INFINITY', 'INSERT', 'INTO', 'KEYSPACE', 'LIMIT', 'MODIFY', 'NAN', 'NORECURSIVE', 'NOT', 'NULL', 'OF', 'ON', 'OR', 'ORDER', 'PRIMARY', 'RENAME', 'REPLACE', 'REVOKE', 'SCHEMA', 'SELECT', 'SET', 'TABLE', 'TO', 'TOKEN', 'TRUNCATE', 'UNLOGGED', 'UPDATE', 'USE', 'USING', 'VIEW', 'WHERE', 'WITH'
    ].map((word) => word.toLowerCase());

    public getColumnName(column: string): string {
        return column;
    }

    public getPropertyName(column: string): string {
        return column;
    }

    public objectToFixedCasing<T>(obj: T): any {
        if (typeof obj !== "object" || obj === null || obj === undefined) return obj;

        if (!Array.isArray(obj)) {
            const newObj: { [key: string]: any } = {};

            for (const [key, value] of Object.entries(obj)) {
                if (value instanceof Date || value === null) {
                    newObj[this.getPropertyName(key)] = value;
                } else if (typeof value === "object") {
                    newObj[this.getPropertyName(key)] = this.objectToFixedCasing(value);
                } else {
                    newObj[this.getPropertyName(key)] = value;
                }
            }

            return newObj;
        } else if (Array.isArray(obj)) {
            return obj.map((value) => this.objectToFixedCasing(value));
        }

        return obj;
    }
}

export default DefaultMapping;