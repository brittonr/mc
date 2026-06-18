## Why

`recipe-book-client-settings` is fixture-bounded because the current crafting-table rail does not toggle recipe-book client settings. A narrow live rail can prove one settings transition without claiming recipe discovery, UI behavior, or crafting breadth.

## What Changes

- Add an isolated recipe-book settings live rail for one configured settings transition.
- Record client action, `RecipeBookDataC2SPacket` row identity, Valence server correlation, child revisions when available, and explicit non-claims.
- Validate normalized evidence with positive and negative targeted-packet checker coverage before any matrix or packet-inventory promotion.
- Keep recipe-book UI behavior, discovery, all recipes, crafting breadth, public-server safety, production readiness, and full protocol coverage as non-claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/**`, `tools/check_targeted_packet_promotions.rs`, `docs/evidence/**`, acceptance matrix/current bundle/packet inventory if live evidence passes.
- **Testing**: Baseline targeted packet checks, runner unit/dry-run checks, recipe-book evidence checker positive/negative fixtures, evidence-manifest/task-evidence checks, Cairn gates and validation.
