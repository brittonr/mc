# Split Valence command example responsibility map

## Question

What responsibilities does `servers/valence/examples/command.rs` own before the split, and which boundaries must remain reviewable after the refactor?

## Inspected evidence

- `servers/valence/examples/command.rs` before implementation.
- `docs/evidence/run-logs/2026-06-29/split-valence-command-example.baseline-command-example-tests.run.log`.
- Change requirements `r[valence_bevy_ecs.command_example.boundaries]`, `r[valence_bevy_ecs.command_example.handler_core]`, and `r[valence_bevy_ecs.command_example.parity]`.

## Decision

The pre-split example owns these interleaved responsibilities:

- App setup shell: `main`, `CommandExamplePlugin`, update phase ordering, plugin contract insertion, system registration, and default Valence plugin wiring.
- Command definition surface: derive-macro command enums/structs, explicit `ComplexRedirectionCommand::assemble_graph`, command paths, scopes, parsers, redirections, modifiers, and documentation comments explaining representative command shapes.
- Handler shell logic: Bevy event readers, queries, entity/client mutation, position and game-mode changes, chat packet side effects, random target selection, and logging.
- Pure or mostly pure handler decisions that can be extracted: gamemode selection, selector-to-action classification, teleport command intent compilation, fixed chat-message text selection, spawn-position constants, phase-order contract, and fixture-state summaries.
- Fixture/test helpers: minimal `App` setup for plugin-contract tests and fixture summaries needed to test handler outcomes without a full Bevy app.
- Non-claims: this change is organization and testability only; it does not promote new command framework behavior, Minecraft compatibility, semantic equivalence, production readiness, public-server safety, or CTF/survival correctness claims.

The split should keep Bevy app wiring, ECS queries, mutation, packet/chat side effects, and logging in imperative shells while moving deterministic decisions into testable helpers.

## Owner

`servers/valence/examples/command.rs` within the Valence subtree.

## Next action

Refactor the command example into focused modules/sections in the same example file or adjacent modules, then rerun focused command example tests and affected example checks before marking tasks complete.
