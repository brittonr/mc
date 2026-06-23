# Hyperion-derived load and packet tool inventory

## Scope

This inventory applies the Hyperion-to-Valence boundary rules from `docs/hyperion-integration-boundaries.md` before adapting load or packet diagnostics. The work is tooling-only: Valence public server APIs and default gameplay behavior remain unchanged.

## Inventory

| Source | Concept | Classification | Valence target | Reason | Public API impact | Evidence / non-claims |
| --- | --- | --- | --- | --- | --- | --- |
| `hyperion/tools/rust-mc-bot` | Multi-bot load driver, bounded bot count, generated names, TCP/UDS target support | reference | `servers/valence/tools/stresser` | The Hyperion tool is GPL-licensed and uses Hyperion-specific packet/runtime code, so it informs only the safety checklist and structured failure vocabulary. Valence keeps its own MIT-compatible stresser implementation. | none | Positive/negative config tests and dry-run receipt; no production capacity, public-server safety, compatibility, or Hyperion-equivalence claim. |
| `hyperion/tools/rust-mc-bot/src/main.rs` | Environment-driven config defaults and bot count/thread controls | reference | `servers/valence/tools/stresser/src/config.rs` | Valence uses explicit CLI config instead of environment defaults so review logs show target, scenario, and authorization choices. | none | Config tests reject DNS and non-loopback targets unless explicitly authorized before any connection is attempted. |
| `hyperion/tools/antithesis-bot` | Antithesis-specific bot launcher | reject | none | It is tied to Antithesis SDK lifecycle hooks and does not provide a general Valence-owned load smoke path. | none | Rejected for this Cairn; no Antithesis, fuzzing, or public load claim. |
| `hyperion/tools/packet-inspector` | Packet-inspection proxy and CLI logging shape | reference | `servers/valence/tools/packet_inspector` | Valence already owns a packet inspector tree; this change ports only bounded/redacted capture policy and makes the existing CLI feature buildable. | none | Packet capture policy tests cover redaction, bounded preview, malformed VarInt, incomplete capture, and zero-bound rejection. |
| `servers/valence/tools/stresser` | Existing Valence load/stress client | port | `servers/valence/tools/stresser` | The existing Valence-owned tool is adapted with typed config validation, loopback-safe defaults, dry-run output, bounded starts, and structured phase failures. | none | Tool output is load/diagnostic evidence only; compatibility and production capacity remain non-claims. |
| `servers/valence/tools/packet_inspector` | Existing Valence packet proxy/GUI source | port | `servers/valence/tools/packet_inspector` | The existing Valence-owned tool is adapted with explicit `gui`/`cli` features and a pure capture policy core. | none | CLI logs redact payloads by default and bound preview bytes when explicitly enabled; malformed-capture fixtures fail closed. |
| `servers/valence/tools/dump_schedule` | Schedule graph diagnostic | reject | none | It is not a load-bot, packet-inspector, capture, or wrapper concept needed for this change. | none | Out of scope; no scheduling or performance claim. |
| `servers/valence/tools/playground` | Scratch reproduction wrapper | reject | none | It is a local experimentation area, not a maintained load or packet diagnostic surface. | none | Out of scope; no compatibility, load, or packet capture claim. |

## Contract summary

- Typed load config is validated by `stresser::config` before network I/O.
- Default load target policy allows loopback IP socket addresses and `localhost:PORT`; DNS targets are rejected to avoid implicit resolution.
- Non-loopback IP socket addresses require both `--allow-non-loopback` and a non-empty `--authorization-note` before the tool attempts any connection.
- Load reports use schema `valence.load_tool.report.v1` and always include target, scenario, session count, max starts, optional session timeout, phase, exit status, and `compatibility_evidence_pass=false`.
- Packet capture policy defaults to payload redaction and bounds previews with `--max-packet-bytes` only when `--include-payload-preview` is explicitly passed.
- Malformed packet-length VarInts, incomplete packets, zero capture bounds, and over-large captures fail closed with deterministic diagnostics.

## Evidence classes and non-claims

Load-tool dry runs, loopback smoke runs, and structured session failures can support tool-operation evidence. Packet-inspector logs can support packet-diagnostic evidence. Neither evidence class proves Minecraft compatibility, vanilla/reference parity, production readiness, public-server safety, Hyperion compatibility, or full CTF/survival correctness without separate accepted scenario evidence and promoted receipts.
