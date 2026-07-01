# Proposal: Add plugin composition preflight

## Why

Some plugin composition errors are currently discovered only while Bevy plugins are being added, often through assertions or duplicate-plugin panics. That makes invalid presets harder to diagnose and risks partial app mutation before failure. Hyperion should validate composition plans before mutating `App` wherever the builder path controls installation.

## What Changes

- Add a pure composition preflight that validates selected mode, default gameplay inclusion, feature disables, replacements, custom slots, and dependency constraints before app mutation.
- Return typed diagnostics for invalid composition instead of requiring callers to interpret panics.
- Make builder paths apply a validated plan transactionally.
- Keep direct Bevy plugin misuse guarded with deterministic diagnostics where Bevy allows it.

## Impact

- **Files**: Hyperion game-mode composition core, Bedwars app builders, preset diagnostics, selected tests, and evidence docs/logs.
- **Testing**: pure preflight tests, builder partial-app-prevention tests, direct-plugin misuse diagnostics, default preset compatibility tests, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests.
- **Non-claims**: direct `App::add_plugins` cannot become fallible through Bevy; this change improves controlled builder paths and best-effort direct diagnostics.
