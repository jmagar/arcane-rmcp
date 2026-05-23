//! MCP tool dispatch — thin shims only.
//!
//! Parse JSON args, call `ArcaneService`, return JSON. Business validation,
//! endpoint selection, and safety gates live in `app.rs`.

use rmcp::{service::Peer, RoleServer};
use serde_json::Value;

use crate::actions::{execute_service_action, ArcaneAction};
use crate::server::AppState;

pub(super) async fn execute_tool(
    state: &AppState,
    name: &str,
    args: Value,
    _peer: &Peer<RoleServer>,
) -> anyhow::Result<Value> {
    match name {
        "arcane" => dispatch_arcane(state, args).await,
        _ => Err(anyhow::anyhow!("unknown tool: {name}")),
    }
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub async fn execute_tool_without_peer_for_test(
    state: &AppState,
    name: &str,
    args: Value,
) -> anyhow::Result<Value> {
    match name {
        "arcane" => dispatch_arcane(state, args).await,
        _ => Err(anyhow::anyhow!("unknown tool: {name}")),
    }
}

async fn dispatch_arcane(state: &AppState, args: Value) -> anyhow::Result<Value> {
    let action = ArcaneAction::from_mcp_args(&args)?;
    execute_service_action(&state.service, &action).await
}

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tests;
