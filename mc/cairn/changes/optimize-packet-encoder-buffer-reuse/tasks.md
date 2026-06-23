# Tasks

- [ ] [serial] Audit Hyperion encoder/buffer reuse patterns and Valence packet encode/flush paths, then record optimization scope and non-goals. r[valence_hyperion_integration.packet_buffer_reuse.audit]
- [ ] [depends:audit] Define buffer lifecycle, compression threshold handling, capacity policy, reset/discard policy, packet limit behavior, and safety invariants. r[valence_hyperion_integration.packet_buffer_reuse.contract]
- [ ] [depends:contract] Add baseline allocation or benchmark evidence for selected encode/flush workloads before implementation. r[valence_hyperion_integration.packet_buffer_reuse.baseline]
- [ ] [depends:baseline] Implement reusable encoder buffers or pools where evidence and invariants justify the change. r[valence_hyperion_integration.packet_buffer_reuse.implementation]
- [ ] [depends:implementation] Add positive and negative tests for compressed/uncompressed packets, reuse after errors, oversized packets, malformed compression state, closed clients, stale bytes, and default behavior. r[valence_hyperion_integration.packet_buffer_reuse.tests]
- [ ] [depends:tests] Run encode tests, compression edge tests, direct-mode regressions, before/after benchmarks, selected mc-compat dry runs, Cairn gates, and Cairn validation. r[valence_hyperion_integration.packet_buffer_reuse.validation]
