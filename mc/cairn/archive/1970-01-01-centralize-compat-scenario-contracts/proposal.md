# Proposal: Centralize compatibility scenario contracts

## Why

Scenario names, probe environment variables, fixture toggles, milestone labels, and contract constants are repeated across `compat/runner`, `clients/stevenarella`, and `servers/valence` examples. That duplication makes drift easy: the runner can set an env var that the client/server no longer consumes, or a fixture can emit a milestone the runner no longer requires.

## What Changes

- Create a typed source of truth for compatibility scenario contracts under the existing config/generated-surface ownership model.
- Generate or validate Rust-facing constants for runner, Stevenarella, and Valence fixture consumers.
- Add drift checks so contract names, env vars, fixture toggles, and milestone identifiers stay aligned.
- Keep runtime code Nickel-free; generation/checks may consume Nickel or checked generated artifacts.
- Preserve current scenario semantics and receipt claims.

## Impact

- **Files**: `compat/config/`, generated contract outputs, `compat/runner/src/*`, `clients/stevenarella/src/server/*`, `servers/valence/examples/*`, docs for generated-surface ownership.
- **Testing**: contract generation/freshness check, runner scenario tests, Stevenarella probe tests, Valence fixture tests, positive/negative drift fixtures, and Cairn gates.
