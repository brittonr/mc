# Tasks

- [ ] [serial] Define normalized parity metrics for the break/place/pickup rail. r[mc_compatibility.survival_reference_parity.metrics]
- [ ] [serial] Add or select a local reference server fixture for the same survival probe. r[mc_compatibility.survival_reference_parity.reference_receipt]
- [ ] [depends:reference_receipt] Produce a reference receipt/log bundle under `docs/evidence/`. r[mc_compatibility.survival_reference_parity.reference_receipt]
- [ ] [depends:metrics] Produce a matching Valence receipt/log bundle from committed child revisions. r[mc_compatibility.survival_reference_parity.valence_receipt]
- [ ] [depends:reference_receipt] Add a comparator/checker with positive and negative tests for exact metric matching. r[mc_compatibility.survival_reference_parity.comparator]
- [ ] [depends:comparator] Update the matrix only for the narrow paired break/place/pickup parity row and preserve full-survival non-claims. r[mc_compatibility.survival_reference_parity.nonclaims]
