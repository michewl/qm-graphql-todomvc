use async_graphql::futures_util::TryStreamExt;
use std::str::FromStr;

use cucumber::{gherkin::Step, when};

use crate::AppWorld;
use bson::doc;
use bson::oid::ObjectId;
use bson::DateTime;
use bson::Document;
use cucumber::given;
use cucumber::then;
use qgt_domain::db::collections::TAGS;

/// Creates a tag with requested name.
///
/// Stores the created tag id with `existing-tag-id` key in the world state as string.
#[given(expr = "a tag with name {string} exists")]
async fn given_tag(w: &mut AppWorld, name: String) -> anyhow::Result<()> {
    let result = w
        .app
        .db()
        .get()
        .collection::<Document>(TAGS)
        .insert_one(doc! { "name": &name, "created": DateTime::now() })
        .await?;

    w.state.insert(
        "existing-tag-id",
        serde_json::Value::from(
            result
                .inserted_id
                .as_object_id()
                .expect("the inserted id should be an object id")
                .to_hex(),
        ),
    );

    Ok(())
}

/// Creates a tag with given payload.
///
/// Stores the response as world data.
#[when(expr = "createTag is sent with body")]
async fn create(w: &mut AppWorld, step: &Step) -> anyhow::Result<()> {
    let docstring = step
        .docstring()
        .expect("a docstring payload should be provided");
    let payload = serde_json::Value::from_str(docstring.trim())
        .expect("docstring should be valid and parsable JSON");
    let response = w
        .graphql(
            String::from("createTag"),
            include_str!("../graphql/tag/create.graphql"),
        )
        .add_variable("input", payload)
        .execute()
        .await;

    w.save_last_response(response);
    Ok(())
}

/// Updates a tag with given payload.
///
/// Stores the response as world data.
#[when(expr = "updateTag is sent with body")]
async fn update(w: &mut AppWorld, step: &Step) -> anyhow::Result<()> {
    let docstring = step
        .docstring()
        .expect("a docstring payload should be provided");
    let mut payload = serde_json::Value::from_str(docstring.trim())
        .expect("docstring should be valid and parsable JSON");

    // Replace id with existing tag value from state
    payload
        .as_object_mut()
        .expect("payload should be an object")
        .insert(
            String::from("id"),
            w.state
                .get("existing-tag-id")
                .expect("world state should have 'existing-tag-id'")
                .clone(),
        );

    let response = w
        .graphql(
            String::from("updateTag"),
            include_str!("../graphql/tag/update.graphql"),
        )
        .add_variable("input", payload)
        .execute()
        .await;

    w.save_last_response(response);
    Ok(())
}

/// Removes tags based on their ids.
/// Retrieves the ids based on provided names.
///
/// Stores the response as world data.
#[when(expr = "removeTags is sent with ids for {string}")]
async fn delete(w: &mut AppWorld, names: String) -> anyhow::Result<()> {
    let names: Vec<&str> = names
        .split(",")
        .into_iter()
        .map(|name| name.trim())
        .collect();
    let result = w
        .app
        .db()
        .get()
        .collection::<Document>(TAGS)
        .find(doc! { "name": { "$in": &names } })
        .await?;
    let tags: Vec<Document> = result.try_collect().await?;
    assert_eq!(
        names.len(),
        tags.len(),
        "each name is expected to match a tag"
    );
    let ids: Vec<String> = tags
        .iter()
        .map(|tag| {
            tag.get_object_id("_id")
                .expect("the tag should have an object id")
                .to_hex()
        })
        .collect();

    let response = w
        .graphql(
            String::from("removeTagsById"),
            include_str!("../graphql/tag/remove.graphql"),
        )
        .add_variable("ids", ids.into())
        .execute()
        .await;

    w.save_last_response(response);
    Ok(())
}

#[then(expr = "the tag with name {string} is in the database")]
async fn is_in_database(w: &mut AppWorld, name: String) -> anyhow::Result<()> {
    let cnt = w
        .app
        .db()
        .get()
        .collection::<Document>(TAGS)
        .count_documents(doc! { "name": &name })
        .await?;

    assert_eq!(cnt, 1, "unexpected tag count '{cnt}' with name '{name}'");
    Ok(())
}

#[then(expr = "the tag with name {string} is not in the database")]
async fn is_not_in_database(w: &mut AppWorld, name: String) -> anyhow::Result<()> {
    let cnt = w
        .app
        .db()
        .get()
        .collection::<Document>(TAGS)
        .count_documents(doc! { "name": &name })
        .await?;

    assert_eq!(cnt, 0, "unexpected tag count '{cnt}' with name '{name}'");
    Ok(())
}

#[then(expr = "the tags with names {string} are not in the database")]
async fn are_not_in_database(w: &mut AppWorld, names: String) -> anyhow::Result<()> {
    let names: Vec<&str> = names
        .split(",")
        .into_iter()
        .map(|name| name.trim())
        .collect();
    let cnt = w
        .app
        .db()
        .get()
        .collection::<Document>(TAGS)
        .count_documents(doc! { "name": { "$in": &names } })
        .await?;

    assert_eq!(
        cnt, 0,
        "unexpected tag count '{cnt}' with names '{names:?}'"
    );
    Ok(())
}

#[then(expr = "the given tag has field {word} with string value {string}")]
async fn tag_by_id_has_value(w: &mut AppWorld, field: String, value: String) -> anyhow::Result<()> {
    let tag_id = w
        .state
        .get("existing-tag-id")
        .expect("world state should have 'existing-tag-id'");
    if let Some(tag) = w
        .app
        .db()
        .get()
        .collection::<Document>(TAGS)
        .find_one(doc! { "_id": ObjectId::from_str(tag_id.as_str().expect("the tag id should be a string")).expect("tag id should be parsable as an object id") })
        .await?
    {
        assert_eq!(tag.get_str(&field), Ok(value.as_str()));
    } else {
        assert!(false, "tag with id {tag_id} not found");
    }

    Ok(())
}
