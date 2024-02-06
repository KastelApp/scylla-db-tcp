import Client from "@/client.ts";
import { ModelMapper } from "./ModelMapper.ts";
import type DefaultMapping from "./tables/Default.ts";

interface Model {
    tables: string[];
    mappings: DefaultMapping
    keyspace: string;
}

interface MapperOptions<T extends { [key: string]: Model }> {
    models?: T
}

class Mapper<T extends { [key: string]: Model }> {

    private client: Client;

    private options: MapperOptions<T>;

    public constructor(
        client: Client,
        options: MapperOptions<T>,
    ) {
        this.client = client;
        this.options = options;
    }

    public forModel<Y = any>(model: keyof T, shouldIncludeType = false): ModelMapper<Y> {
        return new ModelMapper<Y>(
            this.client,
            this.client.keyspace,
            this.options.models![model],
            shouldIncludeType
        );
    }
}

export default Mapper;

export type { Model, MapperOptions };
