#!/usr/bin/env bash
# Run Valence's focused Octet starter rollout gate through the repo flake.

set -euo pipefail

repo_root=$(git rev-parse --show-toplevel 2>/dev/null) || {
  echo "error: not inside a git repository" >&2
  exit 1
}
cd "$repo_root"

exec nix build .#checks.x86_64-linux.octet --no-link -L --impure "$@"
