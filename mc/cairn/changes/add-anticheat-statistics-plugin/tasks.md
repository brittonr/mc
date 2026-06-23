# Tasks

- [ ] [serial] Audit Hyperion statistics behavior and Valence event sources, then define metric scope and non-goals. r[valence_hyperion_integration.anticheat_stats.scope]
- [ ] [depends:scope] Implement a stable pure statistics core for selected rolling/counting metrics with explicit sample windows. r[valence_hyperion_integration.anticheat_stats.core]
- [ ] [depends:core] Add positive and negative fixtures for normal samples, burst samples, empty windows, invalid windows, overflow boundaries, and reset behavior. r[valence_hyperion_integration.anticheat_stats.fixtures]
- [ ] [depends:fixtures] Wire an optional plugin that samples explicit Valence event streams and emits observations without default enforcement. r[valence_hyperion_integration.anticheat_stats.plugin]
- [ ] [depends:plugin] Document limitations, false-positive risks, data retention, and non-claims. r[valence_hyperion_integration.anticheat_stats.docs]
- [ ] [depends:docs] Run statistics tests, plugin-off regressions, sampling smoke tests, Cairn gates, and Cairn validation. r[valence_hyperion_integration.anticheat_stats.validation]
