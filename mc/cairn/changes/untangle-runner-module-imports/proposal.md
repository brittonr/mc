# Proposal: Untangle mc-compat runner module imports

## Why

Several runner modules import the entire root namespace with `use super::*`. That hides dependency direction, makes modules harder to move or test independently, and lets new code depend on unrelated root symbols by accident.

## What Changes

- Replace production `use super::*` imports with explicit imports from owning modules.
- Move shared data types out of the root entrypoint into the modules that own them.
- Add dependency-boundary tests or static checks that prevent broad wildcard imports from returning in production modules.
- Preserve existing public CLI, receipt, scenario, and evidence behavior.

## Impact

- **Files**: `compat/runner/src/*.rs`, focused tests, optional checker/gate updates, and Cairn artifacts.
- **Testing**: baseline compile/test checks, post-refactor runner tests, import-boundary positive/negative fixtures, Cairn gates, and Cairn validation.
- **Non-claims**: code organization only; no new compatibility evidence or gameplay behavior changes.
