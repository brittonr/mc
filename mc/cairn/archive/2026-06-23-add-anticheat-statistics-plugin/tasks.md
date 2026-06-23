# Tasks

- [x] [serial] Audit Hyperion statistics behavior and Valence event sources, then define metric scope and non-goals. r[valence_hyperion_integration.anticheat_stats.scope]
  Evidence: docs/evidence/add-anticheat-statistics-plugin.scope.md; docs/evidence/add-anticheat-statistics-plugin.valence-server-anticheat.run.log; docs/evidence/add-anticheat-statistics-plugin.b3
- [x] [depends:scope] Implement a stable pure statistics core for selected rolling/counting metrics with explicit sample windows. r[valence_hyperion_integration.anticheat_stats.core]
  Evidence: docs/evidence/add-anticheat-statistics-plugin.valence-server-anticheat.run.log; docs/evidence/add-anticheat-statistics-plugin.valence-server-lib.run.log; docs/evidence/add-anticheat-statistics-plugin.b3
- [x] [depends:core] Add positive and negative fixtures for normal samples, burst samples, empty windows, invalid windows, overflow boundaries, and reset behavior. r[valence_hyperion_integration.anticheat_stats.fixtures]
  Evidence: docs/evidence/add-anticheat-statistics-plugin.valence-server-anticheat.run.log; docs/evidence/add-anticheat-statistics-plugin.b3
- [x] [depends:fixtures] Wire an optional plugin that samples explicit Valence event streams and emits observations without default enforcement. r[valence_hyperion_integration.anticheat_stats.plugin]
  Evidence: docs/evidence/add-anticheat-statistics-plugin.valence-server-anticheat.run.log; docs/evidence/add-anticheat-statistics-plugin.valence-server-lib.run.log; docs/evidence/add-anticheat-statistics-plugin.b3
- [x] [depends:plugin] Document limitations, false-positive risks, data retention, and non-claims. r[valence_hyperion_integration.anticheat_stats.docs]
  Evidence: docs/evidence/add-anticheat-statistics-plugin.scope.md; docs/evidence/add-anticheat-statistics-plugin.valence-fmt-check.run.log; docs/evidence/add-anticheat-statistics-plugin.b3
- [x] [depends:docs] Run statistics tests, plugin-off regressions, sampling smoke tests, Cairn gates, and Cairn validation. r[valence_hyperion_integration.anticheat_stats.validation]
  Evidence: docs/evidence/add-anticheat-statistics-plugin.valence-server-baseline.pre.run.log; docs/evidence/add-anticheat-statistics-plugin.valence-server-anticheat.run.log; docs/evidence/add-anticheat-statistics-plugin.valence-server-lib.run.log; docs/evidence/add-anticheat-statistics-plugin.valence-fmt-check.run.log; docs/evidence/add-anticheat-statistics-plugin.valence-server-clippy.run.log; docs/evidence/add-anticheat-statistics-plugin.cairn-gates-final.run.log; docs/evidence/add-anticheat-statistics-plugin.cairn-validation-final.run.log; docs/evidence/add-anticheat-statistics-plugin.b3
