use async_graphql::InputObject;
use async_graphql::SimpleObject;
use bson::doc;
use bson::oid::ObjectId;
use bson::DateTime;
use qm::mongodb::options::UpdateModifications;
use serde::Deserialize;
use serde::Serialize;

/// Database representation of a tag.
#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Tag {
    created: DateTime,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    modified: Option<DateTime>,
    name: String,
}

// TODO: add loader for count to show how often the tag is used

/// The GraphQL input for creating a tag.
#[derive(Debug, Deserialize, InputObject, Serialize)]
pub struct CreateTagInput {
    name: String,
}

impl From<CreateTagInput> for Tag {
    /// This allows for converting the GraphQL [CreateTagInput] to a database [Tag].
    ///
    /// Will hard-coded set the [`created`](Tag) field to the current UTC date.
    fn from(input: CreateTagInput) -> Self {
        Tag {
            created: DateTime::now(),
            id: None,
            modified: None,
            name: input.name,
        }
    }
}

/// The GraphQL input for updating a tag.
#[derive(Debug, Deserialize, InputObject, Serialize)]
pub struct UpdateTagInput {
    pub id: ObjectId,
    name: Option<String>,
}

impl Into<UpdateModifications> for &UpdateTagInput {
    /// Converter to create a update document for MongoDB.
    ///
    /// Will hard-coded set the [modified](Tag) field to the current UTC date.
    fn into(self) -> UpdateModifications {
        // The document which must contain all $set operator updates
        let mut sets = doc! { "modified": DateTime::now() };
        if let Some(name) = &self.name {
            sets.insert("name", name);
        }

        // The document with the final updates combined
        let doc = doc! { "$set": sets };

        UpdateModifications::Document(doc)
    }
}
