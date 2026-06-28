# Tasks

- [ ] [serial] Capture baseline client-driver behavior for dry-run, single-client, reconnect, multi-client, projectile, server-correlation, and failure paths. r[mc_compatibility.runner_modularity.client_driver_core]
- [ ] [depends:client_driver_core] Extract pure client run planning for usernames, sessions, timeouts, log strategies, restart needs, and dry-run evidence selection. r[mc_compatibility.runner_modularity.client_run_planning]
- [ ] [depends:client_run_planning] Extract pure combined-output, scenario evaluation, server-correlation, projectile evidence, and classification logic from process execution. r[mc_compatibility.runner_modularity.client_evidence_classification]
- [ ] [depends:client_evidence_classification] Keep Xvfb/process spawning, timeouts, log reads/writes, server restarts, and stdout/stderr in thin shell functions. r[mc_compatibility.runner_modularity.client_driver_shell]
- [ ] [depends:client_driver_shell] Add positive tests for dry-run evidence, successful single-client, reconnect, multi-client, projectile, and timeout-success classifications. r[mc_compatibility.runner_modularity.client_driver_positive_tests]
- [ ] [depends:client_driver_positive_tests] Add negative tests for missing milestones, forbidden markers, bad exit codes, missing server correlation, projectile order failures, and restart-state failures. r[mc_compatibility.runner_modularity.client_driver_negative_tests]
- [ ] [depends:client_driver_negative_tests] Run focused client-driver tests, runner tests, dry-run smoke checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_modularity.client_driver_validation]
