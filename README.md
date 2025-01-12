<h1 align="center">qm-graphql-todomvc</h1>

This project demonstrates how to build a GraphQL API using the [quick-microservice-rs][qm-github]
crate.

The API is loosely based on the [TodoMVC][todomvc-spec]
project and provides all necessary endpoints to be used as GraphQL backend for
[TodoMVC][todomvc-spec] apps.

## Notes

* It is possible to create, update and delete tags and todos.
* The tags have been added to also include some custom resolver for the GraphQL API and don't serve
  any other purpose.
* The todo ordering must be done manually and the same order number can be used multiple times.

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

> [!NOTE]
> The database uses the `SERVER_APP_NAME` value to identify the application in the server logs.
> See also [Connection Options](https://www.mongodb.com/docs/drivers/rust/current/fundamentals/connections/connection-options/#overview)

## Running the application

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

> ![WARN]
> If the rust logging level `debug` is active and the debug binary is used, a `schema.graphql` will
> be written in the directory from where the binary was executed. Any existing `schema.graphql`
> will be overwritten.

<!-- link references -->

[todomvc-spec]: https://github.com/tastejs/todomvc/blob/master/app-spec.md#functionality
[qm-github]: https://github.com/hd-gmbh-dev/quick-microservice-rs
[qm-crate]: https://crates.io/crates/qm
