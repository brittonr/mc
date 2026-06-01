# Tasks

- [ ] [serial] Define the bounded `hunger/food` evidence contract and normalized metric names. r[mc_compatibility.survival_hunger_food.contract]
- [ ] [depends:contract] Add deterministic checker positive and negative fixtures for paired evidence, missing metrics, mismatches, stale revisions, and Valence-only evidence. r[mc_compatibility.survival_hunger_food.checker]
- [ ] [depends:checker] Add the `survival-hunger-food` runner/client rail without broadening existing scenarios. r[mc_compatibility.survival_hunger_food.runner]
- [ ] [depends:runner] Add Paper and Valence fixture instrumentation for `hunger/food` server-side milestones. r[mc_compatibility.survival_hunger_food.fixtures]
- [ ] [depends:fixtures] Produce reviewable paired Paper and Valence receipts/logs under `docs/evidence/`, plus BLAKE3 manifests. r[mc_compatibility.survival_hunger_food.receipts]
- [ ] [depends:receipts] Promote only the `hunger/food` survival coverage row and keep adjacent non-claims explicit in the matrix and current bundle. r[mc_compatibility.survival_hunger_food.matrix]
- [ ] [depends:matrix] Run checker self-tests, paired comparator, maintained dry-runs where applicable, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.survival_hunger_food.validation]
