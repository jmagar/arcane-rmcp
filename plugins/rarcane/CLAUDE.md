# plugins/rarcane — Claude Code instructions

## What this directory is

Multi-platform plugin package for the Arcane MCP server. Contains manifests for Claude Code, Codex, and Gemini CLI — all pointing at the same MCP connection config and skills.

## File map

| File | Role |
|---|---|
| `.claude-plugin/plugin.json` | Claude Code manifest — identity, hooks, skills, monitors, `userConfig` |
| `.codex-plugin/plugin.json` | Codex manifest — same data + Codex UI fields (`interface`) |
| `gemini-extension.json` | Gemini CLI manifest — uses `settings` array instead of `userConfig` |
| `.mcp.json` | Shared MCP server connection config used by all three platforms |
| `bin/rarcane` | Release binary used by the monitor — populate with `just install` |
| `hooks/hooks.json` | Lifecycle hook definitions: `SessionStart`, `ConfigChange` |
| `hooks/plugin-setup.sh` | Deployment and validation script (server mode or client mode) |
| `monitors/monitors.json` | Background health monitor config (requires Claude Code v2.1.105+) |
| `skills/rarcane/SKILL.md` | Three-tier tool documentation shared by Claude and Codex |

## Versioning rule

**Do not add a `version` field to any manifest.** The marketplace derives version from the git commit SHA. An explicit `version` field causes every push to register as a new version and creates duplicate marketplace entries.

## Updating a manifest

When changing connection config (URL, auth headers), update `.mcp.json` — do not duplicate the values into each manifest separately. All three platforms read `.mcp.json`.

When changing user-configurable settings, update all three manifests: `userConfig` in the Claude and Codex `plugin.json` files, and `settings` in `gemini-extension.json`. Keep field names and descriptions consistent across all three.

## Monitors (Claude Code v2.1.105+)

`monitors/monitors.json` runs `rarcane watch` from `${CLAUDE_PLUGIN_ROOT}/bin/rarcane`. The binary must exist at that path before the plugin is installed. Populate it with:

```bash
just install   # cargo build --release, then copies to plugins/rarcane/bin/rarcane
```

The monitor command uses `${user_config.server_url}` substitution — this is resolved at runtime from the user's plugin settings. Do not hardcode URLs in `monitors.json`.

When adding a new monitor: add an entry to `monitors.json` and reference only `${CLAUDE_PLUGIN_ROOT}/bin/rarcane` or scripts under `${CLAUDE_PLUGIN_ROOT}/scripts/`. Do not reference bare binary names that depend on PATH — the monitor may start before `plugin-setup.sh` has run.

## Updating the skill

`skills/rarcane/SKILL.md` is shared by Claude Code and Codex. Gemini reads it via the `skills` path in `gemini-extension.json`. Edit it once — all platforms see the change.

The three-tier structure must be preserved:
- **Tier 1** (above fold): tool name, quick action table, critical gotchas
- **Tier 2** (middle): full action reference with parameters and response shapes
- **Tier 3** (bottom): workflows, HTTP fallback, error handling

## Updating the setup script

`hooks/plugin-setup.sh` reads `CLAUDE_PLUGIN_OPTION_*` env vars that map to the `userConfig` fields in `plugin.json`. When you add or rename a `userConfig` field, update the env var block in the setup script to match.

Sensitive fields declared `"sensitive": true` in `plugin.json` are available as env vars in hooks but are **never** substituted into skill content.

## Template adaptation

When renaming `rarcane` → your service:

1. Replace all `rarcane` / `Arcane` / `RARCANE_` identifiers in every file in this directory.
2. Rename `skills/rarcane/` to `skills/<your-service>/`.
3. Update `hooks/plugin-setup.sh` — the env var block near the top maps `CLAUDE_PLUGIN_OPTION_*` to your service's actual `RARCANE_*` vars.
4. Keep the no-version rule: do not add `"version"` to any manifest.
