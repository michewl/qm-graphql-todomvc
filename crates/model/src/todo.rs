use bson::oid::ObjectId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    completed: bool,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    order: u64,
    tags: Option<Vec<ObjectId>>,
    title: String,
}
