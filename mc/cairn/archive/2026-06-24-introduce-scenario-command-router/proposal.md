# Proposal: Introduce a typed scenario command router

## Why

The flake exposes many scenario-specific aliases. Those aliases are useful, but their implementations repeat command shapes and make new scenario wiring expensive. A single typed scenario command surface can become the internal route for aliases while preserving the existing public app names.

## What Changes

- Add a typed command shape such as `mc-compat scenario run <scenario>` with explicit backend, dry-run/live, receipt, timeout, and evidence options.
- Route generated flake aliases through the typed scenario command while preserving existing app names and command behavior.
- Validate scenario names, backend choices, receipt paths, and live/dry-run constraints before process launch.
- Add positive and negative CLI/router tests for known scenarios, invalid scenarios, invalid backends, unsafe receipt paths, and overclaiming options.

## Impact

- **Files**: `compat/runner`, flake app wrappers or generated wrapper metadata, README/docs command listings, scenario manifest/generator if touched, and Cairn artifacts.
- **Testing**: CLI parser/router tests, alias parity dry-runs, invalid argument tests, maintained dry-run aggregate, generated freshness checks if wrappers are generated, and Cairn validation/gates.
- **Non-claims**: this unifies command routing only; it does not change scenario semantics, evidence pass criteria, or compatibility claims.
