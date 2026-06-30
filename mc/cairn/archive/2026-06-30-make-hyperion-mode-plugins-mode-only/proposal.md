# Proposal: Make Hyperion mode plugins mode-only

## Why

`BedwarsPlugin`, `DayzPlugin`, and `HardcoreFactionsPlugin` currently install shared gameplay by adding `CommonGameplayPlugin` themselves. That makes each mode plugin a preset bundle instead of a small Bevy plugin that only declares mode identity and mode-local setup.

## What Changes

- Refactor mode plugins so they own only mode identity, mode-local resources, player initialization observers, and mode-specific setup.
- Move default shared gameplay installation into app builders or presets that intentionally compose gameplay plus one mode.
- Preserve existing default entrypoints by composing `HyperionCore`, the default gameplay group, and the selected mode.
- Document the difference between mode plugins and preset/builders.
- Add positive mode-only and default-preset tests plus negative tests for unintended shared gameplay installation.

## Impact

- **Files**: `hyperion/events/bedwars/src/lib.rs`, mode plugin definitions, app builder helpers, docs/evidence for behavior inventory and checks.
- **Testing**: focused Hyperion app-build tests for each mode, mode-only plugin tests, default builder compatibility checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this does not make multiple exclusive modes run together or create a builder API beyond the existing app construction functions.
