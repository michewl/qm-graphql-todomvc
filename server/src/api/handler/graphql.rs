use crate::api::schema::Schema;
use async_graphql_axum::GraphQLResponse;
use axum::Extension;

/// The handler for the GraphQL API.
///
/// The qm server crate provides already a [graphql_handler](qm::server::graphql_handler), but that
/// requires an authorization container, which does not exist for this example.
pub(crate) async fn graphql_handler(
    schema: Extension<Schema>,
    req: async_graphql_axum::GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
