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
- **gmp3.so** - GNU MP BigNum Library. _Install `gmp-devel` (Fedora) or `libgmp3-dev` (Debian)_

Finally, run `cargo build` from the root directory to compile the source code.
All of the additional frameworks listed will be installed automatically when you first build the project.
Be sure to compile the code using at least `Rust 1.55`. The code can be compiled using the `stable` channel.
If you are compiling for a production build, you should compile the code using `cargo build --release` instead.

Once the code is built, you can run the server using `cargo run` (development server) or `cargo run --release` (production server).
You can also optionally specify command-line arguments (Like `--port` or `--host`), which override any environment values specified in the `.env` files.
Use the `--help` flag to list all command-line options

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

<br/>

## Environment Variables

For running the API server, you will need to specify certain environment variables.
This can be done using the following files:

- `.env` - Environment variables shared by both development and production systems
- `.env.development` - Environment variables only on development system
- `.env.production` - Environment variables only on production system

Alternatively, these values can be passed in using command-line parameters when running the API server.
The command-line parameters override any values set in the `.env` files.

|       Variable       |       Command-line Flag        |      Required       | Default Value | Description                                                                                                                                                                                                          |
| :------------------: | :----------------------------: | :-----------------: | :-----------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|         HOST         |         `--host`, `-h`         |         No          |   127.0.0.1   | IP address to use for running the API server. If you use the `localhost` IP address, then you cannot connect to the API server from an external location. This must be an IP address and not a domain name.          |
|         PORT         |         `--port`, `-p`         |         No          |     3000      | Port number for the API server.                                                                                                                                                                                      |
|      USE_HTTPS       |         `--use-https`          |         No          |     false     | If true, then use HTTPS instead of HTTP for API requests. HTTPS encryption is performed using the OpenSSL library.                                                                                                   |
|       KEY_FILE       |          `--key-file`          | Only If `USE_HTTPS` |               | Private key file for OpenSSL. This should be an unencrypted `.pem` file.                                                                                                                                             |
|      CERT_FILE       |         `--cert-file`          | Only If `USE_HTTPS` |               | Certificate file for OpenSSL. This should be the unencrypted `.pem` file generated using the private key. For compatibility with some applications, this should be the full chain file and not just the certificate. |
|     DATABASE_URL     |        `--database-url`        |       **Yes**       |               | [PostgreSQL Connection URI](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING) for accessing the database. _See above for more details._                                                   |
|      JWT_SECRET      |      `--jwt-secret`, `-s`      |         No          |  _Hidden..._  | Secret value for signing the JSON Web Token                                                                                                                                                                          |
| RECAPTCHA_SECRET_KEY | `--recaptcha-secret-key`, `-r` |       **Yes**       |               | Secret key used by [Google reCAPTCHA](https://www.google.com/recaptcha/about/) for server-side validation.                                                                                                           |
|        C1_URL        |           `--c1-url`           |       **Yes**       |               | Base URL to access collector 1. It should **NOT** include the `/api/v1` suffix. If running on the same machine as the API server, this value can be set to `http://localhost:3001`.                                  |
|        C2_URL        |           `--c2-url`           |       **Yes**       |               | Base URL to access collector 2. It should **NOT** include the `/api/v1` suffix. If running on the same machine as the API server, this value can be set to `http://localhost:3002`.                                  |
|  NOTIFICATIONS_URL   |     `--notifications-url`      |       **Yes**       |               | Base URL to access collector 2. It should **NOT** include the `/api/v1` suffix. If running on the same machine as the API server, this value can be set to `http://localhost:3005`.                                  |

**Note:** Google reCAPTCHA provides a [fake testing key](https://developers.google.com/recaptcha/docs/faq#id-like-to-run-automated-tests-with-recaptcha.-what-should-i-do) if you do not want to enable this functionality on the website.

<br />

## Code Structure

- [`/src`](/server/src) - All source code files for the API server
- [`/migrations`](/server/migrations) - Database migrations for the PostgreSQL database

Main files in the `/src` directory:

- [`main.rs`](/server/src/main.rs) - Entry point for the server application
- [`lib.rs`](/server/src/lib.rs) - Entry point for the shared library
- [`config.rs`](/server/src/config.rs) - Handle environment variables
- [`schema.rs`](/server/src/schema.rs) - Auto-generated file by Diesel ORM that exports the database tables for Rust

Main folders in the `/src` directory:

- [`/auth`](/server/src/auth) - Structures and functions for authentication and authorization using JSON Web Tokens
- [`/db`](/server/src/db) - Structures and functions needed for running the database
- [`/errors`](/server/src/errors) - Structures and functions for error handling across the application
- [`/handlers`](/server/src/handlers) - All REST API handlers
- [`/models`](/server/src/models) - Rust `struct` definitions for tables in the database
- [`/notifications`](/server/src/notifications) - Structures and functions for pushing WebSocket notifications to the frontend
- [`/protocol`](/server/src/protocol) - Structures and functions specific to the electronic voting protocol
- [`/utils`](/server/src/utils) - Miscellaneous helper functions
- [`/views`](/server/src/views) - Shared structures that define the return types from the API handlers

**Note:** The API server compiles both a shared library and a main executable.
Using this structure enables other [binary utilities](https://doc.rust-lang.org/cargo/guide/project-layout.html) (`/src/bin` directory) to access the data types and API handlers.
Although this project doesn't have any utilities currently, this may be useful in the future.

### Linting and Formatting

Rust provides a custom code formatter named `rustfmt`, which is configured in the `rustfmt.toml` file.
When working with Rust, try to install a rustfmt plugin to automatically format your code when saving to ensure a consistent style in the codebase.
For example, [VSCode](https://code.visualstudio.com/) provides good Rust integration through the following plugins:

- [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [Rust Grammar](https://marketplace.visualstudio.com/items?itemName=siberianmh.rust-grammar)
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
- [vscode-rust](https://github.com/editor-rs/vscode-rust)

### FromRequest Trait

FromRequest is special trait used by [Actix Web](https://docs.rs/actix-web/3.3.2/actix_web/trait.FromRequest.html) that allows types to be referenced directly in the API handler.
Consider the API handler, which refers to the custom types `ClientToken` and `DbConnection`:

```rust
pub async fn register_for_election(
  token: ClientToken,
  path: web::Path<Uuid>,
  conn: DbConnection,
  jwt_key: web::Data<JWTSecret>,
) -> Result<HttpResponse, ServiceError> {
  // Code omitted
}
```

The following structures implement this trait in the server:

- `JWTToken`
- `RefreshToken`
- `DbConnection`

### Database Structures

The `DbConnection` structure is the main database connection object used by the server. This can store one of two types of connections:

- Unpooled Connection - Single connection to the PostgreSQL database
- Pooled Connection - Shared connection pool between threads in the API server (_used by the API server itself_)

The database system defines a series of Rust macros to automatically generate methods for working with Diesel.
These methods are based on Active Record from Ruby on Rails. The macros are defined in `associations.rs` and are as follows:

- `model_base!()` - Methods common to all database fields
- `belongs_to!()` - Foreign key that points to another field
- `has_one!()` - Another field has a foreign key that points to this field. This is a special case of a one-to-many relationship where only one foreign key points to this field.
- `has_zero_or_one!()` - Another field has a foreign key that points to this field. This is a special case of a one-to-many relationship where the foreign key may or may not exist.
- `has_many!()` - Another field has a foreign key that defines a one-to-many relationship to this field.
- `has_many!(through)` - Defines a many-to-many relationship between two fields

For inheritance in databases, subtypes.rs defines two additional macros:

- `parent_type!()` - Specify the parent field type
- `child_type!()` - Specify the child field type

Many-to-many relationships must implement the `ManyToManyConstructor` trait to work properly with these macros.
Also, for fields that are all key, the `model_base!()` macro has the `"no update"` flag that disables the update() method.
For more specific details on associations, see [Associations.md](Associations.md) for complete documentation.

The `sql_enum!()` macro defines a Rust enum that can be serialized and deserialized as a 32-bit integer in diesel.
This is used several times by database models.

Models in the API server must derive the following traits to work with the association macros:

- `Serialize` - Can serialize structure from SQL data
- `Queryable` - Can search for structure in the database
- `Insertable` - Can insert structure into the database
- `Identifiable` - Structure contains the primary key
- `AsChangeset` - Indicates a structure can updated the table, omit for fields that are all-key
- `Associations` - Structure stores one or more primary keys

Diesel defines the following attributes:

- `#[table_name]` - Usually not required, but necessary if the table name does not match the structure name
- `#[primary_key]` - Specify the Rust properties in the primary key
- `#[belongs_to]` - Specify that a structure has a foreign key that points to another table

By default, Diesel renames the structures to plural snake case when searching for the database name.
So "Election" will search for the table "elections".
Additionally, by default foreign keys expect snake case with an ID appended.
So a foreign key from "Election" to "User" will expect Election to have a field `user_id` that points to `id` in User.

### Error Handling

The `ServiceError` structure is a Rust enum that stores all errors in the system.
The `ErrorResponse` structure defines the error JSON format returned to the user.
ServiceError implements the [Actix `ResponseError`](https://docs.rs/actix-web/3.3.2/actix_web/trait.ResponseError.html) trait so it can be returned directly from API handlers.
Anytime an error is returned from an API handler, it is logged to the terminal.
On the production server, the `ErrorResponse` object does **NOT** return the `developer_notes` field, as it may contain sentivie information about the API server.
However, this field is still printed to the log file on the production server.

The error handling system also defines a few more structures used by the API server:

- `ClientRequestError` - Used when making API requests to other services, such as the collectors or notifications server
- `GlobalErrorCodes` - Integer error codes used by the frontend for more finely-grained error handling
- `ResourceType` and `NamedResourceType` - Used by errors when trying to find a resource that does not exist (or the user does not have permission to access)
- `ResourceAction` - Actions that can be performed on resources, used by errors that test if user has permission to perform an action on a resource

### API Handlers

General guidelines:

- Each API handler is a single [Rust async function](https://rust-lang.github.io/async-book/), and is defined in its own file
- All handler parameters must implement the `FromRequest` trait in [Actix Web](https://docs.rs/actix-web/3.3.2/actix_web/trait.FromRequest.html)
- Usually, API handlers return `HttpResponse` or empty data.
- If an error can occur, use the `Result<HttpResponse, ServiceError>`
- API handlers use the Permissions object to check for user permissions
- If JSON data is passed to the API server, use the Validator library to ensure the data is correct

[Actix Web](https://docs.rs/actix-web/3.3.2/actix_web/index.html) defines special types of `FromRequest` objects to assist with writing API handlers:

- `web::Json<>` - Parse the body of the request as JSON data
- `web::Path<>` - Read parameters from the path, such as string or integer identifiers
- `web::Query<>` - Parse parameters in the URL query string

General guidelines for JSON structures:

- Structures that parse data from the user should implement the Deserialize trait
- Structures that return data to the user should implement the Serialize trait
- All structure fields should be renamed to camelCase using #[serde(rename_all = "camelCase")]

All API routes are defined in `src/main.rs`.
This is handled by the Actix Web framework, which provides the following types of objects for defining routes:

- `.route()` - Specify a route using a string and a HTTP method
- `web::scope()` - Define a new subpath in the route
- `web::resource()` - Define a single path which supports multiple HTTP methods

Routes in the API server define parameters using brackets `{}`, such as `/api/users/{userId}/roles`

Most API handlers follow CRUD rules for naming and function (Create, Read, Update, Delete)
In general, HTTP methods work as follows:

- `GET` - Fetch a resource
- `POST` - Create a new resource
- `PATCH` - Modify various properties of a resource
- `PUT` - Replace a resource (Such as with file upload)
- `DELETE` - Delete a resource

### Miscellaneous Objects and Functions

- `new_safe_uuid_v4()` - Since UUIDs are represented as a base-64 string, it may be possible for a UUID to contain a curse word. This method filters the most common types of curse words and curse variants.
- `ConvertBigInt` - Trait that provides convenient functions to convert between `BigInt` and `BigDecimal` data types.
- `serialize_option_bigint` - Functions to serialize and deserialize `Option<BigInt>`, as `BigInt` doesn't implement the Serialize trait.
- `validate_password_complexity` - Test for the complexity of a password when changing passwords
