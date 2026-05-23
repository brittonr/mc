#!/usr/bin/env bash
set -euo pipefail

ROOT=${ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." >/dev/null && pwd -P)}
export MC_COMPAT_ROOT=${MC_COMPAT_ROOT:-$ROOT}

exec nix run "path:$ROOT#mc-compat-smoke" -- "$@"
