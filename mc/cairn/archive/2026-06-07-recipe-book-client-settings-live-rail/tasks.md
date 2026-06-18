# Tasks

- [x] [serial] Record the recipe-book settings live contract: actor, settings fields, packet row, backend/client path, server correlation, and non-claims. r[mc_compatibility.recipe_book_settings_live_rail.contract]
  - Evidence: `docs/evidence/recipe-book-client-settings-live-contract-2026-06-07.md`, `docs/evidence/recipe-book-client-settings-live-contract-2026-06-07.run.log`, and `docs/evidence/recipe-book-client-settings-live-2026-06-07.b3`.
- [x] [depends:contract] Run baseline targeted packet, matrix, current-bundle, and packet-inventory checks before live-rail changes. r[mc_compatibility.recipe_book_settings_live_rail.baseline]
  - Evidence: `docs/evidence/recipe-book-client-settings-live-baseline-2026-06-07.run.log` and `docs/evidence/recipe-book-client-settings-live-2026-06-07.b3`.
- [x] [depends:baseline] Add the isolated recipe-book settings rail or record a deterministic missing-driver blocker without changing crafting-table scenario semantics. r[mc_compatibility.recipe_book_settings_live_rail.rail]
  - Evidence: `docs/evidence/recipe-book-client-settings-live-rail-blocker-2026-06-07.run.log`, `docs/evidence/recipe-book-client-settings-live-blocker-2026-06-07.receipt.json`, and `docs/evidence/recipe-book-client-settings-live-2026-06-07.b3`.
- [x] [depends:rail] Emit reviewable recipe-book KV, receipt, and log evidence with row id, packet row, settings fields, client action, server correlation, and explicit non-claims. r[mc_compatibility.recipe_book_settings_live_rail.evidence]
  - Evidence: `docs/evidence/recipe-book-client-settings-live-evidence-2026-06-07.run.log`, `docs/evidence/recipe-book-client-settings-live-blocker-2026-06-07.kv`, `docs/evidence/recipe-book-client-settings-live-blocker-2026-06-07.receipt.json`, and `docs/evidence/recipe-book-client-settings-live-2026-06-07.b3`.
- [x] [depends:evidence] Validate recipe-book evidence with positive checker coverage and negative wrong-fields, missing-client-action, stale-digest, missing-correlation, and overclaim fixtures. r[mc_compatibility.recipe_book_settings_live_rail.checker]
  - Evidence: `docs/evidence/recipe-book-client-settings-live-checker-2026-06-07.run.log` and `docs/evidence/recipe-book-client-settings-live-2026-06-07.b3`.
- [x] [depends:checker] Promote only `recipe-book-client-settings` in matrix/current-bundle/packet-inventory docs when live evidence passes; otherwise leave it fixture-bounded with a blocker. r[mc_compatibility.recipe_book_settings_live_rail.promotion]
  - Evidence: `docs/evidence/recipe-book-client-settings-live-promotion-2026-06-07.run.log` and `docs/evidence/recipe-book-client-settings-live-2026-06-07.b3`.
- [x] [depends:promotion] Run evidence-manifest/task-evidence checks, Cairn gates, sync/archive checks, and post-archive validation. r[mc_compatibility.recipe_book_settings_live_rail.validation]
  - Evidence: `docs/evidence/recipe-book-client-settings-live-precloseout-validation-2026-06-07.run.log` and `docs/evidence/recipe-book-client-settings-live-2026-06-07.b3`.
