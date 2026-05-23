---
title: "systemd Deployment"
doc_type: "guide"
status: "active"
owner: "rustcane"
audience:
  - "contributors"
  - "agents"
scope: "template"
source_of_truth: false
last_reviewed: "2026-05-15"
---

# systemd

The template supports user-level systemd deployments when a unit named `rustcane-mcp.service` is installed by the derived service.

## Install the binary

```bash
cargo build --release
install -m 755 target/release/rustcane ~/.local/bin/rustcane
```

Or use the install script:

```bash
curl -fsSL https://raw.githubusercontent.com/jmagar/rustcane-mcp/main/install.sh | bash
```

The binary installs to `~/.local/bin/`. Verify it's in `$PATH`:

```bash
rustcane --version
rustcane doctor
```

## Unit file pattern

```ini
[Unit]
Description=rustcane MCP server
After=network.target

[Service]
Type=simple
ExecStart=%h/.local/bin/rustcane serve mcp
Restart=on-failure
RestartSec=5
EnvironmentFile=%h/.rustcane/.env

[Install]
WantedBy=default.target
```

Key points:
- Use `EnvironmentFile` pointing at `~/.rustcane/.env` — never hardcode tokens in unit files.
- `%h` expands to the user home directory.
- `serve mcp` is the canonical Streamable HTTP mode (see `docs/DEPLOYMENT.md`).

## Restart flow

```bash
systemctl --user daemon-reload
systemctl --user restart rustcane-mcp.service
systemctl --user status rustcane-mcp.service
```

## Runtime verification

`just runtime-current` detects stale running processes. The checker compares:

- `/proc/<pid>/exe` for the running service process
- the unit `ExecStart` binary
- optional `--expected-binary`

```bash
scripts/check-runtime-current.sh --mode systemd --expected-binary target/release/rustcane
just runtime-current
```

If hashes differ, install the new binary and restart the unit.

## Logging

With systemd, logs go to the journal:

```bash
journalctl --user -u rustcane-mcp.service -f
journalctl --user -u rustcane-mcp.service --since "1h ago"
```

The binary also writes structured JSON logs to `~/.rustcane/logs/rustcane.log` regardless of deployment mode (see `docs/OBSERVABILITY.md`).

## Doctor pre-flight

Run `rustcane doctor` before starting the unit to validate the environment:

```bash
rustcane doctor
```

Exit code 0 = ready to start. Exit code 1 = one or more issues found.

## Environment

Prefer an `EnvironmentFile` that points at the repo or appdata `.env`. See `docs/ENV.md` for variable meanings.

See `docs/PATTERNS.md` §46, §47, §48 for binary commands, installation, and the doctor command patterns.
