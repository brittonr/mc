# Scenario behavior traits evidence — 2026-06-06

## Scope

This evidence covers the fixture-level refactor of `tools/mc-compat-runner/src/main.rs` from scattered scenario match tables to static `ScenarioSpec` rows plus explicit `ScenarioBehavior` hooks.

## Implemented behavior

- `Scenario` remains the stable identity for CLI/config parsing, receipt names, and manifest references.
- Static specs now hold canonical names, aliases, client milestones, server milestones, and forbidden patterns in one reviewable surface.
- Behavior hooks cover dynamic projectile-health matching, MCP controlled smoke, restart/crash-recovery enrichment, persistence storage, multi-client/reconnect strategy, count markers, and probe/server environment setup.
- Runtime execution validates static scenario specs before producing compatibility evidence.
- Positive tests cover every scenario in `ALL_SCENARIOS`; negative tests reject unknown names, missing aliases, duplicate canonical names, missing milestones, missing forbidden patterns, and missing required hooks.

## Validation

`docs/evidence/scenario-behavior-traits-focused-checks-2026-06-06.run.log` records passing focused Nix checks for the runner, scenario manifest, smoke dry-run, and multi-client dry-run with `exit_status=0`.

`docs/evidence/scenario-behavior-traits-evidence-checks-2026-06-06.run.log` and `docs/evidence/scenario-behavior-traits-task-evidence-final-2026-06-06.run.log` record passing evidence-manifest/task-evidence gates for the completed task citations.

`docs/evidence/scenario-behavior-traits-sync-2026-06-06.run.log` records accepted-spec sync, and `docs/evidence/scenario-behavior-traits-archive-2026-06-06.run.log` records archive plus post-archive Cairn validation with zero active changes.

## Non-claims

This refactor does not promote new live Paper/Valence parity claims, does not expand public-server safety, and does not change scenario receipt schema or canonical scenario strings.
