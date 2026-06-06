# Tasks

- [ ] [serial] Define the bounded recipe-book settings contract, configured state fields, packet row, and explicit non-claims. r[mc_compatibility.recipe_book_client_settings_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid settings evidence, missing row id, stale revisions, wrong settings, missing client/server correlation, and broad recipe-book/crafting overclaims. r[mc_compatibility.recipe_book_client_settings_promotion.checker]
- [ ] [depends:checker] Add the isolated runner/client/fixture rail for one recipe-book settings transition. r[mc_compatibility.recipe_book_client_settings_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts/logs, normalized inputs, child revision metadata, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.recipe_book_client_settings_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured settings row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.recipe_book_client_settings_promotion.matrix]
- [ ] [depends:matrix] Run checker, runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.recipe_book_client_settings_promotion.validation]
