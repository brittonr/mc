## Context

Scenario definitions have been split into a pure scenario core, but live packet promotion capabilities are still implicit in evidence docs and operator reasoning. Making capabilities explicit helps avoid accidental overclaims and gives future changes a stable place to discover whether a row has a deterministic live path.

## Goals / Non-Goals

Goals:
- Define a pure registry of scenario live-probe capabilities.
- Validate capability rows for uniqueness, packet row correctness, backend support, evidence mode, required signals, and non-claims.
- Keep runner orchestration and file/log/receipt I/O outside the registry core.

Non-goals:
- Promoting any packet row by itself.
- Replacing scenario identity or receipt schemas.

## Design

1. Add a `ScenarioLiveCapability` data model with scenario id, packet row ids, capability kind, backend/client path, evidence mode, required live signals, required non-claims, and optional blocker reason.
2. Store static capability definitions near scenario core or in a sibling pure module.
3. Add pure validation for duplicate scenario/row pairs, unknown scenarios, unknown packet rows, unsupported evidence modes, empty required signals, and missing non-claims.
4. Extend scenario manifest or a focused checker to include capability-registry validation.
5. Document how future packet live rails should add a capability before claiming live promotion.

## Risks

- If registry data drifts from checker row specs, selection could be misleading; validation should reuse or cross-check known packet rows where practical.
- Registry should not become a second source of evidence truth; live promotion still requires receipts and checker validation.

## Validation

Run scenario capability tests, scenario manifest checks, relevant runner dry-runs, targeted packet checks if row surfaces are referenced, evidence-manifest/task-evidence checks, Cairn gates/sync/archive, and post-archive validation.
