# Tasks

- [ ] [serial] Define the bounded chat/command containment contract, harmless payload, owned-local scope, packet rows, redaction policy, and explicit non-claims. r[mc_compatibility.chat_command_containment_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid containment evidence, missing row id, missing owned-local scope, stale revisions, wrong payload/packet, missing server correlation, missing redaction policy, and public-server/security overclaims. r[mc_compatibility.chat_command_containment_promotion.checker]
- [ ] [depends:checker] Add the isolated runner/client/fixture rail for one chat/command containment scenario. r[mc_compatibility.chat_command_containment_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts/logs, normalized inputs, redaction metadata, child revisions, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.chat_command_containment_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured owned-local row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.chat_command_containment_promotion.matrix]
- [ ] [depends:matrix] Run checker, runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.chat_command_containment_promotion.validation]
