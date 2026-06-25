# Design: Centralize compatibility scenario contracts

## Context

The compatibility rails currently coordinate through strings: scenario names, env vars, fixture flags, milestone needles, and evidence identifiers. Some strings already live in `scenario_core`, but clients and servers still define parallel constants. A single checked contract should prevent accidental drift without adding runtime Nickel evaluation.

## Decisions

### 1. Use checked configuration as source of truth

Author the contract in the same ownership model as existing `compat/config/` surfaces. Nickel is appropriate for typed authoring, while runtime Rust should consume checked-in generated Rust/static data or checked manifests.

### 2. Generate names, not behavior

The contract should own identifiers, env var names, fixture ownership, scenario linkage, and milestone IDs. It should not generate game logic, client actions, or Valence systems.

### 3. Validate all consumers

Runner, Stevenarella, and Valence fixture code should either import generated constants or be checked against the generated manifest. The check must fail when any consumer uses an undeclared or stale scenario contract value.

### 4. Preserve compatibility claims

The contract unifies wiring. It does not broaden compatibility claims, add public-server safety claims, or imply full Minecraft semantic parity.

## Risks / Trade-offs

- Generated Rust can obscure simple constants; keep output small, documented, and source-linked.
- Some historical strings are receipt-stable; keep aliases or compatibility shims until a separate Cairn retires them.
- Cross-component checks must account for Stevenarella and Valence command boundaries.
