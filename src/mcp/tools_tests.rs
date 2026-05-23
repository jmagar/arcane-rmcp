use serde_json::json;

use crate::{mcp::execute_tool_without_peer_for_test, testing::loopback_state};

#[tokio::test]
async fn arcane_help_dispatches_without_peer() {
    let state = loopback_state();
    let value = execute_tool_without_peer_for_test(&state, "arcane", json!({"action": "help"}))
        .await
        .expect("help should dispatch");
    assert_eq!(value["tool"], "arcane");
}

#[tokio::test]
async fn unknown_tool_is_rejected() {
    let state = loopback_state();
    let error = execute_tool_without_peer_for_test(&state, "missing", json!({}))
        .await
        .expect_err("unknown tool should fail");
    assert!(error.to_string().contains("unknown tool"));
}
