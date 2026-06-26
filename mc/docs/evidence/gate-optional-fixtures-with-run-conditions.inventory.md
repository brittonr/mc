# Gate optional fixtures with Bevy run conditions inventory

## Question

Which Valence compatibility fixture systems are targeted for this run-condition change, and what disabled-mode contract applies before moving any guard into Bevy scheduling?

## Inspected evidence

- `servers/valence/examples/survival_compat.rs` installs survival fixture systems in `SurvivalCompatibilityPlugin` and refreshes runtime fixture config from environment toggles.
- `handle_survival_chest_store` reads `ClickSlotEvent` and is owned by the optional `MC_COMPAT_SURVIVAL_CHEST_FIXTURE` path through `SurvivalChestFixture`.
- `advance_survival_mob_drop_pickup` has no event reader and is owned by the optional `MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE` path through `SurvivalMobDropFixture`.
- Other optional survival systems remain explicitly guarded in-system unless a later accepted change inventories and tests their event-reader semantics.

## Decision

| Target | Runtime enabled check | Event reader | Resource access | Disabled contract | Re-enable expectation | Compatibility milestones |
| --- | --- | --- | --- | --- | --- | --- |
| `handle_survival_chest_store` | `SurvivalChestFixture` exists after `MC_COMPAT_SURVIVAL_CHEST_FIXTURE=1` startup setup | `ClickSlotEvent` | Optional `ResMut<SurvivalChestFixture>`, client `Username`, optional `SurvivalOpenContainer` | Drain explicit in-system guard; do not use `run_if` because skipped readers could replay stale clicks. | Disabled-period clicks are consumed for this reader; after the fixture resource is inserted, only future valid chest-store clicks set `store_logged`. | `survival_chest_store` |
| `advance_survival_mob_drop_pickup` | `SurvivalMobDropFixture` exists after `MC_COMPAT_SURVIVAL_MOB_DROP_FIXTURE=1` startup setup | None | `ResMut<SurvivalMobDropFixture>`, client inventory/query, commands | Skip with Bevy `run_if`; disabled body is a pure no-op and has no event cursor to drain. | Missing-resource updates do not run the body; inserting the fixture starts future pickup-delay ticks without replaying disabled work. | `survival_mob_drop_pickup`, `survival_mob_drop_state` |

## Owner

Valence survival compatibility fixture (`servers/valence/examples/survival_compat.rs`).

## Validation notes

Focused tests cover enabled chest-store output, disabled/stale chest-store events across runtime resource insertion, and mob-drop pickup run-condition behavior across resource absence, insertion, and removal. Schedule hygiene is required because the mob-drop pickup hook now has a Bevy run condition.

Selected mc-compat live rails are not promoted for this slice: the changed enabled milestones remain the same, and the new disabled/stale behavior is an internal event-reader contract covered by focused Bevy tests rather than a live compatibility claim.

## Non-claims

This evidence does not claim broad Minecraft compatibility, semantic equivalence, public-server safety, production readiness, full CTF correctness, full survival correctness, or that every optional fixture system is eligible for `run_if`.
