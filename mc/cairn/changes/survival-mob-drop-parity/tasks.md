# Tasks

- [x] [serial] Define the bounded `mob drops` evidence contract and normalized metric names. Evidence: `docs/evidence/survival-gap-cairns-2026-05-31.run.log`, `docs/evidence/survival-gap-cairns-2026-05-31.b3`. r[mc_compatibility.survival_mob_drop.contract]
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for paired evidence, missing metrics, mismatches, stale revisions, and Valence-only evidence. r[mc_compatibility.survival_mob_drop.checker] Evidence: `docs/evidence/survival-row-parity-checker-2026-06-01.run.log`, `docs/evidence/survival-row-parity-checker-2026-06-01.b3`.
- [x] [depends:checker] Add the `survival-mob-drop` runner/client rail without broadening existing scenarios. r[mc_compatibility.survival_mob_drop.runner] Evidence: `docs/evidence/survival-mob-drop-runner-client-2026-06-02.run.log`, `docs/evidence/survival-mob-drop-runner-client-2026-06-02.b3`, `docs/evidence/survival-mob-drop-client-rail-oracle-2026-06-02.md`.
- [ ] [depends:runner] Add Paper and Valence fixture instrumentation for `mob drops` server-side milestones. r[mc_compatibility.survival_mob_drop.fixtures]
- [ ] [depends:fixtures] Produce reviewable paired Paper and Valence receipts/logs under `docs/evidence/`, plus BLAKE3 manifests. r[mc_compatibility.survival_mob_drop.receipts]
- [ ] [depends:receipts] Promote only the `mob drops` survival coverage row and keep adjacent non-claims explicit in the matrix and current bundle. r[mc_compatibility.survival_mob_drop.matrix]
- [ ] [depends:matrix] Run checker self-tests, paired comparator, maintained dry-runs where applicable, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.survival_mob_drop.validation]
