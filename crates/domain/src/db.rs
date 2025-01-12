use anyhow::anyhow;
use bson::DateTime;
use qm::mongodb::bson::doc;
use qm::mongodb::bson::Document;
use qm::mongodb::DB;

pub(crate) mod collections {
    pub(crate) const TODOS: &str = "todos";
    pub(crate) const TAGS: &str = "tags";
}

/// Set up the database.
///
/// Ensures that all required collections exist, that necessary indexes are created and also creates
/// some test [Tags](qgt_domain::model::tag::Tag).
pub(crate) async fn setup_database(db: &DB) -> anyhow::Result<()> {
    init_collection(db, collections::TODOS, vec![]).await?;
    init_collection(db, collections::TAGS, vec![(doc! { "name": 1 }, true)]).await?;

    // Initialize example tags
    let docs = vec![
        doc! { "name": "private", "created": DateTime::now() },
        doc! { "name": "social:instagram", "created": DateTime::now() },
        doc! { "name": "social:tiktok", "created": DateTime::now() },
        doc! { "name": "social:youtube", "created": DateTime::now() },
        doc! { "name": "work", "created": DateTime::now() },
    ];

    for doc in docs {
        let col = db.get().collection::<Document>(collections::TAGS);
        let cnt = col
            .count_documents(
                doc! { "name": doc.get("name").expect("the tag name should be in the document") },
            )
            .await?;
        if cnt == 0 {
            db.get()
                .collection::<Document>(collections::TAGS)
                .insert_one(doc)
                .await?;
        } else {
            tracing::info!(
                "Not inserting tag '{}' since it already exists",
                doc.get("name")
                    .map(|v| v.as_str().unwrap_or("unknown"))
                    .expect("the tag name should be in the document")
            )
        }
    }

    Ok(())
}

/// Initialize a specific collection.
async fn init_collection(
    db: &DB,
    collection_name: &str,
    indexes: Vec<(Document, bool)>,
) -> anyhow::Result<()> {
    let collections: Vec<String> = db
        .collections()
        .await
        .iter()
        .map(|c| c.to_string())
        .collect();
    match db
        .ensure_collection_with_indexes(&collections, collection_name, indexes)
        .await
    {
        Ok(created) => {
            if !created {
                tracing::info!("Collection and indexes not created for '{collection_name}'.")
            }
            Ok(())
        }
        Err(err) => Err(anyhow!(
            "Collection initialization for '{collection_name}' failed:\n{err}"
        )),
    }
}
