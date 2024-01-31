import Client from "@/client.ts";

interface Model {
    tables: string[];
    mappings: unknown // temp
    keyspace: string;
}

interface MapperOptions<T extends { [key: string]: Model }> {
    models?: T
}

class Mapper<T extends { [key: string]: Model }> {
    public constructor(
        client: Client,
        options: MapperOptions<T>,
    ) {

    }

    public forModel(model: keyof T) {}
}

export default Mapper;