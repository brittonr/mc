# Add schedule hygiene gates inventory

## Scope

This evidence note records the schedule hygiene inventory and policy for `add-schedule-hygiene-gates`. It is review evidence for the Cairn requirements `r[valence_bevy_ecs.schedule_hygiene.inventory]`, `r[valence_bevy_ecs.schedule_hygiene.policy]`, `r[valence_bevy_ecs.schedule_hygiene.receipts]`, `r[valence_bevy_ecs.schedule_hygiene.tests]`, `r[valence_bevy_ecs.schedule_hygiene.evidence]`, and `r[valence_bevy_ecs.schedule_hygiene.validation]`.

## Current schedule tooling

- `servers/valence/tools/dump_schedule` builds a `valence::prelude::App`, adds `DefaultPlugins`, lists available schedules when no schedule label is supplied, rejects unknown schedule labels with a diagnostic, and renders a selected schedule through `bevy_mod_debugdump::schedule_graph::schedule_graph_dot` plus Graphviz `dot`/`tred`.
- The tool currently sets `ambiguity_enable: false`, so generated graph evidence does not claim a complete ambiguity review unless a focused checker or receipt records the ambiguity policy separately.
- `servers/valence/tools/dump_schedule/README.md` now defines when schedule evidence is expected and which receipt fields reviewers should see.
- `tools/check_valence_schedule_hygiene.rs` is the focused gate for this policy. Its pure receipt validator has a positive valid-schedule fixture and negative unknown schedule, missing set, unintended default plugin, and ambiguity regression fixtures.

## Named schedules

| Schedule label | Owner | Schedule note |
| --- | --- | --- |
| `First` | Bevy app default | Main-loop startup phase before `PreUpdate`. |
| `PreUpdate` | Bevy app default | Valence spawn/input preparation systems run here. |
| `RunEventLoop` | `valence_server::event::loop::EventLoopPlugin` | Inserted after `PreUpdate`; runs Valence packet/event-loop child schedules. |
| `EventLoopPreUpdate` | `EventLoopPlugin` | Packet/event handling phase before gameplay update. |
| `EventLoopUpdate` | `EventLoopPlugin` | Event-loop update phase used by examples and optional systems. |
| `EventLoopPostUpdate` | `EventLoopPlugin` | Event-loop post-update phase for post-packet observations. |
| `Update` | Bevy app default | Main gameplay update phase. |
| `PostUpdate` | Bevy app default | Packet flushes, registry/cache updates, entity/layer updates, and many Valence output systems run here. |
| `Last` | Bevy app default | End-of-main-loop cleanup phase. |
| `PreStartup`, `Startup`, `PostStartup` | Bevy app defaults | App initialization schedules. |

## Named system sets

| System set | Owner path | Scheduled phase recorded by source/docs |
| --- | --- | --- |
| `SpawnClientsSet` | `servers/valence/crates/valence_server/src/client.rs` | `PreUpdate`. |
| `FlushPacketsSet` | `servers/valence/crates/valence_server/src/client.rs` | `PostUpdate`. |
| `UpdateClientsSet` | `servers/valence/crates/valence_server/src/client.rs` | `PostUpdate`. |
| `UpdateLayersPreClientSet` | `servers/valence/crates/valence_server/src/layer.rs` | `PostUpdate`, before client updates. |
| `UpdateLayersPostClientSet` | `servers/valence/crates/valence_server/src/layer.rs` | `PostUpdate`, after client updates. |
| `InitEntitiesSet` | `servers/valence/crates/valence_entity/src/lib.rs` | `PostUpdate`. |
| `UpdateTrackedDataSet` | `servers/valence/crates/valence_entity/src/lib.rs` | `PostUpdate`. |
| `ClearEntityChangesSet` | `servers/valence/crates/valence_entity/src/lib.rs` | `PostUpdate`. |
| `HitboxShapeUpdateSet` | `servers/valence/crates/valence_entity/src/hitbox.rs` | `PreUpdate`. |
| `HitboxUpdateSet` | `servers/valence/crates/valence_entity/src/hitbox.rs` | `PreUpdate`, after `HitboxShapeUpdateSet`. |
| `HitboxComponentsAddSet` | `servers/valence/crates/valence_entity/src/hitbox.rs` | `PostUpdate`. |
| `RegistrySet` | `servers/valence/crates/valence_registry/src/lib.rs` | `PostUpdate`. |
| `CommandSystemSet` | `servers/valence/crates/valence_command/src/lib.rs` | `EventLoopPreUpdate`. |
| `ScoreboardSet` | `servers/valence/crates/valence_scoreboard/src/lib.rs` | `PostUpdate`. |
| `WriteAdvancementPacketToClientsSet` | `servers/valence/crates/valence_advancement/src/lib.rs` | `PostUpdate`. |
| `WriteAdvancementToCacheSet` | `servers/valence/crates/valence_advancement/src/lib.rs` | `PostUpdate`. |
| `UpdateWorldBorderSet` | `servers/valence/crates/valence_world_border/src/lib.rs` | `PostUpdate`. |
| `PlayerListSet` | `servers/valence/crates/valence_player_list/src/lib.rs` | Private plugin set; source-owned schedule membership. |

## Default plugin behavior

`DefaultPlugins` unconditionally add the core server, registry, biome, dimension type, entity, hitbox, layer, client, event-loop, movement, packet/action, interaction, status, resource-pack, and abilities plugins. Optional/default behavior is Cargo feature gated for `log`, `network`, `player_list`, `equipment`, `inventory`, `anvil`, `advancement`, `weather`, `world_border`, `boss_bar`, `command`, and `scoreboard` plugins.

A schedule-impacting change that touches optional/default membership must include a plugin-enabled and disabled-plugin comparison when disabled behavior is part of the contract. The receipt should name the disabled plugin and assert that the forbidden plugin-owned schedule facts are absent from the disabled configuration.

## Trigger policy

Schedule evidence is expected when a Cairn or direct change touches any of these surfaces:

- new or removed Bevy plugins;
- new or removed schedule labels;
- new or removed `SystemSet`s;
- ordering constraints such as `before`, `after`, chained sets, or set membership changes;
- event-loop phase placement or `MainScheduleOrder` changes;
- default plugin membership or optional feature gating.

Schedule evidence is not required for changes that do not affect Bevy schedule behavior, but those changes should preserve this non-requirement explicitly when reviewers could otherwise infer schedule coverage.

## Receipt contract

Focused schedule receipts should record the command, working directory or source root, selected schedule label, plugin configuration, expected sets/systems, disabled-plugin comparison when relevant, ambiguity policy, and non-claims. Large SVG graph artifacts remain optional unless reviewers need graph shape evidence. A disabled-plugin comparison should name both the enabled/default configuration and the disabled configuration.

## Active checks and evidence gaps

- `tools/check_valence_schedule_hygiene.rs --self-test` covers one positive valid-schedule receipt and the negative unknown schedule, missing set, unintended default plugin, and ambiguity regression paths.
- `tools/check_valence_schedule_hygiene.rs --root .` verifies this inventory, the `dump_schedule` README policy, current Valence schedule/source tokens, and the Nix check wiring.
- `nix build .#checks.x86_64-linux.mc-valence-schedule-hygiene --no-link -L` makes the checker available as a tracked gate.
- `cargo test -p dump_schedule` remains the Valence package compile/test smoke for the graph utility.
- Evidence gaps are intentional: the checker does not prove full graph semantic equivalence, public-server safety, production readiness, full gameplay correctness, or broad compatibility. No broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness claim is made by this schedule hygiene evidence.
