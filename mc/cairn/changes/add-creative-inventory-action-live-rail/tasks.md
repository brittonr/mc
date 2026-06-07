# Tasks

- [ ] [serial] Record the creative live contract: actor, creative-mode precondition, semantic slot, wire slot, item id/count, packet row, backend/client path, and non-claims. r[mc_compatibility.creative_inventory_live_rail.contract]
- [ ] [depends:contract] Run baseline targeted packet, matrix, current-bundle, and packet-inventory checks before modifying the runner. r[mc_compatibility.creative_inventory_live_rail.baseline]
- [ ] [depends:baseline] Add the isolated creative live rail or deterministic fixture path without changing existing survival/player-inventory scenario semantics. r[mc_compatibility.creative_inventory_live_rail.rail]
- [ ] [depends:rail] Emit reviewable creative live KV/receipt/log evidence under `docs/evidence/` with row id, packet row, scenario, backend/client path, and explicit non-claims. r[mc_compatibility.creative_inventory_live_rail.evidence]
- [ ] [depends:evidence] Validate creative live evidence through the targeted packet live-evidence checker, including negative wrong-row, stale receipt, missing correlation, and overclaim fixtures. r[mc_compatibility.creative_inventory_live_rail.checker]
- [ ] [depends:checker] Promote only `creative-inventory-action` in the matrix, current bundle, and packet inventory if live evidence passes; otherwise record the blocker and keep it fixture-bounded. r[mc_compatibility.creative_inventory_live_rail.promotion]
- [ ] [depends:promotion] Run runner checks, targeted packet checks, matrix/bundle/inventory checks, evidence-manifest/task-evidence checks, Cairn gates, sync, archive, and post-archive validation. r[mc_compatibility.creative_inventory_live_rail.validation]
