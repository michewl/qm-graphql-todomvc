//! Service with convenience functions for database access.

use async_graphql::futures_util::TryStreamExt;
use bson::doc;
use bson::oid::ObjectId;
use bson::Document;
use serde::de::DeserializeOwned;

/// Get one object of type `T` by id.
///
/// This is a convenience function to not require getting a filter [Document].\
/// The default filter [Document] used is `{"_id": id}`.
pub(crate) async fn get_one_by_id<T>(
    db: &qm::mongodb::Database,
    collection: &str,
    id: &ObjectId,
) -> anyhow::Result<Option<T>>
where
    T: DeserializeOwned + Send + Sync,
{
    get_one_by_filter(db, collection, doc! { "_id": id }).await
}

/// Get one object of type `T` with provided filter [Document].
pub(crate) async fn get_one_by_filter<T>(
    db: &qm::mongodb::Database,
    collection: &str,
    filter: Document,
) -> anyhow::Result<Option<T>>
where
    T: DeserializeOwned + Send + Sync,
{
    db.collection::<T>(collection)
        .find_one(filter)
        .await
        .map_err(|e| e.into())
}

/// Get many objects of type `T` with provided filter [Document].
pub(crate) async fn get_many_by_filter<T>(
    db: &qm::mongodb::Database,
    collection: &str,
    filter: Document,
) -> anyhow::Result<Vec<T>>
where
    T: DeserializeOwned + Send + Sync,
{
    let cursor = db.collection::<T>(collection).find(filter).await?;
    cursor.try_collect().await.map_err(|e| e.into())
}
