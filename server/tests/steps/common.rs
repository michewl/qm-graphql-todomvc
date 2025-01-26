use crate::common::AppWorld;
use cucumber::then;
use jsonpath_rust::JsonPath;
use std::str::FromStr;

#[then(expr = "the response data JSON node {string} should have the value {string}")]
async fn json_node_value_eq(
    w: &mut AppWorld,
    json_path: String,
    value: String,
) -> anyhow::Result<()> {
    let path = JsonPath::from_str(&json_path)?;
    let response_data = w.get_last_response_data();
    let result = path.find(&response_data);

    assert!(
        !result.is_null(),
        "the node with path '{json_path}' was not found"
    );

    let arr = result.as_array().expect("the value should be an array");
    assert_eq!(
        arr.len(),
        1,
        "unexpected length '{}' of found values",
        arr.len()
    );

    assert_eq!(arr.first().unwrap().as_str().unwrap_or(""), &value,);
    Ok(())
}

#[then(expr = "the response data JSON node {string} should have a value")]
async fn json_node_value(w: &mut AppWorld, json_path: String) -> anyhow::Result<()> {
    let path = JsonPath::from_str(&json_path)?;
    let response_data = w.get_last_response_data();
    let result = path.find(&response_data);

    assert!(
        !result.is_null(),
        "the node with path '{json_path}' was not found"
    );

    let arr = result.as_array().expect("the value should be an array");
    assert_eq!(
        arr.len(),
        1,
        "unexpected length '{}' of found values",
        arr.len()
    );

    Ok(())
}

#[then("the response should have errors")]
async fn response_has_error(w: &mut AppWorld) -> anyhow::Result<()> {
    let errors = w.get_last_response_errors();
    assert!(!errors.is_empty());

    Ok(())
}

#[then("the response has no errors")]
async fn response_has_no_errors(w: &mut AppWorld) -> anyhow::Result<()> {
    assert!(
        w.get_last_response_errors().is_empty(),
        "errors found:\n{:?}",
        &w.get_last_response_errors()
    );
    Ok(())
}

#[then(expr = "a response error with message containing {string} exists")]
async fn error_containing(w: &mut AppWorld, value: String) -> anyhow::Result<()> {
    let errors = w.get_last_response_errors();
    assert!(errors.iter().any(|e| { e.message.contains(&value) }));

    Ok(())
}

#[then(expr = "the response data is integer value {int}")]
async fn response_data_value(w: &mut AppWorld, value: i64) -> anyhow::Result<()> {
    assert_eq!(
        w.get_last_response_data()
            .as_i64()
            .expect("the response data value should be an integer"),
        value
    );
    Ok(())
}
