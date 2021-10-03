# Electronic Voting Server

Rust CRUD API server to run the electronic voting system backend

<br/>

## Main Frameworks Used

- **[Rust](https://www.rust-lang.org/)** - Programming language
- **[Actix Web](https://actix.rs/)** - Rust web framework
- **[Diesel](http://diesel.rs/)** - Database ORM for Rust
- **[Serde](https://serde.rs/)** - Serialization framework
- **[Validator](https://github.com/Keats/validator)** - Validate user data in API handlers
- **[Log](https://docs.rs/log/0.4.14/log/)** - Log server information to the terminal

<br/>

## Compiling

You will need to install Rust by following the directions on the [main website](https://www.rust-lang.org/tools/install).
If you want to add the Rust utilities to your path, you will need to manually run `~/.cargo/env`,
or you can edit your `.bashrc` file to run this script automatically.

You will also need to install the following shared libraries:

- **libpq.so** - PostgreSQL development library. _Install `postgresql-devel` (Fedora) or `libpq-dev` (Debian)_
- **ssl.so** - OpenSSL development library. _Install `openssl-devel` (Fedora) or `libssl-dev` (Debian)_

Finally, run `cargo build` from the root directory to compile the source code.
All of the additional frameworks listed will be installed automatically when you first build the project.
Be sure to compile the code using at least `Rust 1.55`. The code can be compiled using the `stable` channel.
If you are compiling for a production build, you should compile the code using `cargo build --release` instead.

<br/>

## Database

### Setup and Configure

The API server runs using a [PostgreSQL](https://www.postgresql.org/) database instance.
Follow the directions from [PostgreSQL Downloads](https://www.postgresql.org/download/) to install the database on your target platform.
The code requires using `PostgreSQL 12` or above for the database to run properly.
You will also need to install the `postgresql-contrib` library for your platform to enable PostgreSQL extensions.
If you already have an instance of PostgreSQL 12 or later running, you can skip this step.

After the database has been installed, you should create a new database user specifically for the API server.
Connect to the database using `psql`:

```
sudo su - postgres
psql
```

Run the following SQL commands to create a new user to access the database.
For running migrations, the user should have permissions to create new databases (`CREATEDB`).
_Don't forget the semicolon at the end of the query._

```sql
CREATE USER user WITH PASSWORD 'password' CREATEDB;
```

After the new user has been created, you will need to modify `pg_hba.conf` to allow a username/password connection to the database.
On Ubuntu, this file is typically found at `/etc/postgresql/12/main/pg_hba.conf`. If needed, change the following lines:

```
# IPv4 local connections:
host    all             all             127.0.0.1/32            ident
# IPv6 local connections:
host    all             all             ::1/128                 ident
```

and replace `ident` with `md5`.

```
# IPv4 local connections:
host    all             all             127.0.0.1/32            md5
# IPv6 local connections:
host    all             all             ::1/128                 md5
```

When running a production server, you may need to increase the number of simultaneous database connections.
Modify `postgresql.conf` (once again, this file is typically found at `/etc/postgresql/12/main/`), and
set `max_connections` to `500`. You should also increase the size of `shared_buffers` to allocate more memory for caching data.

After these changes are made, you will need to restart the PostgreSQL service if you have not already done so.

### Database Connection

Connections to the database must be done using a [PostgreSQL Connection URI](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING).
The basic format is as follows:

```
postgres://username:password@host/database
```

For the API server to connect to the database, you will need to set the `DATABASE_URL` environment variable in one
of the environment variable files. _See below for more details._

### Migrations

All of the files in `migrations/` folder can create the working database from scratch.
Before running migrations, be sure you have the `postgresql-contrib` package installed on your platform for certain PostgreSQL extensions.

Start by installing `diesel-cli` using the following command:

```
cargo install diesel_cli --no-default-features --features postgres
```

This will install the `diesel` executable, which allows you to manage the database using migrations.
Diesel needs a valid connection to the database for this to work. There are 3 ways to specify the connection:

1. Manually set the `DATABASE_URL` environment variable
2. Use the `--database-url <DATABASE_URL>` flag
3. Set `DATABASE_URL` in the `.env` file (_This is the easiest method_)

Here are some of the commands you can use to manage the database:

- `diesel database setup` - Run all of the migrations to initialize the database schema
- `diesel database reset` - Drop the database and re-run all migrations (_Erases all data_)
- `diesel migration generate <Name>` - Create a new database migration
- `diesel migration run` - Run all pending migrations
- `diesel migration revert` - Revert the latest migration
- `diesel migration redo` - Undo and re-run the latest migration

All migrations are handled by SQL files. Generating a migration creates two files:

- `up.sql` - Run code for the database migration
- `down.sql` - Undo any changes made in `up.sql`

If you are initializing the database for the first time, you might also want to run the `init-database`
utility to load default data values and to generate the default administrator account. Like the API server,
you will need to specify the database connection in the `.env` file.

```
cargo run --bin init-database
```

<br/>

## Environment Variables

For running the API server, you will need to specify certain environment variables.
This can be done using the following files:

- `.env` - Environment variables shared by both development and production systems
- `.env.development` - Environment variables only on development system
- `.env.production` - Environment variables only on production system

|   Variable   |      Required       | Default Value | Description                                                                                                                                                                                                          |
| :----------: | :-----------------: | :-----------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     HOST     |         No          |   127.0.0.1   | IP address to use for running the API server. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.          |
|     PORT     |         No          |     3000      | Port number for the API server.                                                                                                                                                                                      |
|  USE_HTTPS   |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                   |
|   KEY_FILE   | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                             |
|  CERT_FILE   | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate. |
| DATABASE_URL |       **Yes**       |               | [PostgreSQL Connection URI](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING) for accessing the database. _See above for more details._                                                   |
|  JWT_SECRET  |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                          |
