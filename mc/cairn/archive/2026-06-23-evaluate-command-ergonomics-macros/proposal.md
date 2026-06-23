# Proposal: Evaluate command ergonomics macros

## Why

Hyperion's Clap-based command derives are ergonomic for simple command definitions, but Valence has its own command graph, parsers, scopes, and handler model. Valence may benefit from optional derive/macros or builder helpers that reduce boilerplate while preserving Valence command internals.

## What Changes

- Review Hyperion command ergonomics and Valence command graph APIs.
- Define a reference-only integration boundary that forbids replacing Valence's command internals.
- Specify optional derive or builder helper semantics for literals, arguments, suggestions, scopes, execution handlers, and diagnostics.
- Add positive and negative tests for generated command graphs, parser errors, missing handlers, duplicate literals, invalid scopes, suggestions, and plugin-disabled behavior.
- Document migration guidance and when manual command graph construction remains preferred.

## Impact

- **Files**: optional `valence_command` macros/builders, examples/docs, tests, and Cairn artifacts.
- **Testing**: macro expansion or builder tests, graph parity tests, parser/suggestion fixtures, duplicate/invalid command negative tests, and Cairn gates/validation.
- **Non-claims**: this does not replace Valence command internals, does not import Hyperion's Clap command framework directly, and does not change existing command behavior unless users opt in.
