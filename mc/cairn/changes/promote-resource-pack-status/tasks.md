# Tasks

- [ ] [serial] Define the bounded resource-pack status contract, local offer/status fields, no-external-fetch guarantee, redaction policy, packet rows, and explicit non-claims. r[mc_compatibility.resource_pack_status_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid status evidence, missing row id, missing local scope, stale revisions, wrong offer/status, missing server correlation, missing no-external-fetch/redaction fields, and asset/trust/public-server overclaims. r[mc_compatibility.resource_pack_status_promotion.checker]
- [ ] [depends:checker] Add the isolated runner/client/fixture rail for one resource-pack offer/status exchange. r[mc_compatibility.resource_pack_status_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts/logs, normalized inputs, redaction metadata, child revisions, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.resource_pack_status_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured local status row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.resource_pack_status_promotion.matrix]
- [ ] [depends:matrix] Run checker, runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.resource_pack_status_promotion.validation]
