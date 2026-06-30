# Proposal: Add Hyperion gameplay composition tests

## Why

The composable-mode refactors need a durable regression matrix. Without explicit tests for default presets, custom feature disables, replacement mechanics, mode-only plugins, exclusive-mode rejection, and disabled plugins, Bevy composition can regress silently.

## What Changes

- Build a focused composition test matrix for Hyperion game-mode plugin APIs.
- Cover default presets for Bedwars, Dayz, and HardcoreFactions.
- Cover custom presets that disable or replace one feature and add a harmless test plugin.
- Cover mode-only plugin behavior and exclusive-mode rejection.
- Promote logs and manifests so Cairn task evidence can prove composition behavior.

## Impact

- **Files**: Hyperion game-mode tests, potential test helpers, docs/evidence composition logs and BLAKE3 manifests.
- **Testing**: focused Hyperion test matrix with positive and negative cases, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this Cairn adds tests and evidence; it does not by itself refactor plugin APIs unless small test hooks are required.
