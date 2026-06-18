# Recipe-book client settings live rail contract — 2026-06-07

## Contract

- Targeted row: `recipe-book-client-settings`.
- Actor: `compatbot`.
- Packet row: `play/serverbound/0x22 RecipeBookDataC2SPacket`.
- Settings fields: `book=crafting`, `open=true`, `filtering=false`.
- Expected observations: one client settings-transition action plus one Valence server correlation.
- Backend path: `recipe-book-settings-rail-missing` until a maintained owned-local settings rail exists.
- Client path: `stevenarella-crafting-rail` until it emits maintained recipe-book settings telemetry.

## Current decision

The existing deterministic fixture remains valid for the targeted packet row, but the scenario capability registry records `recipe-book-client-settings` as `targeted-packet-live-blocker` with evidence mode `fixture-bounded-blocker`. The blocker reason is: crafting-table rail does not toggle recipe-book client settings.

## Non-claims

This change does not claim recipe-book UI behavior, recipe discovery, all recipe categories, all recipes, crafting breadth, public-server safety, production readiness, broad Minecraft compatibility, or full protocol 763 compatibility.

## Owner / next action

Owner: local Cairn drain agent. Next action: leave matrix/current-bundle/packet-inventory status fixture-bounded, record blocker evidence, and require a future isolated recipe-book settings receipt before live promotion.
