# Valence network session responsibility inventory

## Question
What responsibilities currently live in `valence_network`, and which module should own each responsibility after the network-session modularization?

## Inspected evidence
- `servers/valence/crates/valence_network/src/lib.rs`
- `servers/valence/crates/valence_network/src/connect.rs`
- `servers/valence/crates/valence_network/src/legacy_ping.rs`
- `servers/valence/crates/valence_network/src/packet_io.rs`
- `servers/valence/crates/valence_network/src/profile_cache.rs`
- `servers/valence/crates/valence_network/README.md`
- Baseline log: `docs/evidence/modularize-valence-network-session.valence-network-baseline.pre.run.log`

## Responsibility owners

| Responsibility | Current location | Target owner | Boundary |
| --- | --- | --- | --- |
| Bevy plugin construction, shared runtime resources, callback traits, and public root exports | `lib.rs` | `lib.rs` plus focused private modules | Public API and resource wiring remain at the crate root; detailed decisions move to pure modules or async shells. |
| TCP listener, accept permits, connection timeout, TCP setup, and handshake dispatch | `connect.rs` | `connect.rs` | Async socket orchestration remains an imperative shell. |
| Status response callback handling and status JSON composition | `connect.rs`, `lib.rs` | `status.rs` | Callback invocation remains async; JSON/status composition is a pure helper over explicit inputs. |
| Legacy ping detection, payload decoding, and response bytes | `legacy_ping.rs` | `legacy_ping.rs` | Header classification and formatting decisions become pure; stream reads/writes stay async. |
| Login protocol negotiation, online/offline/BungeeCord/Velocity login paths, compression negotiation, and callback disconnects | `connect.rs` | `login.rs` plus `session_core.rs` | Socket/session-server work remains shell code; protocol, compression, handshake, transition, and profile decisions are pure over explicit inputs. |
| Packet framing, packet read/write, client channel handoff, and byte budgets | `packet_io.rs`, `byte_channel.rs` | `packet_io.rs` plus pure budget helpers | Stream/task/channel IO remains shell code; frame cost and budget decisions are pure. |
| Optional profile cache/provider helpers | `profile_cache.rs` | `profile_cache.rs` | Existing adapter traits keep IO at the edge and pure cache/provider decisions in core helpers. |

## Decision
Proceed with a scoped internal split under `servers/valence/crates/valence_network/src/`. No Hyperion code or concepts are used for this Valence change, so no Hyperion adopt/port/reference/reject classification is needed.

## Non-claims
This inventory and the following implementation are network architecture evidence only. They do not promote broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claims.
