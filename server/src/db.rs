use anyhow::anyhow;
use qm::mongodb::bson::doc;
use qm::mongodb::bson::Document;
use qm::mongodb::DB;

mod collections {
    pub(crate) const TODOS: &str = "todos";
    pub(crate) const TAGS: &str = "tags";
}

/// Set up the database.
pub(crate) async fn setup_database(db: &DB) -> anyhow::Result<()> {
    init_collection(db, collections::TODOS, vec![]).await?;
    init_collection(db, collections::TAGS, vec![(doc! { "name": 1 }, true)]).await?;

    // Initialize example tags
    let docs = vec![
        doc! { "name": "work" },
        doc! { "name": "private" },
        doc! { "name": "social:youtube" },
        doc! { "name": "social:tiktok" },
        doc! { "name": "social:instagram" },
    ];

    for doc in docs {
        let col = db.get().collection::<Document>(collections::TAGS);
        let cnt = col.count_documents(doc.clone()).await?;
        if cnt == 0 {
            db.get()
                .collection::<Document>(collections::TAGS)
                .insert_one(doc)
                .await?;
        } else {
            tracing::info!(
                "Not insertind tag '{}' since it already exists",
                doc.get("name")
                    .map(|v| v.as_str().unwrap_or("unknown"))
                    .unwrap()
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
