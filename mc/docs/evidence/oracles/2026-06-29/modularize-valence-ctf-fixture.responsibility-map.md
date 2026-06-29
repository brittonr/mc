# Valence CTF fixture responsibility map — 2026-06-29

## Scope

This map supports Cairn change `modularize-valence-ctf-fixture`. It records the modular ownership after extracting CTF fixture cores and shells. It is architecture evidence only: it does not claim full CTF correctness, public-server safety, production readiness, semantic equivalence, or broad Minecraft compatibility.

## Pre-edit baseline

- Workflow docs read: `AGENTS.md`, `README.md`, `docs/check-tiers.md`, `servers/valence/AGENTS.md`, `servers/valence/README.md`, and `servers/valence/CONTRIBUTING.md`.
- Baseline focused test: `docs/evidence/run-logs/2026-06-29/modularize-valence-ctf-fixture.baseline-valence-ctf-example-tests.run.log` (`exit_status=0`, 64 tests passed).
- Preflight Cairn gates: proposal, design, tasks, and validate logs under `docs/evidence/run-logs/2026-06-29/` all contain `exit_status=0`.

## Module ownership

| Responsibility | Owner | Pure core | Shell boundary |
| --- | --- | --- | --- |
| Runtime config | `servers/valence/examples/fixture_core/ctf/runtime_config.rs` | `parse_runtime_config`, `runtime_config_issues`, env flag mode helpers | `CtfRuntimeConfigInputs::from_env`, `CtfRuntimeConfigSourcePlugin`, and `reload_ctf_runtime_config_from_source` read env/events and mutate the Bevy resource. |
| Arena setup | `servers/valence/examples/ctf/arena.rs` | Boundary documented in `fixture_core::ctf::boundaries`; arena chunk/sign/portal mutation is intentionally shell code. | `setup` delegates flag/spawn-box construction to `arena::build_flag` and `arena::build_spawn_box`; `LayerBundle`, `Commands`, NBT signs, and portal resources remain in the shell. |
| Schedule contracts | `servers/valence/examples/ctf/schedule_contracts.rs` | Constant contract data names plugin install mode, schedules, owned resources/events, scope, and non-claims. | `CtfRuntimeConfigSourcePlugin` and `CtfGameplayPlugin` register contracts and systems with Bevy schedules. |
| Team rules | `servers/valence/examples/fixture_core/ctf/team.rs` and `spawn.rs` | Team labels/opponents plus spawn reset assignment decisions. | `do_team_selector_portals` applies components, inventory, position, packet, chat, and layer mutations. |
| Flag and race rules | `servers/valence/examples/fixture_core/ctf/flags.rs` | Flag pickup acceptance, stale/held flag rejection, race final-state checks, invalid pickup/return/drop milestone text. | `digging`, `do_flag_capturing`, and disconnect handling mutate `FlagManager`, blocks, `HasFlag`, and logs. |
| Scoring rules | `servers/valence/examples/fixture_core/ctf/scoring.rs` | Score snapshots and score-limit milestone text. | `initial_score`, `log_score_limit_capture_and_win`, `Score`, and `WinConditionState` own Bevy resource mutation and emissions. |
| Inventory probes | `servers/valence/examples/fixture_core/ctf/inventory.rs` | Stack split/merge and drag transaction classifiers over in-memory snapshots. | `log_inventory_click_state` maps `ClickSlotEvent`/`ItemStack` values and emits milestones or opens containers. |
| Combat probes | `servers/valence/examples/fixture_core/ctf/combat.rs` | Reference-hit, armor mitigation, vanilla armor formula, and knockback metric decisions. | `handle_combat_events` owns ECS queries, cooldown, velocity, status, health, flag return, and logs. |
| Projectile probes | `servers/valence/examples/fixture_core/ctf/projectile.rs` plus CTF arrow-policy helpers in `ctf.rs` | Sequence matching and bounded projectile travel/collision marker formatting; arrow damage policy evaluation remains deterministic and tested. | `emit_projectile_travel_collision_probe_markers`, `handle_projectile_events`, and the combat projectile branch own events, health mutation, status packets, and log emission. |
| Milestone formatting | `fixture_core::ctf::{flags, scoring, spawn, projectile}` | Formatters return strings from explicit snapshots/contracts. | Systems own `info!`/`println!`, resource mutation, packet/event emission, and filesystem reads for Steel policy loading. |
| Boundary checks | `servers/valence/examples/fixture_core/ctf/boundaries.rs` | `validate_module_boundaries` rejects missing categories and empty owner/core/shell/non-claim fields. | Example tests execute the positive and negative boundary fixtures without starting a server. |

## Validation notes

- Focused post-change CTF example test logs show positive and negative core tests passing, including runtime config, boundary, projectile, inventory, combat, score, team-balance, flag, and unsupported Steel arrow-policy inputs.
- `cargo +nightly` is unavailable in the mc devshell because rustup toolchain selection is absent; the devshell `cargo fmt --all -- --check` was used instead and passes with stable-channel warnings only.
- `tools/check_runtime_steel_config.rs` currently fails for broader pre-existing runner/runtime inventory drift and stale historical call-site span expectations. It is not cited as completion evidence for this change.
