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

|  Variable  |      Required       | Default Value | Description                                                                                                                                                                                                          |
| :--------: | :-----------------: | :-----------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    HOST    |         No          |   127.0.0.1   | IP address to use for running the API server. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.          |
|    PORT    |         No          |     3000      | Port number for the API server.                                                                                                                                                                                      |
| USE_HTTPS  |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                   |
|  KEY_FILE  | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                             |
| CERT_FILE  | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate. |
| JWT_SECRET |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                          |
