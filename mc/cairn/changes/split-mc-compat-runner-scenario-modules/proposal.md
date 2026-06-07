# Proposal: Split mc-compat-runner scenario modules

## Why

The scenario behavior trait refactor centralized names, aliases, milestones, forbidden patterns, and exceptional hooks, but the runner still keeps most of that implementation inside one large `main.rs`. The single-file shape makes the new review surface harder to maintain and increases the risk that future scenarios reintroduce scattered matches or side-effectful validation logic.

## What Changes

- Extract scenario identity, static specs, behavior hooks, and validation into dedicated runner modules.
- Keep CLI parsing, process spawning, filesystem access, and receipt writing in the main imperative shell.
- Preserve all scenario names, aliases, dry-run output, receipt fields, manifest surfaces, and compatibility claims.
- Add module-level parity and negative tests so the split cannot hide behavior drift.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, new module files under `tools/mc-compat-runner/src/`, scenario manifest fixtures if needed, and evidence docs.
- **Testing**: runner tests, scenario manifest check, dry-run checks, positive parity tests, negative invalid-spec tests, and Cairn gates/validation.