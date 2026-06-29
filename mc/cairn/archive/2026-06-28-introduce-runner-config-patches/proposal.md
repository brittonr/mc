# Proposal: Introduce composable runner config patches

## Why

`compat/runner/src/main.rs` builds runner configuration by mutating one large `Config` through defaults, Nickel-exported JSON, restricted Steel config, environment variables, and CLI arguments. That hides precedence, makes partial configuration hard to test without global environment state, and couples source parsing to final validation.

## What Changes

- Introduce pure `ConfigPatch` and `ConfigSource` data for partial configuration updates and source labels.
- Parse defaults, config files, environment variables, and CLI arguments into ordered patches before applying them to a base configuration.
- Move precedence and validation into deterministic pure functions that can be tested without filesystem, process, or environment mutation.
- Preserve every existing CLI flag, environment variable, config-file field, default value, receipt shape, dry-run behavior, and non-claim boundary.
- Add positive and negative tests for precedence, missing values, invalid values, unsafe paths, and conflicting source updates.

## Impact

- **Files**: `compat/runner/src/main.rs`, new or existing config/runtime modules, config tests, optional docs for runner architecture, and Cairn artifacts.
- **Testing**: baseline config/runner tests, positive and negative config-patch fixtures, dry-run smoke checks, Cairn proposal/design/tasks gates, and Cairn validation.
- **Non-claims**: configuration architecture only; this does not add Minecraft compatibility evidence or change scenario semantics.
