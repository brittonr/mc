# Tasks

- [x] [serial] Define the bounded `redstone` evidence contract and normalized metric names. Evidence: `docs/evidence/survival-gap-cairns-2026-05-31.run.log`, `docs/evidence/survival-gap-cairns-2026-05-31.b3`. r[mc_compatibility.survival_redstone_toggle.contract]
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for paired evidence, missing metrics, mismatches, stale revisions, and Valence-only evidence. r[mc_compatibility.survival_redstone_toggle.checker] Evidence: `docs/evidence/survival-row-parity-checker-2026-06-01.run.log`, `docs/evidence/survival-row-parity-checker-2026-06-01.b3`.
- [ ] [depends:checker] Add the `survival-redstone-toggle` runner/client rail without broadening existing scenarios. r[mc_compatibility.survival_redstone_toggle.runner]
- [ ] [depends:runner] Add Paper and Valence fixture instrumentation for `redstone` server-side milestones. r[mc_compatibility.survival_redstone_toggle.fixtures]
- [ ] [depends:fixtures] Produce reviewable paired Paper and Valence receipts/logs under `docs/evidence/`, plus BLAKE3 manifests. r[mc_compatibility.survival_redstone_toggle.receipts]
- [ ] [depends:receipts] Promote only the `redstone` survival coverage row and keep adjacent non-claims explicit in the matrix and current bundle. r[mc_compatibility.survival_redstone_toggle.matrix]
- [ ] [depends:matrix] Run checker self-tests, paired comparator, maintained dry-runs where applicable, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.survival_redstone_toggle.validation]
