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

# GUI lifecycle cleanup

`GuiViewer` is client-owned GUI state. `GuiPlugin` removes it and emits `GuiCloseEvent` when the client's `OpenInventory` no longer points at the GUI inventory or when Valence marks the client `Despawned` before final entity removal. This keeps GUI cleanup component-owned where possible while preserving Valence's explicit despawn window.

`OpenInventory` close packets and stale backing-inventory references remain explicit inventory cleanup because they need the current `ClientInventoryState`, packet emission before flush, and cross-entity liveness checks.

# Schedule contracts

`InventoryPlugin` exposes named Bevy `SystemSet`s so downstream plugins can order around inventory without depending on anonymous tuple grouping:

- `InventoryInitSet` runs in `PreUpdate` after `SpawnClientsSet` and attaches inventory components to newly spawned clients.
- `InventoryInputSet` runs in `EventLoopPreUpdate` for inventory-related packet and interaction input.
- `InventoryMutationSet` covers inventory model mutations from setup, packet handling, player actions, creative actions, hotbar input, and readonly resynchronization.
- `InventoryWindowSyncSet` runs in `PostUpdate` for open-window, close-window, and player-inventory synchronization.
- `InventoryPresentationSet` runs in `PostUpdate` before `FlushPacketsSet` for inventory packet preparation.
- `InventoryCleanupSet` covers inventory close handling and stale open-inventory cleanup.

Packet decoding and model mutation remain co-located in existing systems where splitting them would change timing or duplicate packet semantics. Internal ordering inside click validation, readonly resynchronization, slot-delta filtering, and packet serialization remains private unless a future contract promotes a narrower set. These sets do not change default plugin membership, inventory protocol semantics, vanilla parity, broad Minecraft compatibility, public-server safety, or production-readiness claims.

### See also

Examples related to inventories in the `valence/examples/` directory:
- `building`
- `chest`
