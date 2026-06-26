# organize-remaining-valence-examples-as-plugins implementation evidence

## Question

How were the selected remaining Valence examples reorganized as opt-in Bevy plugins while preserving example behavior and non-claim boundaries?

## Implementation summary

- `servers/valence/examples/parkour.rs` now has `ParkourGameplayPlugin`, `ParkourGameplayPhase`, and `ParkourGameplayPluginContract`. `main` only installs `DefaultPlugins` plus the opt-in example plugin. The phase contract orders input, rule evaluation, world mutation, presentation, and cleanup systems; parkour block generation remains in the existing helper functions and the plugin remains a scheduling shell.
- `servers/valence/examples/game_of_life.rs` now has `LifeGameplayPlugin`, `LifeGameplayPhase`, and `LifeGameplayPluginContract`. The plugin owns startup/setup and update-phase wiring; `LifeBoard` remains the pure simulation core with existing cadence tests plus plugin-enabled/disabled tests.
- `servers/valence/examples/combat.rs` now has `CombatGameplayPlugin`, `CombatGameplayPhase`, and `CombatGameplayPluginContract`. The existing `CombatCooldownPlugin` remains the cooldown subplugin/resource owner, while the gameplay plugin installs the arena setup, event-loop combat handler, update systems, and cooldown subplugin. Existing cooldown pure helpers and negative tests remain intact.
- `servers/valence/examples/command.rs` now has `CommandExamplePlugin`, `CommandExamplePhase`, and `CommandExamplePluginContract`. Command registration for `test`/`t`, `teleport`/`tp`, `gamemode`/`gm`, `complex`, and `struct` moved into the opt-in plugin; command names, aliases, admin scope setup, and handler text were not intentionally changed.
- `servers/valence/examples/advancement.rs` now has `AdvancementExamplePlugin`, `AdvancementExamplePhase`, and `AdvancementExamplePluginContract`. `ClientSave` initialization and deferred advancement setup moved into the plugin. Root2 tab thresholds are named constants with pure helper tests, and the deferred boundary remains explicit.
- `servers/valence/examples/building.rs` now has `BuildingExamplePlugin`, `BuildingExamplePhase`, and `BuildingExamplePluginContract`. Deterministic decisions for gamemode toggling, block-break eligibility, and placement axis selection moved into pure helpers with positive and negative tests; ECS systems remain adapters over queries/events/resources.
- `servers/valence/examples/death.rs` now has `DeathExamplePlugin`, `DeathExamplePhase`, and `DeathExamplePluginContract`. Respawn layer cycling moved into `next_respawn_layer_index`, which has positive wraparound and negative empty-layer tests; the system remains the shell that mutates Valence layer components.
- `servers/valence/examples/world_border.rs` now has `WorldBorderExamplePlugin`, `WorldBorderExamplePhase`, and `WorldBorderExamplePluginContract`. Chat command parsing and border mutation planning moved into pure helpers with positive `add`/`center` and negative malformed/unknown command tests; the system applies the planned mutation to Valence resources.

## Compatibility boundaries

- Example binary names remain unchanged.
- No selected example gained a default Valence gameplay path; all gameplay remains opt-in through the same example binary entry points.
- No CLI or environment contract was added or removed for the selected examples.
- Command names and aliases in `command` remain `test`/`t`, `teleport`/`tp`, `gamemode`/`gm`, `complex`, and `struct`.
- World border chat controls remain `add <value> <ticks>` and `center <x> <z>`.
- Joining-client messages and setup intents are preserved unless a pure helper now fails closed for malformed world-border chat input instead of panicking.

## Verification

- Pre-implementation Cairn gates and validation: `docs/evidence/run-logs/2026-06-26/organize-remaining-valence-examples-pre-gates.run.log`.
- Pre-edit selected-example and schedule baseline: `docs/evidence/run-logs/2026-06-26/organize-remaining-valence-examples-baseline.run.log`.
- Focused final formatting, selected example tests, and Valence schedule hygiene: `docs/evidence/run-logs/2026-06-26/organize-remaining-valence-examples-focused-final.run.log`.
- Individual focused example logs were also captured under `docs/evidence/run-logs/2026-06-26/organize-remaining-valence-examples-*-test.run.log` while the implementation was built incrementally.

## Non-claims

This implementation does not claim broad Minecraft compatibility, vanilla parity, semantic equivalence, production readiness, public-server safety, full CTF correctness, full survival correctness, or default Valence gameplay behavior. The selected examples remain opt-in examples and schedule documentation surfaces.
