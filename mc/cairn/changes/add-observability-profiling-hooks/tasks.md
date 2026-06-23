# Tasks

- [ ] [serial] Review Hyperion observability/profiling patterns and Valence tracing/logging surfaces, then record hook scope and non-goals. r[valence_hyperion_integration.observability.scope]
- [ ] [depends:scope] Define span/metric names, bounded labels, redaction policy, overhead expectations, exporter/profiler adapter boundaries, and disabled-mode behavior. r[valence_hyperion_integration.observability.contract]
- [ ] [depends:contract] Implement pure metric/span classification helpers with exporter/profiler integration as optional shells. r[valence_hyperion_integration.observability.core]
- [ ] [depends:core] Wire optional hooks for selected tick, network, chunk, entity, or plugin phases without changing default behavior. r[valence_hyperion_integration.observability.wiring]
- [ ] [depends:wiring] Add positive and negative tests for disabled hooks, enabled labels, redaction, unknown metric names, exporter failure, and overhead checks if claims are made. r[valence_hyperion_integration.observability.tests]
- [ ] [depends:tests] Run observability tests, plugin-disabled regressions, smoke trace/export checks, overhead checks if claimed, Cairn gates, and Cairn validation. r[valence_hyperion_integration.observability.validation]
