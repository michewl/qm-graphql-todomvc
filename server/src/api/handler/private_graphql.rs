use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use axum::Extension;
use qgt_domain::private_schema::PrivateSchema;

/// The handler for the secured GraphQL API.
///
/// The qm server crate provides already a [graphql_handler](qm::server::graphql_handler), but that
/// requires an [AuthContainer from the qm-role crate](https://docs.rs/qm-role/latest/qm_role/struct.AuthContainer.html),
/// which is not used for this example.
pub(crate) async fn private_graphql_handler(
    schema: Extension<PrivateSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
