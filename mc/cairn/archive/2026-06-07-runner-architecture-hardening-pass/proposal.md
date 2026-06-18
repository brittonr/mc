## Why

The runner has grown many scenario, checker, evidence, backend, and targeted-packet rails. Recent trait and module splits reduced some risk, but the next expansion should be preceded by a bounded architecture hardening pass that removes one remaining ad hoc branch or duplication while preserving public evidence surfaces.

## What Changes

- Inventory one small remaining runner/checker area where behavior is still encoded through ad hoc string or backend branches.
- Move that area behind a pure functional core and thin imperative shell without changing receipt schemas, scenario names, milestone IDs, or checker-visible outputs.
- Add positive parity tests and negative fail-closed tests before claiming the refactor complete.
- Record reviewable validation evidence without adding new compatibility claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/**`, `tools/check_*.rs`, focused docs/evidence logs, and Cairn spec/task artifacts.
- **Testing**: Baseline runner/checker tests, positive parity fixtures, negative malformed/unknown/overclaim fixtures, evidence-manifest/task-evidence checks, Cairn gates and validation.
