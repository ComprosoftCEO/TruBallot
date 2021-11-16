# Electronic Voting Collector

Rust CRUD API server and daemon to run the vote collectors

<br/>

## Compiling and Running

Please see the [server README.md](../server/README.md) for instructions on compiling the code and configuring the database.
Each collector should run in its own separate database instance.
In the `.env` file, you will need to manually specify the `DATABASE_URL` for each collector when running migrations.

When running the code, you will need to specify either `1` or `2` as a parameter to indicate which collector to run.
You can also optionally specify the port and/or host using `--port` (or `-p`) and `--host` (or `-h`) command-line arguments.
These values override any environment values specified in the `.env` files.
Use the `--help` flag to list all command-line options.

```bash
# Start Collector 1 on port 3024
cargo run -- 1 -p 3024

# Start Collector 2 on a custom host with port 3456
cargo run -- 2 -h 192.168.1.234 --port 3456
```

Be sure to specify the `--release` flag for the production server:

```bash
cargo run --release -- 1
cargo run --release -- 2
```

<br/>

## Database

See the [Server README.md](../server/README.md) file for details on configuring the PostgreSQL database.

When running the database migrations, you will need to set `DATABASE_URL` (no prefix) environment variable individually to configure each database separately.
Alternatively, this can be passed in as a command-line argument to the `diesel` command.

<br/>

## Environment Variables

For running the collector, you will need to specify certain environment variables.
This can be done using the following files:

- `.env` - Environment variables shared by both development and production systems
- `.env.development` - Environment variables only on development system
- `.env.production` - Environment variables only on production system

Alternatively, these values can be passed in using command-line parameters when running the API server.
The command-line parameters override any values set in the `.env` files.

|       Variable       |  Command-line Flag   |      Required       | Default Value | Description                                                                                                                                                                                                          |
| :------------------: | :------------------: | :-----------------: | :-----------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|      Collector       |      _Argument_      |       **Yes**       |               | Index of the collector (`1` or `2`)                                                                                                                                                                                  |
|     C{1,2}\_HOST     |       `--host`       |         No          |   127.0.0.1   | IP address to use for running collector 1 or 2. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.        |
|     C{1,2}\_PORT     |       `--port`       |         No          |     3001      | Port number for collector 1 or 2.                                                                                                                                                                                    |
|  C{1,2}\_USE_HTTPS   |    `--use-https`     |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                   |
|   C{1,2}\_KEY_FILE   |     `--key-file`     | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                             |
|  C{1,2}\_CERT_FILE   |    `--cert-file`     | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate. |
| C{1,2}\_DATABASE_URL |   `--database-url`   |       **Yes**       |               | [PostgreSQL Connection URI](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING) for accessing the collector 1 or 2 database.                                                                |
|      JWT_SECRET      | `--jwt_secret`, `-s` |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                          |
|        C1_URL        |      `--c1-url`      |       **Yes**       |               | Base URL to access collector 1. It should **NOT** include the `/api/v1` suffix nor the `http://` or `https://` prefix. If running on the same machine as the API server, this value can be set to `localhost:3001`.  |
|        C2_URL        |      `--c2-url`      |       **Yes**       |               | Base URL to access collector 2. It should **NOT** include the `/api/v1` suffix nor the `http://` or `https://` prefix. If running on the same machine as the API server, this value can be set to `localhost:3002`.  |

Since the same executable is used for both collector 1 and 2, many of the environment variables need to be prefixed with either a `C1_` or a `C2_`.
The command-line flags do not require this prefix, as the collector index is known when running the program (It is a required argument).

<br />

## Code Structure

- [`/src`](/src) - All source code files for the API server
- [`/migrations`](/migrations) - Database migrations for the PostgreSQL database

Main files in the `/src` directory:

- [`main.rs`](/src/main.rs) - Entry point for the collector application
- [`lib.rs`](/src/lib.rs) - Entry point for the shared library
- [`config.rs`](/src/config.rs) - Handle environment variables
- [`schema.rs`](/src/schema.rs) - Auto-generated file by Diesel ORM that exports the database tables for Rust

Main folders in the `/src` directory:

- [`/auth`](/src/auth) - Structures and functions for authentication and authorization using JSON Web Tokens
- [`/db`](/src/db) - Structures and functions needed for running the database
- [`/errors`](/src/errors) - Structures and functions for error handling across the application
- [`/handlers`](/src/handlers) - All REST API handlers
- [`/models`](/src/models) - Rust `struct` definitions for tables in the database
- [`/protocol`](/src/protocol) - Structures and functions specific to the electronic voting protocol
- [`/utils`](/src/utils) - Miscellaneous helper functions
- [`/views`](/src/views) - Shared structures that define the return types from the API handlers

**Note:** The collector daemon compiles both a shared library and a main executable.
Using this structure enables other [binary utilities](https://doc.rust-lang.org/cargo/guide/project-layout.html) (`/src/bin` directory) to access the data types and API handlers.
Although this project doesn't have any utilities currently, this may be useful in the future.

The collectors communicate using WebSockets for the verification protocol.
Websocket communication is handled using the `VerificationWebsocket` [Actix Actor](https://actix.rs/book/actix/).

See the [Server README.md](../server/README.md) file for more details on authentication, database structures, and error handling.
