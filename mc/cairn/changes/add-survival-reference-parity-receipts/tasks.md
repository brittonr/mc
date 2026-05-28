# Tasks

- [x] [serial] Define normalized parity metrics for the break/place/pickup rail. r[mc_compatibility.survival_reference_parity.metrics]
  - Evidence: `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md` defines exact-match receipt/log metrics for join/render, break target/update, pickup/inventory transition, and placement target/update. `tools/check_survival_reference_parity.py --self-test` passes.
- [ ] [serial] Add or select a local reference server fixture for the same survival probe. r[mc_compatibility.survival_reference_parity.reference_receipt]
- [ ] [depends:reference_receipt] Produce a reference receipt/log bundle under `docs/evidence/`. r[mc_compatibility.survival_reference_parity.reference_receipt]
- [ ] [depends:metrics] Produce a matching Valence receipt/log bundle from committed child revisions. r[mc_compatibility.survival_reference_parity.valence_receipt]
- [ ] [depends:reference_receipt] Add a comparator/checker with positive and negative tests for exact metric matching. r[mc_compatibility.survival_reference_parity.comparator]
  - Progress: `tools/check_survival_reference_parity.py` now has positive exact-match and negative `missing_reference`, `missing_metric`, `mismatched_metric`, and `wrong_backend` fixtures. Kept unchecked until a real reference receipt/log bundle exists and the comparator is exercised against the paired artifacts.
- [ ] [depends:comparator] Update the matrix only for the narrow paired break/place/pickup parity row and preserve full-survival non-claims. r[mc_compatibility.survival_reference_parity.nonclaims]
