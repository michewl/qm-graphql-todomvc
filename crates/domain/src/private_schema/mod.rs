use crate::app::App;
use crate::schema::mutation::DomainMutationRoot;
use crate::schema::query::DomainQueryRoot;
use async_graphql::EmptySubscription;
use async_graphql::MergedObject;
use mutation::PrivateDomainMutationRoot;
use query::PrivateDomainQueryRoot;

mod mutation;
mod query;

/// The base private schema type for the application.
pub type PrivateSchema =
    async_graphql::Schema<PrivateQueryRoot, PrivateMutationRoot, EmptySubscription>;

/// The global query root, which combines alls sub-schemas.
#[derive(Default, MergedObject)]
pub struct PrivateQueryRoot(DomainQueryRoot, PrivateDomainQueryRoot);

/// The global mutation root, which combines alls sub-schemas.
#[derive(Default, MergedObject)]
pub struct PrivateMutationRoot(DomainMutationRoot, PrivateDomainMutationRoot);

/// The schema builder for the GraphQL Schema.
#[derive(Default)]
pub struct SchemaBuilder {}

impl SchemaBuilder {
    pub fn build(self, app: App) -> PrivateSchema {
        async_graphql::Schema::build(
            PrivateQueryRoot::default(),
            PrivateMutationRoot::default(),
            EmptySubscription,
        )
        .data(app.clone())
        .finish()
    }
}
