# Modularize Valence chunk layer responsibility inventory

## Question

What current `valence_server` chunk-layer responsibilities must remain owned after modularization?

## Inspected evidence

- `servers/valence/crates/valence_server/src/layer/chunk.rs` before the refactor owned chunk-layer storage, public chunk entry APIs, global/local message enums, packet writer adapters, `Layer` trait integration, and pre/post-client update systems.
- `servers/valence/crates/valence_server/src/client.rs` consumed chunk-layer global/local messages and applied view, radius, exception, chunk-state, and biome delivery decisions in the client shell.
- Baseline logs in this directory record `cargo test -p valence_server layer::chunk -- --nocapture` and `cargo test -p valence layer -- --nocapture` passing before core edits.

## Decision

The modularized owner boundary is `servers/valence/crates/valence_server/src/layer/chunk.rs` plus focused child modules:

- `storage.rs`: `ChunkLayer` storage and public chunk APIs.
- `entry.rs`: occupied/vacant entry API and chunk-state message planning.
- `messages.rs`: global/local chunk-layer messages and client message-target decisions.
- `targeting.rs`: pure view, radius, exception, and update-plan decisions.
- `writers.rs`: packet writer adapters.
- `layer_impl.rs`: `Layer` trait wiring.
- `systems.rs`: Bevy schedule/system shell.

Packet writes, Bevy queries, layer mutation, and schedule registration remain shell responsibilities. No broad Minecraft compatibility, semantic-equivalence, public-server safety, production-readiness, full CTF correctness, or survival correctness claim is promoted.

## Owner

Valence owner subtree: `servers/valence/crates/valence_server/`.

## Next action

Use focused chunk/layer tests, Valence schedule hygiene, Cairn gates, and Cairn validation as review evidence before archive.
