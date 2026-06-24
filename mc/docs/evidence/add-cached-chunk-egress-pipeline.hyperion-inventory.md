# Cached chunk egress Hyperion boundary inventory

## Question

Which Hyperion chunk egress/cache concepts are safe to use for Valence cached chunk egress?

## Inspected evidence

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion/src/egress/sync_chunks.rs` (`send_full_loaded_chunks`) | reference | `add-cached-chunk-egress-pipeline` | Hyperion sends prebuilt `chunk.base_packet_bytes` from loaded columns; this motivates Valence-owned optional cached egress but no code is copied. | `servers/valence/crates/valence_server/src/layer/chunk/*` | Hyperion code is nightly/ECS/runtime-coupled; Valence keeps stable Rust, existing layer APIs, and thin send shells. | Focused Valence chunk tests in `docs/evidence/add-cached-chunk-egress-pipeline.valence-chunk-tests.run.log`. | No Hyperion compatibility, map-loader parity, production-scale claim, or default behavior claim. |
| `hyperion/crates/hyperion/src/simulation/blocks/chunk.rs` (`Column::base_packet_bytes`) | reference | `add-cached-chunk-egress-pipeline` | Hyperion stores raw packet bytes next to chunk data; Valence ports the idea as a keyed cache around its own chunk snapshots and renderer. | `LoadedChunk` optional cached init-packet path | No Hyperion data structures or packed unsafe layout are imported. Valence invalidates on its existing observable mutation paths. | Cache hit, block/biome/entity invalidation, missing-light fail-closed, and stale-version tests in focused log. | No world generation semantics, Hyperion map-loader parity, or broad chunk correctness claim. |
| `hyperion/crates/hyperion-proxy/src/cache.rs` (`BufferedEgress`) | reference | `add-cached-chunk-egress-pipeline` | Proxy-side locality/broadcast cache stays out of Valence core; it only confirms the boundary between packet bytes and network flushing. | `none` for proxy behavior; Valence keeps network writes in existing client shell. | No proxy runtime, mTLS, BVH broadcast, or public-server load behavior is adopted. | Direct layer regressions and mc-compat dry-run are recorded separately before archive. | No production load, proxy compatibility, public-server safety, or Hyperion proxy compatibility claim. |

## Decision

Port the idea only: Valence owns a deterministic chunk egress cache core over explicit packet snapshots and render settings, while storage and network writes remain in `LoadedChunk`/client shells. The optional cache is disabled by default and existing uncached rendering remains available.

## Owner

`add-cached-chunk-egress-pipeline` Cairn change.

## Next action

Run direct Valence layer regressions, selected mc-compat dry-run evidence, evidence manifest checks, Cairn gates, sync, and archive before claiming closeout.
