#!/usr/bin/env bash
set -euo pipefail

BIN="${1:-target/debug/sxmc}"

"${BIN}" --version >/dev/null
"${BIN}" --help >/dev/null

echo "Startup sanity checks passed for ${BIN}."
