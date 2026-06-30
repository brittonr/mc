# Tasks

- [ ] [serial] Read Stevenarella workflow docs, inventory `world`, `model`, `ui`, `ecs`, and `control` responsibilities/public APIs, choose the first migration wave, and run baseline focused tests. r[mc_compatibility.stevenarella_hotspot_modules.inventory]
- [ ] [serial] Convert selected hotspot modules into thin façades over focused child modules while preserving public names or documenting intentional local call-site updates. r[mc_compatibility.stevenarella_hotspot_modules.facades]
- [ ] [serial] Extract deterministic parsing, normalization, layout, ECS planning, and state-transition logic into pure helpers while leaving renderer, GL, filesystem, network, input, and global state effects in shells. r[mc_compatibility.stevenarella_hotspot_modules.core_shell]
- [ ] [serial] Preserve default client behavior, compat instrumentation boundaries, module API compatibility where practical, protocol behavior, rendering behavior for touched paths, and non-claim boundaries. r[mc_compatibility.stevenarella_hotspot_modules.parity]
- [ ] [serial] Add positive tests for extracted pure logic and negative tests for invalid inputs, missing resources, empty collections, malformed state, unsupported layouts, and API drift. r[mc_compatibility.stevenarella_hotspot_modules.tests]
- [ ] [serial] Run focused Stevenarella tests, affected mc-compat dry-runs if instrumentation behavior changes, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks with reviewable logs before archive. r[mc_compatibility.stevenarella_hotspot_modules.validation]
