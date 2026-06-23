# Proposal: Optimize packet encoder buffer reuse

## Why

Hyperion emphasizes reusable packet buffers and compression-aware egress paths. Valence may benefit from reducing allocation churn in packet encoding and flushing, especially for high-frequency entity, chunk, and broadcast packets. This should be handled separately from byte-backed ingress so compression and malformed-packet behavior stay testable.

## What Changes

- Audit Hyperion packet encoder/buffer reuse patterns and Valence packet encoding/flush paths.
- Define buffer lifecycle, compression threshold, capacity growth/shrink, error handling, and safety invariants.
- Implement reusable buffer pools or encoder state only where measurements show allocation or latency improvement.
- Add positive and negative tests for compressed/uncompressed packets, buffer reuse after errors, oversized packets, malformed compression state, closed clients, and plugin-disabled/default behavior.
- Record before/after allocation or benchmark evidence before claiming performance improvement.

## Impact

- **Files**: `valence_protocol`, `valence_network`, flush/packet encoding helpers, benchmarks/tests, docs/performance notes, and Cairn artifacts.
- **Testing**: packet encode fixtures, compression edge cases, malformed packet tests, direct-mode regressions, benchmarks/allocation checks, selected mc-compat dry runs, and Cairn gates/validation.
- **Non-claims**: this does not change protocol semantics and does not replace byte-backed protocol work.
