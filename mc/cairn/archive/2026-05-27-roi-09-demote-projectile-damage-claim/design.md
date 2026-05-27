# Design: Demote projectile damage claim

## Approach

Use the acceptance matrix as the source of maintained claims. Remove the projectile damage attribution row, decrease checker expectations, and make residual docs identify the blocked proof requirements:

1. pin or include the Valence commit that emits `projectile_use`/`projectile_hit`,
2. add a repo-local dependency checkpoint for that Valence evidence,
3. strengthen the runner/receipt so the client damage milestone is causally ordered after projectile use/hit rather than found anywhere in combined logs.

Keep the existing ROI 08 artifacts tracked for auditability, but mark them superseded so future agents do not treat them as accepted evidence.

## Verification

- `python3 tools/check_acceptance_matrix.py --self-test`
- `python3 tools/check_acceptance_matrix.py`
- `python3 tools/check_current_evidence_bundle.py`
- `python3 tools/check_evidence_manifests.py`
- Nix checks for matrix, bundle, manifest, and maintained dry-runs
- Cairn validation
