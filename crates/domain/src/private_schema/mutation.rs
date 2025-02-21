use async_graphql::MergedObject;

/// Additional mutations for a secured API
#[derive(Default, MergedObject)]
pub(crate) struct PrivateDomainMutationRoot;
