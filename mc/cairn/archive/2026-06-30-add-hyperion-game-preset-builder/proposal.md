# Proposal: Add Hyperion game preset builder

## Why

After shared gameplay and mode identity are separated, callers need a clear API for building common presets and custom compositions. Today selection is mostly `GameType` plus fixed app builders, so disabling bow, replacing damage, or adding custom mechanics requires editing the event crate wiring.

## What Changes

- Define a typed preset/builder API that plans Hyperion core, shared gameplay, mode plugin, optional feature disables/replacements, and custom plugin additions.
- Keep preset validation in a pure core over explicit configuration so invalid composition can be tested without standing up the world.
- Add imperative-shell builder functions that apply a validated plan to Bevy `App` construction.
- Preserve existing entrypoints as compatibility wrappers around the default preset.
- Add positive custom-preset tests and negative invalid-composition diagnostics.

## Impact

- **Files**: `hyperion/events/bedwars/src/lib.rs`, possible `preset.rs` or `builder.rs`, public exports, docs/evidence for API inventory and checks.
- **Testing**: pure preset-plan tests, app-builder shell tests, default-entrypoint compatibility checks, negative invalid mode/feature tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks.
- **Non-claims**: this does not require runtime hot-loading, dynamic shared libraries, or support for multiple exclusive world modes in one app.
