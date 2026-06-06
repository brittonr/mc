# Tasks

- [ ] [serial] Define the bounded movement contract, start/target fields, packet variants, tolerance, and explicit non-claims. r[mc_compatibility.movement_packet_family_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid movement evidence, missing row id, stale revisions, wrong packet variant, missing fields, tolerance mismatch, missing server correlation, and physics/security overclaims. r[mc_compatibility.movement_packet_family_promotion.checker]
- [ ] [depends:checker] Add the isolated runner/client/fixture rail for one movement transition. r[mc_compatibility.movement_packet_family_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts/logs, normalized inputs, child revision metadata, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.movement_packet_family_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured movement row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.movement_packet_family_promotion.matrix]
- [ ] [depends:matrix] Run checker, runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.movement_packet_family_promotion.validation]
