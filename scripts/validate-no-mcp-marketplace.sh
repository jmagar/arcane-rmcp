#!/usr/bin/env bash
# Validate a generated marketplace plugin that relies on an external MCP gateway.
set -uo pipefail

PLUGIN_ROOT="${1:-dist/marketplace-no-mcp/rarcane}"
FAILED=0
CHECKS=0

check() {
  local description="$1"
  shift
  CHECKS=$((CHECKS + 1))
  printf 'Checking: %s... ' "${description}"
  if "$@" >/dev/null 2>&1; then
    printf 'PASS\n'
  else
    printf 'FAIL\n'
    FAILED=$((FAILED + 1))
  fi
}

echo "=== Validating no-MCP marketplace artifact ==="
echo "Plugin root: ${PLUGIN_ROOT}"
echo

check "artifact marker exists" test -f "${PLUGIN_ROOT}/.no-mcp-marketplace-artifact"
check "current MCP registration is absent" test ! -e "${PLUGIN_ROOT}/.mcp.json"
check "legacy MCP registration is absent" test ! -e "${PLUGIN_ROOT}/mcp.json"

for manifest in \
  "${PLUGIN_ROOT}/.claude-plugin/plugin.json" \
  "${PLUGIN_ROOT}/.codex-plugin/plugin.json" \
  "${PLUGIN_ROOT}/gemini-extension.json" \
  "${PLUGIN_ROOT}/hooks/hooks.json" \
  "${PLUGIN_ROOT}/monitors/monitors.json"; do
  check "$(basename "${manifest}") is valid JSON" jq empty "${manifest}"
done

for manifest in \
  "${PLUGIN_ROOT}/.claude-plugin/plugin.json" \
  "${PLUGIN_ROOT}/.codex-plugin/plugin.json" \
  "${PLUGIN_ROOT}/gemini-extension.json"; do
  check "${manifest#"${PLUGIN_ROOT}/"} has no version field" \
    jq -e 'has("version") | not' "${manifest}"
done

check "Claude manifest has no inline MCP registration" \
  jq -e 'has("mcpServers") | not' "${PLUGIN_ROOT}/.claude-plugin/plugin.json"
check "Codex manifest has no inline MCP registration" \
  jq -e 'has("mcpServers") | not' "${PLUGIN_ROOT}/.codex-plugin/plugin.json"
check "Gemini manifest has no inline MCP registration" \
  jq -e 'has("mcpServers") | not' "${PLUGIN_ROOT}/gemini-extension.json"
check "Gemini settings remain available" \
  jq -e '.settings | type == "array" and length > 0' "${PLUGIN_ROOT}/gemini-extension.json"
check "Claude settings remain available" \
  jq -e '.userConfig.server_url' "${PLUGIN_ROOT}/.claude-plugin/plugin.json"
check "Codex interface metadata remains available" \
  jq -e '.interface.displayName and (.interface.capabilities | length > 0)' \
  "${PLUGIN_ROOT}/.codex-plugin/plugin.json"
check "hooks remain available" test -f "${PLUGIN_ROOT}/hooks/hooks.json"
check "rarcane skill remains available" test -f "${PLUGIN_ROOT}/skills/rarcane/SKILL.md"
check "agent instruction symlinks remain intact" sh -c \
  "test -L '$PLUGIN_ROOT/AGENTS.md' && test \"\$(readlink '$PLUGIN_ROOT/AGENTS.md')\" = CLAUDE.md && test -L '$PLUGIN_ROOT/GEMINI.md' && test \"\$(readlink '$PLUGIN_ROOT/GEMINI.md')\" = CLAUDE.md"

echo
echo "Checks: ${CHECKS}; failures: ${FAILED}"
if (( FAILED > 0 )); then
  exit 1
fi
echo "No-MCP marketplace artifact is valid."
