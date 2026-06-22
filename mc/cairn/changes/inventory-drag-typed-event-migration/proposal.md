# Proposal: Migrate inventory drag transactions to typed-event pass/fail

## Why

`inventory-drag-transactions` is the next maintained inventory row still using waiver-backed substring fallback. The row already has bounded client milestones, Valence quick-craft server correlation, and a dry-run wrapper, so it is ready to join the typed-event-ready inventory rows without broadening inventory semantics.

Moving it to typed-event pass/fail makes missing structured client/server drag evidence fail closed before legacy substring evidence can satisfy the row. That reduces receipt ambiguity while preserving the existing bounded drag transaction claim.

## What Changes

- Mark `inventory-drag-transactions` as `typed-event-ready` in the scenario manifest and generated surfaces.
- Extend the typed-event pass/fail gate to include `Scenario::InventoryDragTransactions`.
- Add positive and negative runner fixtures proving required drag client/server events, forbidden surfaces, and ordered drag phases are enforced.
- Update documentation that names the typed-event-ready scenario set.
- Preserve the existing row contract, wrapper name, receipt schema, dry-run shape, and non-claims.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, generated scenario surfaces, `compat/runner/src/main.rs`, README/evidence docs, and checker fixtures.
- **Testing**: focused runner tests, scenario-manifest self-tests/checks, generated-surface freshness, dry-run wrapper checks, Cairn gates, and Cairn validation.
- **Non-claims**: this change only changes the validation basis for the existing bounded `inventory-drag-transactions` row. It does not claim broad inventory drag semantics, all quick-craft variants, full inventory compatibility, public-server safety, production readiness, or semantic equivalence.
