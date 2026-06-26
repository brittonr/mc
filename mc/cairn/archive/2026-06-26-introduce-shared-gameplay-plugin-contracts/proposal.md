# Proposal: Introduce shared gameplay plugin composition contracts

## Why

Valence gameplay examples now use opt-in Bevy plugin shells, but CTF, survival compatibility, terrain, and smaller examples each define local phase sets and local contract resources. That proves the pattern, but it leaves downstream users without one reviewable composition vocabulary for ordering plugins together. A shared contract should make gameplay plugins easier to inspect, compose, and test without promoting examples into default Valence gameplay.

## What Changes

- Inventory existing gameplay/example plugin phases, contract resources, schedule labels, resources, events, and disabled-plugin tests.
- Define a shared gameplay phase vocabulary for input, rule evaluation, world mutation, presentation, and cleanup where those phases exist.
- Add a minimal gameplay plugin contract surface that records schedule labels, phase order, owned resources/events, scope model, installation mode, and explicit non-claims.
- Add reusable positive and negative schedule/contract test helpers for opt-in gameplay plugins.
- Keep plugin-local subphases private unless a plugin deliberately promotes them as a stable ordering contract.

## Impact

- **Files**: shared Valence gameplay/plugin support code, selected example plugin shells, focused tests, `docs/evidence/` receipts.
- **Testing**: focused example/plugin tests, contract helper tests, Valence schedule hygiene, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not add dynamic runtime plugins, does not make CTF/survival default Valence behavior, does not touch Hyperion/BedWars, and does not claim vanilla parity or production readiness.
