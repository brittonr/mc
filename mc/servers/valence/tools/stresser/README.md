# stresser

A Valence-owned Minecraft client for loopback load smoke and bounded stress diagnostics.

This is a development tool, not a compatibility oracle. Structured reports from this tool can support load-tool evidence only; they do not prove Minecraft compatibility, vanilla/reference parity, production readiness, public-server safety, Hyperion compatibility, or full gameplay correctness.

## Safe target policy

`stresser` validates its typed CLI config before opening network connections.

- Loopback IP socket addresses and `localhost:PORT` are allowed by default.
- DNS names are rejected to avoid implicit resolution before safety checks.
- Non-loopback IP socket addresses require both `--allow-non-loopback` and a non-empty `--authorization-note`.

## Usage

```sh
# Run a Valence example separately.
cargo run --example bench_players

# Validate config without network I/O and write a structured report.
cargo run -p stresser -- \
  --target 127.0.0.1:25565 \
  --count 1 \
  --dry-run \
  --max-starts 1 \
  --session-timeout-ms 5000 \
  --report target/valence-stresser-dry-run.json

# Run a bounded loopback load smoke.
cargo run -p stresser -- \
  --target 127.0.0.1:25565 \
  --count 1 \
  --max-starts 1 \
  --session-timeout-ms 5000 \
  --scenario loopback-smoke
```

Reports use schema `valence.load_tool.report.v1` and record target, scenario, session count, optional max starts, target-safety classification, failing phase, exit status, and explicit `compatibility_evidence_pass=false`.
