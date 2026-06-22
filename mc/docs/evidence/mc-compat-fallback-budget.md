# mc-compat scenario fallback budget

## Report shape

`tools/check_scenario_manifest.rs` reads `compat/config/scenario-manifest.ncl` plus `compat/config/scenario-fallback-budget-baseline.ncl` and prints:

```text
fallback budget: approved=[...]; removed=[...]; new=[...]; typed_event_regressions=[...]; missing_waiver_metadata=[...]
```

- `approved` names current `substring-fallback` rows that are still present in the checked baseline with owner, reason, non-claim, and next-action waiver metadata.
- `removed` names baseline fallback rows that left fallback, which is migration progress.
- `new` names unapproved current fallback rows and fails the gate.
- `typed_event_regressions` names rows recorded as typed-event-ready in the baseline that moved back to fallback and fails the gate.
- `missing_waiver_metadata` names baseline rows with incomplete waiver metadata and fails the gate.

## Non-claim boundary

The fallback-budget report is migration accounting only. It does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, production readiness, broad Minecraft compatibility, gameplay correctness, or row-level semantic parity for fallback scenarios.
