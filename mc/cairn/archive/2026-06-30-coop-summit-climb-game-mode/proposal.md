# Proposal: Add cooperative summit climb game mode

## Why

We want a Minecraft-native co-op mountain-climbing mode that captures the teamwork, stamina pressure, rescue loop, and vertical expedition fantasy from the user's Peak-inspired request without copying protected names, assets, characters, presentation, or branded items. Hyperion already hosts game-specific event logic under `events/bedwars`, so a separate summit-climb event crate can prove the mode without changing Bedwars or default engine behavior.

The key mechanic is free-surface climbing: players must be able to climb eligible mountain faces in this mode without relying on no-cost vines, ladders, scaffolding, or other vanilla climbable block scaffolds. That requires a mode-local server-authoritative climbing core over contact, input, stamina, and surface policy rather than a map trick that paints walls with climbable blocks.

## What Changes

- Introduce a Hyperion-owned `summit_climb` game mode/event plugin with lobby/start, co-op team assignment, mountain-region progression, campsite checkpoints, win/loss, and clean mode-local state.
- Use Bevy as the primary runtime model: plugins, components, resources, events, states, system sets, run conditions, hierarchy relationships, observers, and Bevy task-pool integration where they fit.
- Add a free-surface climbing mechanic that detects eligible wall/ledge contact and applies climb motion or server correction without requiring no-cost vine, ladder, scaffold, or water-column blocks.
- Add summit terrain enablement for a mode-owned mountain map or generator source, staged routes, regions, campsites, summit trigger, hazards, rescue affordances, and terrain validation.
- Add stamina, hunger, injury/ailment, downed/revive, campsite-rest, item, and hazard rule cores with thin ECS/network shells.
- Add climbing-assist items such as rope, rope launcher, piton/rest anchor, food, medicine, and stamina boost using original names/presentation.
- Treat ladder, vine, scaffold, water, and bubble-column route aids as optional stamina-consuming climb surfaces when configured, never as no-cost replacements for free-surface climbing.
- Add positive and negative tests for free-surface climbing, terrain affordances, stamina-consuming vanilla assists, stamina failure, invalid climb attempts, revive rules, checkpoint restoration, item effects, hazards, and mode isolation.

## Impact

- **Files**: `hyperion/Cargo.toml`, new `hyperion/events/summit_climb/` crate, summit terrain metadata/map fixtures or generator inputs, possible Hyperion movement/physics extension seams under `hyperion/crates/*` if the mode needs a reusable contact/input hook, focused tests, `docs/evidence/` receipts.
- **Testing**: baseline Hyperion checks before core movement changes when applicable, summit-climb pure-core tests, plugin wiring tests, terrain affordance tests, focused Hyperion tests from `hyperion/`, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation before archive.
- **Non-claims**: this does not implement Peak assets or branding, vanilla climbing parity, vanilla terrain parity, production balance, custom client support, Bedwars behavior changes, Valence core behavior changes, public-server safety, or a general Hyperion movement rewrite.
