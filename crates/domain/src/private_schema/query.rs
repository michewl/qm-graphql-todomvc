use async_graphql::MergedObject;

/// Additional queries for a secured API
#[derive(Default, MergedObject)]
pub(crate) struct PrivateDomainQueryRoot;
