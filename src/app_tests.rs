use super::*;
use crate::{actions::ArcaneAction, arcane::ArcaneClient, config::ArcaneConfig};
use serde_json::json;

fn stub_service() -> ArcaneService {
    let client = ArcaneClient::new(&ArcaneConfig {
        api_url: "http://localhost:1".to_string(),
        api_key: "test-key".to_string(),
    })
    .expect("stub client should build");
    ArcaneService::new(client)
}

#[tokio::test]
async fn status_returns_local_ok() {
    let result = stub_service().status().await.expect("status should work");
    assert_eq!(result["status"], "ok");
    assert_eq!(result["upstream"], "arcane");
}

#[tokio::test]
async fn help_does_not_call_upstream() {
    let result = stub_service()
        .dispatch(&ArcaneAction {
            action: "help".into(),
            subaction: Some("container".into()),
            env_id: None,
            id: None,
            params: json!({}),
        })
        .await
        .expect("help should work without upstream");
    assert_eq!(result["tool"], "arcane");
    assert!(result["actions"].is_array());
}

#[tokio::test]
async fn destructive_actions_require_boolean_confirm() {
    let error = stub_service()
        .dispatch(&ArcaneAction {
            action: "project".into(),
            subaction: Some("down".into()),
            env_id: Some("env-1".into()),
            id: Some("stack".into()),
            params: json!({}),
        })
        .await
        .expect_err("destructive action should be blocked before network");
    assert!(error.to_string().contains("confirmation required"));
}

#[tokio::test]
async fn browse_rejects_path_traversal_before_network() {
    let error = stub_service()
        .dispatch(&ArcaneAction {
            action: "volume".into(),
            subaction: Some("browse".into()),
            env_id: Some("env-1".into()),
            id: Some("data".into()),
            params: json!({"path": "../secret"}),
        })
        .await
        .expect_err("bad path should be blocked");
    assert!(error.to_string().contains("relative path"));
}

#[test]
fn scaffold_intent_transformation_lives_in_service() {
    let result = stub_service()
        .scaffold_intent(ScaffoldIntent {
            display_name: "Lab Gateway".into(),
            crate_name: "lab-gateway-mcp".into(),
            binary_name: "lab-gateway".into(),
            server_category: "application platform".into(),
            env_prefix: "lab".into(),
            auth_kind: "api key".into(),
            host: "".into(),
            port: 3100,
            mcp_transport: "streamable-http".into(),
            mcp_primitives: "tools, resources, tools".into(),
            deployment: "containers".into(),
            plugins: "claude, gemini, none".into(),
            publish_mcp: true,
            crawl_urls: "https://docs.rustcane.test".into(),
            crawl_repos: "".into(),
            crawl_search_topics: "Lab API".into(),
        })
        .expect("valid scaffold intent should build");

    assert_eq!(result["kind"], "rustcane_scaffold_intent");
    assert_eq!(result["server_category"], "application-platform");
    assert_eq!(result["project"]["env_prefix"], "LAB");
    assert_eq!(
        result["required_surfaces"],
        json!(["api", "cli", "mcp", "web"])
    );
}
