# rarcane — Agent instructions

## What this project is

A Rust template for building MCP servers with the rmcp crate. The stub binary is named `rarcane`. All `Arcane*` / `RARCANE_*` identifiers are renamed when the template is adapted for a real service.

## Key files

| File | Role |
|------|------|
| `src/rarcane.rs` | `ArcaneClient` — transport stub; replace with your HTTP/API client |
| `src/app.rs` | `ArcaneService` — ALL business logic lives here |
| `src/mcp/tools.rs` | MCP dispatch shim — parse args, call service, return Value |
| `src/mcp/schemas.rs` | Tool JSON schema and action list |
| `src/mcp/rmcp_server.rs` | `ServerHandler` impl: tools, resources, prompts, scope enforcement |
| `src/mcp/routes.rs` | Axum router (`/mcp`, `/health`, OAuth routes) |
| `src/mcp/prompts.rs` | MCP prompts |
| `src/mcp.rs` | `AppState`, `AuthPolicy`, auth layer builder |
| `src/config.rs` | Config structs and env loading |
| `src/cli.rs` | CLI dispatch shim |
| `src/main.rs` | Mode dispatch: HTTP / stdio / CLI |
| `src/lib.rs` | Public API and test helpers |
| `tests/` | Integration tests (`cli_parse.rs`, `tool_dispatch.rs`) |

## Architecture

```
ArcaneClient  (rarcane.rs)    ← network calls only
      ↓
ArcaneService (app.rs)        ← all business logic
      ↓
  ┌─────────────────────────────┐
  │  MCP shim (mcp/tools.rs)   │  JSON args → service → Value
  │  CLI shim (cli.rs)         │  CLI args  → service → print
  └─────────────────────────────┘
```

## Surface parity policy

Every business action MUST be exposed through both MCP and CLI. Treat MCP + CLI as the minimum supported surface for every scaffolded server.

REST API and Web UI are optional surfaces based on server type:

| Server type | Required surfaces | Examples |
|---|---|---|
| Upstream-client MCP server | MCP + CLI | `unrust`, `rustifi`, `rustify`, `rustscale`, `apprise` |
| Application/platform server | API + CLI + MCP + Web | `axon`, `lab`, `syslog` |

Do not add a REST/Web surface just to mirror an upstream HTTP API. For upstream-client servers, the value is the MCP tool surface plus an equivalent CLI for scripting, debugging, and parity tests.

Exception: `scaffold_intent` is MCP-only because it is specifically an MCP elicitation + plugin skill handoff workflow. There is no true CLI equivalent for exercising client-rendered elicitation and skill selection inside the user's agent/editor permission model.

## Invariant: zero logic in shims

`mcp/tools.rs` and `cli.rs` must not contain business logic. They parse inputs and delegate to `ArcaneService`. All computation, validation, and transformation belongs in `app.rs`.

## How to add an action

MCP + CLI steps are mandatory for every business action:

1. `src/rarcane.rs` — add transport method returning `Result<Value>`
2. `src/app.rs` — add service method delegating to client
3. `src/actions.rs` — add action metadata to `ACTION_SPECS`
4. `src/mcp/schemas.rs` — add new parameter schema entries to `tool_definitions()`
5. `src/mcp/tools.rs` — add match arm in `dispatch_example()`; update `HELP_TEXT`
6. `src/cli.rs` — add `Command` variant, parse arm, dispatch arm
7. `tests/tool_dispatch.rs` and CLI tests — add parity coverage

For application/platform servers only, also update:

8. REST API handlers/schemas for the action
9. `apps/web/lib/template.ts`, web forms, and API explorer examples

## Auth policy

| State | Condition | Behavior |
|-------|-----------|----------|
| `LoopbackDev` | `no_auth=true` or host starts with `127.` | No auth, no scope checks |
| `TrustedGatewayUnscoped` | `RARCANE_NOAUTH=true` behind an authz-enforcing gateway | No auth, no scope checks |
| `Mounted { auth_state: None }` | Default non-loopback | Static bearer token required |
| `Mounted { auth_state: Some(_) }` | `RARCANE_MCP_AUTH_MODE=oauth` | Google OAuth + RS256 JWT |

`help` action requires no scope. Read actions require `rarcane:read`; mutating actions require `rarcane:write`, which satisfies read.

## Environment variables

```
RARCANE_API_URL              Upstream service base URL
RARCANE_API_KEY              Upstream service API key
RARCANE_MCP_HOST             Bind host (default 0.0.0.0)
RARCANE_MCP_PORT             Bind port (default 3100)
RARCANE_MCP_NO_AUTH          Disable auth — loopback only (1/true/yes)
RARCANE_MCP_TOKEN            Static bearer token
RARCANE_MCP_ALLOWED_HOSTS    Comma-separated extra Host header values
RARCANE_MCP_ALLOWED_ORIGINS  Comma-separated extra CORS origins
RARCANE_MCP_PUBLIC_URL       Public URL for OAuth metadata
RARCANE_MCP_AUTH_MODE        bearer (default) or oauth
RARCANE_MCP_GOOGLE_CLIENT_ID     Google OAuth client ID (OAuth mode)
RARCANE_MCP_GOOGLE_CLIENT_SECRET  Google OAuth client secret (OAuth mode)
RARCANE_MCP_AUTH_ADMIN_EMAIL  OAuth admin email (OAuth mode)
RUST_LOG                     Log filter (e.g. info,rmcp=warn)
```

## Transports

- `rarcane serve` (or no args) — Streamable HTTP on `RARCANE_MCP_PORT` (default 3100)
- `rarcane mcp` — stdio transport for child-process MCP clients
- `rarcane greet / echo / status` — direct CLI

## MCP tool actions

Single tool `rarcane`, dispatched by `action` parameter:

| Action | Scope | Description |
|--------|-------|-------------|
| `greet` | `rarcane:read` | Greeting; optional `name` string |
| `echo` | `rarcane:read` | Echo; required `message` string |
| `status` | `rarcane:read` | Server status |
| `elicit_name` | `rarcane:read` | Elicitation demo — asks user for name mid-call |
| `scaffold_intent` | `rarcane:read` | Elicitation setup wizard — returns JSON for the scaffold-project skill |
| `help` | none (public) | Full action reference |

## MCP features implemented

- **Tools** — `rarcane` tool with action dispatch
- **Resources** — `rarcane://schema/mcp-tool` (JSON schema for the tool)
- **Prompts** — `quick_start` prompt
- **Elicitation** — `elicit_name` and `scaffold_intent` actions use `peer.elicit::<...>(...)` (spec 2025-06-18)
- **Scaffold handoff** — `scaffold_intent` returns JSON only; the `scaffold-project` plugin skill turns it into an approval-first plan

## Plugin versioning

Plugin manifests (`.claude-plugin/plugin.json`, `.codex-plugin/plugin.json`, `gemini-extension.json`) do **not** contain a `version` field. The marketplace derives version from the git commit SHA — an explicit version causes every push to create a duplicate entry. Never add `version` to a plugin manifest.

## Build and test

```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt
```

## Test helpers

`rarcane::testing::loopback_state()` builds `AppState` with no auth — use in all integration tests. `bearer_state(token)` builds a bearer-only state.

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:ca08a54f -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

## Session Completion

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd dolt push
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
<!-- END BEADS INTEGRATION -->

## Plugin setup hooks

Plugin setup is owned by the binary. `plugins/rarcane/hooks/hooks.json` calls `${CLAUDE_PLUGIN_ROOT}/bin/rarcane setup plugin-hook` directly (no shell wrapper). The binary's `apply_plugin_options()` (`src/cli/setup.rs`), hoisted in `run_cli` before `Config::load()` (rarcane is template-style — `setup_check` validates the pre-loaded `&Config`), maps `CLAUDE_PLUGIN_OPTION_*` values to the binary's `RARCANE_*` env vars; `install_self()` self-installs the binary into `~/.local/bin`.

`rarcane setup check` is read-only, `rarcane setup repair` is idempotent, and `rarcane setup plugin-hook --no-repair` is audit mode. Do not add Docker Compose, systemd, or service bootstrap logic into the hook path. Use `scripts/check-plugin-hook-contract.py` to audit this pattern across the Rust servers.
