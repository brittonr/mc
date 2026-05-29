# Oracle checkpoint: survival coverage matrix implementation evidence — 2026-05-28

## Review-critical question

Can the archived `build-survival-coverage-matrix` task completion be trusted when the review path did not include the implementation files `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` and `tools/check_survival_coverage_matrix.py`?

## Inspected evidence

| Evidence | BLAKE3 | Observation |
| --- | --- | --- |
| `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` | `0e649d28bcff7c9bcf15ee7f76b7646a418a5f8766ef1eda81c85a1b8b9645d5` | Matrix has 9 rows. `break/place/pickup` is now `reference_parity_covered` and cites paired Paper and Valence reference receipts plus the parity doc; `crafting`, `chest persistence`, `furnace persistence`, `hunger/food`, `mob drops`, `redstone`, `biome/dimension`, and `world persistence` remain `missing` with `none` Valence/reference evidence and explicit `No ... coverage` non-claims. |
| `tools/check_survival_coverage_matrix.py` | `f2ba8ba0628c3ebd99a2ecc8cc95b66b4b0c844f0e1b9f06cc43bdc7afdf81c0` | Checker requires 9 rows, exact required systems, row-level non-claims, paired Paper/Valence receipt references for break/place/pickup, the parity evidence doc, current bundle survival non-claim text, and acceptance-matrix full-survival gap text. It rejects stale `valence_covered_reference_missing`, missing Paper receipt, promoted missing rows, and forbidden claims: `full_survival_compatibility is covered`, `full survival compatibility is covered`, `vanilla parity is covered`, and `full survival compatibility passes`. |
| `cairn/archive/2026-05-28-build-survival-coverage-matrix/tasks.md` | `c6e3ec78a314f27b567fffc33039dfc168497859e606a07e5d15c699c8242e63` | Archived tasks mark completion against matrix/checker evidence and current bundle/matrix docs and link this oracle checkpoint. |

## Command evidence

Ran from `/home/brittonr/git/mc`:

```sh
python3 tools/check_survival_coverage_matrix.py --self-test
python3 tools/check_survival_coverage_matrix.py
nix run --no-update-lock-file .#cairn -- validate --root .
```

Observed output:

```text
survival coverage matrix self-test ok
survival coverage matrix ok: 9 rows
{
  "change_issues": [],
  "changes": 3,
  "issues": [],
  "layout": "cairn",
  "policy": "cairn-default",
  "spec_issues": [],
  "specs_validated": 5,
  "valid": true
}
```

## Decision

Accept the archived `build-survival-coverage-matrix` tasks as review-supported, with the later `sync-survival-coverage-reference-parity` correction applied. The implementation evidence blocks full-survival/vanilla-parity overclaims, records break/place/pickup as the only paired reference parity row, and records every currently uncovered survival system as an explicit non-claim with row-level promotion requirements.

## Owner

Compatibility evidence owner: `mc` maintainers.

## Next action

Use this checkpoint when a review path omits the matrix/checker files. Future survival row promotion must update the matrix, checker fixtures, current evidence bundle, acceptance matrix, and BLAKE3 evidence before changing any row from `missing` or promoting vanilla/full-survival claims.
