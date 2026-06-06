# Tasks

- [ ] [serial] Define the bounded creative inventory contract, creative-mode precondition, exact slot/item/count, packet row, and explicit non-claims. r[mc_compatibility.creative_inventory_action_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid creative evidence, missing row id, missing creative mode, stale revisions, wrong slot/item/count, missing server acceptance, final-state mismatch, and broad overclaims. r[mc_compatibility.creative_inventory_action_promotion.checker]
- [ ] [depends:checker] Add the isolated runner/client/fixture rail for one creative inventory mutation. r[mc_compatibility.creative_inventory_action_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts/logs, normalized inputs, child revision metadata, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.creative_inventory_action_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured creative action row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.creative_inventory_action_promotion.matrix]
- [ ] [depends:matrix] Run checker, runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.creative_inventory_action_promotion.validation]
