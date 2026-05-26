# Design: Residual combat non-claim catalog

## Approach

Implement this as a narrow evidence-maintenance slice. Prefer deterministic docs/checker or runner unit-test updates over live gameplay runs unless the slice introduces runtime behavior.

## Verification

- `python3 tools/check_acceptance_matrix.py` when matrix/evidence docs are touched.
- `nix run .#cairn -- validate --root .` for lifecycle validity.
- `git diff --check` before commit.
- Runner tests when `tools/mc-compat-runner/src/main.rs` changes.
