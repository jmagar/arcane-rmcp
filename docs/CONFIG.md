# rustcane Configuration

## Arcane

| Variable | Purpose |
|---|---|
| `RUSTCANE_API_URL` | Arcane base URL. `/api` is appended when needed. |
| `RUSTCANE_API_KEY` | Arcane API key. Stored in env/config only. |

## MCP

| Variable | Default | Purpose |
|---|---|---|
| `RUSTCANE_MCP_HOST` | `0.0.0.0` | HTTP bind host |
| `RUSTCANE_MCP_PORT` | `3100` | HTTP bind port |
| `RUSTCANE_MCP_TOKEN` | unset | Static bearer token |
| `RUSTCANE_MCP_NO_AUTH` | false | Disable auth on loopback only |
| `RUSTCANE_NOAUTH` | false | Explicit trusted gateway mode |
| `RUSTCANE_MCP_ALLOWED_HOSTS` | unset | Extra Host header values |
| `RUSTCANE_MCP_ALLOWED_ORIGINS` | unset | Extra CORS origins |
| `RUSTCANE_MCP_AUTH_MODE` | `bearer` | `bearer` or `oauth` |

## Auth Policy

| State | Condition | Behavior |
|---|---|---|
| `LoopbackDev` | loopback bind or loopback no-auth | no auth, no scopes |
| `TrustedGatewayUnscoped` | `RUSTCANE_NOAUTH=true` behind a trusted gateway | no local auth or scopes |
| `Mounted` bearer | non-loopback with `RUSTCANE_MCP_TOKEN` | bearer auth and scope checks |
| `Mounted` OAuth | `RUSTCANE_MCP_AUTH_MODE=oauth` | OAuth/JWT auth and scope checks |

Use `rustcane setup check` for read-only validation and `rustcane setup repair` to create a local `.env`.
