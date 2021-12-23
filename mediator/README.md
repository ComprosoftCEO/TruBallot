# Electronic Voting Collector Mediator

Rust CRUD API server that manages websocket communication between the collectors

<br/>

## Compiling and Running

Please see the [server README.md](../server/README.md) for instructions on compiling the code and configuring the database.

<br/>

## Database

See the [Server README.md](../server/README.md) file for details on configuring the PostgreSQL database.

<br/>

## Environment Variables

For running the mediator, you will need to specify certain environment variables.
This can be done using the following files:

- `.env` - Environment variables shared by both development and production systems
- `.env.development` - Environment variables only on development system
- `.env.production` - Environment variables only on production system

Alternatively, these values can be passed in using command-line parameters when running the API server.
The command-line parameters override any values set in the `.env` files.

|     Variable      |   Command-line Flag   |      Required       | Default Value | Description                                                                                                                                                                                                           |
| :---------------: | :-------------------: | :-----------------: | :-----------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|       HOST        |    `--host`, `-h`     |         No          |   127.0.0.1   | IP address to use for running the mediator. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.             |
|       PORT        |    `--port`, `-p`     |         No          |     3004      | Port number for the mediator.                                                                                                                                                                                         |
|     USE_HTTPS     |     `--use-https`     |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                    |
|     KEY_FILE      |     `--key-file`      | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                              |
|     CERT_FILE     |     `--cert-file`     | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate.  |
|    JWT_SECRET     | `--jwt-secret`, `-s`  |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                           |
| NOTIFICATIONS_URL | `--notifications-url` |       **Yes**       |               | Base URL to access the notification server. It should **NOT** include the `/api/v1` suffix. If running on the same machine as the API server with default settings, this value can be set to `http://localhost:3005`. |

<br />

## Code Structure

- [`/src`](/mediator/src) - All source code files for the mediator
- [`/migrations`](/mediator/migrations) - Database migrations for the PostgreSQL database

Main files in the `/src` directory:

- [`main.rs`](/mediator/src/main.rs) - Entry point for the mediator application
- [`lib.rs`](/mediator/src/lib.rs) - Entry point for the shared library
- [`config.rs`](/mediator/src/config.rs) - Handle environment variables
- [`schema.rs`](/mediator/src/schema.rs) - Auto-generated file by Diesel ORM that exports the database tables for Rust

Main folders in the `/src` directory:

- [`/auth`](/mediator/src/auth) - Structures and functions for authentication and authorization using JSON Web Tokens
- [`/db`](/mediator/src/db) - Structures and functions needed for running the database
- [`/errors`](/mediator/src/errors) - Structures and functions for error handling across the application
- [`/handlers`](/mediator/src/handlers) - All REST API handlers
- [`/models`](/mediator/src/models) - Rust `struct` definitions for tables in the database
- [`/notifications`](/mediator/src/notifications) - Structures and functions for pushing WebSocket notifications to the frontend
- [`/protocol`](/mediator/src/protocol) - Structures and functions specific to the electronic voting protocol
- [`/utils`](/mediator/src/utils) - Miscellaneous helper functions
- [`/views`](/mediator/src/views) - Shared structures that define the return types from the API handlers

**Note:** The mediator compiles both a shared library and a main executable.
Using this structure enables other [binary utilities](https://doc.rust-lang.org/cargo/guide/project-layout.html) (`/src/bin` directory) to access the data types and API handlers.
Although this project doesn't have any utilities currently, this may be useful in the future.

Upon initialization, all collectors should call the `/api/v1/mediator/collectors` endpoint to register the collector with the mediator.
This handler requires all network information to access the collector (like IP address and port).
Each collector is assigned a unique UUID and can also be given a website-friendly name.
If a collector is ever restarted, calling this handler again will update the database to contain the most recent network and name information.
(_Just be sure to use the same UUID when calling the handler again, or it will register a new collector instead._)

In the current implementation, a collector can never be unregistered from the system once registered, or otherwise some elections could not be verified anymore.
At some point, there might be a way to "move" data from one collector to another, but this feature is not currently supported.

The [NGINX gateway](https://www.nginx.com/) has the ability to proxy directly to a collector given the UUID in the database.
This is handled by the `/api/v1/mediator/collectors/{id}/proxy` route, which returns the base API URL in the `x-collector-url` header.
NGINX calls this route by using the [ngx_http_auth_request_module module](http://nginx.org/en/docs/http/ngx_http_auth_request_module.html), then reads the returned header to complete the reverse proxy.
See the [frontend README.md](../frontend/README.md) file for details on properly configuring NGINX.

The mediator facilitates collector websocket communication using WebSockets for the verification protocol.
This is handled using the `MediatorActor` [Actix Actor](https://actix.rs/book/actix/).

See the [Server README.md](../server/README.md) file for more details on authentication, database structures, and error handling.
