#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_ROOT="$(mktemp -d)"
trap 'rm -rf "${TMP_ROOT}"' EXIT

FIRST="${TMP_ROOT}/first/rarcane"
SECOND="${TMP_ROOT}/second/rarcane"

python3 "${ROOT}/scripts/build-no-mcp-marketplace.py" --output "${FIRST}"
bash "${ROOT}/scripts/validate-no-mcp-marketplace.sh" "${FIRST}"
python3 "${ROOT}/scripts/build-no-mcp-marketplace.py" --output "${SECOND}"
bash "${ROOT}/scripts/validate-no-mcp-marketplace.sh" "${SECOND}"

diff -r "${FIRST}" "${SECOND}"
git -C "${ROOT}" diff --exit-code -- plugins/rarcane

mkdir -p "${TMP_ROOT}/occupied"
if python3 "${ROOT}/scripts/build-no-mcp-marketplace.py" \
  --output "${TMP_ROOT}/occupied" >/dev/null 2>&1; then
  echo "generator replaced an unmarked directory" >&2
  exit 1
fi

echo "No-MCP marketplace generation is deterministic and leaves source unchanged."
