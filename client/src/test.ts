import { newprocessArgs } from "./temp.ts";
import Client from "./client.ts";
import UnderScoreCqlToCamelCaseMappings from "./mapping/tables/UnderscoreCqlToCamelCaseMappings.ts";
import UnderscoreCqlToPascalCaseMappings from "./mapping/tables/UnderscoreCqlToPascalCaseMappings.ts";
import Mapper from "./mapping/Mapper.ts";
import type { Commands } from "./index.ts";

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
});

client.on("connect", async () => {
    console.log("Connected!");

    // const lols = await client.execute("select * from kstltest.users");

    // const test = new UnderscoreCqlToPascalCaseMappings();

    // const fixed = test.objectToNormalCasing(lols);

    // console.log(fixed);
    // const mapping = new Mapper(client, {
    //     models: {
    //         users: {
    //             keyspace: "kstltest",
    //             tables: ["users"],
    //             mappings: new UnderScoreCqlToCamelCaseMappings(),
    //         },
    //         guildMembers: {
    //             keyspace: "kstltest",
    //             tables: ["guild_members"],
    //             mappings: new UnderScoreCqlToCamelCaseMappings(),
    //         }
    //     }
    // });

    // const mapper = mapping.forModel<{
    //     userId: string;
    //     email: string;
    // }>("users")

    // const got = await client.execute("select * from kstltest.settings");

    // console.log(Bun.inspect(got, { colors: true, depth: 20 }));

    const command: Commands = {
        command: "insert",
        table: "settings",
        keyspace: "kstltest",
        data: {
            columns: {
                bio: null,
                language: "en-US",
                max_file_upload_size: 50,
                max_guilds: 50,
                mentions: [],
                privacy: 0,
                status: 3,
                theme: "dark",
                custom_status: null,
                tokens: [ // UDT type
                    {
                        created_date: "2024-02-03T18:34:33.632Z",
                        flags: 0,
                        ip: "",
                        token_: "cats",
                        token_id: "cats",
                    }
                ],
                user_id: "cats",
                guild_order: [],
                allowed_invites: 0,
            },
            ifNotExists: false,
        },
        hash: "45baed9197ea599ca57881653bd73d4ef1060bb64f6ab8b82f639b2be0cd1d444c573022765c26f7ebab85c38f05a5729e225bf918040958cc2bd976b376f42c",
        length: 723,
        nonce: "a5ff38a6-8451-47da-a29c-c0d1bc8fea74",
        type: "settings"
    }

    client.nonces.set(command.nonce!, (data, client) => {
        console.log("YAY", data);
    });

    client.handleCommad(command);
});

client.on("raw_message", (msg) => {
    console.log(msg);
})

await client.connect();