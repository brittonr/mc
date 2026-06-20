# Tasks

- [x] [serial] Capture baseline runner tests and define the planning-core contract, side-effect boundary, public-behavior parity scope, and non-claim boundary. r[mc_compatibility.harness_planning_core.contract]
  Evidence: docs/evidence/harness-planning-core-baseline-2026-06-20.run.log; docs/evidence/harness-planning-core-baseline-2026-06-20.b3
- [x] [depends:contract] Extract pure server, client-session, receipt, artifact, and cleanup plan structs with deterministic constructors from validated config and scenario metadata. r[mc_compatibility.harness_planning_core.plan_structs]
  Evidence: docs/evidence/harness-planning-core-2026-06-20.run.log; docs/evidence/harness-planning-core-2026-06-20.b3
- [x] [depends:plan_structs] Keep process execution, filesystem mutation, Docker calls, environment mutation, sleeps, clocks, and network probes in thin shell functions that consume plans. r[mc_compatibility.harness_planning_core.shell]
  Evidence: docs/evidence/harness-planning-core-2026-06-20.run.log; docs/evidence/harness-planning-core-2026-06-20.b3
- [x] [depends:shell] Add positive plan fixtures for dry-run, live, matrix, reconnect, multi-client, Paper, Valence, cleanup, and failure-bundle paths. r[mc_compatibility.harness_planning_core.positive_tests]
  Evidence: docs/evidence/harness-planning-core-2026-06-20.run.log; docs/evidence/harness-planning-core-2026-06-20.b3
- [x] [depends:positive_tests] Add negative plan fixtures for invalid backend/config combinations, unsafe public-server inputs, missing receipt destinations, matrix flag conflicts, path hazards, and cleanup hazards. r[mc_compatibility.harness_planning_core.negative_tests]
  Evidence: docs/evidence/harness-planning-core-2026-06-20.run.log; docs/evidence/harness-planning-core-2026-06-20.b3
- [x] [depends:negative_tests] Run post-refactor runner tests, scenario-manifest checks, affected dry-run checks, evidence manifest checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.harness_planning_core.validation]
  Evidence: docs/evidence/harness-planning-core-baseline-2026-06-20.run.log; docs/evidence/harness-planning-core-2026-06-20.run.log; docs/evidence/harness-planning-core-evidence-manifest-2026-06-20.run.log; docs/evidence/harness-planning-core-baseline-2026-06-20.b3; docs/evidence/harness-planning-core-2026-06-20.b3; docs/evidence/harness-planning-core-evidence-manifest-2026-06-20.b3
