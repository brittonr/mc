# Hyperion/Valence plugin integration inventory

## Question
Can the active Hyperion/Valence integration Cairns be closed with bounded, reviewable evidence for typed Hyperion preset slots/preflight/diagnostics and Valence schedule receipts/scoped contract metadata/template helpers?

## Inspected evidence
- Hyperion local files: `hyperion/crates/hyperion-game-modes/src/composition.rs` and `hyperion/events/bedwars/src/lib.rs`.
- Valence local files: `servers/valence/src/tests/core_plugin_sets.rs`, `servers/valence/examples/gameplay_contracts/mod.rs`, `servers/valence/examples/ctf.rs`, `servers/valence/examples/ctf/schedule_contracts.rs`, `servers/valence/examples/survival_compat.rs`, and `servers/valence/examples/terrain.rs`.
- Cairn change packages: `add-plugin-composition-preflight`, `execute-hyperion-preset-plugin-slots`, `expose-plugin-diagnostics-registry`, `add-structured-schedule-receipts`, `support-scoped-plugin-instances`, `add-gameplay-plugin-template-helper`, and `unify-plugin-contract-metadata`.

## Inventory
- Hyperion composition previously allowed name-only preset declarations to imply plugins that could not execute. The new boundary keeps a pure `PresetPlan`/diagnostics core in `hyperion-game-modes` and a Bedwars app-builder shell that validates typed executable slots before mutating `App`.
- Hyperion default/custom/replacement composition risks were duplicate modes, duplicate features, unsupported replacements, missing dependencies, missing executable slots, stale diagnostics, direct plugin misuse, and partial app mutation. The implementation now returns typed composition or diagnostics errors before adding resources/plugins.
- Hyperion diagnostics now record mode, installed/disabled/replaced features, dependency decisions, custom slots, provenance, and compiled-plugin non-claims. Direct Bevy plugin insertion remains outside the fallible builder guarantee.
- Valence schedule hygiene previously depended on selected DOT graph string assertions. The new structured receipt helper keeps Bevy schedule collection in shell code and validates expected/observed/absent facts in deterministic pure functions.
- Valence scoped gameplay fixtures already used primary CTF/survival/terrain scope IDs and wrong-scope filters in selected systems. The shared contract metadata now carries optional concrete scope, install mode, scope model, schedules, owned resources/events, and non-claims so reviewers can distinguish source adapters from scoped gameplay plugins.
- Valence template-helper work centralizes contract validation/registration for compiled examples while leaving gameplay logic, schedule wiring, packet I/O, resource mutation, and milestone emission in plugin-owned shells.

## Decision
Close the seven active integration changes with focused unit/example tests, formatting checks, Cairn gates, task-evidence validation, and evidence-manifest validation. No live Paper/Valence compatibility semantics are promoted by this closeout; the evidence is bounded to compiled plugin composition, metadata, schedule receipt facts, and selected scope guards.

## Owner
Pi agent / Britton working copy on 2026-06-30.

## Next action
Run final focused Hyperion and Valence checks, refresh the BLAKE3 manifest, mark tasks with promoted evidence, run Cairn/task/evidence-manifest gates, then archive completed changes if all gates pass.
