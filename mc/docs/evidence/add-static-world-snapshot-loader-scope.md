# Static world snapshot loader scope

## Question

What Hyperion loading ideas are safe to use for a Valence static world snapshot loader?

## Inspected evidence

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion-genmap/src/lib.rs` (`GenMapPlugin`) | reference | `add-static-world-snapshot-loader` | Shows a static map startup flow that downloads/caches a save and installs preloaded blocks, but the download/bootstrap policy is event-specific. | `servers/valence/crates/valence_anvil` docs/API only | No code copied; no default Valence startup behavior changed; avoids external download policy. | `docs/evidence/add-static-world-snapshot-loader-valence-anvil-full-tests.run.log` | No Hyperion map-loader parity, production startup guarantee, external asset policy, or default behavior claim. |
| `hyperion/crates/hyperion/src/simulation/blocks/manager.rs` (`RegionManager`) | reference | `add-static-world-snapshot-loader` | Useful as a checklist for region-file discovery, async ownership, and cache boundaries. | `servers/valence/crates/valence_anvil/src/snapshot.rs` | No code copied; Valence keeps synchronous filesystem access in a shell around pure validation; mmap is not adopted. | `docs/evidence/add-static-world-snapshot-loader-valence-anvil-full-tests.run.log` | No async runtime replacement, mmap safety claim, or throughput claim. |
| `hyperion/crates/hyperion/src/simulation/blocks/region.rs` (`Region`) | reject | `add-static-world-snapshot-loader` | Hyperion's mmap-backed region reader is tied to its runtime/cache design and includes unsafe mmap use that is unnecessary for first Valence scope. | none | Rejected for direct adoption; Valence continues to use its existing `RegionFolder` reader/decompression path. | `docs/evidence/add-static-world-snapshot-loader-valence-anvil-clippy-nodeps.run.log` | No mmap parity, no direct unsafe adoption, no performance claim. |
| `hyperion/crates/hyperion/src/simulation/blocks/loader/parse.rs` (`parse_chunk`) | reference | `add-static-world-snapshot-loader` | Mirrors Valence Anvil parsing concepts and validates that dimension/biome checks belong in Valence-owned pure code before layer mutation. | `servers/valence/crates/valence_anvil/src/snapshot.rs` | No code copied; implementation calls existing Valence parsing and adds Valence-owned summary validation. | `docs/evidence/add-static-world-snapshot-loader-valence-anvil-full-tests.run.log` | No full chunk semantic equivalence, lighting parity, or Hyperion compatibility claim. |
| `hyperion/crates/hyperion/src/egress/sync_chunks.rs` | reference | `add-static-world-snapshot-loader` | Shows that cached/pre-encoded chunk egress is a separate send-path concern from snapshot validation. | `servers/valence/crates/valence_anvil/README.md` | No egress code copied; snapshot loader only produces `UnloadedChunk` values and leaves cached egress opt-in on `ChunkLayer`. | `docs/evidence/add-static-world-snapshot-loader-chunk-dimension-dry-run.run.log` | No cached-egress performance claim, protocol parity claim, or default send-path change. |

## Decision

Use Hyperion as reference-only evidence for startup scope, region discovery, and cached-egress separation. Do not adopt Hyperion mmap, runtime, proxy, Bedwars, or packet egress code. Implement a Valence-owned static snapshot loader in `valence_anvil` with a pure typed plan and chunk normalization core, and keep filesystem discovery, decompression, cancellation, and layer application in shells.

## Static-world scope

The loader covers static snapshots and controlled reload inputs. It validates selected regions/chunks, dimension bounds, biome identifiers, resource limits, missing-file policy, partial-load policy, and cancellation boundaries before client-visible layer mutation.

## Non-goals

This change does not add terrain generation, arbitrary save editing, Hyperion loader parity, production startup readiness, public-server safety, vanilla/reference parity, full CTF/survival correctness, or broad Minecraft compatibility. Memory mapping remains optional adapter policy and is not used by the implemented Valence shell.

## Cached chunk egress interaction

Snapshot loading produces `UnloadedChunk` values and applies them to `ChunkLayer` only after descriptor validation. The existing cached chunk egress flag remains a separate `ChunkLayer` send-path choice; this change does not enable it by default or claim cached-egress performance.

## Owner

`add-static-world-snapshot-loader`

## Next action

Keep future Hyperion-derived loader work behind the same boundary table and add paired evidence before making parity, performance, or default-behavior claims.
