# Furnace persistence validation oracle checkpoint

## Question
Can the session claim the `survival-furnace-persistence` runner/client/fixture rail and validation pass without paired live Paper/Valence receipts?

## Inspected evidence
- `docs/evidence/survival-furnace-persistence-rail-validation-2026-06-01.run.log` records:
  - runner tests: `test result: ok. 93 passed`
  - Valence furnace example tests: `test result: ok. 3 passed`
  - Stevenarella furnace tests: `test result: ok. 3 passed` for both emitted test binaries
  - direct scenario manifest checker: `scenario manifest check passed: 23 rows validated`
  - furnace dry-run Nix check: `exit_status=0`
  - scenario manifest Nix check: `exit_status=0`
  - Cairn tasks gate: `"verdict": "PASS"`
  - Paper fixture static guard check: `exit_status=0`
- `docs/evidence/survival-furnace-persistence-task-evidence-gate-2026-06-01.run.log` records:
  - post-task-update Cairn tasks gate: `"verdict": "PASS"`
  - Cairn task-evidence Nix check: `exit_status=0`
- Paper fixture code now records post-collect quit plus reconnect join before emitting `survival_furnace_reconnect_reopen`.
- Paper fixture code now verifies input slot `RawIron`, fuel slot `Coal`, and empty output before emitting `survival_furnace_server_state`.

## Decision
The rail implementation and dry-run/static validation may be claimed. Full survival-row parity, aggregate survival parity, and paired live Paper/Valence equivalence remain non-claims until live receipts and row promotion evidence are produced.

## Owner
Current agent session.

## Next action
Produce paired live Paper and Valence receipts/logs under `docs/evidence/`, generate BLAKE3 manifests, then promote only the furnace persistence survival row.
