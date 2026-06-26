# Proposal: Add Bevy schedule hygiene gates

## Why

Valence has a `dump_schedule` tool and many named `SystemSet`s, but schedule structure is not yet a routine evidence artifact for Bevy-heavy changes. As examples and plugins become more schedule-driven, reviewers need lightweight gates that show new systems are in the intended phases, ambiguity is understood, and optional plugins do not silently alter default schedules.

## What Changes

- Inventory current schedule inspection tooling, named sets, and validation gaps for Valence Bevy changes.
- Define when a Cairn must include schedule evidence, such as new plugins, new schedules, changed ordering, or optional/default plugin changes.
- Extend or document schedule-dump checks for selected schedules and plugin-disabled comparisons.
- Add positive valid-schedule tests and negative unknown schedule, missing set, unintended default plugin, and ambiguity regression tests where feasible.
- Record generated schedule evidence under `docs/evidence/` with BLAKE3 manifests when cited by tasks.

## Impact

- **Files**: `servers/valence/tools/dump_schedule`, Valence docs/evidence, Cairn task guidance, tests/check scripts if needed.
- **Testing**: schedule-dump smoke tests, invalid schedule negative tests, plugin-disabled schedule comparisons, Cairn gates, and Cairn validation.
- **Non-claims**: this does not require full schedule graph review for every small change; it applies when Bevy schedule behavior is part of the change contract.
