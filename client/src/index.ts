
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
                        "username": "",
                        "password": ""
                    },
                    "keyspace": "test"
                }
            };

            socket.write(Buffer.from(JSON.stringify(cmd)));
            
            setTimeout(() => {
                const cat = {
                    "command": "test",
                    "data": {}
                }

                socket.write(Buffer.from(JSON.stringify(cat)));
            }, 1000);
        },
        data: (socket, data) => {
            console.log(data.toString() + "\n");
        },
    }
});