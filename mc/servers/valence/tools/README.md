# Valence development tools

Valence tools are repo-owned diagnostics, not public server APIs. They may produce load or packet-diagnostic evidence, but their output alone does not prove Minecraft compatibility, vanilla/reference parity, production readiness, public-server safety, Hyperion compatibility, or full gameplay correctness.

## Load stresser

`tools/stresser` is a Valence-owned load smoke/stress client inspired only by Hyperion load-tool concepts. It validates a typed CLI config before network I/O.

Safe target rules:

- Loopback IP socket addresses and `localhost:PORT` are allowed by default.
- DNS targets are rejected to avoid implicit resolution before safety checks.
- Non-loopback IP socket addresses require both `--allow-non-loopback` and a non-empty `--authorization-note`.

Examples:

```sh
# Validate config and write a structured non-network report.
cargo run -p stresser -- \
  --target 127.0.0.1:25565 \
  --count 1 \
  --dry-run \
  --max-starts 1 \
  --session-timeout-ms 5000 \
  --report target/valence-stresser-dry-run.json

# Bounded loopback smoke: at most one session start, then exit with a structured report.
cargo run -p stresser -- \
  --target 127.0.0.1:25565 \
  --count 1 \
  --max-starts 1 \
  --session-timeout-ms 5000 \
  --scenario loopback-smoke
```

Structured load reports use schema `valence.load_tool.report.v1`. They include phase, target, scenario, session count, optional max starts, exit status, load evidence status, and `compatibility_evidence_pass=false`.

## Packet inspector

`tools/packet_inspector` is a Valence-owned packet proxy/GUI. The default GUI remains enabled through the `gui` feature. The CLI can be built without GUI dependencies:

```sh
cargo run -p packet_inspector --no-default-features --features cli -- \
  127.0.0.1:25566 \
  127.0.0.1:25565
```

Packet payloads are redacted by default. To include bounded hexadecimal payload previews during local diagnostics, opt in explicitly:

```sh
cargo run -p packet_inspector --no-default-features --features cli -- \
  127.0.0.1:25566 \
  127.0.0.1:25565 \
  --include-payload-preview \
  --max-packet-bytes 512
```

The capture policy fails closed for malformed packet-length VarInts, incomplete captures, zero byte bounds, and over-large capture declarations.

## Checks

Run focused tool checks from the Valence root through the mc devshell:

```sh
nix develop --no-update-lock-file /home/brittonr/git/mc -c \
  cargo test -p stresser --all-targets

nix develop --no-update-lock-file /home/brittonr/git/mc -c \
  cargo test -p packet_inspector --no-default-features --features cli --all-targets

nix develop --no-update-lock-file /home/brittonr/git/mc -c \
  cargo test -p packet_inspector --all-targets
```
