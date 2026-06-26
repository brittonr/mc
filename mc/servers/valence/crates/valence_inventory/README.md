# `valence_inventory`

The inventory system.

This module contains the systems and components needed to handle
inventories. By default, clients will have a player inventory attached to
them.

# Components

- [`Inventory`]: The inventory component. This is the thing that holds
  items.
- [`OpenInventory`]: The component that is attached to clients when they
  have an inventory open.

# Examples

An example system that will let you access all player's inventories:

```
# use bevy_ecs::prelude::*;
# use valence_inventory::*;
# use valence_server::client::Client;
fn system(clients: Query<(&Client, &Inventory)>) {}
```

# GUI helper

The optional `gui` module provides a thin helper for common inventory-backed menu screens. It builds on the existing `Inventory`, `OpenInventory`, readonly inventory handling, and `ClickSlotEvent` surfaces instead of replacing packet semantics.

```rust
# use valence_inventory::{InventoryKind, gui::{GuiMenu, GuiMenuModel, GuiSlot}};
const SETTINGS_SLOT: u16 = 13;
let mut model = GuiMenuModel::new(InventoryKind::Generic9x3.slot_count()).unwrap();
model.set_slot(SETTINGS_SLOT, GuiSlot::action("open_settings").unwrap()).unwrap();
let (_inventory, _menu) = GuiMenu::readonly_inventory(
    InventoryKind::Generic9x3,
    "Settings",
    model,
);
```

Add `GuiPlugin` only when the helper should route GUI open/click/close events. Keeping the plugin disabled leaves Valence's lower-level inventory APIs unchanged. Readonly GUI slots emit explicit action events and plan no inventory mutation; use Valence inventory checks for packet validation and synchronization.

This helper is for ergonomic menus and does not claim full vanilla container parity, all container behavior, production readiness, or Hyperion compatibility.

# GUI relationship model

`GuiViewer` is an explicit relationship component on the client entity, not a Bevy hierarchy edge. A GUI inventory can be viewed by more than one client, and close handling needs `ClientInventoryState`, packet emission, and cross-entity liveness checks, so tree ownership would hide protocol state instead of clarifying it.

# GUI lifecycle cleanup

`GuiViewer` is client-owned GUI state. `GuiPlugin` removes it and emits `GuiCloseEvent` when the client's `OpenInventory` no longer points at the GUI inventory, when a client opens a different GUI inventory, or when Valence marks the client `Despawned` before final entity removal. This keeps GUI cleanup component-owned where possible while preserving Valence's explicit despawn window.

`OpenInventory` close packets and stale backing-inventory references remain explicit inventory cleanup because they need the current `ClientInventoryState`, packet emission before flush, and cross-entity liveness checks.

# Schedule contracts

`InventoryPlugin` exposes named Bevy `SystemSet`s so downstream plugins can order around inventory without depending on anonymous tuple grouping:

- `InventoryInitSet` runs in `PreUpdate` after `SpawnClientsSet` and attaches inventory components to newly spawned clients.
- `InventoryInputSet` runs in `EventLoopPreUpdate` for inventory-related packet and interaction input.
- `InventoryMutationSet` covers inventory model mutations from setup, packet handling, player actions, creative actions, hotbar input, and readonly resynchronization.
- `InventoryWindowSyncSet` runs in `PostUpdate` for open-window, close-window, and player-inventory synchronization.
- `InventoryPresentationSet` runs in `PostUpdate` before `FlushPacketsSet` for inventory packet preparation.
- `InventoryCleanupSet` covers inventory close handling and stale open-inventory cleanup.

Selected inventory packet semantics decode once in `EventLoopSet::TypedAdapters` and emit typed packet events before domain consumers mutate inventory state. Hotbar selection, click-slot input, creative inventory actions, and handled-screen close packets now carry the source client, packet arrival timestamp, and decoded fields through `UpdateSelectedSlotPacketEvent`, `ClickSlotPacketEvent`, `CreativeInventoryActionPacketEvent`, and `CloseHandledScreenEvent`. Domain systems still own slot-range validation, game-mode checks, readonly resynchronization, drop events, open-inventory cleanup, and downstream public events such as `ClickSlotEvent` and `UpdateSelectedSlotEvent`.

| Selected semantic | Previous raw reader | Packet type | Event-loop phase | Typed adapter event | Domain consumer and mutation target | Malformed or stale behavior |
| --- | --- | --- | --- | --- | --- | --- |
| Hotbar selection | `hotbar::handle_update_selected_slot` | `UpdateSelectedSlotC2s` | `EventLoopPreUpdate` typed adapter before domain consumer | `UpdateSelectedSlotPacketEvent` | `hotbar::handle_update_selected_slot` updates `HeldItem` and emits `UpdateSelectedSlotEvent` | Wrong IDs, decode failures, partial decodes, and missing `ClientInventoryState` emit no typed event; out-of-range slots emit no mutation. |
| Slot clicks | `click::handle_packets` | `ClickSlotC2s` | `EventLoopPreUpdate` typed adapter before domain consumer | `ClickSlotPacketEvent` | `click::handle_packets` validates, resynchronizes, mutates inventory/cursor state, and emits `ClickSlotEvent` or `DropItemStackEvent` | Wrong IDs, decode failures, partial decodes, malformed click modes, and missing `ClientInventoryState` emit no typed event; invalid clicks still use existing resync behavior. |
| Creative inventory actions | `mode::handle_creative_inventory_action` | `CreativeInventoryActionC2s` | `EventLoopPreUpdate` typed adapter before domain consumer | `CreativeInventoryActionPacketEvent` | `mode::handle_creative_inventory_action` checks creative mode, mutates inventory, writes slot updates, and emits creative/drop events | Wrong IDs, decode failures, partial decodes, and missing `ClientInventoryState` emit no typed event; non-creative clients or invalid slots emit no mutation. |
| Handled-screen close | `handle_close_handled_screen` and the survival fixture close hook | `CloseHandledScreenC2s` | `EventLoopPreUpdate` typed adapter before domain consumer | `CloseHandledScreenEvent` | Inventory removes `OpenInventory`; selected fixtures remove their own open-container components | Wrong IDs, decode failures, partial decodes, and missing `ClientInventoryState` emit no typed event; stale open containers are ignored. |

The remaining inventory `PlayerActionC2s` drop/swap reader is intentionally still listed as a direct raw packet consumer because preserving standalone inventory behavior without requiring `ActionPlugin` remains a separate compatibility decision. Raw `PacketEvent` access remains public for low-level users and unsupported packet semantics. These sets and events do not change default plugin membership, vanilla parity, broad Minecraft compatibility, public-server safety, or production-readiness claims.

### See also

Examples related to inventories in the `valence/examples/` directory:
- `building`
- `chest`
