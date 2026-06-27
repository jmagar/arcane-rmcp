#!/usr/bin/env bash
# SessionStart / ConfigChange hook for the Rarcane plugin.
set -euo pipefail

binary="${RARCANE_MCP_BIN:-rarcane}"

if ! command -v "${binary}" >/dev/null 2>&1; then
  printf 'rarcane plugin setup: rarcane is not installed or not on PATH.\n' >&2
  printf 'Install rarcane separately, then run: rarcane setup\n' >&2
  exit 0
fi

exec "${binary}" setup plugin-hook "$@"
