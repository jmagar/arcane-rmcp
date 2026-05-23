# rustcane API

`rustcane` exposes one MCP tool named `arcane`, one REST action endpoint at `/v1/rustcane`, and equivalent CLI commands.

## MCP Tool

Required field: `action`.

Common fields:

| Field | Type | Required | Notes |
|---|---|---:|---|
| `action` | string | yes | Domain such as `container`, `project`, `image`, or `status` |
| `subaction` | string | domain actions | Operation within the domain |
| `envId` | string | environment-scoped actions | Arcane environment id |
| `id` | string | item actions | Resource id |
| `params` | object | action-dependent | Body/control parameters |

Examples:

```json
{"action":"status"}
{"action":"container","subaction":"list","envId":"default"}
{"action":"container","subaction":"stop","envId":"default","id":"nginx","params":{"confirm":true}}
{"action":"image","subaction":"pull","envId":"default","params":{"image":"alpine:latest"}}
```

## CLI Parity

```bash
rustcane status
rustcane help container
rustcane call --action container --subaction list --env-id default
rustcane call --action container --subaction stop --env-id default --id nginx --confirm
rustcane call --action image --subaction pull --env-id default --params '{"image":"alpine:latest"}'
```

## REST Endpoint

`POST /v1/rustcane`

```json
{
  "action": "container",
  "params": {
    "subaction": "list",
    "envId": "default"
  }
}
```

## Safety and Auth

- `help` is public.
- Read operations require `rustcane:read`.
- Mutating operations require `rustcane:write`.
- Destructive operations require explicit confirmation.
- Credentials are never accepted as tool parameters.
- Arcane API error strings are redacted before being returned.
