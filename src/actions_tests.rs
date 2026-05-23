use serde_json::json;

use super::*;

#[test]
fn action_metadata_covers_arcane_domains() {
    let names = action_names();
    for domain in [
        "help",
        "environment",
        "project",
        "container",
        "image",
        "network",
        "volume",
        "system",
        "image-update",
        "vulnerability",
        "registry",
        "gitops",
    ] {
        assert!(names.contains(&domain), "missing {domain}");
    }
    assert_eq!(required_scope_for_action("help"), None);
    assert_eq!(required_scope_for_action("container"), Some(WRITE_SCOPE));
    assert_eq!(required_scope_for_action("missing"), Some(DENY_SCOPE));
}

#[test]
fn mcp_args_parse_arcane_shape() {
    let action = ArcaneAction::from_mcp_args(&json!({
        "action": "container",
        "subaction": "list",
        "envId": "env-1",
        "params": {"limit": 5}
    }))
    .expect("MCP args should parse");
    assert_eq!(action.action, "container");
    assert_eq!(action.subaction.as_deref(), Some("list"));
    assert_eq!(action.env_id.as_deref(), Some("env-1"));
    assert_eq!(action.params["limit"], 5);
}

#[test]
fn missing_action_is_validation_error() {
    let error = ArcaneAction::from_mcp_args(&json!({})).unwrap_err();
    assert!(error.to_string().contains("action is required"));
    assert!(is_validation_error(&error));
}

#[test]
fn spec_lookup_rejects_unknown_subaction() {
    let error = spec_for("container", Some("logs")).unwrap_err();
    assert!(error.to_string().contains("unknown subaction"));
}

#[test]
fn relative_path_validation_blocks_traversal() {
    assert!(validate_relative_path(&json!({"path": "etc/config"}), "path").is_ok());
    assert!(validate_relative_path(&json!({"path": "/etc"}), "path").is_err());
    assert!(validate_relative_path(&json!({"path": "../secret"}), "path").is_err());
}

#[test]
fn scopes_satisfy_write_implies_read() {
    let write = vec![WRITE_SCOPE.to_string()];
    assert!(scopes_satisfy(&write, READ_SCOPE));
    assert!(scopes_satisfy(&write, WRITE_SCOPE));
    let read = vec![READ_SCOPE.to_string()];
    assert!(!scopes_satisfy(&read, WRITE_SCOPE));
}
