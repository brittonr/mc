#!/usr/bin/env bash
# Build and smoke-run Valence's parkour example until it listens on the Minecraft port.

set -euo pipefail

readonly DEFAULT_PORT=25565
readonly DEFAULT_READY_TIMEOUT=30
readonly DEFAULT_RUN_SECONDS=2

usage() {
  cat <<'USAGE'
usage: scripts/smoke-parkour.sh [--dry-run] [--receipt PATH]

Build and smoke-run Valence's parkour example until it listens on :25565.

Options:
  --dry-run       Do not build or run the example; emit a non-live receipt.
  --receipt PATH  Write a machine-readable JSON receipt.
USAGE
}

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$repo_root"

dry_run=0
receipt_path="${VALENCE_SMOKE_RECEIPT:-}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      dry_run=1
      shift
      ;;
    --receipt)
      if [[ $# -lt 2 ]]; then
        echo "error: --receipt requires a path" >&2
        exit 2
      fi
      receipt_path="$2"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ "$dry_run" -eq 0 && "${VALENCE_SMOKE_IN_NIX:-}" != "1" ]]; then
  exec env VALENCE_SMOKE_IN_NIX=1 VALENCE_SMOKE_RECEIPT="$receipt_path" nix develop -c bash --noprofile --norc "$0"
fi

port="$DEFAULT_PORT"
ready_timeout="${VALENCE_SMOKE_READY_TIMEOUT:-$DEFAULT_READY_TIMEOUT}"
run_seconds="${VALENCE_SMOKE_RUN_SECONDS:-$DEFAULT_RUN_SECONDS}"
target_dir="${VALENCE_SMOKE_TARGET_DIR:-}"
created_target_dir=0
log_file=""
server_pid=""
status="failed"
reason="not_started"
build_passed=false
listened=false
process_stayed_alive=false
cleanup_port_closed=false

json_escape() {
  local value="$1"
  value=${value//\/\\}
  value=${value//'"'/'\"'}
  value=${value//$'\n'/\\n}
  value=${value//$'\r'/\\r}
  value=${value//$'\t'/\\t}
  printf '%s' "$value"
}

write_receipt() {
  if [[ -z "$receipt_path" ]]; then
    return 0
  fi

  mkdir -p "$(dirname "$receipt_path")"
  local escaped_reason escaped_log
  escaped_reason=$(json_escape "$reason")
  escaped_log=$(json_escape "$log_file")
  cat >"$receipt_path" <<JSON
{
  "schema": "valence.parkour-smoke.receipt.v1",
  "example": "parkour",
  "port": $port,
  "status": "$status",
  "reason": "$escaped_reason",
  "dry_run": $([[ "$dry_run" -eq 1 ]] && echo true || echo false),
  "live_smoke": $([[ "$dry_run" -eq 0 ]] && echo true || echo false),
  "build_passed": $build_passed,
  "listened": $listened,
  "process_stayed_alive": $process_stayed_alive,
  "cleanup_port_closed": $cleanup_port_closed,
  "claims_client_compat": false,
  "claims_semantic_correctness": false,
  "log_file": "$escaped_log"
}
JSON
}

cleanup() {
  local exit_status=$?

  if [[ -n "$server_pid" ]] && kill -0 "$server_pid" 2>/dev/null; then
    kill "$server_pid" 2>/dev/null || true
    timeout 5 tail --pid="$server_pid" -f /dev/null 2>/dev/null || kill -KILL "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi

  if ! ss -ltnH "sport = :$port" | grep -q LISTEN; then
    cleanup_port_closed=true
  fi

  if [[ "$created_target_dir" -eq 1 ]]; then
    rm -rf "$target_dir"
  fi

  if [[ "$status" == "failed" && "$reason" == "not_started" && "$exit_status" -ne 0 ]]; then
    reason="command_failed"
  fi
  write_receipt
  exit "$exit_status"
}
trap cleanup EXIT INT TERM

if [[ "$dry_run" -eq 1 ]]; then
  status="passed"
  reason="dry_run"
  cleanup_port_closed=true
  echo "smoke.parkour.status=dry_run"
  echo "smoke.parkour.receipt=${receipt_path:-none}"
  exit 0
fi

if [[ -z "$target_dir" ]]; then
  target_dir=$(mktemp -d -p "${TMPDIR:-/tmp}" valence-parkour-target.XXXXXX)
  created_target_dir=1
fi

log_file=$(mktemp -p "${TMPDIR:-/tmp}" valence-parkour-smoke.XXXXXX.log)

echo "smoke.parkour.target_dir=$target_dir"
echo "smoke.parkour.log=$log_file"

if ss -ltnH "sport = :$port" | grep -q LISTEN; then
  reason="port_already_listening"
  echo "smoke.parkour.status=failed" >&2
  echo "smoke.parkour.reason=$reason" >&2
  exit 1
fi

env RUSTC_WRAPPER= CARGO_TARGET_DIR="$target_dir" cargo build --release --example parkour
build_passed=true

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
  reason="port_not_ready"
  echo "smoke.parkour.status=failed" >&2
  echo "smoke.parkour.reason=$reason" >&2
  sed -n '1,160p' "$log_file" >&2
  exit 1
fi

listened=true
echo "smoke.parkour.status=listening"
echo "smoke.parkour.port=$port"

if ! timeout "$run_seconds" tail --pid="$server_pid" -f /dev/null >/dev/null 2>&1; then
  : # Expected: the server should still be running until cleanup stops it.
fi

if ! kill -0 "$server_pid" 2>/dev/null; then
  reason="process_exited_after_ready"
  echo "smoke.parkour.status=failed" >&2
  echo "smoke.parkour.reason=$reason" >&2
  sed -n '1,160p' "$log_file" >&2
  exit 1
fi

process_stayed_alive=true
status="passed"
reason="ok"
echo "smoke.parkour.status=passed"
