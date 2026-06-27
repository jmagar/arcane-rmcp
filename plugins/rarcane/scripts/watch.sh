#!/usr/bin/env bash
# Claude monitor entry point. Uses an installed rarcane from PATH.
set -euo pipefail

binary="${RARCANE_MCP_BIN:-rarcane}"

if ! command -v "${binary}" >/dev/null 2>&1; then
  printf 'rarcane monitor: rarcane is not installed or not on PATH.\n' >&2
  exit 0
fi

exec "${binary}" watch "$@"
