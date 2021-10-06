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

## Environment Variables

For running the collector, you will need to specify certain environment variables.
This can be done using the following files:

- `.env` - Environment variables shared by both development and production systems
- `.env.development` - Environment variables only on development system
- `.env.production` - Environment variables only on production system

|    Variable     |      Required       | Default Value | Description                                                                                                                                                                                                          |
| :-------------: | :-----------------: | :-----------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     C1_HOST     |         No          |   127.0.0.1   | IP address to use for running collector 1. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.             |
|     C1_PORT     |         No          |     3001      | Port number for collector 1.                                                                                                                                                                                         |
|     C2_HOST     |         No          |   127.0.0.1   | IP address to use for running collector 2. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.             |
|     C2_PORT     |         No          |     3002      | Port number for collector 2.                                                                                                                                                                                         |
|    USE_HTTPS    |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                   |
|    KEY_FILE     | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                             |
|    CERT_FILE    | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate. |
| C1_DATABASE_URL |       **Yes**       |               | [PostgreSQL Connection URI](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING) for accessing the collector 1 database. _See above for more details._                                       |
| C2_DATABASE_URL |       **Yes**       |               | [PostgreSQL Connection URI](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING) for accessing the collector 2 database. _See above for more details._                                       |
|   JWT_SECRET    |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                          |
