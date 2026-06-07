# Tasks

- [ ] [serial] Record the sign editor live contract: actor, sign position, initial sign state, submitted payload, open/update packet rows, backend/client path, accepted-update correlation, and non-claims. r[mc_compatibility.sign_editor_live_rail.contract]
- [ ] [depends:contract] Run baseline targeted packet, matrix, current-bundle, and packet-inventory checks before modifying sign editor behavior. r[mc_compatibility.sign_editor_live_rail.baseline]
- [ ] [depends:baseline] Add an isolated sign editor open/update rail or deterministic fixture path without changing sign persistence/block-entity row semantics. r[mc_compatibility.sign_editor_live_rail.rail]
- [ ] [depends:rail] Emit reviewable KV/receipt/log evidence under `docs/evidence/` with packet rows, sign position, submitted payload, client open/update milestones, backend accepted-update correlation, and explicit non-claims. r[mc_compatibility.sign_editor_live_rail.evidence]
- [ ] [depends:evidence] Validate sign editor evidence through positive and negative checker coverage for missing open/update correlation, wrong packet row, wrong sign position/payload, stale receipt, and broad sign-editing overclaims. r[mc_compatibility.sign_editor_live_rail.checker]
- [ ] [depends:checker] Promote only `sign-editor-open-update` in matrix/current-bundle/packet-inventory docs if live evidence passes; otherwise record blocker evidence and keep it fixture-bounded. r[mc_compatibility.sign_editor_live_rail.promotion]
- [ ] [depends:promotion] Run runner checks, targeted packet checks, matrix/bundle/inventory checks, evidence-manifest/task-evidence checks, Cairn gates, sync, archive, and post-archive validation. r[mc_compatibility.sign_editor_live_rail.validation]
