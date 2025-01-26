use std::collections::HashMap;

use bson::{doc, Document};
use cucumber::{then, when};

use crate::common::AppWorld;

#[when(expr = "test number {int} is ran")]
async fn run(w: &mut AppWorld, n: i32) -> anyhow::Result<()> {
    w.state.insert("number", serde_json::Value::from(n));
    Ok(())
}

#[then("all database collections are empty")]
async fn all_collections_empty(w: &mut AppWorld) -> anyhow::Result<()> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    let collections = w.app.db().collections().await;

    for collection in collections.iter() {
        let cnt = w
            .app
            .db()
            .get()
            .collection::<Document>(collection)
            .count_documents(doc! {})
            .await
            .expect("documents should be countable");
        counts.insert(collection.to_string(), cnt);
    }
    tracing::info!(
        "Test number {} has the following collection counts: {counts:?}",
        w.state
            .get("number")
            .expect("a state variable 'number' should have been set")
    );

    assert!(counts.into_values().all(|v| v == 0));
    Ok(())
}
