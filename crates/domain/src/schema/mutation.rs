use crate::db::collections::TAGS;
use crate::db::collections::TODOS;
use crate::model::tag::CreateTagInput;
use crate::model::tag::Tag;
use crate::model::tag::UpdateTagInput;
use crate::model::todo::CreateTodoInput;
use crate::model::todo::Todo;
use crate::model::todo::UpdateTodoInput;
use crate::service::get_one_by_id;
use async_graphql::Context;
use async_graphql::Object;
use bson::doc;
use qm::mongodb::bson::oid::ObjectId;

#[derive(Default)]
pub(crate) struct DomainMutationRoot {}

#[Object]
impl DomainMutationRoot {
    /// Create a new [Tag].
    async fn create_tag(
        &self,
        ctx: &Context<'_>,
        input: CreateTagInput,
    ) -> async_graphql::Result<Tag> {
        let app = ctx.data::<crate::app::App>()?;
        let result = app
            .db()
            .get()
            .collection::<Tag>(TAGS)
            .insert_one(Tag::from(input))
            .await?;

        get_one_by_id(
            &app.db().get(),
            TAGS,
            &result
                .inserted_id
                .as_object_id()
                .expect("inserted id should be an ObjectId"),
        )
        .await
        .map_err(|e| e.into())
        .map(|t| {
            t.unwrap_or_else(|| {
                panic!(
                    "the inserted tag should exist for id '{}'",
                    &result.inserted_id
                )
            })
        })
    }

    /// Update an existing [Tag].
    async fn update_tag(
        &self,
        ctx: &Context<'_>,
        input: UpdateTagInput,
    ) -> async_graphql::Result<Tag> {
        let app = ctx.data::<crate::app::App>()?;
        let result = app
            .db()
            .get()
            .collection::<Tag>(TAGS)
            .update_one(doc! { "_id": &input.id }, &input)
            .await?;

        if result.modified_count != 1 {
            tracing::warn!(
                "Unexpected modified count of '{}' for tag update with id '{}'",
                &result.modified_count,
                &input.id
            )
        }

        get_one_by_id(&app.db().get(), TAGS, &input.id)
            .await
            .map_err(|e| e.into())
            .map(|t| {
                t.unwrap_or_else(|| panic!("the updated tag should exist for id '{}'", &input.id))
            })
    }

    /// Delete multiple [Tags](Tag) by id.
    async fn remove_tags_by_id(
        &self,
        ctx: &Context<'_>,
        ids: Vec<ObjectId>,
    ) -> async_graphql::Result<usize> {
        let app = ctx.data::<crate::app::App>()?;
        let result = app
            .db()
            .get()
            .collection::<Tag>(TAGS)
            .delete_many(doc! { "_id": { "$in": &ids } })
            .await?;

        Ok(result
            .deleted_count
            .try_into()
            .expect("the deleted count should fit"))
    }

    /// Create a new [Todo].
    async fn create_todo(
        &self,
        ctx: &Context<'_>,
        input: CreateTodoInput,
    ) -> async_graphql::Result<Todo> {
        let app = ctx.data::<crate::app::App>()?;
        let result = app
            .db()
            .get()
            .collection::<Todo>(TODOS)
            .insert_one(Todo::from(input))
            .await?;

        get_one_by_id(
            &app.db().get(),
            TODOS,
            &result
                .inserted_id
                .as_object_id()
                .expect("inserted id should be an ObjectId"),
        )
        .await
        .map_err(|e| e.into())
        .map(|t| {
            t.unwrap_or_else(|| {
                panic!(
                    "the inserted todo should exist for id '{}'",
                    &result.inserted_id
                )
            })
        })
    }

    /// Update an existing [Todo].
    async fn update_todo(
        &self,
        ctx: &Context<'_>,
        input: UpdateTodoInput,
    ) -> async_graphql::Result<Todo> {
        let app = ctx.data::<crate::app::App>()?;
        let result = app
            .db()
            .get()
            .collection::<Todo>(TODOS)
            .update_one(doc! { "_id": &input.id }, &input)
            .await?;

        if result.modified_count != 1 {
            tracing::warn!(
                "Unexpected modified count of '{}' for todo update with id '{}'",
                &result.modified_count,
                &input.id
            )
        }

        get_one_by_id(&app.db().get(), TODOS, &input.id)
            .await
            .map_err(|e| e.into())
            .map(|t| {
                t.unwrap_or_else(|| panic!("the updated todo should exist for id '{}'", &input.id))
            })
    }

    /// Delete multiple [Todos](Todo) by id.
    async fn remove_todos_by_id(
        &self,
        ctx: &Context<'_>,
        ids: Vec<ObjectId>,
    ) -> async_graphql::Result<usize> {
        let app = ctx.data::<crate::app::App>()?;
        let result = app
            .db()
            .get()
            .collection::<Todo>(TODOS)
            .delete_many(doc! { "_id": { "$in": &ids } })
            .await?;

        Ok(result
            .deleted_count
            .try_into()
            .expect("the deleted count should fit"))
    }
}
