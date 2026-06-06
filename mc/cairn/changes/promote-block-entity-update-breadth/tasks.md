# Tasks

- [ ] [serial] Define the bounded non-sign block-entity contract, kind, position, payload metric, packet row, and explicit non-claims. r[mc_compatibility.block_entity_update_breadth_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid evidence, missing row id, wrong kind/position/payload, wrong packet, missing backend evidence, stale revisions, and arbitrary NBT/all-block-entity overclaims. r[mc_compatibility.block_entity_update_breadth_promotion.checker]
- [ ] [depends:checker] Add or select the isolated fixture/runner rail for one non-sign block-entity update. r[mc_compatibility.block_entity_update_breadth_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts or fixtures, logs, normalized inputs, revision metadata, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.block_entity_update_breadth_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured non-sign row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.block_entity_update_breadth_promotion.matrix]
- [ ] [depends:matrix] Run checker, fixture/runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.block_entity_update_breadth_promotion.validation]
