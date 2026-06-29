# Modularize Valence command manager responsibility map

## Scope

This evidence supports Cairn change `modularize-valence-command-manager`. It covers the Valence command manager architecture only and does not promote broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, or full CTF/survival correctness claims.

## Inspected inputs

- `servers/valence/crates/valence_command/src/manager.rs` before implementation combined plugin setup, packet event adapters, command tree updates, incoming command parsing, argument parsing, processed event emission, and tests.
- `servers/valence/crates/valence_command/src/lib.rs` keeps public command APIs and system-set exports stable.
- `servers/valence/crates/valence_command/README.md` documents the existing packet/event behavior and non-claims.

## Responsibility owners after the split

| Responsibility | Owner module | Boundary |
| --- | --- | --- |
| Plugin wiring, event registration, schedule placement, and initial registry creation | `servers/valence/crates/valence_command/src/manager/plugin.rs` | Imperative Bevy shell |
| Bevy systems for scope insertion, packet event emission, tree sync, event conversion, and command parsing orchestration | `servers/valence/crates/valence_command/src/manager/systems.rs` | Imperative Bevy shell |
| Raw command packet decoding and decoded packet-event planning | `servers/valence/crates/valence_command/src/manager/packet_adapter.rs` | Pure packet-to-event decision plus decode adapter |
| Public command execution event types and processed-event planning | `servers/valence/crates/valence_command/src/manager/event_plan.rs` | Pure event-shape planning |
| Command parse requests, argument parse plans, modifier execution plans, executable-node outcomes, scope rejection, unknown-command rejection, and invalid-argument rejection | `servers/valence/crates/valence_command/src/manager/parse_core.rs` | Pure parse core over explicit registry, scope, and command inputs |
| Command tree update decisions and visible command tree planning for one client scope set | `servers/valence/crates/valence_command/src/manager/tree_sync.rs` | Pure tree decision core over explicit graph, registry, and scope inputs |
| Public manager module surface | `servers/valence/crates/valence_command/src/manager.rs` | Thin re-export boundary preserving `manager::CommandPlugin`, `manager::CommandExecutionPacketEvent`, `manager::CommandExecutionEvent`, and `manager::CommandProcessedEvent` |

## Verification summary

- Baseline focused command-manager tests passed before implementation.
- Post-split focused command-manager tests passed with positive and negative cases for packet adapters, tree decisions, parsing, argument plans, processed events, plugin wiring, malformed packets, unknown commands, invalid arguments, stale command trees, missing scopes, and disabled clients.
- Affected command crate tests, no-deps clippy, schedule hygiene, and the Valence wrapper dry-run passed with reviewable logs under `docs/evidence/run-logs/2026-06-29/`.
