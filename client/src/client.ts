interface ClientOptions {
    contactPoints?: string[];
    localDataCenter?: string;
    keyspace?: string;
    credentials?: {
        username: string;
        password: string;
    };
    durableWrites?: boolean;
    networkTopologyStrategy?: {
        [key: string]: number;
    };
    scyllatcp?: {
        host: string;
        port: number;
        /**
         * @description Whether the client should start a pre-downloaded Scylla-TCP server locally. (may not be the latest version)
         */
        startLocally: boolean;
        /**
         * @description Whether the client should pull the latest Scylla-TCP server from GitHub, requires internet connection.
         */
        pullLatest?: boolean;
    }
}

class Client {
    public contactPoints: string[];

    public localDataCenter: string;

    public keyspace: string;

    #credentials: {
        username: string;
        password: string;
    }

    public get credentials() {
        return this.#credentials;
    }

    public durableWrites: boolean;

    public networkTopologyStrategy: {
        [key: string]: number;
    };

    public scyllatcp: {
        host: string;
        port: number;
        /**
         * @description Whether the client should start a pre-downloaded Scylla-TCP server locally. (may not be the latest version)
         */
        startLocally: boolean;
    }

    public constructor(options: ClientOptions) {
        this.contactPoints = options.contactPoints ?? ["localhost:9042"];
        
        this.localDataCenter = options.localDataCenter ?? "datacenter1";
        
        this.keyspace = options.keyspace ?? "test";
        
        this.#credentials = options.credentials ?? {
            username: "cassandra",
            password: "cassandra"
        };

        this.durableWrites = options.durableWrites ?? true;

        this.networkTopologyStrategy = options.networkTopologyStrategy ?? {
            "datacenter1": 1
        };

        this.scyllatcp = options.scyllatcp ?? {
            host: "localhost",
            port: 9042,
            startLocally: false
        };
    }
}

export default Client;

export { Client };