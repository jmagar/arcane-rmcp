//! Tool JSON schemas for the MCP rarcane tool.
//!
//! This file defines the action list and input schema for the `arcane` tool.
//! MCP clients inspect this schema to know what arguments are valid.
//!
//! **Template**: rename `rarcane` to your tool name. Add/remove actions and
//! parameters to match your service. Use `"required": [...]` for mandatory args.

use std::sync::OnceLock;

use serde_json::{json, Value};

use crate::actions::action_names;

/// Cached JSON schema definitions (static data, built once at first call).
static TOOL_DEFINITIONS: OnceLock<Vec<Value>> = OnceLock::new();

/// Return the JSON schema definitions for all tools (cached after first call).
///
/// Returns a `Vec<Value>` where each item is a tool definition object matching
/// the MCP `Tool` schema: `{ name, description, inputSchema }`.
///
/// This is also used by the schema resource (`rarcane://schema/mcp-tool`).
pub(super) fn tool_definitions() -> &'static Vec<Value> {
    TOOL_DEFINITIONS.get_or_init(build_tool_definitions)
}

fn build_tool_definitions() -> Vec<Value> {
    vec![json!({
        "name": "arcane",
        "description": "Manage Arcane Docker resources. Use action=help for full documentation.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Resource family or help.",
                    "enum": action_names()
                },
                "subaction": {
                    "type": "string",
                    "description": "Operation to perform within the resource family."
                },
                "envId": {
                    "type": "string",
                    "description": "Target Arcane environment ID. Required for environment-scoped domains."
                },
                "id": {
                    "type": "string",
                    "description": "Resource ID for single-resource operations."
                },
                "params": {
                    "type": "object",
                    "description": "Action-specific parameters. Include {\"confirm\": true} for destructive operations.",
                    "additionalProperties": true
                }
            },
            "required": ["action"],
            "additionalProperties": false
        }
    })]
}

#[cfg(test)]
#[path = "schemas_tests.rs"]
mod tests;
