# Tasks

- [x] [serial] Define the bounded `biome/dimension` evidence contract and normalized metric names. Evidence: `docs/evidence/survival-gap-cairns-2026-05-31.run.log`, `docs/evidence/survival-gap-cairns-2026-05-31.b3`. r[mc_compatibility.survival_biome_dimension.contract]
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for paired evidence, missing metrics, mismatches, stale revisions, and Valence-only evidence. r[mc_compatibility.survival_biome_dimension.checker]
- [ ] [depends:checker] Add the `survival-biome-dimension-state` runner/client rail without broadening existing scenarios. r[mc_compatibility.survival_biome_dimension.runner]
- [ ] [depends:runner] Add Paper and Valence fixture instrumentation for `biome/dimension` server-side milestones. r[mc_compatibility.survival_biome_dimension.fixtures]
- [ ] [depends:fixtures] Produce reviewable paired Paper and Valence receipts/logs under `docs/evidence/`, plus BLAKE3 manifests. r[mc_compatibility.survival_biome_dimension.receipts]
- [ ] [depends:receipts] Promote only the `biome/dimension` survival coverage row and keep adjacent non-claims explicit in the matrix and current bundle. r[mc_compatibility.survival_biome_dimension.matrix]
- [ ] [depends:matrix] Run checker self-tests, paired comparator, maintained dry-runs where applicable, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.survival_biome_dimension.validation]
