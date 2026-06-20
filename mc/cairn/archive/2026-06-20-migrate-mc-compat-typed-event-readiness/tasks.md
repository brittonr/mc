# Tasks

- [x] [serial] Define typed-event readiness criteria, fallback waiver metadata, and the non-claim boundary for observability-only migration. r[mc_compatibility.typed_event_readiness.contract]
  Evidence: docs/evidence/typed-event-readiness-2026-06-20.run.log; docs/evidence/typed-event-readiness-2026-06-20.b3
- [x] [depends:contract] Extend the manifest checker with pure readiness evaluation and positive/negative fixtures for ready rows, missing client/server typed events, forbidden-pattern gaps, and waiver-backed fallback. r[mc_compatibility.typed_event_readiness.checker]
  Evidence: docs/evidence/typed-event-readiness-2026-06-20.run.log; docs/evidence/typed-event-readiness-2026-06-20.b3
- [x] [depends:checker] Migrate eligible scenario rows from `substring-fallback` to `typed-event-ready` with parity fixtures that preserve milestone IDs and receipt wording. r[mc_compatibility.typed_event_readiness.migration]
  Evidence: docs/evidence/typed-event-readiness-2026-06-20.run.log; docs/evidence/typed-event-readiness-2026-06-20.b3
- [x] [depends:migration] Update runner receipt/oracle tests so typed-event-ready rows fail closed on missing typed events before relying on substring fallback. r[mc_compatibility.typed_event_readiness.tests]
  Evidence: docs/evidence/typed-event-readiness-2026-06-20.run.log; docs/evidence/typed-event-readiness-2026-06-20.b3
- [x] [depends:tests] Update README/evidence docs to explain typed-event readiness, fallback waivers, and reviewer expectations. r[mc_compatibility.typed_event_readiness.docs]
  Evidence: docs/evidence/typed-event-readiness-2026-06-20.run.log; docs/evidence/typed-event-readiness-2026-06-20.b3
- [x] [depends:docs] Run focused runner tests, scenario-manifest checks, affected dry-run checks, evidence manifest checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.typed_event_readiness.validation]
  Evidence: docs/evidence/typed-event-readiness-2026-06-20.run.log; docs/evidence/typed-event-readiness-2026-06-20.b3
