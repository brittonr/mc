# Proposal: Separate gameplay config sources from plugin behavior

## Why

Several gameplay examples refresh runtime configuration directly from environment variables inside gameplay systems. That is convenient for fixtures, but it hides side effects inside the plugin, makes tests depend on process state, and blocks multi-arena composition where each arena may need different config. Gameplay plugins should consume typed config supplied by a separate source shell.

## What Changes

- Inventory CTF, survival compatibility, terrain, and selected example env/CLI/config reads, runtime refresh systems, defaults, validation, and test assumptions.
- Define typed config values with pure parsing/validation cores over explicit inputs.
- Split env/CLI/file source adapters from gameplay plugins so gameplay systems consume typed resources or arena-owned config components.
- Make runtime reload explicit and scoped, rather than implicit process-env reads during gameplay phases.
- Add positive typed-config/default tests and negative malformed, missing, stale, wrong-scope, and disabled-source tests.

## Impact

- **Files**: selected `servers/valence/examples/` plugins, shared config helpers if introduced, focused tests, `docs/evidence/` receipts.
- **Testing**: pure config parser tests, plugin config-source tests, focused CTF/survival/terrain/example checks, schedule hygiene when wiring changes, Cairn gates, Cairn validation, and task-evidence validation.
- **Non-claims**: this does not add Nickel or file-backed runtime config unless explicitly scoped by implementation, does not change default gameplay claims, and does not touch BedWars/Hyperion.
