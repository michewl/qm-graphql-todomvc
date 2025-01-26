use crate::common::AppWorld;
use crate::common::CustomBool;
use async_graphql::futures_util::TryStreamExt;
use bson::doc;
use bson::oid::ObjectId;
use bson::DateTime;
use bson::Document;
use cucumber::gherkin::Step;
use cucumber::given;
use cucumber::then;
use cucumber::when;
use qgt_domain::db::collections::TODOS;
use std::str::FromStr;

/// Creates a todo with requested title.
///
/// Stores the created todo id with `existing-todo-id` key in the world state as string.
#[given(expr = "a todo with title {string} exists")]
async fn given_todo(w: &mut AppWorld, title: String) -> anyhow::Result<()> {
    let result = w
        .app
        .db()
        .get()
        .collection::<Document>(TODOS)
        .insert_one(
            doc! { "completed": false, "created": DateTime::now(), "order": 1, "title": &title },
        )
        .await?;

    w.state.insert(
        "existing-todo-id",
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

/// Creates a todo with given payload.
///
/// Stores the response as world data.
#[when(expr = "createTodo is sent with body")]
async fn create(w: &mut AppWorld, step: &Step) -> anyhow::Result<()> {
    let docstring = step
        .docstring()
        .expect("a docstring payload should be provided");
    let payload = serde_json::Value::from_str(docstring.trim())
        .expect("docstring should be valid and parsable JSON");
    let response = w
        .graphql(
            String::from("createTodo"),
            include_str!("../graphql/todo/create.graphql"),
        )
        .add_variable("input", payload)
        .execute()
        .await;

    w.save_last_response(response);
    Ok(())
}

// Updates a todo with given payload.
///
/// Stores the response as world data.
#[when(expr = "updateTodo is sent with body")]
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
                .get("existing-todo-id")
                .expect("world state should have 'existing-todo-id'")
                .clone(),
        );

    let response = w
        .graphql(
            String::from("updateTodo"),
            include_str!("../graphql/todo/update.graphql"),
        )
        .add_variable("input", payload)
        .execute()
        .await;

    w.save_last_response(response);
    Ok(())
}

/// Removes todos based on their ids.
/// Retrieves the ids based on provided titles.
///
/// Stores the response as world data.
#[when(expr = "removeTodos is sent with ids for {string}")]
async fn delete(w: &mut AppWorld, titles: String) -> anyhow::Result<()> {
    let titles: Vec<&str> = titles
        .split(",")
        .into_iter()
        .map(|title| title.trim())
        .collect();
    let result = w
        .app
        .db()
        .get()
        .collection::<Document>(TODOS)
        .find(doc! { "title": { "$in": &titles } })
        .await?;
    let todos: Vec<Document> = result.try_collect().await?;
    assert_eq!(
        titles.len(),
        todos.len(),
        "each title is expected to match a todo"
    );
    let ids: Vec<String> = todos
        .iter()
        .map(|todo| {
            todo.get_object_id("_id")
                .expect("the todo should have an object id")
                .to_hex()
        })
        .collect();

    let response = w
        .graphql(
            String::from("removeTodosById"),
            include_str!("../graphql/todo/remove.graphql"),
        )
        .add_variable("ids", ids.into())
        .execute()
        .await;

    w.save_last_response(response);
    Ok(())
}

#[then(expr = "the todo with title {string} is in the database")]
async fn is_in_database(w: &mut AppWorld, title: String) -> anyhow::Result<()> {
    let cnt = w
        .app
        .db()
        .get()
        .collection::<Document>(TODOS)
        .count_documents(doc! { "title": &title })
        .await?;

    assert_eq!(cnt, 1, "unexpected todo count '{cnt}' with title '{title}'");
    Ok(())
}

#[then(expr = "the todos with titles {string} are not in the database")]
async fn are_not_in_database(w: &mut AppWorld, titles: String) -> anyhow::Result<()> {
    let titles: Vec<&str> = titles
        .split(",")
        .into_iter()
        .map(|title| title.trim())
        .collect();
    let cnt = w
        .app
        .db()
        .get()
        .collection::<Document>(TODOS)
        .count_documents(doc! { "title": { "$in": &titles } })
        .await?;

    assert_eq!(
        cnt, 0,
        "unexpected todo count '{cnt}' with titles '{titles:?}'"
    );
    Ok(())
}

#[then(expr = "the given todo has field {word} with string value {string}")]
async fn todo_by_id_has_string_value(
    w: &mut AppWorld,
    field: String,
    value: String,
) -> anyhow::Result<()> {
    let todo_id = w
        .state
        .get("existing-todo-id")
        .expect("world state should have 'existing-todo-id'");
    if let Some(todo) = w
        .app
        .db()
        .get()
        .collection::<Document>(TODOS)
        .find_one(doc! { "_id": ObjectId::from_str(todo_id.as_str().expect("the todo id should be a string")).expect("todo id should be parsable as an object id") })
        .await?
    {
        assert_eq!(todo.get_str(&field), Ok(value.as_str()));
    } else {
        assert!(false, "todo with id {todo_id} not found");
    }

    Ok(())
}

#[then(expr = "the given todo has field {word} with boolean value {bool}")]
async fn todo_by_id_has_boolean_value(
    w: &mut AppWorld,
    field: String,
    value: CustomBool,
) -> anyhow::Result<()> {
    let todo_id = w
        .state
        .get("existing-todo-id")
        .expect("world state should have 'existing-todo-id'");
    if let Some(todo) = w
        .app
        .db()
        .get()
        .collection::<Document>(TODOS)
        .find_one(doc! { "_id": ObjectId::from_str(todo_id.as_str().expect("the todo id should be a string")).expect("todo id should be parsable as an object id") })
        .await?
    {
        assert_eq!(todo.get_bool(&field), Ok(*value));
    } else {
        assert!(false, "todo with id {todo_id} not found");
    }

    Ok(())
}
