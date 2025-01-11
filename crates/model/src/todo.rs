use async_graphql::InputObject;
use async_graphql::MaybeUndefined;
use async_graphql::SimpleObject;
use bson::doc;
use bson::oid::ObjectId;
use bson::DateTime;
use qm::mongodb::options::UpdateModifications;
use serde::Deserialize;
use serde::Serialize;

/// Database representation of a todo.
#[derive(Debug, Deserialize, Serialize, SimpleObject)]
pub struct Todo {
    created: DateTime,
    completed: bool,
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    modified: Option<DateTime>,
    order: u64,
    #[graphql(skip)]
    tags: Option<Vec<ObjectId>>,
    title: String,
}

// TODO: add resolver for tags
// Requires restructuring because of cyclic dependencies
// Introduce `domain`, move `models` to module of `domain`
// Move schema and service to `domain`

/// The GraphQL input for creating a todo.
#[derive(Debug, Deserialize, InputObject, Serialize)]
pub struct CreateTodoInput {
    completed: bool,
    order: u64,
    tags: Option<Vec<ObjectId>>,
    title: String,
}

impl From<CreateTodoInput> for Todo {
    /// This allows for converting the GraphQL [CreateTodoInput] to a database [Todo].
    ///
    /// Will hard-coded set the [created](Todo) field to the current UTC date.
    fn from(input: CreateTodoInput) -> Self {
        Todo {
            created: DateTime::now(),
            completed: input.completed,
            id: None,
            modified: None,
            order: input.order,
            tags: input.tags,
            title: input.title,
        }
    }
}

/// The GraphQL input for updating a todo.
#[derive(Debug, Deserialize, InputObject, Serialize)]
pub struct UpdateTodoInput {
    completed: Option<bool>,
    pub id: ObjectId,
    order: Option<u32>,
    tags: MaybeUndefined<Vec<ObjectId>>,
    title: Option<String>,
}

impl Into<UpdateModifications> for &UpdateTodoInput {
    /// Converter to create a update document for MongoDB.
    ///
    /// Will hard-coded set the [modified](Todo) field to the current UTC date.
    fn into(self) -> UpdateModifications {
        // The document which must contain all $set operator updates
        let mut sets = doc! { "modified": DateTime::now() };
        // The document which must contain all $unset operator updates
        let mut unsets = doc! {};
        if let Some(completed) = &self.completed {
            sets.insert("completed", completed);
        }
        if let Some(order) = &self.order {
            sets.insert("order", order);
        }
        match &self.tags {
            MaybeUndefined::Undefined => {}
            MaybeUndefined::Null => {
                unsets.insert("$unset", doc! {"tags": ""});
            }
            MaybeUndefined::Value(tags) => {
                sets.insert("tags", tags);
            }
        }
        if let Some(title) = &self.title {
            sets.insert("title", title);
        }

        // The document with all update operations combined
        let mut doc = doc! { "$set": sets };
        doc.insert("$unset", unsets);

        UpdateModifications::Document(doc)
    }
}
