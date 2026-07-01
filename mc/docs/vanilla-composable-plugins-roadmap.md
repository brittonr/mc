# Minecraft Wiki-guided composable plugin roadmap

## Scope

This roadmap uses the Minecraft Wiki as a guide and vocabulary index for future Valence plugin work. It does not treat wiki text as authoritative evidence. Follow-on implementation Cairns must prove target-version behavior with extracted-data checks, Paper/vanilla parity receipts, or another accepted vanilla-reference artifact before promoting compatibility claims.

Target scope for this roadmap is Java Edition 1.20.1 / protocol 763 unless a follow-on Cairn explicitly changes the target.

## Source inventory

Retrieval date: 2026-07-01.

| Source | URL | Target scope | Roadmap use | Version-drift risk | Follow-up evidence |
| --- | --- | --- | --- | --- | --- |
| Minecraft Wiki main page | https://minecraft.wiki/w/Minecraft_Wiki | Java Edition 1.20.1 / protocol 763 for this repo; page itself tracks current releases | Domain index for blocks, items, biomes, effects, crafting, smelting, smithing, redstone, commands, data packs/resource packs, and protocol docs | High: main page currently advertises newer release trains and upcoming drops | Use only as an entry-point index; version-scope every selected page before implementation |
| Java Edition 1.20.1 | https://minecraft.wiki/w/Java_Edition_1.20.1 | Java Edition 1.20.1 | Target-version checkpoint; page says 1.20.1 is a minor hotfix compatible with 1.20 servers | Medium: page is narrow, but follow-on pages may describe later behavior | Pair with extracted data and protocol constants before claims |
| Protocol version | https://minecraft.wiki/w/Protocol_version | Java Edition 1.20.1 / protocol 763 | Confirms protocol-version vocabulary and protocol 763 row for Java Edition 1.20.1 | Medium: protocol table is mutable and long | Cross-check with Valence `PROTOCOL_VERSION` and runner receipts |
| Protocol documentation | https://minecraft.wiki/w/Minecraft_Wiki:Protocol_documentation | Java protocol 763 where supported by local code | Index for packet, registry data, inventory, slot data, chunk format, block actions, entity metadata, and plugin channels | High: page warns current protocol docs may not be completely up to date | Prefer Valence generated protocol data and maintained mc-compat dry-runs |
| Crafting | https://minecraft.wiki/w/Crafting | Java Edition 1.20.1 | Recipe matching, 2x2/3x3 grids, shaped/shapeless/fixed vocabulary, recipe-book boundary | High: current page includes newer crafter behavior not in target scope | Extract recipe JSON for target version before claiming breadth |
| Smelting | https://minecraft.wiki/w/Smelting | Java Edition 1.20.1 | Furnace, smoker, blast furnace, fuel, output-slot, pause, and XP behavior-card seed | High: recipe tables and item sets drift; some current item references are newer | Extract recipe/fuel tables and compare Paper behavior for selected rows |
| Food/Hunger | https://minecraft.wiki/w/Hunger | Java Edition 1.20.1 | Hunger, saturation, eating, starvation, regeneration, sprint gate behavior seams | High: food tables can include current-version items and edition notes | Extract item food components and verify selected state transitions |
| Armor | https://minecraft.wiki/w/Armor | Java Edition 1.20.1 | Equipment slots, armor material vocabulary, damage-mitigation roadmap seam | High: current page includes post-1.20.1 armor/materials; treat as index only | Use target-version item/equipment data and combat parity fixtures |
| Block entity | https://minecraft.wiki/w/Block_entity | Java Edition 1.20.1 | Storage, ticking, rendering, inventory, and NBT boundaries for block entities | High: page includes newer block entities and Bedrock-only notes | Extract block entity IDs/NBT and use persistence receipts |
| Redstone circuits | https://minecraft.wiki/w/Redstone_circuits | Java Edition 1.20.1 | High-risk redstone domain inventory and deferral criteria | Very high: circuits encode emergent behavior and edition/version quirks | Require architecture, schedule, tick-order, and Paper parity before work starts |
| Mob | https://minecraft.wiki/w/Mob | Java Edition 1.20.1 | Spawn, despawn, AI, behavior categories, loot, damage, and pathfinding boundaries | Very high: current mob list includes newer, unreleased, removed, joke, and edition-only entries | Defer broad AI; use bounded mob-drop or single-mob cards first |
| Biome | https://minecraft.wiki/w/Biome | Java Edition 1.20.1 | Biome registry, environment, spawn, color, and world-generation seams | High: current page includes newer biome names and full worldgen breadth | Use target-version biome registry and existing dimension/biome receipts |

Existing Valence/plugin surfaces inspected:

- `servers/valence/README.md` describes Valence as a modular Minecraft server framework where opinionated vanilla mechanics are expected to be optional plugins.
- `servers/valence/src/lib.rs` defines `DefaultPlugins` with core server, registry, biome, dimension, entity, layer, client, event-loop, movement, action, interaction, status, effect, abilities, and optional feature plugins.
- `servers/valence/crates/valence_server/src/event/loop.rs` exposes event-loop schedules and event-loop system sets for packet-driven shells.
- `servers/valence/crates/valence_server/src/tick_scheduler.rs` already demonstrates a pure scheduler core with an optional Bevy shell plugin.
- `servers/valence/examples/gameplay_contracts/mod.rs` records gameplay plugin contracts, schedule phases, owned resources/events, and non-claims for example plugins.
- Existing mc-compat rails in `docs/check-tiers.md` cover dry-runs, evidence manifests, current evidence bundles, survival aggregate gates, Paper/Valence live rails, and Cairn archive closeout.

High-risk version-drift areas:

- Current wiki pages include post-1.20.1 releases, upcoming drops, Bedrock-only rows, education features, removed/joke mobs, and newer protocol rows.
- Recipe/item/armor/biome/mob tables should be treated as discovery prompts until target-version extracted data confirms membership.
- Redstone and world generation are emergent systems; wiki circuit or biome descriptions are insufficient for direct implementation claims.

## Domain-to-plugin taxonomy

| Wiki domain | Candidate plugin group | Candidate feature plugins | Required dependencies | Optional dependencies | Schedule impact | Evidence need | Non-claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Crafting / recipes | `VanillaCraftingPlugins` | `CraftingRecipeCorePlugin`, `CraftingTableShellPlugin`, `RecipeBookSyncPlugin` | inventory, item registry, recipe data | commands, advancements | Inventory click and interaction phases; recipe-book packet sync | Positive and negative recipe-core tests; extracted target recipe JSON; selected Paper parity | Not all data packs, not all recipes, not automated crafter until separately scoped |
| Smelting / cooking | `VanillaFurnacePlugins` | `FurnaceRuleCorePlugin`, `FurnaceBlockEntityShellPlugin`, `SmokerPlugin`, `BlastFurnacePlugin` | block entity storage, item registry, recipe/fuel data, tick scheduler | XP, hoppers, advancements | Block-entity tick phase plus inventory output updates | Core transition tests; invalid-output negative tests; extracted recipe/fuel data; Paper row | Not all block entities, not hopper automation breadth, not XP parity until tested |
| Food, hunger, health | `VanillaSurvivalStatsPlugins` | `FoodConsumeCorePlugin`, `HungerTickPlugin`, `NaturalRegenPlugin`, `StarvationPlugin` | player state, item food data, damage/healing events | status effects, difficulty settings | Tick cadence plus action/eating packet phases | State-transition tests, malformed food data negatives, survival receipts | Not all effects, not all difficulty semantics, not broad survival correctness |
| Equipment / armor / combat mitigation | `VanillaEquipmentCombatPlugins` | `ArmorMitigationCorePlugin`, `EquipmentSlotRulePlugin`, `ProjectileAttributionPlugin` | equipment, damage events, item attributes | enchantments, effects | Combat event phase before damage commit | Existing armor/reference combat checks plus target item attributes | Not full combat parity, not all enchantments, not PVP balance |
| Effects / brewing / enchanting | `VanillaEffectPlugins` | `StatusEffectTickCorePlugin`, `PotionApplyPlugin`, `EnchantmentRuleCorePlugin` | status effect registry, item data, tick scheduler | particles, commands, advancement | Tick phase and interaction shell | Positive/negative effect tick tests; data extraction; Paper rows | Not all potion effects, not all enchantment conflicts initially |
| Block interactions | `VanillaBlockInteractionPlugins` | `BlockBreakCorePlugin`, `BlockPlaceCorePlugin`, `ToolHarvestRulePlugin` | block state registry, action/interact events, item data | world persistence, drops | Packet event-loop shell, layer update pre-client | Block/action dry-runs, extracted block state data, Paper rows | Not all block update semantics, not all drops initially |
| Block entities / containers | `VanillaBlockEntityPlugins` | `ContainerInventoryPlugin`, `FurnaceBlockEntityPlugin`, `SignTextPlugin`, `PersistenceBridgePlugin` | block entity NBT, inventory, world storage | hoppers, redstone, commands | Block-entity tick and persistence phases | Persistence receipts, invalid NBT negatives, Paper parity for selected block | Not all block entities or all NBT paths |
| Redstone | `VanillaRedstonePlugins` | `RedstonePowerCorePlugin`, `SimpleDoorLeverPlugin`, `ObserverUpdatePlugin` | block updates, tick scheduler, block states | block entities, pistons, fluids | Strict tick/update order; high schedule risk | Dedicated architecture, tick-order tests, Paper parity matrix | Not broad redstone, not all BUD/quasi-connectivity semantics |
| Mobs / AI / loot | `VanillaMobPlugins` | `MobSpawnRuleCorePlugin`, `MobDropCorePlugin`, `SingleMobAiPlugin` | entity, biome, difficulty, loot tables | pathfinding, worldgen, equipment | Spawn/despawn tick phases and entity update phases | Bounded mob-drop receipts and single-mob tests | Not broad mob AI, not all spawning, not all loot tables initially |
| Biomes / world | `VanillaWorldPlugins` | `BiomeRegistryBridgePlugin`, `DimensionJoinStatePlugin`, `WorldPersistencePlugin` | biome/dimension registries, chunk/layer systems, persistence | worldgen, structures | Join-state and chunk update phases | Existing biome/dimension receipts, extracted registries | Not world generation, not portal travel, not all biome behavior |
| Commands / data packs / protocol | `VanillaIntegrationPlugins` | `CommandSurfacePlugin`, `DatapackRecipeLoadPlugin`, `ProtocolStateGuardPlugin` | command crate, config/data loader, protocol crate | permissions, resource packs | Startup/load phases and packet command phases | Parser tests, invalid pack negatives, dry-run command shape | Not arbitrary command parity, not full data-pack engine |

## Behavior card template

Every follow-on wiki-guided plugin Cairn should include a card with this shape:

```markdown
# Behavior card: <bounded feature seam>

## Source pages
- <title>: <URL>, retrieved <date>

## Target scope
- Edition: Java Edition
- Version/protocol: <version> / <protocol>
- Data source: <extracted-data artifact or Paper fixture>

## Bounded claim
<One sentence claim the evidence may promote.>

## Non-claims
- broad Minecraft compatibility
- broad vanilla parity
- public-server safety
- production readiness
- <domain-specific non-claims>

## Pure rule core
- Inputs: <deterministic value inputs>
- Outputs: <deterministic result or diagnostic>
- Error cases: <invalid or missing data cases>

## Thin Bevy/ECS shell
- Resources/components/events owned:
- Systems:
- Schedule phase and ordering:
- I/O and mutation boundary:

## Tests
- Positive tests:
- Negative tests:

## Evidence
- Extracted-data check:
- Paper/vanilla parity receipt:
- mc-compat matrix row:
```

## Crafting recipe behavior-card handoff

`docs/crafting-recipe-behavior-card.md` is now the prerequisite card for future `CraftingRecipeCorePlugin` or `CraftingTableShellPlugin` work. It uses the existing `survival-crafting-recipe-breadth` shaped chest, shapeless oak-planks, invalid stick-input rejection, and primary-click collection receipts as predecessor row evidence only. Follow-on implementation must still add target-version recipe JSON extraction, a pure recipe core, a thin opt-in Valence shell, positive and negative tests, disabled-plugin evidence, and separate closeout evidence before claiming runtime behavior.

The card preserves non-claims for all recipes, arbitrary collection modes, shift-click/drag/split breadth, data-pack loading, recipe-book UI behavior, automated crafter behavior, DefaultPlugins membership, broad vanilla parity, broad Minecraft compatibility, public-server safety, and production readiness.

## Filled behavior card example: furnace smelting

## Source pages

- Smelting: https://minecraft.wiki/w/Smelting, retrieved 2026-07-01.
- Block entity: https://minecraft.wiki/w/Block_entity, retrieved 2026-07-01.
- Java Edition 1.20.1: https://minecraft.wiki/w/Java_Edition_1.20.1, retrieved 2026-07-01.

## Target scope

- Edition: Java Edition.
- Version/protocol: 1.20.1 / protocol 763.
- Data source: target-version recipe and fuel extraction plus selected Paper furnace receipts.

## Bounded claim

A selected furnace row advances one smeltable input into one compatible output when a valid fuel source and output slot capacity are present under Java Edition 1.20.1 rules.

## Non-claims

- No broad vanilla parity.
- No full survival correctness.
- No all-recipe breadth until all target-version recipe rows are extracted and tested.
- No hopper automation, XP rounding, smoker/blast-furnace category breadth, or chunk-unload semantics until separately scoped.
- No public-server safety or production readiness.

## Pure rule core

Inputs:

- `FurnaceKind` (`standard`, `smoker`, `blast_furnace`).
- `FurnaceState` with input stack, fuel stack, output stack, `cook_progress_ticks`, `remaining_burn_ticks`, and accumulated recipe counters.
- `RecipeTable` filtered to Java Edition 1.20.1.
- `FuelTable` filtered to Java Edition 1.20.1.
- `standard_furnace_cook_ticks = 200` and `fast_furnace_cook_ticks = 100` as named constants in implementation.

Outputs:

- New `FurnaceState`.
- `FurnaceTransition` diagnostic such as `StartedFuel`, `AdvancedCooking`, `ProducedOutput`, `PausedNoFuel`, `PausedNoRecipe`, or `PausedOutputBlocked`.

Error cases:

- Missing recipe table row.
- Fuel item absent or invalid.
- Output stack full.
- Output stack item kind mismatch.
- Smoker receives non-food recipe.
- Blast furnace receives non-ore/non-gear recipe.
- Malformed extracted recipe row.

## Thin Bevy/ECS shell

- Owns block entity component/resource access, inventory slot mutation, chunk-loaded gating, packet or layer update emission, and schedule registration.
- Calls the pure core with an in-memory state snapshot and writes back only the returned state.
- Runs in a named block-entity tick phase before client-visible inventory/layer updates.
- Does not read files, fetch wiki pages, parse data packs, write logs, or decide recipe semantics inside the system body.

## Tests

Positive tests:

- Standard furnace consumes one valid fuel and advances a valid ore or food recipe.
- Output merges into a compatible non-full stack.
- Smoker accepts a food recipe when data says it is smoker-eligible.
- Blast furnace accepts an ore or gear recipe when data says it is blast-furnace-eligible.

Negative tests:

- Invalid input produces `PausedNoRecipe` without consuming fuel.
- Empty fuel with no remaining burn produces `PausedNoFuel`.
- Wrong output item produces `PausedOutputBlocked` and preserves input.
- Full output stack produces `PausedOutputBlocked`.
- Smoker rejects non-food recipe data.
- Blast furnace rejects non-ore/non-gear recipe data.
- Malformed extracted recipe row fails validation before the core is called.

## Evidence

- Extracted-data check for Java Edition 1.20.1 recipes and fuels.
- Positive and negative pure-core tests.
- Paper/vanilla receipt for at least one selected furnace scenario before promoting behavior beyond local unit semantics.
- mc-compat row with non-claims for broad survival, all recipes, all block entities, public-server safety, and production readiness.

## Functional core / Bevy shell policy

Follow-on plugin implementation MUST keep rule decisions in pure deterministic cores and leave side effects in thin Bevy/ECS shells.

Pure cores:

- Accept in-memory values and target-version data tables.
- Return new values, typed transitions, or deterministic diagnostics.
- Are covered by positive tests and negative tests.
- Do not read files, inspect environment, fetch network pages, emit packets, write logs, query wall-clock time, or mutate Bevy world state.

Thin shells:

- Read resources/components/events from Bevy.
- Convert ECS state into core inputs.
- Apply returned state and emit events/packets.
- Register systems in named phases and record schedule-order impact.
- Remain small enough that behavior semantics are inspectable in core tests.

Schedule-hygiene triggers:

- New gameplay phase or changed ordering.
- New default plugin membership.
- New packet event adapter.
- Block entity or entity tick loop additions.
- Cross-plugin resource ownership or event ownership changes.

## Evidence and test policy

Wiki-guided behavior claims MUST include:

- Positive tests for valid target-version behavior.
- Negative tests for invalid input, missing target data, blocked output/state, malformed rows, unsupported edition/version rows, and boundary conditions.
- Extracted-data checks or Paper/vanilla parity receipts before claiming target-version vanilla behavior.
- A non-overclaiming matrix row naming tested inputs, rejected invalid inputs, target Java/protocol version, and explicit non-claims.

Wiki text alone MAY justify investigation, taxonomy, and behavior-card drafting. It MUST NOT justify broad Minecraft compatibility, broad Java Edition parity, public-server safety, production readiness, all recipes, all block entities, all mobs, all redstone, all world generation, or semantic equivalence.

## Implementation sequence and stop conditions

| Sequence | Domain | First bounded slice | Why now | Stop conditions before broader work |
| --- | --- | --- | --- | --- |
| 1 | Bounded survival stats | Hunger/food or furnace smelting pure core | Reuses survival rails and deterministic state cores | Stop if target-version data extraction is missing or Paper row is unavailable |
| 2 | Crafting and inventory | Shaped/shapeless recipe core for a small extracted recipe set | Reuses inventory click rails and recipe JSON | Stop before all-recipe breadth, data-pack loading, or automated crafter behavior |
| 3 | Equipment/combat | Armor mitigation or equipment slot validation | Existing armor/reference combat checks already exist | Stop before enchantment breadth or full combat parity |
| 4 | Block entities | Furnace or sign persistence card | Existing block-entity docs and persistence rails exist | Stop before all container types, hoppers, or XP parity |
| 5 | World/biome/dimension | Biome/dimension join-state validation | Existing biome/dimension receipts exist | Stop before world generation, portal travel, or structure generation |
| 6 | Redstone | Single lever-door or simple signal rule | Useful but schedule-risky | Stop until tick-order architecture, block-update semantics, and Paper parity matrix exist |
| 7 | Mobs | Single mob-drop or passive despawn rule | Can be bounded with one mob and one loot table | Stop before broad AI, pathfinding, spawning ecosystem, or all loot tables |

Default Valence behavior is unchanged by this roadmap. DefaultPlugins membership remains unchanged until a follow-on accepted change explicitly adds or removes plugin membership and records schedule/evidence impact.
