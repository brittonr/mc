# Proposal: Validate exclusive Hyperion modes

## Why

The current app path selects one `GameType`, but Bevy plugins can be added manually. Once mode plugins become more composable, adding two world-mode plugins should fail clearly instead of leaving the last `ActiveGameType` write or overlapping observers to define behavior implicitly.

## What Changes

- Define a mode exclusivity contract that distinguishes exclusive world modes from additive feature plugins.
- Add a lightweight registry or validation resource for selected exclusive mode identity.
- Make mode plugins or preset builders reject duplicate exclusive modes with deterministic diagnostics.
- Preserve additive gameplay feature composition.
- Add positive one-mode-plus-features tests and negative two-mode tests.

## Impact

- **Files**: Hyperion game-mode plugin definitions, preset/app-builder validation, possible mode registry core, docs/evidence for exclusivity checks.
- **Testing**: pure exclusivity validation tests, Bevy app composition tests, negative duplicate-mode tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this does not implement multi-arena or multi-world concurrent modes; it explicitly keeps current single exclusive world-mode semantics.
