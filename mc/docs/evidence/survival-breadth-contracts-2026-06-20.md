# Survival breadth contracts — 2026-06-20

## Scope

This document defines bounded contracts for the remaining survival breadth rows. It is not live promotion evidence. Each row remains blocked from archive until an isolated rail produces paired Paper/reference and Valence receipts/logs with clean child revision metadata and BLAKE3 manifests.

## Furnace smelting breadth

`survival-furnace-smelting-breadth-parity` covers one `minecraft:iron_ingot` smelt from `RawIron` plus `Coal`, normalized burn/cook/output checkpoints, and one invalid-fuel rejection where `RawIron` in the fuel slot produces no burn.

Non-claims: all smelting recipes, all fuel semantics, hopper automation, long-running timing parity, furnace minecarts, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## Hunger health cycle

`survival-hunger-health-cycle-parity` covers one configured hunger deficit, one `Bread` consume action, food/saturation checkpoints, one bounded health recovery checkpoint, and inventory decrement metrics.

Non-claims: all foods, all exhaustion sources, starvation loops, potion effects, offhand consumption, regeneration breadth, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## Mob AI loot breadth

`survival-mob-ai-loot-breadth-parity` covers one configured `Zombie` identity, one bounded approach/targeting checkpoint, one player kill, one `RottenFlesh` drop, pickup, and inventory increment.

Non-claims: all mob AI, all mobs, loot-table distribution, spawn rules, pickup races, hostile pathfinding breadth, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## Redstone circuit breadth

`survival-redstone-circuit-breadth-parity` covers one finite lever/lamp/repeater circuit with normalized off/on/off powered checkpoints and a configured tick sequence.

Non-claims: general redstone circuit parity, all components, observers, pistons, comparators, clocks, farms, tick-order breadth, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## Biome dimension travel

`survival-biome-dimension-travel-parity` covers one configured overworld-to-nether transition, normalized source/destination dimension identifiers, source/destination biome identifiers, and client/server transition checkpoints.

Non-claims: all dimensions, all portals, all biome lookup semantics, respawn rules, long-term world durability, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## World multichunk durability

`survival-world-multichunk-durability-parity` covers one bounded two-chunk mutation set, controlled restart, reconnect, post-restart primary block observations, and an auxiliary marker that cannot pass alone.

Non-claims: arbitrary durability, arbitrary crash consistency, concurrent saves, backups, all chunks, all block entities, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## Container block entity breadth

`survival-container-block-entity-breadth-parity` covers one configured `Barrel` container/block-entity row, item transfer, persisted payload summary, metadata summary, and reopen observation.

Non-claims: all containers, arbitrary NBT parity, all block entities, sign editing UI, hopper automation, world durability, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## Sign editing live parity

`survival-sign-editing-live-parity` covers one configured sign editor session at `28,64,0`, front-side payload `MC|Compat|Sign|Edit`, client open/update milestones, server acceptance, and post-update observation.

Non-claims: all sign UI behavior, all sign variants, all text formatting, arbitrary NBT parity, all block entities, full survival compatibility, broad vanilla parity, public-server safety, and production readiness.

## Checker

`tools/check_survival_breadth_contracts.rs` validates key/value evidence for the row contracts above. It requires paired `paper` and `valence` records, exact metric values, cross-backend agreement, clean child revision metadata, and fail-closed broad-overclaim rejection. Its self-test exercises positive fixtures plus negative Valence-only, missing-metric, mismatched-metric, stale-revision, and broad-overclaim cases for each row.
