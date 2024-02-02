import DefaultMapping from "./Default.ts";

class UnderScoreCqlToCamelCaseMappings extends DefaultMapping {
    public override getColumnName(column: string): string {
        const propName = column.split(/(?=[A-Z])/).join('_').toLowerCase();

        if (this.reservedWords.includes(propName)) {
            return `${propName}_`;
        } else {
            return propName;
        }
    }

    public override getPropertyName(table: string): string {
        return table.split('_').map((word, index) => {
            if (index === 0) return word;
            return word.charAt(0).toUpperCase() + word.slice(1);
        }).join('');
    }
}

export default UnderScoreCqlToCamelCaseMappings;