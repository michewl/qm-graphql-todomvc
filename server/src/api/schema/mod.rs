use async_graphql::EmptySubscription;
use async_graphql::MergedObject;
use mutation::DomainMutationRoot;
use qgt_server::App;
use query::DomainQueryRoot;

mod mutation;
mod query;

/// The base schema type for the application.
pub(crate) type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// The global query root, which combines alls sub-schemas.
#[derive(Default, MergedObject)]
pub struct QueryRoot(DomainQueryRoot);

/// The global mutation root, which combines alls sub-schemas.
#[derive(Default, MergedObject)]
pub struct MutationRoot(DomainMutationRoot);

/// The schema builder for the GraphQL Schema.
#[derive(Default)]
pub struct SchemaBuilder {}

impl SchemaBuilder {
    pub fn build(self, app: App) -> Schema {
        async_graphql::Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription,
        )
        .data(app.clone())
        .finish()
    }
}
