use crate::api::service::get_many_by_filter;
use crate::api::service::get_one_by_filter;
use crate::api::service::get_one_by_id;
use async_graphql::Context;
use async_graphql::Object;
use bson::doc;
use qgt_model::tag::Tag;
use qgt_model::todo::Todo;
use qgt_server::db::collections::TAGS;
use qgt_server::db::collections::TODOS;
use qm::mongodb::bson::oid::ObjectId;

#[derive(Default)]
pub(crate) struct DomainQueryRoot {}

#[Object]
impl DomainQueryRoot {
    /// Get all [Tags](Tag).
    async fn tags(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Tag>> {
        let app = ctx.data::<qgt_server::App>()?;
        get_many_by_filter(&app.db().get(), TAGS, doc! {})
            .await
            .map_err(|e| e.into())
    }

    /// Get a [Tag] by `id`.
    async fn tag_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> async_graphql::Result<Option<Tag>> {
        let app = ctx.data::<qgt_server::App>()?;
        get_one_by_id(&app.db().get(), TAGS, &id)
            .await
            .map_err(|e| e.into())
    }

    /// Get a [Tag] by `name``.
    async fn tag_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> async_graphql::Result<Option<Tag>> {
        let app = ctx.data::<qgt_server::App>()?;
        get_one_by_filter(&app.db().get(), TAGS, doc! { "name": name })
            .await
            .map_err(|e| e.into())
    }

    /// Get [Todos](Todo).
    async fn todos(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Todo>> {
        let app = ctx.data::<qgt_server::App>()?;
        get_many_by_filter(&app.db().get(), TODOS, doc! {})
            .await
            .map_err(|e| e.into())
    }

    /// Get a [Todo] by `id`.
    async fn todo_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> async_graphql::Result<Option<Todo>> {
        let app = ctx.data::<qgt_server::App>()?;
        get_one_by_id(&app.db().get(), TODOS, &id)
            .await
            .map_err(|e| e.into())
    }
}
