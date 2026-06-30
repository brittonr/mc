# Tasks

- [ ] [serial] Read Stevenarella `AGENTS.md` and `README.md`, inventory current `server/mod.rs` handler/probe responsibilities, and run the smallest relevant baseline server/probe tests. r[mc_compatibility.stevenarella_server_modularization.inventory]
- [ ] [serial] Extract focused server modules for login/session, chunks/world, entities, inventory/windows, block entities/signs, chat/plugin messages, and dispatch helpers without changing packet semantics. r[mc_compatibility.stevenarella_server_modularization.module_boundaries]
- [ ] [serial] Group mc-compat probe state into cohesive CTF, inventory, survival, combat/projectile, and sign/dimension state modules with pure transition helpers. r[mc_compatibility.stevenarella_server_modularization.probe_state]
- [ ] [serial] Preserve packet dispatch behavior, compat milestone/event vocabulary, environment variable contracts, receipt non-claims, and default non-instrumented client behavior. r[mc_compatibility.stevenarella_server_modularization.parity]
- [ ] [serial] Add positive tests for representative extracted handlers/probe transitions and negative tests for malformed packet/probe inputs, invalid state transitions, missing windows/entities, and disabled probes. r[mc_compatibility.stevenarella_server_modularization.tests]
- [ ] [serial] Run focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks with reviewable logs before archive. r[mc_compatibility.stevenarella_server_modularization.validation]
