# Tasks

- [x] [serial] Inventory selected Valence tick counters, modulo checks, cooldowns, delayed resets, temporary world changes, and despawn timers. r[valence_bevy_ecs.scheduler_adoption.inventory]
  - Evidence: docs/evidence/adopt-tick-scheduler.inventory.md; docs/evidence/adopt-tick-scheduler.schedule-hygiene.run.log; docs/evidence/adopt-tick-scheduler.evidence.b3
- [x] [depends:inventory] Classify each timing behavior as scheduler-suitable, immediate state, wall-clock/async work, or intentionally custom policy. r[valence_bevy_ecs.scheduler_adoption.classification]
  - Evidence: docs/evidence/adopt-tick-scheduler.inventory.md; docs/evidence/adopt-tick-scheduler.schedule-hygiene.run.log; docs/evidence/adopt-tick-scheduler.evidence.b3
- [x] [depends:classification] Replace scheduler-suitable ad hoc delays with `ServerTickScheduler` resources or typed scheduler events. r[valence_bevy_ecs.scheduler_adoption.wiring]
  - Evidence: docs/evidence/adopt-tick-scheduler.combat-example.final.run.log; docs/evidence/adopt-tick-scheduler.valence-server-tick-scheduler.final.run.log; docs/evidence/adopt-tick-scheduler.evidence.b3
- [x] [depends:wiring] Keep gameplay policy durations, compatibility milestone choices, and mutation decisions outside the scheduler core. r[valence_bevy_ecs.scheduler_adoption.policy]
  - Evidence: docs/evidence/adopt-tick-scheduler.inventory.md; docs/evidence/adopt-tick-scheduler.combat-example.final.run.log; docs/evidence/adopt-tick-scheduler.evidence.b3
- [x] [depends:wiring] Add positive due-work tests and negative cancellation, stale entity, duplicate event, invalid tick, and plugin-disabled tests. r[valence_bevy_ecs.scheduler_adoption.tests]
  - Evidence: docs/evidence/adopt-tick-scheduler.combat-example.final.run.log; docs/evidence/adopt-tick-scheduler.evidence.b3
- [x] [depends:tests] Run focused scheduler/example checks, selected mc-compat rails if fixture timing changes, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks with promoted logs. r[valence_bevy_ecs.scheduler_adoption.validation]
  - Evidence: docs/evidence/adopt-tick-scheduler.combat-example.final.run.log; docs/evidence/adopt-tick-scheduler.valence-server-tick-scheduler.final.run.log; docs/evidence/adopt-tick-scheduler.valence-fmt.after-format.run.log; docs/evidence/adopt-tick-scheduler.schedule-hygiene.run.log; docs/evidence/adopt-tick-scheduler.accepted-spec-verify.run.log; docs/evidence/adopt-tick-scheduler.gate-proposal.after-sync.run.log; docs/evidence/adopt-tick-scheduler.gate-design.after-sync.run.log; docs/evidence/adopt-tick-scheduler.gate-tasks.after-sync.run.log; docs/evidence/adopt-tick-scheduler.validate.after-sync.run.log; docs/evidence/adopt-tick-scheduler.evidence.b3
