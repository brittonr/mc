# Tasks

- [ ] [serial] Review Hyperion scheduler code and Valence tick/timer patterns, then record scheduler scope and non-goals. r[valence_hyperion_integration.tick_scheduler.scope]
- [ ] [depends:scope] Define the scheduler API, key ordering, drain behavior, equal-key policy, cancellation policy, and error model. r[valence_hyperion_integration.tick_scheduler.contract]
- [ ] [depends:contract] Implement a pure scheduler core over explicit ordered keys and values with no clocks, I/O, or ECS access. r[valence_hyperion_integration.tick_scheduler.core]
- [ ] [depends:core] Add positive and negative fixtures for empty queues, due work, not-due work, same-tick ordering, clear/cancel behavior, overflow boundaries, and invalid keys if applicable. r[valence_hyperion_integration.tick_scheduler.fixtures]
- [ ] [depends:fixtures] Wire an optional Valence plugin or utility shell and add docs/examples for cooldowns, despawns, and temporary blocks. r[valence_hyperion_integration.tick_scheduler.wiring]
- [ ] [depends:wiring] Run scheduler tests, plugin smoke tests, example timer checks, Cairn gates, and Cairn validation with reviewable logs. r[valence_hyperion_integration.tick_scheduler.validation]
