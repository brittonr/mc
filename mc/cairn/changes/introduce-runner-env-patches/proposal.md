# Proposal: Introduce composable runner environment patches

## Why

Scenario env setup currently mutates `Command` directly inside scenario behavior methods. That makes env derivation hard to test without command construction, prevents composition of reusable env fragments, and hides conflicts between client, Valence, and Paper env settings.

## What Changes

- Introduce pure `EnvPatch` data for key/value additions, removals, and conflict diagnostics.
- Have scenario client, Valence server, and Paper server env derivation return `EnvPatch` values instead of mutating `Command` directly.
- Keep `Command` mutation in a thin shell that applies validated patches.
- Add positive and negative tests for patch composition, per-scenario env output, and conflict handling.

## Impact

- **Files**: scenario behavior modules, backend/client command shells, focused env-patch tests, possibly generated scenario metadata, and Cairn artifacts.
- **Testing**: baseline env behavior checks, positive env-patch fixtures, negative conflict/malformed fixtures, runner tests, Cairn gates, and Cairn validation.
- **Non-claims**: env composition architecture only; no new compatibility evidence or gameplay behavior changes.
