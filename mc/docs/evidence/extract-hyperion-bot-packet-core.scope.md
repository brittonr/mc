# Extract Hyperion bot packet core scope

## Question

Should `extract-hyperion-bot-packet-core` change Valence or parent mc-compat behavior?

## Decision

No. This change is Hyperion tool-owned nested-repo work. It extracts deterministic packet helper logic inside Hyperion's `rust-mc-bot` tool only. No Valence code, mc-compat runner code, public-server safety control, or compatibility evidence claim is adopted or promoted by this change.

## Captured responsibilities

- `hyperion/tools/rust-mc-bot/src/packet_utils.rs`: in-memory byte buffer, primitive read/write helpers, packet ID varint encoding support.
- `hyperion/tools/rust-mc-bot/src/states/login.rs`: login packet constructors and login-state mutation shell.
- `hyperion/tools/rust-mc-bot/src/states/play.rs`: play packet constructors plus shells that mutate bot position, kick state, teleport state, and send responses.
- `hyperion/tools/rust-mc-bot/src/packet_processors.rs`: server packet classification and handler dispatch shell.
- `hyperion/tools/rust-mc-bot/src/net.rs`: socket reads/writes, buffering, decompression shell, and fail-closed IO decisions.
- `hyperion/tools/rust-mc-bot/src/lib.rs`: bot orchestration, polling, sleeps/timing, randomness, connection state, and login sequencing.

## Boundary inventory

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/tools/rust-mc-bot/src/packet_utils.rs` | reference | `extract-hyperion-bot-packet-core` | Buffer helpers are kept in Hyperion and only expose deterministic readable/written slices for local packet tests. | none | No Valence API change; existing unsafe buffer behavior is not ported. | `docs/evidence/extract-hyperion-bot-packet-core.hyperion-bot-tests-final.run.log` | No Valence adoption, production readiness, public-server safety, or compatibility claim. |
| `hyperion/tools/rust-mc-bot/src/packet_core.rs` | reference | `extract-hyperion-bot-packet-core` | Pure packet construction, classification, byte-shape validation, and protocol-assumption checks are Hyperion-local. | none | No socket IO, async runtime, sleeps, logging, or bot mutation in the core. | `docs/evidence/extract-hyperion-bot-packet-core.hyperion-bot-tests-final.run.log` | No broad Minecraft compatibility or semantic-equivalence claim. |
| `hyperion/tools/rust-mc-bot/src/net.rs` | reference | `extract-hyperion-bot-packet-core` | Socket outcome classification is pure, while reads/writes and bot kicked-state mutation remain shell work. | none | Closed-socket behavior is tested as a fail-closed local bot helper decision. | `docs/evidence/extract-hyperion-bot-packet-core.hyperion-clippy.run.log` | No public-server authorization or safety envelope is asserted. |
| `hyperion/tools/rust-mc-bot/src/states/*.rs` and `src/packet_processors.rs` | reference | `extract-hyperion-bot-packet-core` | Packet builders and classifiers delegate to the core; handlers keep bot state mutation and send calls in shells. | none | CLI/API packet sequence and supported protocol assumptions remain local to rust-mc-bot. | `docs/evidence/extract-hyperion-bot-packet-core.hyperion-focused-tests.run.log` | No Valence default behavior, CTF/survival correctness, or vanilla parity claim. |

## Next action

Use the focused Hyperion logs plus Cairn gates/validation as archive evidence. A separate accepted integration change is required before any Valence adoption, porting, public-server safety, or compatibility claim can be made.
