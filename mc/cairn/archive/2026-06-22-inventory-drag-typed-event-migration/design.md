# Design: inventory drag typed-event migration

## Scope

The migration is limited to the existing `inventory-drag-transactions` row. It changes the row's oracle readiness from waiver-backed substring fallback to typed-event-ready pass/fail. Existing receipt fields, wrapper names, required milestones, forbidden patterns, and non-claim language remain stable unless a generated surface needs the migration-state wording refreshed.

## Functional core

The pure runner core already models typed-event validation through:

- `typed_event_oracle_contributes_to_pass_fail`
- `typed_event_required_events_for_graph`
- `typed_event_ordered_edges_for_scenario`
- `evaluate_typed_event_graph`
- manifest readiness checks in `tools/check_scenario_manifest.rs`

This change extends that core for `Scenario::InventoryDragTransactions` and verifies it with in-memory fixtures. The core inputs are scenario identity plus structured client/server milestone evidence; outputs are pass/fail diagnostics listing missing events, forbidden events, and ordering violations.

## Imperative shell

The shell remains unchanged except for regenerated surfaces and wrapper/dry-run validation. Live runner behavior still writes the same receipt plus typed-event sidecar when receipt evidence is available. No new external services, clocks, network behavior, or filesystem layout are introduced.

## Validation strategy

- Run baseline runner and manifest checks before edits.
- Add positive fixture coverage for a complete drag transaction typed-event graph.
- Add negative fixture coverage for missing drag events and drag phase order violations.
- Regenerate scenario surfaces and require the generated-surface checker to pass.
- Run the `inventory-drag-transactions` dry-run wrapper check so receipt shape stays stable.
- Run Cairn gates and task-evidence validation before archive.

## Non-claims

The migrated row remains a bounded evidence row for one RedWool stack drag from slot 37 into slots 38 and 39 with final 32/32 distribution and Valence quick-craft correlation. It does not cover arbitrary drag shapes, all inventory containers, all item types, client UI correctness, broad inventory semantics, public-server safety, production readiness, or semantic equivalence.
