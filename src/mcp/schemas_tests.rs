use crate::actions::action_names;

use super::tool_definitions;

#[test]
fn schema_action_enum_comes_from_action_metadata() {
    let tools = tool_definitions();
    let enum_values = tools[0]["inputSchema"]["properties"]["action"]["enum"]
        .as_array()
        .expect("action enum should be an array")
        .iter()
        .map(|value| value.as_str().expect("action enum values are strings"))
        .collect::<Vec<_>>();

    assert_eq!(enum_values, action_names());
}

#[test]
fn schema_exposes_arcane_dispatch_fields() {
    let schema = &tool_definitions()[0]["inputSchema"]["properties"];
    assert_eq!(schema["subaction"]["type"], "string");
    assert_eq!(schema["envId"]["type"], "string");
    assert_eq!(schema["id"]["type"], "string");
    assert_eq!(schema["params"]["type"], "object");
}

#[test]
fn schema_disallows_unknown_top_level_properties() {
    let tools = tool_definitions();
    assert_eq!(tools[0]["inputSchema"]["additionalProperties"], false);
}
