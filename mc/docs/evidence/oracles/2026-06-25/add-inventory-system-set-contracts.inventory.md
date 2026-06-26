# Inventory SystemSet contract inventory

## Question

Can `InventoryPlugin` expose named schedule sets without changing inventory behavior, default plugin membership, or claim scope for `r[valence_bevy_ecs.inventory_sets.inventory]`, `r[valence_bevy_ecs.inventory_sets.contract]`, `r[valence_bevy_ecs.inventory_sets.wiring]`, `r[valence_bevy_ecs.inventory_sets.compatibility]`, and `r[valence_bevy_ecs.inventory_sets.tests]`?

## Inspected evidence

- `servers/valence/crates/valence_inventory/src/lib.rs` before and after the change.
- `servers/valence/crates/valence_inventory/src/systems/{actions,hotbar,mode,place,viewer}.rs` and `servers/valence/crates/valence_inventory/src/click.rs` for system access patterns.
- `servers/valence/src/lib.rs` for `DefaultPlugins` membership and the `inventory` feature gate.
- `servers/valence/src/tests/core_plugin_sets.rs` for schedule smoke tests.
- `servers/valence/crates/valence_inventory/README.md` for public schedule contract text and non-claims.
- `docs/evidence/run-logs/2026-06-25/add-inventory-system-set-contracts-baseline.run.log` for the pre-change focused inventory baseline.
- `docs/evidence/run-logs/2026-06-25/add-inventory-system-set-contracts-focused.run.log` for focused schedule, behavior, and crate checks after wiring.
- `docs/evidence/run-logs/2026-06-25/add-inventory-system-set-contracts-format-schedule-final.run.log` for formatting, schedule hygiene, and focused checks after formatting cleanup.

## Current inventory systems

| System | Schedule | Set contract after change | Ordering constraints | Resource/event access | Mutation target |
| --- | --- | --- | --- | --- | --- |
| `init_new_client_inventories` | `PreUpdate` | `InventoryInitSet`, `InventoryMutationSet` | After `SpawnClientsSet` through `InventoryInitSet` | Reads `Added<Client>` query; no inventory events | Inserts `Inventory`, `CursorItem`, `ClientInventoryState`, and `HeldItem` on new client entities |
| `hotbar::handle_update_selected_slot` | `EventLoopPreUpdate` | `InventoryInputSet`, `InventoryMutationSet` | Existing unordered tuple membership preserved | Reads `PacketEvent<UpdateSelectedSlotC2s>`; writes `UpdateSelectedSlotEvent` | Mutates `HeldItem` |
| `click::handle_packets` | `EventLoopPreUpdate` | `InventoryInputSet`, `InventoryMutationSet` | Existing unordered tuple membership preserved | Reads `PacketEvent<ClickSlotC2s>`; writes `ClickSlotEvent` and `DropItemStackEvent` | Mutates client/open `Inventory`, `ClientInventoryState`, `OpenInventory`, and `CursorItem`; may write resync packets |
| `mode::handle_creative_inventory_action` | `EventLoopPreUpdate` | `InventoryInputSet`, `InventoryMutationSet` | Existing unordered tuple membership preserved | Reads `PacketEvent<CreativeInventoryActionC2s>`; writes `CreativeInventoryActionEvent` and `DropItemStackEvent` | Mutates client `Inventory` and `ClientInventoryState`; may write slot update packets |
| `handle_close_handled_screen` | `EventLoopPreUpdate` | `InventoryInputSet`, `InventoryCleanupSet` | Existing unordered tuple membership preserved | Reads `PacketEvent<CloseHandledScreenC2s>` | Removes `OpenInventory` from the client entity |
| `control::handle_player_actions` | `EventLoopPreUpdate` | `InventoryInputSet`, `InventoryMutationSet` | Existing unordered tuple membership preserved | Reads `PacketEvent<PlayerActionC2s>`; writes `DropItemStackEvent` | Mutates player `Inventory` and `ClientInventoryState`; may write readonly resync packets |
| `place::resync_readonly_inventory_after_block_interaction` | `EventLoopPreUpdate` | `InventoryInputSet`, `InventoryMutationSet` | Existing unordered tuple membership preserved | Reads `InteractBlockEvent` | Marks readonly player inventory slots changed for later sync |
| `update_client_on_close_inventory` | `PostUpdate` | `InventoryCleanupSet`, `InventoryWindowSyncSet`, `InventoryPresentationSet` | Before `viewer::update_open_inventories`; before `FlushPacketsSet` through `InventoryPresentationSet` | Reads removed `OpenInventory`; no inventory events | Writes close-screen packets to affected clients |
| `hotbar::update_player_selected_slot` | `PostUpdate` | `InventoryPresentationSet` | Before `FlushPacketsSet` through `InventoryPresentationSet` | Reads changed `HeldItem`; no inventory events | Writes selected-slot packets |
| `viewer::update_open_inventories` | `PostUpdate` | `InventoryWindowSyncSet`, `InventoryPresentationSet` | After close notification through the existing local edge; before `FlushPacketsSet` through `InventoryPresentationSet` | No inventory events | Opens, closes, and synchronizes viewed inventories; clears relevant change bits |
| `update_player_inventories` | `PostUpdate` | `InventoryWindowSyncSet`, `InventoryPresentationSet` | Before `FlushPacketsSet` through `InventoryPresentationSet` | No inventory events | Synchronizes player inventory packet state and clears change bits |
| `update_cursor_item` | `PostUpdate` | `InventoryPresentationSet` | Before `FlushPacketsSet` through `InventoryPresentationSet` | Reads changed `CursorItem`; no inventory events | Writes cursor-item slot update and clears client cursor tracking |

## Feature/default plugin membership

`servers/valence/src/lib.rs` still adds `valence_inventory::InventoryPlugin` to `DefaultPlugins` only behind `#[cfg(feature = "inventory")]`. This change adds no dependency, feature, default plugin membership, event name, resource type, packet type, or behavior toggle.

## Downstream ordering dependencies

- Existing inventory tests rely on packet behavior, open/close packet order, readonly resynchronization, creative actions, hotbar selection, drop events, and window ID behavior; the focused behavior baseline and post-change run both pass.
- Existing equipment synchronization reads `valence_inventory` player inventory state through `EquipmentInventorySync`; this change does not move equipment systems or change their sets.
- Downstream plugins can now order relative to phase sets for client input, model mutation, window synchronization, presentation, initialization, and cleanup instead of anonymous inventory tuples.

## Compatibility and non-claims

Packet decoding and model mutation remain co-located in current systems to preserve timing and avoid creating duplicate typed packet semantics. Internal click validation, slot-delta filtering, readonly resynchronization, and packet serialization order stay private. This change does not claim vanilla inventory parity, broad Minecraft compatibility, semantic equivalence, public-server safety, production readiness, or full CTF/survival correctness.

## Decision

Adopt named `Inventory*Set` contracts in Valence-owned inventory code. The wiring classifies existing systems into phase-level sets, keeps the previous explicit `SpawnClientsSet`, close-before-open-sync, and pre-flush ordering constraints, and preserves default plugin membership.

## Owner

`servers/valence/crates/valence_inventory` and `servers/valence/src/tests/core_plugin_sets.rs`.

## Next action

Use focused inventory behavior tests, inventory schedule smoke tests, Valence schedule hygiene, Cairn gates, Cairn validation, task-evidence validation, and the BLAKE3 manifest before archiving. No live compatibility rail is required because the change is schedule-contract wiring and does not change fixture input handling or promote broader compatibility claims.
