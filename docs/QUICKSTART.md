# rustcane Quickstart

## 1. Configure Arcane

```bash
export RUSTCANE_API_URL=https://arcane.example.com
export RUSTCANE_API_KEY=change-me
export RUSTCANE_MCP_HOST=127.0.0.1
export RUSTCANE_MCP_PORT=3100
export RUSTCANE_MCP_NO_AUTH=true
```

## 2. Try the CLI

```bash
cargo run -- status
cargo run -- help container
cargo run -- call --action container --subaction list --env-id default
```

Destructive operations need confirmation:

```bash
cargo run -- call --action container --subaction stop --env-id default --id nginx --confirm
```

## 3. Start HTTP MCP

```bash
cargo run -- serve
```

```bash
curl -s http://127.0.0.1:3100/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"arcane","arguments":{"action":"status"}}}'
```

## 4. Verify

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
cargo build --release
```
