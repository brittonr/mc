# Tasks

- [ ] [serial] Define the bounded sign editor open/update contract, exact packet rows, configured sign payload, and explicit non-claims. r[mc_compatibility.sign_editor_open_update_promotion.contract]
- [ ] [depends:contract] Add deterministic positive and negative checker fixtures for valid sign edit evidence, missing row id, missing open/update correlation, stale revisions, wrong position/payload, missing server acceptance, and broad overclaims. r[mc_compatibility.sign_editor_open_update_promotion.checker]
- [ ] [depends:checker] Add the isolated runner/client/fixture rail for one sign editor open/update without changing existing sign persistence rows. r[mc_compatibility.sign_editor_open_update_promotion.rail]
- [ ] [depends:rail] Produce reviewable receipts/logs, normalized inputs, child revision metadata, and BLAKE3 manifests under `docs/evidence/`. r[mc_compatibility.sign_editor_open_update_promotion.artifacts]
- [ ] [depends:artifacts] Promote only the configured sign editor row in packet inventory, acceptance matrix, and current bundle while preserving broad non-claims. r[mc_compatibility.sign_editor_open_update_promotion.matrix]
- [ ] [depends:matrix] Run checker, runner, packet inventory, evidence manifest, task-evidence, Cairn gate, and validation checks with reviewable logs. r[mc_compatibility.sign_editor_open_update_promotion.validation]
