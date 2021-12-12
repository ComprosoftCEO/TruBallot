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

|  Variable  |  Command-line Flag   |      Required       | Default Value | Description                                                                                                                                                                                                          |
| :--------: | :------------------: | :-----------------: | :-----------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    HOST    |    `--host`, `-h`    |         No          |   127.0.0.1   | IP address to use for running the API server. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.          |
|    PORT    |    `--port`, `-p`    |         No          |     3000      | Port number for the API server.                                                                                                                                                                                      |
| USE_HTTPS  |    `--use-https`     |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                   |
|  KEY_FILE  |     `--key-file`     | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                             |
| CERT_FILE  |    `--cert-file`     | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate. |
| JWT_SECRET | `--jwt-secret`, `-s` |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                          |
