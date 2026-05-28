# Tasks

- [x] [serial] Define normalized parity metrics for the break/place/pickup rail. r[mc_compatibility.survival_reference_parity.metrics]
  - Evidence: `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md` defines exact-match receipt/log metrics for join/render, break target/update, pickup/inventory transition, and placement target/update. `tools/check_survival_reference_parity.py --self-test` passes.
- [x] [serial] Add or select a local reference server fixture for the same survival probe. r[mc_compatibility.survival_reference_parity.reference_receipt]
  - Evidence: `tools/paper-survival-fixture/src/main/java/mc/compat/paper/SurvivalFixturePlugin.java` provides the Paper 1.20.1 fixture instrumentation, mounted by `PAPER_PLUGIN_JAR` in `tools/mc-compat-runner/src/main.rs`.
- [x] [depends:reference_receipt] Produce a reference receipt/log bundle under `docs/evidence/`. r[mc_compatibility.survival_reference_parity.reference_receipt]
  - Evidence: `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json`, `.client.log`, `.server.log`, `.run.log`, and `docs/evidence/protocol-763-survival-reference-pair-2026-05-28.b3`.
- [x] [depends:metrics] Produce a matching Valence receipt/log bundle from committed child revisions. r[mc_compatibility.survival_reference_parity.valence_receipt]
  - Evidence: `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.receipt.json`, `.client.log`, `.server.log`, `.run.log`; receipt records Stevenarella `d758630ad77b444d80e4bd8dca8585b5507f556b` and Valence `7d13a242742347a05c9752501880a2e986819ae7` clean.
- [x] [depends:reference_receipt] Add a comparator/checker with positive and negative tests for exact metric matching. r[mc_compatibility.survival_reference_parity.comparator]
  - Evidence: `tools/check_survival_reference_parity.py --self-test` passes; paired CLI comparison passes in `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.compare.log`.
- [x] [depends:comparator] Update the matrix only for the narrow paired break/place/pickup parity row and preserve full-survival non-claims. r[mc_compatibility.survival_reference_parity.nonclaims]
  - Evidence: `docs/evidence/protocol-763-acceptance-matrix.md`, `docs/evidence/protocol-763-current-evidence-bundle.md`, and `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md` promote only the narrow paired row and keep full survival compatibility/broad vanilla parity as non-claims.
