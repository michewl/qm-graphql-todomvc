use async_graphql::ServerError;
use async_graphql::{Request, Response, Variables};
use cucumber::Parameter;
use cucumber::World;
use derive_more::derive::Deref;
use derive_more::derive::FromStr;
use qgt_domain::schema::{Schema, SchemaBuilder};
use std::{collections::HashMap, fmt::Debug};

#[derive(World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub app: qgt_domain::app::App,
    pub schema: Schema,
    pub state: HashMap<&'static str, serde_json::Value>, // TODO: change to only be serde_json::Value
    pub last_query_operation: String,
    pub last_response: Response,
    last_response_data: serde_json::Value,
    last_response_json: String,
}

impl Debug for AppWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "AppWorld {{")?;
            writeln!(f, "    schema: {:#?},", self.schema.sdl())?;
            writeln!(f, "    state: {:#?}", self.state)?;
            writeln!(
                f,
                "    last_query_operation: {:#?}",
                self.last_query_operation
            )?;
            writeln!(f, "    last_response: {:#?}", self.last_response)?;
            writeln!(f, "    last_response_data: {:#?}", self.last_response_data)?;
            writeln!(f, "    last_response_json: {:#?}", self.last_response_json)?;
            write!(f, "}}")
        } else {
            write!(
                f,
                "AppWorld {{ schema: {:?}, state: {:?}, last_query_operation: {:?}, last_response: {:?}, last_response_data: {:?}, last_response_json: {:?} }}",
                self.schema.sdl(),
                self.state,
                self.last_query_operation,
                self.last_response,
                self.last_response_data,
                self.last_response_json
            )
        }
    }
}

impl AppWorld {
    pub async fn new() -> Self {
        let app = qgt_domain::app::App::new()
            .await
            .expect("the app should be constructed");
        let schema = SchemaBuilder::default().build(app.clone());

        Self {
            app,
            schema,
            state: HashMap::new(),
            last_query_operation: String::default(),
            last_response: async_graphql::Response::default(),
            last_response_data: serde_json::Value::Null,
            last_response_json: String::default(),
        }
    }

    /// Get a [GraphQLQueryBuilder].
    pub fn graphql(
        &mut self,
        query_operation: String,
        query: &'static str,
    ) -> GraphQLQueryBuilder<'_, 'static> {
        self.last_query_operation = query_operation;
        GraphQLQueryBuilder::new(&self.schema, query)
    }

    /// Store the response.
    ///
    /// Does also extract the response data and operation name.
    pub fn save_last_response(&mut self, response: Response) {
        self.last_response_data = response
            .data
            .clone()
            .into_json()
            .ok()
            .unwrap_or(serde_json::Value::Null);
        self.last_response_json =
            serde_json::to_string(&response).expect("response json serialization should work");
        self.last_response = response;
    }

    /// Get the last response data.
    ///
    /// This function will strip the root with the operation from the data and return only the
    /// actual data.
    ///
    /// If nothing was found, will return [serde_json::Value::Null].
    pub fn get_last_response_data(&self) -> serde_json::Value {
        self.last_response_data
            .get(&self.last_query_operation)
            .unwrap_or(&serde_json::Value::Null)
            .clone()
    }

    /// Get the last response errors.
    pub fn get_last_response_errors(&self) -> &Vec<ServerError> {
        &self.last_response.errors
    }

    // /// Get the last response json.
    // pub fn get_last_response_json(&self) -> &String {
    //     &self.last_response_json
    // }
}

/// Builder for GraphQL requests.
pub struct GraphQLQueryBuilder<'s, 'o> {
    schema: &'s Schema,
    query: &'o str,
    variables: Option<serde_json::Value>,
}

impl<'s, 'o> GraphQLQueryBuilder<'s, 'o> {
    pub fn new(schema: &'s Schema, query: &'o str) -> Self {
        Self {
            schema,
            query,
            variables: None,
        }
    }

    // /// Set variables
    // ///
    // /// Will overwrite any value, if present.
    // pub fn variables(mut self, variables: serde_json::Value) -> Self {
    //     self.variables = Some(variables);
    //     self
    // }

    /// Add variable for the request.
    pub fn add_variable(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.variables
            .get_or_insert(serde_json::json!({}))
            .as_object_mut()
            .unwrap()
            .insert(key.into(), value);
        self
    }

    /// Execute the request against the provided schema.
    pub async fn execute(self) -> Response {
        let mut request = Request::new(self.query);
        if let Some(variables) = self.variables {
            request = request.variables(Variables::from_json(variables));
        }
        tracing::debug!("GraphQL execute request:\n{request:?}");
        let response = self.schema.execute(request).await;
        tracing::debug!("GraphQL execute response:\n{response:?}");
        response
    }
}

/// A custom parameter to support `bool`
#[derive(Debug, Deref, FromStr, Parameter)]
#[param(regex = r"true|false", name = "bool")]
pub struct CustomBool(bool);
