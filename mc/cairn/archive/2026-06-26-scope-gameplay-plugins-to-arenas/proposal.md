# Proposal: Scope gameplay plugins to arenas and layers

## Why

CTF and survival compatibility are now opt-in gameplay plugins, but their current shape still assumes broad app-level state and systems. That makes them useful as examples and fixtures, but not yet safe to compose as multiple modes or multiple arenas in one Valence app. To make plugin composition real, gameplay state and events need explicit arena/layer ownership, and systems need to fail closed when they see entities outside their scope.

## What Changes

- Inventory CTF and survival compatibility global resources, entity/layer assumptions, event payloads, milestone emitters, cleanup paths, and cross-mode mutation risks.
- Introduce an arena/scope model that ties gameplay state to arena entities or layer-owned components instead of one global game-mode resource.
- Require gameplay systems to filter by scope and mutate only the arena, layer, clients, and entities they own.
- Include arena/scope identity in gameplay events and compatibility milestones where cross-arena ambiguity would otherwise exist.
- Add positive multi-mode/multi-arena tests and negative wrong-scope, stale-scope, missing-scope, disabled-plugin, and cross-layer mutation tests.

## Impact

- **Files**: `servers/valence/examples/ctf.rs`, `servers/valence/examples/survival_compat.rs`, shared gameplay scope helpers if introduced, focused tests, `docs/evidence/` receipts.
- **Testing**: CTF and survival plugin tests, selected compatibility rails when fixture behavior is touched, schedule hygiene, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not add BedWars/Hyperion scope, dynamic plugin loading, default Valence gameplay, full survival correctness, full CTF correctness, vanilla parity, or production readiness.
