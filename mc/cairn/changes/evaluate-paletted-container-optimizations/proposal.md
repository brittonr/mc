# Proposal: Evaluate paletted container optimizations

## Why

Hyperion has a paletted container implementation optimized for chunk sections, unique-block queries, and compact transitions between single, indirect, and direct storage. Valence already has chunk paletted containers, so integration should start with measurement and parity checks before any optimization is ported.

## What Changes

- Compare Hyperion's paletted container behavior with Valence's chunk paletted container implementation.
- Define correctness invariants for indexing, palette transitions, unique-count queries, iteration, encode parity, and invalid indices.
- Build benchmarks and fixtures before changing Valence behavior.
- Port only stable, audited concepts that improve measured Valence workloads.
- Add positive and negative tests for single-value sections, palette growth, direct fallback, invalid indices, encode parity, and stale unique caches if any are introduced.

## Impact

- **Files**: Valence chunk/paletted container code, benchmark fixtures, tests, docs/performance notes, and Cairn artifacts.
- **Testing**: correctness fixtures, encode parity checks, negative index/bounds cases, before/after benchmarks, mc-compat chunk dry runs if behavior changes, and Cairn gates/validation.
- **Non-claims**: this does not import Hyperion's nightly SIMD or unsafe-heavy code directly and does not change client-visible chunks without parity evidence.
