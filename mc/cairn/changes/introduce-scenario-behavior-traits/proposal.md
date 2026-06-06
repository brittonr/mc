# Proposal: Introduce scenario behavior traits

## Why

`tools/mc-compat-runner` encodes scenario parsing, names, client milestones, server milestones, forbidden patterns, and probe behavior in large matches. Adding or reviewing a scenario requires touching several distant tables, which makes omissions likely and makes behavior harder to test independently.

## What Changes

- Add a `ScenarioBehavior` boundary backed by static scenario specs for simple scenarios and explicit implementations only for exceptional behavior.
- Preserve the current `Scenario` enum as the configured identity and receipt name source.
- Move required client/server milestones, forbidden patterns, and probe hooks behind one scenario spec/behavior surface.
- Add parity tests that prove every existing scenario keeps the same name, accepted aliases, required milestones, forbidden patterns, and special probe behavior.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, generated scenario manifest checks if needed, and focused runner tests.
- **Testing**: scenario parse/name parity, milestone parity, forbidden-pattern parity, special-case probe parity, unknown-scenario rejection, manifest checker, and Cairn validation/gates.