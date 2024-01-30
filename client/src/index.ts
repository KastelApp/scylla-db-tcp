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
]);

if (args.clear) {
    console.clear();
}

const generateHash = (str: string) => {
    const hash = new Bun.SHA512;

    hash.update(str);

    return hash.digest("hex");
};

Bun.connect({
    hostname: "localhost",
    port: 8080,
    socket: {
        open: (socket) => {
            const cmd = {
                "hash": "",
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
                },
                "length": 1,
            };

            cmd.length = cmd.command.length + JSON.stringify(cmd.data).length;

            cmd.hash = generateHash(cmd.command + cmd.length + JSON.stringify(cmd.data));

            socket.write(Buffer.from(JSON.stringify(cmd)) + "\n");

            setTimeout(() => {
                // setInterval(() => {
                // const cat = { // command example: SELECT id, name FROM test WHERE id = '1' LIMIT 1
                //     "hash": "",
                //     "command": "insert",
                //     "table": "users",
                //     "keyspace": "kstltest", // if you don't provide a keyspace, we will use the keyspace provided in the connect command
                //     "data": {
                //         "where": { // extra elements will have an AND operator for example: { "id": "1", "name": "test" } will be `WHERE id = '1' AND name = 'test'`
                //             "tag": "9541"
                //         },
                //         "columns": [],
                //         "limit": 1
                //     },
                //     "length": 1,
                //     "nonce": "15"
                // };
                const cat = { // command example: SELECT id, name FROM test WHERE id = '1' LIMIT 1
                    "hash": "",
                    "command": "insert",
                    "table": "users",
                    "keyspace": "kstltest", // if you don't provide a keyspace, we will use the keyspace provided in the connect command
                    "data": {
                        "columns": {
                            "user_id": "1",
                            "username": "test",
                            "flags": "0",
                            "guilds": [],
                            "email": ""
                        },
                        "ifNotExists": false
                    },
                    "length": 1,
                    "nonce": "15"
                };

                cat.length = cat.command.length + JSON.stringify(cat.data).length;

                cat.hash = generateHash(cat.command + String(cat.length) + JSON.stringify(cat.data));

                console.log(cat);

                socket.write(Buffer.from(JSON.stringify(cat)) + "\n");

                // }, 0);
            }, 2000);
        },
        data: (socket, data) => {
            try {
                const json = JSON.parse(data.toString());

                const hash = generateHash(json.command + json.length + JSON.stringify(json.data));

                if (hash !== json.hash) {
                    console.log("Hashes do not match!");
                    console.log(`Expected: ${hash}`);
                    console.log(`Received: ${json.hash}`);
                }

                console.log(Bun.inspect(json, { colors: true, depth: 20 }));
            } catch {
                console.log(data.toString());
            }

            console.log("\n");
        },
    }
});