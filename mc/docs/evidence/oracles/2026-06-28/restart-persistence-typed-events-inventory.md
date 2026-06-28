# Restart persistence typed-events inventory

## Question

Can `survival-world-persistence-restart`, `survival-crash-recovery-parity`, and `survival-block-entity-persistence-parity` leave substring fallback without broadening their bounded compatibility claims?

## Inspected evidence

| Scenario | Current fallback state before this change | Sessions | Existing tracked receipts | Typed contract promoted by this change |
| --- | --- | ---: | --- | --- |
| `survival-world-persistence-restart` | `migration_state = "substring-fallback"`; fallback-budget row present; typed-event oracle sidecar exists but `contributes_to_pass_fail = false` in existing receipts | 2 | `docs/evidence/survival-world-persistence-paper-2026-06-02.receipt.json`; `docs/evidence/survival-world-persistence-valence-2026-06-02.receipt.json` | client pre-restart update, reconnect, post-restart observation; server mutation, clean shutdown, backend restart, post-restart observation, restored state |
| `survival-crash-recovery-parity` | `migration_state = "substring-fallback"`; fallback-budget row present; typed-event oracle sidecar exists but `contributes_to_pass_fail = false` in existing receipts | 2 | `docs/evidence/survival-crash-recovery-paper-2026-06-04.receipt.json`; `docs/evidence/survival-crash-recovery-valence-2026-06-04.receipt.json` | client pre-crash update, reconnect, post-crash observation; server mutation, forced stop, backend restart, post-crash observation, restored state |
| `survival-block-entity-persistence-parity` | `migration_state = "substring-fallback"`; fallback-budget row present; typed-event oracle sidecar exists but `contributes_to_pass_fail = false` in existing receipts | 2 | `docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json`; `docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json` | client pre-restart sign observation, reconnect, post-restart sign observation; server sign mutation, clean shutdown, backend restart, post-restart observation, restored state |

## Decision

Promote only these three rows to typed-event-ready after the pure validator accepts complete ordered typed milestones and rejects missing boundary/reconnect, unordered milestones, duplicate restored state, and mismatched restored state. Raw logs remain review evidence only; pass/fail for these rows is tied to typed receipt fields emitted by the runner.

## Owner

mc-compat

## Next action

Run focused runner tests, scenario manifest/generated-surface checks, evidence-manifest validation, task-evidence validation, Cairn gates, sync, archive, and final Cairn validation before marking the change archived.

## Non-claims

This inventory and promotion do not claim arbitrary world durability, arbitrary crash consistency, all chunks, all block entities, full survival compatibility, broad vanilla parity, public-server safety, production readiness, or unbounded load behavior.
