<h1 align="center">qm-graphql-todomvc</h1>

This project demonstrates how to build a GraphQL API using the [quick-microservice-rs][qm-github]
crate.

The API is loosely based on the [TodoMVC][todomvc-spec]
project and provides all necessary endpoints to be used as GraphQL backend for
[TodoMVC][todomvc-spec] apps.

## Notes

- It is possible to create, update and delete tags and to-dos.
- The tags have been added to also include some custom resolver for the GraphQL API and don't serve
  any other purpose.
- The to-do ordering must be done manually, and the same order number can be used multiple times.

> [!NOTE]
> Please be aware that while this example aims to provide some best practices, it also uses a lot of
> shortcuts for simplicity. E.g. it would be beneficial to use
> [`DataLoader`](https://async-graphql.github.io/async-graphql/en/dataloader.html),
> but these are not used in the example.

## Quick Microservice components

This project uses the following [QM crate][qm-crate] features.

### `server`

Used for the server configuration.\
The configuration must be done trough environment variables with the `SERVER_` prefix.

Available variables are: `SERVER_HOST`, `SERVER_PORT`, `SERVER_APP_NAME`

### `mongodb`

Available variables are: `MONGODB_HOST`, `MONGODB_PORT`, `MONGODB_DATABASE`, `MONGODB_USERNAME`,
`MONGODB_PASSWORD`, `MONGODB_ROOT_DATABASE`, `MONGODB_ROOT_USERNAME`, `MONGODB_ROOT_PASSWORD`,
`MONGODB_SHARDED`

The variables `MONGODB_USERNAME` and `MONGODB_PASSWORD` are required.

> [!NOTE]
> The database uses the `SERVER_APP_NAME` value to identify the application in the server logs.
> See also [Connection Options](https://www.mongodb.com/docs/drivers/rust/current/fundamentals/connections/connection-options/#overview)

## Running the application

The required infra structure can be started using the provided `docker-compose.yml`.
It contains a MongoDB database which will expose port `27017`.

> [!WARNING]
> Services in the `docker-compose.yml` are **not** configured for production use.

### Environment variables

To set the environment variables for the application, a `.env` file can be created.

Example content of `.env`

```shell
RUST_LOG=debug
SERVER_APP_NAME=qm-graphql-todomvc
MONGODB_DATABASE=qgt
MONGODB_USERNAME=qgt
MONGODB_PASSWORD=qm-graphql-todomvc
```

### Ways to run

To run the project locally, execute:

```shell
cargo run -p qgt-server
```

Or build the binary and run that directly.\
For the debug binary, run:

```shell
cargo build && ./target/debug/qgt-server
```

And for the release binary, run:

```shell
cargo build --release && ./target/release/qgt-server
```

> [!WARNING]
> If the rust logging level `debug` is active and the debug binary is used, a `schema.graphql` will
> be written in the directory from where the binary was executed. Any existing `schema.graphql`
> will be overwritten.

### Tests

#### Environment variables

When running tests, a `.env.test` can be provided to overwrite the `.env` variables.

> [!IMPORTANT]
> Variables in the `.env.test` file have priority and are not overwritten by `.env`. Only new
> variables are added from `.env`.

Example content of `.env.test`

```shell
RUST_LOG=warn,integration=debug,setup=debug
SERVER_APP_NAME=qm-graphql-todomvc-test
MONGODB_DATABASE=qgttest
MONGODB_USERNAME=qgttest
MONGODB_PASSWORD=qm-graphql-todomvc-test
```

The environment variables `TEST_SKIP_CLEANUP_BEFORE` and `TEST_SKIP_CLEANUP_AFTER` can be set to
`true`, if the test data cleanup should be skipped.\
Skipping test data cleanup should only be used for debugging purposes. Tests will interfere with
each other and cause inconsistent results.

Data cleanup "before" will be [executed before the first step in every scenario](https://cucumber-rs.github.io/cucumber/main/writing/hooks.html#before-hook).
Data cleanup "after" will be [executed after the last step in every scenario](https://cucumber-rs.github.io/cucumber/main/writing/hooks.html#after-hook).

The environment variable `TEST_EXECUTE_TAGS` allows setting a comma separated list of tags which
will be executed, instead of the default.

> [!WARNING]
> Tests create, update and delete entries. Do not use the production database.
> Configure a test database with the `MONGODB_DATABASE` environment variable.

For demonstration purposes, there are currently two tests available.
One is `setup`, which showcases some context manipulation and database access.
The other is the main test called `integration`, which actually performs tests on tags and to-dos

To run tests, execute:

```shell
cargo test --test integration
```

> [!NOTE]
> Due to use of a centralized database, tests can not be executed concurrently.

<!-- link references -->

[todomvc-spec]: https://github.com/tastejs/todomvc/blob/master/app-spec.md#functionality
[qm-github]: https://github.com/hd-gmbh-dev/quick-microservice-rs
[qm-crate]: https://crates.io/crates/qm
