## Why

The current CTF evidence promotes one invalid pickup row and one invalid return/drop row. The rule ledger still keeps broader invalid-action coverage as a non-claim because adjacent team, carrier, owner, and base permutations are not specified or checked as a maintained family. Adding another one-off row would repeat the same evidence drift problem instead of making invalid-action breadth reviewable.

## What Changes

- Define a CTF invalid-action breadth matrix that names the next bounded pickup and return/drop permutations before promotion.
- Add a parameterized validation contract for invalid-action rows so each permutation records actor/team/flag/base/pre-state/post-state/server rejection/forbidden transition evidence consistently.
- Extend runner/checker/evidence tasks to add the next narrow invalid-action row while preserving full CTF correctness and all-invalid-actions as non-claims.
- Record reviewable receipts, logs, manifests, matrix/current-bundle updates, and Cairn closeout evidence before archive.

## Impact

- **Files**: expected changes under `tools/`, `docs/evidence/`, `config/mc-compat/`, `tools/mc-compat-runner/`, `flake.nix`, and `cairn/specs/mc-compatibility/spec.md`.
- **Testing**: focused baseline gates, runner/checker positive and negative tests, dry-run/live row checks as available, evidence-manifest checks, CTF rule ledger/current-bundle checks, maintained dry-runs, Cairn gates, task-evidence checks, and final validation.
