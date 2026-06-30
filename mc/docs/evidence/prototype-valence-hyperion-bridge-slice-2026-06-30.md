# Prototype Valence/Hyperion bridge slice evidence — 2026-06-30

## Scope

This document closes the `prototype-valence-hyperion-bridge-slice` Cairn. The implementation is a Valence-owned optional fixture harness in `valence_network`; it is not enabled by `NetworkPlugin`, does not change direct-mode defaults, does not connect to Hyperion binaries, does not replace Valence networking, does not claim Bedwars behavior, does not claim production-scale readiness, and does not claim vanilla parity.

## Source checkpoint and prerequisites

- Roadmap prerequisite archived: `cairn/archive/2026-06-30-valence-hyperion-integration-roadmap/`.
- Type-ownership prerequisite archived: `cairn/archive/2026-06-30-valence-hyperion-type-ownership-audit/`.
- Reused historical evidence: packet compose, cached chunk egress, proxy broadcast, Hyperion player-join extraction, and Hyperion boundary archives are treated as classified reference or Valence-owned surfaces, not blanket adoption rights.
- Valence source-tree checkpoint before bridge edits: parent repository commit `69057be95d5108d7f2612d40a7666f68fa938caa` for `mc/servers/valence`.
- Hyperion checkpoint during prerequisite audit: `jj` working copy commit `e1cc02f36b5d9f6e26ab1d58eda1d452697d34bd`, parent `a70294d8`; dirty Bedwars files remained unrelated to this Valence-only bridge slice.

## Implementation layout

The bridge slice is implemented as a default-disabled fixture harness and pure planning core:

- `valence_network::bridge_slice::BridgeSliceConfig` is disabled by default and must be explicitly enabled by tests or future adapter shells.
- `plan_join_and_chunks` plans player join and initial chunk delivery from explicit player, session, registry, dimension, chunk/view, and packet-order facts.
- `map_movement_update` maps one movement update to one known player/session/entity/dimension and rejects stale, lossy, malformed, or ambiguous inputs.
- `plan_chat_route` returns deterministic recipients for authorized chat/broadcast routes and rejects malformed, unauthorized, closed-client, or empty-recipient inputs.
- `apply_bridge_harness_event` is the thin fixture harness: it gathers explicit harness state, calls the pure cores, and records only returned shell intents. When disabled, it returns `BridgeDisabled` and preserves state unchanged.
- `valence_network` exports the module; `NetworkPlugin` does not install it or change direct networking behavior.
- `valence_network` README documents the optional harness and non-claims.

## Positive fixture coverage

- Valid join/chunk facts produce deterministic packet order and chunk delivery plan.
- Valid movement maps exactly one known entity for the known player/session/dimension.
- Valid chat/broadcast route returns deterministic recipients in sorted order.
- Enabled harness records only approved join shell intents.
- Full `valence_network` library tests pass after adding the bridge module.

## Negative fixture coverage

- Missing registry facts reject join planning.
- Missing chunk facts reject initial chunk planning.
- Unsupported packet order rejects join planning before delivery.
- Oversized initial radius rejects chunk planning.
- Stale sessions reject movement.
- Entity mismatches reject movement.
- NaN position and invalid pitch reject movement.
- Unauthorized recipients and closed clients reject route planning.
- Disabled harness preserves direct state and emits no shell intent.
- Unknown movement session in the harness rejects without mutation.

## Validation evidence

- Baseline before bridge implementation: `docs/evidence/prototype-valence-hyperion-bridge-slice-baseline-valence-network-2026-06-30.run.log`.
- Formatting: `docs/evidence/prototype-valence-hyperion-bridge-slice-fmt-check-2026-06-30.run.log`.
- Focused bridge tests: `docs/evidence/prototype-valence-hyperion-bridge-slice-tests-final-2026-06-30.run.log`.
- Full `valence_network` library tests: `docs/evidence/prototype-valence-hyperion-bridge-slice-valence-network-tests-2026-06-30.run.log`.
- Focused changed-package clippy with `--no-deps`: `docs/evidence/prototype-valence-hyperion-bridge-slice-valence-network-clippy-nodeps-2026-06-30.run.log`.

A broader `cargo clippy -p valence_network --all-targets -- -D warnings` run is blocked by pre-existing workspace/dependency lint debt in `valence_text` and `valence_generated`; the focused `--no-deps` clippy run validates the changed `valence_network` code.

## Non-claims

This bridge slice is a prototype fixture harness. It does not send real packets, does not mutate live ECS state, does not implement a Hyperion proxy adapter, does not enable any Valence default behavior, does not prove production throughput, does not prove Hyperion wire compatibility, does not prove Bedwars compatibility, and does not prove vanilla gameplay parity.
