## Phase 1: Baseline and inventory

- [ ] [serial] Capture the current furnace selected-row baseline and Valence runtime-shell inventory before implementation, including fixture validation, core self-test with the selected fixture, receipt-handoff validation, shell-contract validation, and inspection of the exact Valence furnace/inventory/block-entity/layer APIs and schedule surfaces that the shell will touch. r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.inventory] r[valence_bevy_ecs.schedule_hygiene.policy]

## Phase 2: Opt-in shell implementation

- [ ] [depends:vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.inventory] Implement the selected-row opt-in Valence shell so it snapshots runtime furnace state into the existing pure core, commits only returned state or typed diagnostics, registers only explicit opt-in resources/events/systems, preserves the documented data-loading and mutation boundaries, and leaves `DefaultPlugins` unchanged. r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell]
- [ ] [depends:vanilla_composable_plugins.furnace_smelting_valence_runtime_shell] Add focused positive shell tests for selected-row fuel start, active burn progress, output production, and compatible output merge without starting a live server. r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests] r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests.positive]
- [ ] [depends:vanilla_composable_plugins.furnace_smelting_valence_runtime_shell] Add focused negative shell tests for invalid input, no fuel, blocked output, unsupported furnace kind, malformed data, stale or unloaded block entity/state, and disabled-plugin behavior, proving no false mutation or false milestone is emitted. r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests] r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests.negative]

## Phase 3: Evidence and documentation

- [ ] [depends:vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests] Record focused schedule hygiene evidence for any new plugin wiring, schedule labels, system sets, ordering constraints, resources, or events; include a disabled-plugin comparison when relevant. r[valence_bevy_ecs.schedule_hygiene.receipts] r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.evidence]
- [ ] [depends:vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests] Update furnace selected-row docs to distinguish local pure-core evidence, selected-row receipt handoff evidence, opt-in runtime-shell evidence, and deferred breadth/live-rail claims. r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.docs]

## Phase 4: Closeout

- [ ] [depends:vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.evidence,vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.docs] Run baseline fixture/core/receipt/contract checks, focused shell positive and negative tests, the smallest affected Valence Cargo check, schedule hygiene when wiring changes, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and archive receipts before closeout. r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.closeout]
