# Proposal: Extract Hyperion common gameplay crate

## Why

Shared gameplay currently lives under `events/bedwars/src/plugin`, even though Dayz, HardcoreFactions, and future modes need reusable mechanics. Keeping common mechanics in a Bedwars-named event crate makes the public API misleading and encourages cross-mode changes to land in a mode-specific tree.

## What Changes

- Inventory each `events/bedwars/src/plugin/*` module as common, Bedwars-specific, or uncertain.
- Create or select a shared Hyperion gameplay crate/module boundary for mode-neutral mechanics.
- Move safe common mechanics and public plugin-group exports behind that boundary.
- Keep Bedwars-specific setup in the Bedwars event crate.
- Update imports, docs, and tests to prove default behavior is preserved.

## Impact

- **Files**: `hyperion/events/bedwars/src/plugin/*`, `hyperion/events/bedwars/src/lib.rs`, `hyperion/Cargo.toml`, possible `hyperion/crates/hyperion-gameplay/*`, docs/evidence inventories and checks.
- **Testing**: focused Hyperion build/test checks for moved modules, public API compile checks, default preset compatibility, positive/negative boundary tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this does not redesign mechanics, add new modes, or claim all existing plugins are reusable; uncertain modules may remain mode-local with documentation.
