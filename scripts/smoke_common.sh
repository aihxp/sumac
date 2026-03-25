#!/usr/bin/env bash

pick_python() {
  if command -v python3 >/dev/null 2>&1; then
    command -v python3
    return
  fi
  if command -v python >/dev/null 2>&1; then
    command -v python
    return
  fi
  echo "python3 or python is required for ${0}" >&2
  exit 1
}

wait_for_health() {
  local url="$1"
  for _ in $(seq 1 40); do
    if curl --silent --fail "${url}" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.25
  done
  echo "Timed out waiting for ${url}" >&2
  return 1
}

json_check() {
  local file="$1"
  local expr="$2"
  local python_bin="${3:-$(pick_python)}"
  JSON_PATH="$file" JSON_EXPR="$expr" "$python_bin" - <<'PY'
import json
import os
from pathlib import Path

value = json.loads(Path(os.environ["JSON_PATH"]).read_text())
ok = bool(eval(os.environ["JSON_EXPR"], {"__builtins__": {}}, {"d": value}))
raise SystemExit(0 if ok else 1)
PY
}

tool_name_from_manifest() {
  local manifest_path="$1"
  local python_bin="${2:-$(pick_python)}"
  MANIFEST_PATH="$manifest_path" "$python_bin" - <<'PY'
import json
import os
from pathlib import Path

manifest = json.loads(Path(os.environ["MANIFEST_PATH"]).read_text())
name = manifest["generated_tools"][0]["name"]
print(f"discovery__{name}")
PY
}
