# Tasks

- [x] [serial] Satisfy `matrix_index`. r[mc_compatibility.roi_07_post_drain_evidence_index.matrix_index]
- [x] [serial] Satisfy `bundle_alignment`. r[mc_compatibility.roi_07_post_drain_evidence_index.bundle_alignment]
- [x] [serial] Satisfy `residual_catalog_alignment`. r[mc_compatibility.roi_07_post_drain_evidence_index.residual_catalog_alignment]
- [x] [serial] Satisfy `checker_coverage`. r[mc_compatibility.roi_07_post_drain_evidence_index.checker_coverage]

## Evidence

- Matrix checker passed: `python3 tools/check_acceptance_matrix.py` => `acceptance matrix ok: 14 seams, 14 hashes`.
- Bundle checker passed: `python3 tools/check_current_evidence_bundle.py` => `current evidence bundle ok: 14 seams`.
- Manifest checker passed: `nix develop --no-update-lock-file -c python3 tools/check_evidence_manifests.py` => `evidence manifests ok: 4 manifests, 12 entries, 30 receipts scanned`.
- Nix checks passed: `mc-compat-acceptance-matrix`, `mc-compat-current-evidence-bundle`, `mc-compat-evidence-manifests`, and `mc-compat-maintained-dry-runs`.
- Cairn validation passed: `nix run --no-update-lock-file .#cairn -- validate --root .`.
