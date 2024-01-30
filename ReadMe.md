# Scylla-DB TCP Server

This is a simple TCP server built with Rust and Tokio, which supports commands to interact with Scylla-DB. It's designed to be used with the NPM package `@kastelapp/scylla`, but you can use it independently if you wish.

## Features

- Built with Rust and Tokio for high performance and reliability (maybe I suck at rust ;3)
- Supports a variety of commands including `select`, `insert`, `update`, `delete`, `connect`, and `raw`.
- Can be used with the NPM package `@kastelapp/scylla`, or independently.

## Installation

To install the server, checkout github releases. If you wish to build it yourself, you can do so by cloning the repository and running `cargo build --release`.

## Usage

After building the server, you can start it by running the resulting binary in your terminal. The server will start listening on port 8080 by default (adding --port will change it).

You can then send commands to the server using a TCP client. Here's an example of how to send a `select` command:

```js
{
  "command": "select",
  "table": "test",
  "keyspace": "test",
  "data": {
    "where": { "id": "1" },
    "columns": ["id", "name"],
    "limit": 1
  }
}
```

The server then will respond with a resonse like this:

```js
{
  "command": "select",
  "table": null,
  "keyspace": null,
  "data": {
    "result": [
      { "id": "1", "name": "test" }
    ],
    "error": null
  }
}
```

## Commands

The server supports the following commands:

- `connect`: Connect to a Scylla-DB instance.
- `select`: Select data from a table.
- `insert`: Insert data into a table.
- `update`: Update data in a table.
- `delete`: Delete data from a table.
- `raw`: Send a raw CQL query to the Scylla-DB instance (not recommended).

For more information on how to use these commands, see the section below.

Before we get started, this is a generalized payload, of an error the server may send if you are not authorized to do something:

```js
{
    "command": "error", // Not a real command, just used for errors
    "table": null, // always null
    "keyspace": null, // always null
    "data": {
        "result": null,
        "error": "You are not authorized to do this."
    }
}
```

<details>
<summary><strong>Connect</strong></summary>

The `connect` command is used to connect to a Scylla-DB instance. It is required to be the first command sent to the server.

```js
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

The server will respond with a JSON object containing the result of the command:

```js
{
    "command": "connect",
    "data": {
        "success": true,
        "error": null
    }
}
```

</details>

<details>
<summary><strong>Select</strong></summary>

The `select` command is used to select data from a table.

```js
{
    "command": "select",
    "table": "test",
    "keyspace": "test",
    "data": {
        "where": { "id": "1" },
        "columns": ["id", "name"],
        "limit": 1
    }
}
```

The server will respond with a JSON object containing the result of the command:

```js
{
    "command": "select",
    "table": null, // always null
    "keyspace": null, // always null
    "data": {
        "result": [
            {
                "id": "1",
                "name": "test"
            }
        ],
        "error": null
    }
}
```

</details>

<details>
<summary><strong>Insert</strong></summary>

The `insert` command is used to insert data into a table.

```js
{
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

The server will respond with a JSON object containing the result of the command:

```js
{
    "command": "insert",
    "data": {
        "success": true,
        "error": null
    }
}
```

</details>

<details>
<summary><strong>Update</strong></summary>

The `update` command is used to update data in a table.

```js
{
    "command": "update",
    "table": "test",
    "keyspace": "test",
    "data": {
        "where": { "id": "1" },
        "columns": {
            "name": "test2"
        }
    }
}
```

The server will respond with a JSON object containing the result of the command:

```js
{
    "command": "update",
    "data": {
        "success": true,
        "error": null
    }
}
```

</details>

<details>
<summary><strong>Delete</strong></summary>

The `delete` command is used to delete data from a table.

```js
{
    "command": "delete",
    "table": "test",
    "keyspace": "test",
    "data": {
        "where": { "id": "1" }
    }
}
```

The server will respond with a JSON object containing the result of the command:

```js
{
    "command": "delete",
    "data": {
        "success": true,
        "error": null
    }
}
```

</details>

<details>
<summary><strong>Raw</strong></summary>

The `raw` command is used to send a raw CQL query to the Scylla-DB instance, this is not recommended to be used.

```js
{
    "command": "raw",
    "keyspace": null, // should always be null
    "table": null, // should always be null
    "data": {
        "query": "SELECT * FROM test.test WHERE id = '1'"
    }
}
```

The server will respond with a JSON object containing the result of the command:

```js
{
    "command": "raw",
    "data": {
        "result": [
            {
                "id": "1",
                "name": "test"
            }
        ],
        "error": null
    }
}
```

</details>


## Contributing

We welcome contributions from the community. Please read our [contributing guidelines](Contributing.md) before getting started.

## License

This project is licensed under the MIT License. See the [LICENSE](License.md) file for details.

## Contact

If you have any questions or suggestions, please feel free to open an issue.

## Acknowledgements

This project uses the following third-party libraries:

- [Rust](https://www.rust-lang.org/)
- [Tokio](https://tokio.rs/)
- [Scylla Rust Driver](https://rust-driver.docs.scylladb.com)

