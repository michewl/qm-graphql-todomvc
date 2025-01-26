//! Runs some tests for the GraphQL API.
//!
//! Run with `cargo test --test integration`

use common::AppWorld;
use cucumber::World;
use tracing_subscriber::{layer::SubscriberExt, Layer};
use utils::has_tag_condition;

pub(crate) mod common;
mod steps;
mod utils;

#[tokio::main]
async fn main() {
    // This loads the environment variables from `.env.test` if it exists
    if let Err(e) = dotenv::from_filename(".env.test") {
        tracing::info!("The '.env.test' file could not be loaded.\n{}", &e);
    }
    // This loads the environment variables from `.env` if it exists
    // Does not overwrite already set variables
    if let Err(e) = dotenv::dotenv() {
        tracing::info!("The '.env' file could not be loaded.\n{}", &e);
    }

    // Run all relevant tests
    AppWorld::cucumber()
        .fail_on_skipped()
        .configure_and_init_tracing(
            tracing_subscriber::fmt::format::DefaultFields::new(),
            tracing_subscriber::fmt::format::Format::default(),
            |layer| {
                tracing_subscriber::registry()
                    .with(tracing_subscriber::EnvFilter::from_default_env().and_then(layer))
            },
        )
        .before(move |_feature, _rule, _scenario, w| {
            if std::env::var("TEST_SKIP_CLEANUP_BEFORE").as_deref() == Ok("true") {
                Box::pin(async move { () })
            } else {
                Box::pin(async move {
                    w.app
                        .db()
                        .cleanup()
                        .await
                        .expect("before hook app database cleanup should work");
                })
            }
        })
        .after(move |_feature, _rule, _scenario, _result, w| {
            if std::env::var("TEST_SKIP_CLEANUP_AFTER").as_deref() == Ok("true") {
                Box::pin(async move { () })
            } else {
                Box::pin(async move {
                    w.expect("world should exist")
                        .app
                        .db()
                        .cleanup()
                        .await
                        .expect("after hook app database cleanup should work");
                })
            }
        })
        .max_concurrent_scenarios(1)
        .filter_run("tests/features", |feature, _rule, scenario| {
            let tags: Vec<String> = feature
                .tags
                .iter()
                .chain(scenario.tags.iter())
                .cloned()
                .collect();
            match std::env::var("TEST_EXECUTE_TAGS") {
                Ok(env_tags) => has_tag_condition(
                    &tags,
                    &env_tags
                        .split(",")
                        .into_iter()
                        .map(|t| t.trim().to_string())
                        .collect::<Vec<String>>(),
                    true,
                ),
                Err(_) => has_tag_condition(
                    &tags,
                    &vec![String::from("smoketest"), String::from("setup")],
                    false,
                ),
            }
        })
        .await;
}
