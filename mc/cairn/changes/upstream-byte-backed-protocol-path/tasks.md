# Tasks

- [ ] [serial] Audit Hyperion fork usage and current Valence protocol/event surfaces, then record required byte-backed capabilities and non-goals. r[valence_hyperion_integration.byte_protocol.audit]
- [ ] [depends:audit] Define stable byte-backed raw-payload and validated text/byte field APIs with explicit ownership and lifetime invariants. r[valence_hyperion_integration.byte_protocol.api]
- [ ] [depends:api] Implement packet framing, compression, and validation logic as pure cores with thin network/event-loop shells. r[valence_hyperion_integration.byte_protocol.core]
- [ ] [depends:core] Add positive and negative fixtures for complete/split/compressed frames, malformed VarInts, zero-length packets, oversized packets, invalid compression, invalid UTF data, and stale ownership cases. r[valence_hyperion_integration.byte_protocol.fixtures]
- [ ] [depends:fixtures] Migrate selected event-loop packet paths behind compatibility shims or feature flags while preserving existing direct-mode behavior. r[valence_hyperion_integration.byte_protocol.migration]
- [ ] [depends:migration] Run protocol tests, event-loop regressions, selected mc-compat dry runs, Cairn gates, and Cairn validation with reviewable logs. r[valence_hyperion_integration.byte_protocol.validation]
