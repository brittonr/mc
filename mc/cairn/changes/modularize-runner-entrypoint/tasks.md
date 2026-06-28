# Tasks

- [ ] [serial] Capture the current `main.rs` responsibility map and baseline runner checks before moving entrypoint-owned behavior. r[mc_compatibility.runner_modularity.entrypoint_boundary]
- [ ] [depends:entrypoint_boundary] Extract config, backend runtime, app/mode dispatch, planning types, receipt shell, failure-bundle shell, and scenario behavior adapters into cohesive owner modules while keeping `main.rs` thin. r[mc_compatibility.runner_modularity.entrypoint_modules]
- [ ] [depends:entrypoint_modules] Preserve public CLI, environment, receipt schema, scenario behavior, and non-claim parity through compatibility adapters. r[mc_compatibility.runner_modularity.entrypoint_parity]
- [ ] [depends:entrypoint_parity] Add positive tests for representative dry-run, run, build-client, status, cleanup, matrix, receipt, and failure-bundle paths after extraction. r[mc_compatibility.runner_modularity.entrypoint_positive_tests]
- [ ] [depends:entrypoint_positive_tests] Add negative tests for unknown args, missing option values, unsafe cleanup/path plans, receipt/failure-bundle follow-up errors, and invalid mode combinations. r[mc_compatibility.runner_modularity.entrypoint_negative_tests]
- [ ] [depends:entrypoint_negative_tests] Run focused runner tests, dry-run smoke checks, scenario manifest checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_modularity.entrypoint_validation]
