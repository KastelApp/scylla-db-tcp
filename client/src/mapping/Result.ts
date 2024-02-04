import type DefaultMapping from "./tables/Default.ts";

class Result<T = any> {

    private data: T[];

    private mapper: DefaultMapping;

    public constructor(
        data: T[],
        mapper: DefaultMapping
    ) {
        this.data = data;
        this.mapper = mapper;
    }

    public first(): T | null {
        const first = this.data[0];

        return first ? this.mapper.objectToNormalCasing(first) : null;
    }

    public toArray(): T[] {
        return this.mapper.objectToNormalCasing(this.data);
    }

    public forEach(callback: (row: T) => void, thisArg: Object): void {
        const array = this.toArray();

        for (let i = 0; i < array.length; i++) {
            callback.call(thisArg, array[i]);
        }
    }

    inspect(): T[] {
        return this.toArray();
    }
}

export default Result;