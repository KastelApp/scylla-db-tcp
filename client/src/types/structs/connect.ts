interface CredentialsData {
    username: string;
    password: string;
}

export interface ConnectData {
    contactPoints: string[];
    localDataCenter: string;
    credentials: CredentialsData;
    keyspace: string;
}
