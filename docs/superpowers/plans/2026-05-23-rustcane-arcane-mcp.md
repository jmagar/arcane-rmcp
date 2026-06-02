# Rarcane Arcane MCP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `rarcane`, a Rust rmcp MCP server and CLI for Arcane Docker management.

**Architecture:** Start from `rarcane`, rename the scaffold to rarcane, and keep the upstream-client shape: `src/arcane.rs` owns HTTP calls, `src/app.rs` owns action validation/confirmation/dispatch, and `src/mcp/tools.rs` plus `src/cli.rs` only parse and delegate. Use a centralized action table so MCP schema, CLI validation, scopes, help, and dispatch stay aligned.

**Tech Stack:** Rust, rmcp, axum, reqwest, serde_json, clap, tokio, wiremock, cargo test/clippy/fmt.

---

## Source References

- Template rules: `AGENTS.md`, `README.md`
- TypeScript prior art: `/home/jmagar/workspace/arcane-mcp/src/mcp/tools/arcane.ts`, `src/mcp/tools/dispatch/*.ts`, `src/services/*.ts`, `README.md`
- Epic bead: `rarcane-r6l`
- Research findings: Arcane public API docs are instance-hosted; use TypeScript prior art for concrete endpoints. MCP 2025-06-18 requires human-in-loop tool safety and says elicitation must not request secrets.

## File Map

- Modify: `Cargo.toml`, `Cargo.lock` for package/bin/dependencies.
- Rename/replace: `src/rarcane.rs` -> `src/arcane.rs`.
- Modify: `src/config.rs`, `src/app.rs`, `src/actions.rs`, `src/cli.rs`, `src/main.rs`, `src/lib.rs`.
- Modify: `src/mcp/tools.rs`, `src/mcp/schemas.rs`, `src/mcp/rmcp_server.rs`, `src/mcp/prompts.rs`, `src/mcp.rs`.
- Modify: `tests/cli_parse.rs`, `tests/tool_dispatch.rs`; create `tests/arcane_client.rs`.
- Modify docs/config/plugin surfaces: `README.md`, `AGENTS.md`, `.env.rarcane`, `config.rarcane.toml`, `server.json`, `plugins/rarcane/**` renamed or rewritten to `plugins/rarcane/**`.

## Task 1: Baseline Scaffold Rename

**Files:**
- Modify: `Cargo.toml`, `src/main.rs`, `src/lib.rs`, `src/config.rs`, `src/mcp.rs`, docs/config/plugin files.
- Rename: `src/rarcane.rs` to `src/arcane.rs`; `plugins/rarcane/` to `plugins/rarcane/`.

- [ ] **Step 1: Write/adjust tests for names**

Run:
```bash
rg -n "rarcane|Arcane|rarcane|EXAMPLE" src tests README.md AGENTS.md plugins config* server.json .env.rarcane
```
Expected before implementation: matches exist. Expected after implementation: no template identifiers remain except intentional historical references in docs.

- [ ] **Step 2: Apply mechanical rename**

Use repository-aware replacement, preserving case:
```bash
mv src/rarcane.rs src/arcane.rs
mv plugins/rarcane plugins/rarcane
```
Then replace:
```text
rarcane -> rarcane
ArcaneClient -> ArcaneClient
ArcaneService -> ArcaneService
ArcaneConfig -> ArcaneConfig
ArcaneRmcpServer -> ArcaneRmcpServer
RARCANE_ -> RARCANE_
rarcane:read -> rarcane:read
rarcane:write -> rarcane:write
rarcane://schema/mcp-tool -> rarcane://schema/mcp-tool
```

- [ ] **Step 3: Verify compile catches only expected missing Arcane implementation**

Run:
```bash
cargo test --no-run
```
Expected: failures only from now-stale action/client code that later tasks replace, not unresolved template names.

## Task 2: Arcane Client and Endpoint Table

**Files:**
- Modify: `src/arcane.rs`, `src/app.rs`, `src/actions.rs`, `src/config.rs`.
- Test: `tests/arcane_client.rs`, `tests/tool_dispatch.rs`.

- [ ] **Step 1: Add failing client tests**

Create tests that assert:
```rust
// X-API-Key is sent.
// /api base paths are normalized without double slashes.
// envId/id path segments are percent-encoded.
// 401, 403, 404, 429, 400/422, 5xx map to user-readable errors.
// registry/environment credential params are not included in error text.
```

- [ ] **Step 2: Implement generic HTTP client**

Implement `ArcaneClient` with:
```rust
pub async fn request(
    &self,
    method: reqwest::Method,
    path: &str,
    query: Option<&serde_json::Value>,
    body: Option<&serde_json::Value>,
    timeout: Option<std::time::Duration>,
) -> anyhow::Result<serde_json::Value>
```
Use `X-API-Key`, default 30s timeout, 120s timeout for long-running action specs, and `serde_json::Value` to avoid over-typing every upstream response in the first cut.

- [ ] **Step 3: Implement centralized action metadata**

In `src/actions.rs`, define domains/subactions with method/path templates, required id/envId, destructive flag, timeout class, and scope. Include the TypeScript prior-art domains: `environment`, `project`, `container`, `image`, `network`, `volume`, `system`, `image-update`, `vulnerability`, `registry`, `gitops`, and `help`.

## Task 3: Service Dispatch, Safety Gate, and Pagination

**Files:**
- Modify: `src/app.rs`, `src/actions.rs`.
- Test: `tests/tool_dispatch.rs`.

- [ ] **Step 1: Add failing service tests**

Cover:
```text
environment:get accepts id and envId fallback
registry operations ignore envId
image-update:check accepts either id or params.imageRef, not neither
volume browse and gitops browse reject absolute paths and '..'
list responses support offset, limit 1..200, sort_order, query
destructive actions block unless params.confirm is boolean true or full bypass is enabled
```

- [ ] **Step 2: Implement `ArcaneService::dispatch`**

Service accepts:
```rust
pub struct ArcaneRequest {
    pub action: String,
    pub subaction: Option<String>,
    pub env_id: Option<String>,
    pub id: Option<String>,
    pub params: serde_json::Value,
}
```
It validates the action spec, applies destructive confirmation policy, builds endpoint paths, calls `ArcaneClient`, and returns a truncated JSON value/string suitable for both MCP and CLI.

## Task 4: MCP and CLI Parity

**Files:**
- Modify: `src/mcp/tools.rs`, `src/mcp/schemas.rs`, `src/mcp/rmcp_server.rs`, `src/cli.rs`.
- Test: `tests/tool_dispatch.rs`, `tests/cli_parse.rs`.

- [ ] **Step 1: Add failing MCP/CLI parity tests**

For each representative domain, assert MCP and CLI invoke the same service request:
```text
help
environment:list
container:list
project:down blocked then confirmed
registry:list
image-update:check with params.imageRef
volume:browse path validation
```

- [ ] **Step 2: Implement MCP schema**

Expose one MCP tool named `arcane` with:
```json
{"action":"container","subaction":"list","envId":"env-abc","id":"optional","params":{}}
```
Help remains public; read operations require `rarcane:read`; mutating/destructive operations require `rarcane:write`.

- [ ] **Step 3: Implement CLI**

Use a compact parity CLI:
```bash
rarcane call --action container --subaction list --env-id env-abc
rarcane call --action project --subaction down --env-id env-abc --id stack --confirm
rarcane help --domain container
```
CLI shims parse flags, add `confirm=true` only from `--confirm`, and delegate to `ArcaneService`.

## Task 5: Docs, Verification, Commit, and PR

**Files:**
- Modify: `README.md`, `AGENTS.md`, `.env.rarcane`, `config.rarcane.toml`, `server.json`, plugin docs/manifests.

- [ ] **Step 1: Document the delivered contract**

README must include config, transports, MCP examples, CLI examples, destructive gate semantics, TypeScript prior-art compatibility notes, and the reason REST/Web are deferred.

- [ ] **Step 2: Run full verification**

Run:
```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
cargo build --release
```

- [ ] **Step 3: Commit and publish if possible**

Run:
```bash
git status --short
git add .
git commit -m "feat: implement rarcane Arcane MCP server"
git remote -v
```
If a GitHub remote exists for rarcane, push and create a PR. If no remote exists, record that PR creation is blocked by missing remote.

## Self-Review

- Spec coverage: MCP + CLI parity, Arcane prior art, safety gates, no REST/Web, tests, docs, beads/worktree workflow are covered.
- Placeholder scan: no implementation step relies on TBD/TODO.
- Type consistency: request shape uses `ArcaneRequest`; client returns `serde_json::Value`; shims delegate to service.
