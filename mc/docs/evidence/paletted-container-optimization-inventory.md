# Paletted container optimization inventory

Date: 2026-06-23  
Change: `evaluate-paletted-container-optimizations`  
Owner subtree: `servers/valence` with `hyperion/crates/hyperion-palette` as a reference-only source.

## Requirement coverage

- r[valence_hyperion_integration.palette_optimization.inventory]
- r[valence_hyperion_integration.palette_optimization.invariants]
- r[valence_hyperion_integration.palette_optimization.baseline]
- r[valence_hyperion_integration.palette_optimization.port]
- r[valence_hyperion_integration.palette_optimization.tests]
- r[valence_hyperion_integration.palette_optimization.validation]

## Hyperion-to-Valence classification rows

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion-palette/src/lib.rs` (`PalettedContainer::{Single,Indirect,Direct}`, `unique_count`, `unique_blocks`, `instances_of`, iteration) | port | `evaluate-paletted-container-optimizations` | Hyperion's unique-query idea is useful for Valence's section non-air counting, but Valence keeps its own generic container and private API. | `servers/valence/crates/valence_server/src/layer/chunk/paletted_container.rs` and `loaded.rs` | Stable safe Rust only; no copied code; no cached unique state; no public API expansion. | Focused Valence tests, baseline/final benchmark logs, encode parity tests, task evidence gate. | No Hyperion compatibility claim, no production-scale speedup claim, no broad chunk semantic claim. |
| `hyperion/crates/hyperion-palette/src/indirect.rs` (`portable_simd`, unchecked accessors, fixed array palette) | reject | `evaluate-paletted-container-optimizations` | Direct adoption depends on nightly `portable_simd` and unsafe unchecked indexing. Valence already has safe stable `ArrayVec` palette lookup. | none | Nightly and unsafe-heavy implementation is not copied. Stable-safe scan remains the fallback. | Inventory and final validation logs. | No SIMD performance claim and no unsafe audit claim. |
| `hyperion/crates/hyperion-palette/src/encode.rs` (`encode_mc_format`) | reference | `evaluate-paletted-container-optimizations` | The encode shape is already close to Valence's implementation and is used as parity vocabulary only. | `servers/valence/crates/valence_server/src/layer/chunk/paletted_container.rs` tests | No code copied; Valence-owned encode path remains authoritative. | Encode parity fixture comparing direct-vs-indirect direct encodings. | No byte-equivalence claim for all legal representations; no client-visible compatibility claim beyond focused fixture. |
| `hyperion/crates/hyperion/src/simulation/blocks/loader/parse.rs` and Valence `crates/valence_anvil/src/parsing.rs` | reference | `evaluate-paletted-container-optimizations` | Loader parsing confirms both projects treat single-palette sections, palette-index bounds, and compact long arrays as chunk-storage boundaries. | docs/tests only | Reference only; no parser behavior changed. | Inventory note and unchanged Valence parser tests outside scope. | No anvil/chunk-loader behavior change claim. |

## Valence current behavior inventory

| area | Valence behavior before the optimization |
| --- | --- |
| Representation states | `PalettedContainer<T, LEN, HALF_LEN>` stores `Single(T)`, `Indirect(Box<Indirect<...>>)` with an `ArrayVec<T, 16>` palette plus nibble-packed indices, or `Direct(Box<[T; LEN]>)`. |
| Mutation behavior | `set` returns the previous value. `Single` upgrades to `Indirect` on the first distinct value, `Indirect` adds palette entries until the palette is full, and the next distinct value upgrades to `Direct`. `fill` replaces all states with `Single`. `shrink_to_fit` rebuilds compact state and can downgrade `Direct` or `Indirect`. |
| Indexing behavior | `get`/`set` have `debug_assert!(idx < LEN)` and then use normal Rust indexing, so focused tests document the current panic boundary for invalid indices. |
| Encode paths | `Single` encodes zero bits-per-entry plus one palette value. `Indirect` encodes palette entries unless `bits_per_entry > max_indirect_bits`, in which case it encodes direct logical values. `Direct` always encodes direct logical values. |
| Existing non-air count | `Section::count_non_air_blocks` scans every logical block for `Indirect` and `Direct`, while `Single` is constant-time. |
| Unsafe/nightly dependencies | Valence paletted-container logic is stable safe Rust. Hyperion's SIMD and unchecked helpers are not adopted. |

## Correctness invariants

1. `LEN` must be non-zero and `HALF_LEN` must equal `LEN.div_ceil(2)`.
2. `Single` reads every in-range index as the stored value.
3. `Indirect` palette entries are unique and each nibble index must select an existing palette entry.
4. `set` must return the previous logical value and preserve every untouched index.
5. The transition from `Single` to `Indirect` must preserve all old values except the target index.
6. The transition from `Indirect` to `Direct` must preserve the complete logical section state.
7. `fill` must discard stale representation data and make every index read as the fill value.
8. `shrink_to_fit` must preserve logical values while compacting to `Single` or `Indirect` when possible.
9. `encode_mc_format` must encode the same logical values when indirect storage is forced onto the direct encode path.
10. Invalid indices must hit the documented panic boundary and must not corrupt previously valid in-range values.
11. Derived unique/count helpers must remain cache-free or keep cache invalidation proven by mutation tests; this change uses no cache.

## Benchmark fixture plan

The checked-in benchmark fixture lives at `servers/valence/benches/chunk_paletted_container.rs` and is wired through `servers/valence/benches/main.rs`. It names these section distributions and mutation patterns:

- `single_section_read`: a single-value section read workload after `fill_block_state_section`.
- `indirect_palette_growth`: repeated in-section writes cycling over a bounded indirect palette.
- `direct_fallback_mutation`: repeated writes over more than the indirect palette capacity to force direct storage.
- `shrink_direct_to_indirect`: a direct section compacted back toward indirect storage.

The benchmark is workload evidence only. It does not claim broad speedup, production readiness, full chunk compatibility, or client-visible semantic parity.
