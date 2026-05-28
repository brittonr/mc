# Oracle checkpoint: survival coverage matrix implementation evidence — 2026-05-28

## Review-critical question

Can the archived `build-survival-coverage-matrix` task completion be trusted when the review path did not include the implementation files `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` and `tools/check_survival_coverage_matrix.py`?

## Inspected evidence

| Evidence | BLAKE3 | Observation |
| --- | --- | --- |
| `docs/evidence/protocol-763-survival-coverage-matrix-2026-05-28.md` | `087fd7e14d50dd12895e546f890f4f898709c28c56b3bd8f30bdd55def21d20a` | Matrix has 9 rows. `break/place/pickup` is `valence_covered_reference_missing`; `crafting`, `chest persistence`, `furnace persistence`, `hunger/food`, `mob drops`, `redstone`, `biome/dimension`, and `world persistence` are `missing` with `none` Valence/reference evidence and explicit `No ... coverage` non-claims. |
| `tools/check_survival_coverage_matrix.py` | `e0ba7330a87aa77b45dd9383879b35f5261457d7ea68b5723c4370a05dc8d69b` | Checker requires 9 rows, exact required systems, row-level non-claims, paired-reference/BLAKE3 text, current bundle survival non-claim text, and acceptance-matrix full-survival gap text. It scans docs for forbidden claims: `full_survival_compatibility is covered`, `full survival compatibility is covered`, `vanilla parity is covered`, and `full survival compatibility passes`. |
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

Accept the archived `build-survival-coverage-matrix` tasks as review-supported. The implementation evidence blocks full-survival/vanilla-parity overclaims and records every currently uncovered survival system as an explicit non-claim with row-level promotion requirements.

## Owner

Compatibility evidence owner: `mc` maintainers.

## Next action

Use this checkpoint when a review path omits the matrix/checker files. Future survival row promotion must update the matrix, checker fixtures, current evidence bundle, acceptance matrix, and BLAKE3 evidence before changing any row from `missing` or promoting vanilla/full-survival claims.
