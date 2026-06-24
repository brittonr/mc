# Proposal: Separate compatibility instrumentation from core client/server code

## Why

Compatibility probes, typed-event capture, MCP control, and scenario-specific fixtures are necessary for evidence rails, but they can blur the boundary between product code and harness instrumentation. Clear module or feature boundaries would reduce accidental production-path coupling and make future instrumentation easier to audit.

## What Changes

- Inventory compat-specific instrumentation in Stevenarella and Valence fixture paths.
- Move or gate client probes, capture hooks, MCP control, and scenario-specific actions behind explicit modules/features or harness-only entrypoints.
- Keep core protocol/client/server semantics unchanged unless a separate compatibility change explicitly modifies them.
- Add tests/guards proving instrumentation is opt-in and evidence-critical events remain available when enabled.

## Impact

- **Files**: `clients/stevenarella/src/*`, Valence examples/fixtures, compat runner scenario setup, Cargo features/configs, docs/AGENTS notes, evidence checker expectations if event names move, and Cairn artifacts.
- **Testing**: client unit/integration tests, feature-gating tests, scenario dry-runs/live checks as needed, typed-event fixtures, and Cairn validation/gates.
- **Non-claims**: this improves code boundaries only; it does not claim broader protocol support, production readiness, or new scenario parity.
