# Electronic Voting Notification Server

Rust server to send websocket notifications to the frontend

<br/>

## Compiling and Running

You will need to install Rust by following the directions on the [main website](https://www.rust-lang.org/tools/install).
If you want to add the Rust utilities to your path, you will need to manually run `~/.cargo/env`,
or you can edit your `.bashrc` file to run this script automatically.

You will also need to install the following shared libraries:

- **ssl.so** - OpenSSL development library. _Install `openssl-devel` (Fedora) or `libssl-dev` (Debian)_

Finally, run `cargo build` from the root directory to compile the source code.
All of the additional frameworks listed will be installed automatically when you first build the project.
Be sure to compile the code using at least `Rust 1.55`. The code can be compiled using the `stable` channel.
If you are compiling for a production build, you should compile the code using `cargo build --release` instead.

Once the code is built, you can run the server using `cargo run` (development server) or `cargo run --release` (production server).
You can also optionally specify command-line arguments (Like `--port` or `--host`), which override any environment values specified in the `.env` files.
Use the `--help` flag to list all command-line options

<br/>

## Environment Variables

For running the collector, you will need to specify certain environment variables.
This can be done using the following files:

- `.env` - Environment variables shared by both development and production systems
- `.env.development` - Environment variables only on development system
- `.env.production` - Environment variables only on production system

Alternatively, these values can be passed in using command-line parameters when running the API server.
The command-line parameters override any values set in the `.env` files.

|  Variable  |  Command-line Flag   |      Required       | Default Value | Description                                                                                                                                                                                                          |
| :--------: | :------------------: | :-----------------: | :-----------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    HOST    |    `--host`, `-h`    |         No          |   127.0.0.1   | IP address to use for running the API server. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.          |
|    PORT    |    `--port`, `-p`    |         No          |     3000      | Port number for the API server.                                                                                                                                                                                      |
| USE_HTTPS  |    `--use-https`     |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                   |
|  KEY_FILE  |     `--key-file`     | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                             |
| CERT_FILE  |    `--cert-file`     | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate. |
| JWT_SECRET | `--jwt-secret`, `-s` |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                          |

<br />

## Code Structure

Main files in the `/src` directory:

- [`main.rs`](/src/main.rs) - Entry point for the notification server executable
- [`lib.rs`](/src/lib.rs) - Entry point for the shared library
- [`config.rs`](/src/config.rs) - Handle environment variables

Main folders in the `/src` directory:

- [`/auth`](/src/auth) - Structures and functions for authentication and authorization using JSON Web Tokens
- [`/errors`](/src/errors) - Structures and functions for error handling across the application
- [`/handlers`](/src/handlers) - All REST API handlers
- [`/notifications`](/src/notifications) - Structures and functions for pushing WebSocket notifications to the frontend

**Note:** The notification server compiles both a shared library and a main executable.
Using this structure enables other [binary utilities](https://doc.rust-lang.org/cargo/guide/project-layout.html) (`/src/bin` directory) to access the data types and API handlers.
Although this project doesn't have any utilities currently, this may be useful in the future.

Other important files:

- [`/server_types.rs`](/src/notifications/server_types.rs) - Data structures for notifications pushed from the API server
- [`/client_types.rs`](/src/notifications/server_types.rs) - Data structures for notifications pushed to the frontend
- [`/subscription_actor.rs`](/src/notifications/subscription_actor.rs) - [Actix Actor](https://actix.rs/book/actix/) for managing the list of all active subscriptions
- [`/websocket_actor.rs`](/src/notifications/websocket_actor.rs) - [Actix Actor](https://actix.rs/book/actix/) for managing a single websocket connection
- [`/internal_types.rs`](/src/notifications/internal_types.rs) - Data structures used for internal actor communication between the subscription actor and the websocket actor

All data types in `server_types.rs` must implement either the `GlobalEvent` or `ElectionEvent` trait.
These traits provides a generic way for the websocket handler to convert server notifications into data sent to the frontend.
The `into_output()` method defines the data structure that will be sent to the client.
Additionally, the `protected()` method can be overridden to only send the event to a given user ID.
The list of all events are defined in the `GlobalEvents` and `ElectionEvents` enums.

See the [Server README.md](../server/README.md) file for more details on authentication and error handling.
