# Tasks

- [x] [serial] Define the bounded `furnace persistence` evidence contract and normalized metric names. Evidence: `docs/evidence/survival-gap-cairns-2026-05-31.run.log`, `docs/evidence/survival-gap-cairns-2026-05-31.b3`. r[mc_compatibility.survival_furnace_persistence.contract]
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for paired evidence, missing metrics, mismatches, stale revisions, and Valence-only evidence. r[mc_compatibility.survival_furnace_persistence.checker]
- [ ] [depends:checker] Add the `survival-furnace-persistence` runner/client rail without broadening existing scenarios. r[mc_compatibility.survival_furnace_persistence.runner]
- [ ] [depends:runner] Add Paper and Valence fixture instrumentation for `furnace persistence` server-side milestones. r[mc_compatibility.survival_furnace_persistence.fixtures]
- [ ] [depends:fixtures] Produce reviewable paired Paper and Valence receipts/logs under `docs/evidence/`, plus BLAKE3 manifests. r[mc_compatibility.survival_furnace_persistence.receipts]
- [ ] [depends:receipts] Promote only the `furnace persistence` survival coverage row and keep adjacent non-claims explicit in the matrix and current bundle. r[mc_compatibility.survival_furnace_persistence.matrix]
- [ ] [depends:matrix] Run checker self-tests, paired comparator, maintained dry-runs where applicable, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.survival_furnace_persistence.validation]
