# Add GUI helper plugin scope

## Question

What Hyperion GUI concepts can safely inform a Valence-native optional GUI helper, and which behavior remains owned by `valence_inventory`?

## Inspected evidence

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `hyperion/crates/hyperion-gui/src/lib.rs` (`Gui`, `add_command`, `open`, `open_deferred`) | reference | `add-gui-helper-plugin` | The useful concept is inventory-backed screens with slot-specific click outcomes, but callback storage and spawned display entities are Hyperion-owned. | `servers/valence/crates/valence_inventory/src/gui.rs` | No Hyperion code copied. Valence helper uses stable Rust, pure transition functions, and Bevy ECS shells over Valence-owned inventory components/events. | GUI model and shell tests in `valence_inventory::gui::tests`; validation logs under `docs/evidence/add-gui-helper-plugin/`. | No Hyperion compatibility, default behavior change, production-scale safety, or vanilla container parity claim. |
| `hyperion/events/bedwars/src/command/gui.rs` (`testgui`) | reference | `add-gui-helper-plugin` | Shows a common readonly menu with one action slot; informs the README example only. | `servers/valence/crates/valence_inventory/README.md` | Bedwars command code and callback behavior are not ported. | Scope note plus README docs and GUI tests. | No Bedwars behavior, command parity, or direct callback semantics claim. |
| `hyperion/events/bedwars/src/command/chest.rs` (`chest`) | reference | `add-gui-helper-plugin` | Shows chest-like menu setup and readonly slot intent; informs readonly slot tests. | `servers/valence/crates/valence_inventory/src/gui.rs` | Uses Valence-owned `Inventory::readonly` and pure no-mutation plans rather than Hyperion `ItemSlot` internals. | Readonly positive and invalid/stale negative GUI tests. | No full chest/container parity or all slot interaction claim. |
| `hyperion/crates/hyperion-inventory/src/lib.rs` (`Inventory`, `OpenInventory`, window IDs, readonly) | reject | `add-gui-helper-plugin` | Valence already owns richer inventory/window behavior; importing Hyperion inventory would duplicate packet semantics. | none | Valence `Inventory`, `OpenInventory`, `ClientInventoryState`, and `ClickSlotEvent` remain the owner surfaces. | Valence inventory integration tests and selected inventory dry-runs. | No replacement of `valence_inventory` and no semantic equivalence with Hyperion inventory. |
| `servers/valence/crates/valence_inventory/src/lib.rs`, `click.rs`, `click/flow.rs`, `systems/viewer.rs` | adopt | `add-gui-helper-plugin` | Existing Valence APIs already own open windows, readonly inventory handling, click validation, close handling, and client synchronization. | `servers/valence/crates/valence_inventory/src/gui.rs` | Adopted as Valence-owned dependency surfaces only; helper stays optional and does not alter `InventoryPlugin` default systems. | `pre_valence_inventory_tests.run.log` records the pre-existing test baseline failure; final logs record focused GUI and inventory checks. | No broad Minecraft compatibility, public-server safety, production readiness, or full CTF/survival correctness claim. |

## Decision

Implement a Valence-owned optional `valence_inventory::gui` helper. The pure core plans GUI open, click, close, and disconnect outcomes from explicit model inputs. The thin shell exposes `GuiPlugin`, `GuiOpenEvent`, `GuiClickEvent`, `GuiRejectedClickEvent`, and `GuiCloseEvent` over existing `valence_inventory` components/events. `Inventory`, packet validation, window synchronization, and readonly packet behavior remain owned by `valence_inventory`.

## Owner

`add-gui-helper-plugin`

## Next action

Use the scope above to verify `r[valence_hyperion_integration.gui_helper.scope]` before marking the task complete. Do not promote broader vanilla container, Hyperion compatibility, or production-readiness claims without separate accepted aggregate gates.
