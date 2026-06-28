# Extract Hyperion player join core: boundary and responsibilities

## Question

Should `extract-hyperion-player-join-core` adopt or port Hyperion player-join code into Valence?

## Inspected evidence

- `cairn/changes/extract-hyperion-player-join-core/proposal.md`
- `cairn/changes/extract-hyperion-player-join-core/design.md`
- `cairn/changes/extract-hyperion-player-join-core/specs/valence-hyperion-integration/spec.md`
- `docs/hyperion-integration-boundaries.md`
- `hyperion/crates/hyperion/src/egress/player_join/mod.rs`
- `hyperion/crates/hyperion/src/egress/player_join/core.rs`

## Hyperion source revision

The Hyperion-local implementation was committed in the nested Hyperion repository as `9b844f2a552fad79f5e0c40d4a6e4083f68c6178` (`extract player join decisions for testable boundaries`). This parent-repo evidence note records that revision for review; it does not promote Valence adoption or mc-compat compatibility claims.

## Classification

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion/src/egress/player_join/mod.rs` | reference | `extract-hyperion-player-join-core` | The change is Hyperion-owned nested-repo modularity work; the source is inspected to split Hyperion shell responsibilities from a Hyperion pure core, not to adopt or port behavior into Valence. | none | No Valence code is changed; ECS reads, packet sends, network/proxy state, tracing, scheduling, and runtime mutation remain in the Hyperion shell. | `docs/evidence/extract-hyperion-player-join-core.focused.hyperion-player-join-tests.run.log` | No Valence adoption, no mc-compat evidence claim, no broad Minecraft compatibility, no production readiness, no public-server safety, no CTF/survival correctness claim. |

## Captured pre-extraction responsibilities

Before this change, `player_join/mod.rs` directly combined these responsibilities in one egress system:

- read ECS target components and handle missing join target state;
- derive registry codec and dimension names;
- construct `GameJoinS2c`, chunk render-distance center, and spawn-position packets;
- append cached world data for tags, brand, team creation, crafting, and recipe unlocks;
- broadcast join chat;
- collect other players and construct player-list entries;
- broadcast and unicast the joining player's player-list entry;
- broadcast and unicast team membership updates;
- flush the unicast bundle and enable receive broadcasts;
- insert runtime movement, teleportation, channel, and packet-state components;
- record tracing/log diagnostics.

## Decision

Keep implementation and validation Hyperion-local. `core.rs` now owns deterministic join-plan decisions for server facts validation, target/peer state summaries, chunk/view setup facts, initial packet facts, player-list summaries, packet order, and diagnostics. `mod.rs` remains the imperative shell for ECS access, packet emission, cached packet bytes, network/proxy state, tracing, scheduling, and runtime state insertion.

## Owner

`extract-hyperion-player-join-core`

## Next action

Use Hyperion-focused tests and Cairn gates as closeout evidence; do not promote Valence adoption or compatibility claims from this change.
