#!/usr/bin/env bash
# Build and smoke-run Valence's parkour example until it listens on the Minecraft port.

set -euo pipefail

readonly DEFAULT_PORT=25565
readonly DEFAULT_READY_TIMEOUT=30
readonly DEFAULT_RUN_SECONDS=2

repo_root=$(git rev-parse --show-toplevel 2>/dev/null) || {
  echo "error: not inside a git repository" >&2
  exit 1
}
cd "$repo_root"

if [[ "${VALENCE_SMOKE_IN_NIX:-}" != "1" ]]; then
  exec env VALENCE_SMOKE_IN_NIX=1 nix develop -c bash --noprofile --norc "$0" "$@"
fi

port="$DEFAULT_PORT"
ready_timeout="${VALENCE_SMOKE_READY_TIMEOUT:-$DEFAULT_READY_TIMEOUT}"
run_seconds="${VALENCE_SMOKE_RUN_SECONDS:-$DEFAULT_RUN_SECONDS}"
target_dir="${VALENCE_SMOKE_TARGET_DIR:-}"
created_target_dir=0

if [[ -z "$target_dir" ]]; then
  target_dir=$(mktemp -d -p "${TMPDIR:-/tmp}" valence-parkour-target.XXXXXX)
  created_target_dir=1
fi

log_file=$(mktemp -p "${TMPDIR:-/tmp}" valence-parkour-smoke.XXXXXX.log)
server_pid=""

cleanup() {
  local status=$?

  if [[ -n "$server_pid" ]] && kill -0 "$server_pid" 2>/dev/null; then
    kill "$server_pid" 2>/dev/null || true
    timeout 5 tail --pid="$server_pid" -f /dev/null 2>/dev/null || kill -KILL "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi

  if [[ "$created_target_dir" -eq 1 ]]; then
    rm -rf "$target_dir"
  fi

  exit "$status"
}
trap cleanup EXIT INT TERM

echo "smoke.parkour.target_dir=$target_dir"
echo "smoke.parkour.log=$log_file"

if ss -ltnH "sport = :$port" | grep -q LISTEN; then
  echo "smoke.parkour.status=failed" >&2
  echo "smoke.parkour.reason=port_already_listening" >&2
  exit 1
fi

env RUSTC_WRAPPER= CARGO_TARGET_DIR="$target_dir" cargo build --release --example parkour

env RUSTC_WRAPPER= CARGO_TARGET_DIR="$target_dir" cargo run --release --example parkour >"$log_file" 2>&1 &
server_pid=$!

# shellcheck disable=SC2016
if ! timeout "$ready_timeout" bash -c '
  port="$1"
  pid="$2"
  while kill -0 "$pid" 2>/dev/null; do
    if ss -ltnH "sport = :$port" | grep -q LISTEN; then
      exit 0
    fi
    sleep 0.1
  done
  exit 1
' bash "$port" "$server_pid"; then
  echo "smoke.parkour.status=failed" >&2
  echo "smoke.parkour.reason=port_not_ready" >&2
  sed -n '1,160p' "$log_file" >&2
  exit 1
fi

echo "smoke.parkour.status=listening"
echo "smoke.parkour.port=$port"

if ! timeout "$run_seconds" tail --pid="$server_pid" -f /dev/null >/dev/null 2>&1; then
  : # Expected: the server should still be running until cleanup stops it.
fi

if ! kill -0 "$server_pid" 2>/dev/null; then
  echo "smoke.parkour.status=failed" >&2
  echo "smoke.parkour.reason=process_exited_after_ready" >&2
  sed -n '1,160p' "$log_file" >&2
  exit 1
fi

echo "smoke.parkour.status=passed"
