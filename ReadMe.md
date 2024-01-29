# Scylla-DB TCP Server

This is a simple TCP server which supports commands to do and receive data from Scylla-DB.

Note: This is for the NPM package `@kastelapp/scylla`, you can use this on your own if you wish.

An example command is as follows:

```json
{
    "command": "select", // select, insert, update, delete, connect, raw
    "table": "test", // the table to use
    "keyspace": "test", // the keyspace to use (defaults to the initial keyspace set in the connect command)
    "data": {
        "where": {
            "id": "1"
        },
        "columns": [
            "id",
            "name"
        ],
        "limit": 1
    }
}
```

The server will respond with a JSON object containing the result of the command:

```json
{
    "result": [
        {
            "id": "1",
            "name": "test"
        }
    ]
}
```

## Commands

## Connect

The connect command is used to connect to a Scylla-DB instance. It is required to be the first command sent to the server.

```json
{
    "command": "connect",
    "data": {
        "contactPoints": [
            "localhost:9042",
            "1.2.3.4:9042"
        ],
        "localDataCenter": "datacenter1",
        "credentials": {
            "username": "cassandra",
            "password": "cassandra"
        },
        "keyspace": "test"
    }
}
```

### Response

```json
{
    "command": "connect",
    "data": {
        "success": true, // if the connection was successful
        "error": null // if the connection was not successful, this will contain the error (i.e. "Failed to connect to any host")
    }
}
```

## Select

The select command is used to select data from a table.

```json
{ // command example: SELECT id, name FROM test WHERE id = '1' LIMIT 1
    "command": "select",
    "table": "test",
    "keyspace": "test", // if you don't provide a keyspace, we will use the keyspace provided in the connect command
    "data": {
        "where": { // extra elements will have an AND operator for example: { "id": "1", "name": "test" } will be `WHERE id = '1' AND name = 'test'`
            "id": "1"
        },
        "columns": [ // the columns to select, if empty it will select all columns (not recommended) i.e SELECT id, name FROM test
            "id",
            "name"
        ],
        "limit": 1
    }
}
```

### Response

```json
{
    "command": "select",
    "data": {
        "result": [ // the result of the query
            {
                "id": "1",
                "name": "test"
            }
        ],
        "error": null // if the query was not successful, this will contain the error (i.e. "No keyspace has been specified")
    }
}
```

## Insert

The insert command is used to insert data into a table.

```json
{ // command example: INSERT INTO test (id, name) VALUES ('1', 'test')
    "command": "insert",
    "table": "test",
    "keyspace": "test",
    "data": {
        "columns": {
            "id": "1",
            "name": "test"
        }
    }
}
```

### Response

```json
{
    "command": "insert",
    "data": {
        "success": true, // if the insert was successful
        "error": null // if the insert was not successful, this will contain the error (i.e. "No keyspace has been specified")
    }
}
```

## Update

The update command is used to update data in a table.

```json
{ // command example: UPDATE test SET name = 'test2' WHERE id = '1'
    "command": "update",
    "table": "test",
    "keyspace": "test",
    "data": {
        "columns": {
            "name": "test2"
        },
        "where": {
            "id": "1"
        }
    }
}
```

### Response

```json
{
    "command": "update",
    "data": {
        "success": true, // if the update was successful
        "error": null // if the update was not successful, this will contain the error (i.e. "No keyspace has been specified")
    }
}
```

## Delete

The delete command is used to delete data from a table.

```json
{ // command example: DELETE FROM test WHERE id = '1'
    "command": "delete",
    "table": "test",
    "keyspace": "test",
    "data": {
        "where": {
            "id": "1"
        }
    }
}
```

### Response

```json
{
    "command": "delete",
    "data": {
        "success": true, // if the delete was successful
        "error": null // if the delete was not successful, this will contain the error (i.e. "No keyspace has been specified")
    }
}
```

## Raw

The raw command is used to send a raw CQL query to the Scylla-DB instance. Returns whatever the Scylla-DB instance returns.

```json
{ // command example: SELECT * FROM test WHERE id = '1'
    "command": "raw",
    "data": "SELECT * FROM test WHERE id = '1'"
}
```

### Response

```json
{
    "command": "raw",
    "data": {
        "result": [ // the result of the query
            {
                "id": "1",
                "name": "test"
            }
        ],
        "error": null // if the query was not successful, this will contain the error (i.e. "No keyspace has been specified")
    }
}
```