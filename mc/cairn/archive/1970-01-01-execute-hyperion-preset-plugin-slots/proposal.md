# Proposal: Execute Hyperion preset plugin slots

## Why

Hyperion preset planning already records replacement features and custom plugin intents, but the app builder currently only disables replacement targets and keeps custom plugin names as diagnostics. A caller can describe a custom preset without a typed way to install the replacement or custom plugin, which makes the preset API look more capable than the runtime behavior.

## What Changes

- Add a typed plugin-slot composition surface for Hyperion presets and app builders.
- Keep pure preset planning focused on semantic validation while the app-builder shell owns Bevy plugin installation.
- Make feature replacements disable the default feature and install the provided replacement plugin in a deterministic order.
- Make custom plugin additions executable through typed installers instead of name-only intents.
- Preserve existing default Bedwars, Dayz, and HardcoreFactions entrypoints.

## Impact

- **Files**: `hyperion/crates/hyperion-game-modes`, `hyperion/events/bedwars` preset/app-builder code, public gameplay composition docs, and evidence under `docs/evidence/`.
- **Testing**: pure preset-slot planning tests, app-builder positive installation tests, negative missing-slot/duplicate-slot tests, existing default builder compatibility checks, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this does not add runtime-loaded shared libraries, hot reload, scripting, or untrusted plugin sandboxing; plugins remain compiled Bevy plugins.
