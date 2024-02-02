import { newprocessArgs } from "./temp.ts";
import Client from "./client.ts";
import UnderScoreCqlToCamelCaseMappings from "./mapping/tables/UnderscoreCqlToCamelCaseMappings.ts";
import UnderscoreCqlToPascalCaseMappings from "./mapping/tables/UnderscoreCqlToPascalCaseMappings.ts";

const args = newprocessArgs([ // temporary for development
    {
        name: "username",
        type: "string",
    },
    {
        name: "password",
        type: "string",
    },
    {
        name: "keyspace",
        type: "string",
        optional: true,
        default: "test"
    },
    {
        name: "clear",
        type: "boolean",
        optional: true,
    }
]);

const client = new Client({
    scyllatcp: {
        host: "localhost",
        port: 8080,
        startLocally: false,
        pullLatest: false,
    },
    contactPoints: ["localhost:9042"],
    credentials: {
        username: args.username as string,
        password: args.password as string,
    },
    keyspace: args.keyspace as string,
    localDataCenter: "datacenter1",
})

client.on("connect", async () => {
    console.log("Connected!");

    const lols = await client.execute("select * from kstltest.users");

    const test = new UnderscoreCqlToPascalCaseMappings();

    const fixed = test.objectToFixedCasing(lols);

    console.log(fixed);
})

await client.connect();