# Recipe-book client settings fixture row

## Scope

This promotes only the bounded `recipe-book-client-settings` fixture row. The normalized fixture binds `play/serverbound/0x22 RecipeBookDataC2SPacket` to one `compatbot` crafting-book settings transition with `open=true` and `filtering=false`.

## Artifacts

- Normalized KV: `docs/evidence/recipe-book-client-settings-2026-06-06.kv`
- Receipt: `docs/evidence/recipe-book-client-settings-2026-06-06.receipt.json`
- BLAKE3 manifest: `docs/evidence/recipe-book-client-settings-2026-06-06.b3`
- Checker: `tools/check_targeted_packet_promotions.rs`
- Checker run log: `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`

## Checker contract

The checker requires the row id, packet row, actor, configured settings fields, client transition metric, server correlation, unchanged crafting-row guard, scenario-bounded packet inventory status, positive fixture coverage, negative fixture coverage, and explicit non-claims.

## Explicit non-claims

No recipe-book UI behavior, all recipe categories, recipe discovery, all recipes, full crafting coverage, public-server safety, full protocol-763 compatibility, broad Minecraft compatibility, or production-readiness claim is made.
