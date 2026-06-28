# Design: restart persistence typed events

## Scope

This change migrates selected restart persistence scenarios from substring fallback to typed milestone validation. It does not broaden their live evidence claims and does not introduce a general persistence engine guarantee.

## Selected rows

- `survival-world-persistence-restart`
- `survival-crash-recovery-parity`
- `survival-block-entity-persistence-parity`

Each row remains bounded to its configured fixture and receipt metrics. Existing paired evidence can be reused only if it contains or can be regenerated with the typed milestones required by this change.

## Functional core

Add a pure restart-persistence validator over normalized receipt events:

- expected scenario identity;
- pre-boundary mutation milestone;
- explicit restart or crash boundary milestone;
- reconnect milestone where the row has more than one session;
- post-boundary client observation;
- post-boundary server restored-state milestone;
- non-claim fields.

The validator returns typed diagnostics for missing, unordered, duplicate, mismatched, or stale milestones. It does not read logs, start processes, or inspect filesystem state.

## Imperative shell

The runner/client/server shells emit typed milestone values and write receipts. Shell code may still capture raw logs for review, but promotion checks depend on typed receipt fields rather than substring matching. Generated surfaces are refreshed after the manifest changes.

## Validation strategy

- Positive fixture for each selected row with complete ordered milestones.
- Negative fixture for missing boundary milestone.
- Negative fixture for missing reconnect where required.
- Negative fixture for server/client restored-state mismatch.
- Negative fixture for unordered post-boundary observation.
- Scenario manifest/fallback budget checks prove the rows leave substring fallback only after typed validation exists.

## Non-claims

The migration changes observability and fail-closed validation. It does not claim arbitrary world durability, arbitrary crash consistency, all chunks, all block entities, full survival compatibility, broad vanilla parity, public-server safety, production readiness, or unbounded load behavior.
