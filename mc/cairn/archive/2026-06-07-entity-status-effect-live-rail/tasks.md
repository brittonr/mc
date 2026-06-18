# Tasks

- [x] [serial] Record the status-effect live contract: target, effect id/name, amplifier, duration, packet rows, server correlation, backend/client path, and non-claims. r[mc_compatibility.entity_status_effect_live_rail.contract]
  - Evidence: `docs/evidence/entity-status-effect-live-contract-2026-06-07.md`, `docs/evidence/entity-status-effect-live-contract-2026-06-07.run.log`, and `docs/evidence/entity-status-effect-live-2026-06-07.b3`.
- [x] [depends:contract] Run baseline targeted packet, matrix, current-bundle, and packet-inventory checks before live-rail changes. r[mc_compatibility.entity_status_effect_live_rail.baseline]
  - Evidence: `docs/evidence/entity-status-effect-live-baseline-2026-06-07.run.log` and `docs/evidence/entity-status-effect-live-2026-06-07.b3`.
- [x] [depends:baseline] Add the isolated status-effect rail or record a deterministic missing-signal blocker without changing combat/survival scenario semantics. r[mc_compatibility.entity_status_effect_live_rail.rail]
  - Evidence: `docs/evidence/entity-status-effect-live-rail-blocker-2026-06-07.run.log`, `docs/evidence/entity-status-effect-live-blocker-2026-06-07.receipt.json`, and `docs/evidence/entity-status-effect-live-2026-06-07.b3`.
- [x] [depends:rail] Emit reviewable status-effect KV, receipt, and log evidence with row id, packet rows, effect metrics, client observation, server correlation, and explicit non-claims. r[mc_compatibility.entity_status_effect_live_rail.evidence]
  - Evidence: `docs/evidence/entity-status-effect-live-evidence-2026-06-07.run.log`, `docs/evidence/entity-status-effect-live-blocker-2026-06-07.kv`, `docs/evidence/entity-status-effect-live-blocker-2026-06-07.receipt.json`, and `docs/evidence/entity-status-effect-live-2026-06-07.b3`.
- [x] [depends:evidence] Validate status-effect evidence with positive checker coverage and negative wrong-effect, missing-apply, missing-remove-if-required, stale-digest, missing-correlation, and overclaim fixtures. r[mc_compatibility.entity_status_effect_live_rail.checker]
  - Evidence: `docs/evidence/entity-status-effect-live-checker-2026-06-07.run.log` and `docs/evidence/entity-status-effect-live-2026-06-07.b3`.
- [x] [depends:checker] Promote only `entity-status-effect-packets` in matrix/current-bundle/packet-inventory docs when live evidence passes; otherwise leave it fixture-bounded with a blocker. r[mc_compatibility.entity_status_effect_live_rail.promotion]
  - Evidence: `docs/evidence/entity-status-effect-live-promotion-2026-06-07.run.log` and `docs/evidence/entity-status-effect-live-2026-06-07.b3`.
- [x] [depends:promotion] Run evidence-manifest/task-evidence checks, Cairn gates, sync/archive checks, and post-archive validation. r[mc_compatibility.entity_status_effect_live_rail.validation]
  - Evidence: `docs/evidence/entity-status-effect-live-precloseout-validation-2026-06-07.run.log` and `docs/evidence/entity-status-effect-live-2026-06-07.b3`.
