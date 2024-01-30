import { newprocessArgs } from "./temp.ts";

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
])

if (args.clear) {
    console.clear();
}

Bun.connect({
    hostname: "localhost",
    port: 8080,
    socket: {
        open: (socket) => {
            const cmd = {
                "command": "connect",
                "data": {
                    "contactPoints": [
                        "localhost:9042"
                    ],
                    "localDataCenter": "datacenter1",
                    "credentials": {
                        "username": args.username,
                        "password": args.password
                    },
                    "keyspace": args.keyspace
                }
            };

            socket.write(Buffer.from(JSON.stringify(cmd)));

            setTimeout(() => {
                const cat = { // command example: SELECT id, name FROM test WHERE id = '1' LIMIT 1
                    "command": "select",
                    "table": "users",
                    "keyspace": "kstltest", // if you don't provide a keyspace, we will use the keyspace provided in the connect command
                    "data": {
                        "where": { // extra elements will have an AND operator for example: { "id": "1", "name": "test" } will be `WHERE id = '1' AND name = 'test'`
                            "tag": "9541"
                        },
                        "columns": [],
                        "limit": 1
                    }
                };

                socket.write(Buffer.from(JSON.stringify(cat)));
            }, 1000);
        },
        data: (socket, data) => {
            try {
                const json = JSON.parse(data.toString());

                console.log(Bun.inspect(json, { colors: true, depth: 20 }));
            } catch {
                console.log(data.toString());
            }

            console.log("\n")
        },
    }
});