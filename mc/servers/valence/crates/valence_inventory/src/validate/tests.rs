use valence_server::nbt::Compound;
use valence_server::nbt::Value::Int;
use valence_server::protocol::packets::play::click_slot_c2s::SlotChange;
use valence_server::protocol::VarInt;
use valence_server::{ItemKind, ItemStack};

use super::*;
use crate::InventoryKind;

#[test]
fn net_item_delta_1() {
    let drag_packet = ClickSlotC2s {
        window_id: 2,
        state_id: VarInt(14),
        slot_idx: -999,
        button: 2,
        mode: ClickMode::Drag,
        slot_changes: vec![
            SlotChange {
                idx: 4,
                stack: ItemStack::new(ItemKind::Diamond, 21, None),
            },
            SlotChange {
                idx: 3,
                stack: ItemStack::new(ItemKind::Diamond, 21, None),
            },
            SlotChange {
                idx: 5,
                stack: ItemStack::new(ItemKind::Diamond, 21, None),
            },
        ]
        .into(),
        carried_item: ItemStack::new(ItemKind::Diamond, 1, None),
    };

    let player_inventory = Inventory::new(InventoryKind::Player);
    let inventory = Inventory::new(InventoryKind::Generic9x1);
    let window = InventoryWindow::new(&player_inventory, Some(&inventory));
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Diamond, 64, None));

    assert_eq!(
        calculate_net_item_delta(&drag_packet, &window, &cursor_item),
        0
    );
}

#[test]
fn net_item_delta_2() {
    let drag_packet = ClickSlotC2s {
        window_id: 2,
        state_id: VarInt(14),
        slot_idx: -999,
        button: 2,
        mode: ClickMode::Click,
        slot_changes: vec![
            SlotChange {
                idx: 2,
                stack: ItemStack::new(ItemKind::Diamond, 2, None),
            },
            SlotChange {
                idx: 3,
                stack: ItemStack::new(ItemKind::IronIngot, 2, None),
            },
            SlotChange {
                idx: 4,
                stack: ItemStack::new(ItemKind::GoldIngot, 2, None),
            },
            SlotChange {
                idx: 5,
                stack: ItemStack::new(ItemKind::Emerald, 2, None),
            },
        ]
        .into(),
        carried_item: ItemStack::new(ItemKind::OakWood, 2, None),
    };

    let player_inventory = Inventory::new(InventoryKind::Player);
    let inventory = Inventory::new(InventoryKind::Generic9x1);
    let window = InventoryWindow::new(&player_inventory, Some(&inventory));
    let cursor_item = CursorItem::default();

    assert_eq!(
        calculate_net_item_delta(&drag_packet, &window, &cursor_item),
        10
    );
}

#[test]
fn click_filled_slot_with_empty_cursor_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let mut inventory = Inventory::new(InventoryKind::Generic9x1);
    inventory.set_slot(0, ItemStack::new(ItemKind::Diamond, 20, None));
    let cursor_item = CursorItem::default();
    let packet = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::EMPTY,
        }]
        .into(),
        carried_item: inventory.slot(0).clone(),
    };

    check_packet(&packet, &player_inventory, Some(&inventory), &cursor_item)
        .expect("packet should be valid");
}

#[test]
fn click_filled_slot_with_incorrect_nbt_and_empty_cursor_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let cursor_item = CursorItem(ItemStack::EMPTY);

    let mut inventory = Inventory::new(InventoryKind::Generic9x1);
    // Insert an item with no NBT data that should have NBT Data.
    inventory.set_slot(0, ItemStack::new(ItemKind::DiamondPickaxe, 1, None));

    // Proper NBT Compound
    let mut compound = Compound::new();
    compound.insert("Damage", Int(1));

    let packet = ClickSlotC2s {
        window_id: 1,
        state_id: VarInt(0),
        slot_idx: 0,
        button: 0,
        mode: ClickMode::Click,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::EMPTY,
        }]
        .into(),
        carried_item: ItemStack {
            item: ItemKind::DiamondPickaxe,
            count: 1,
            nbt: Some(compound),
        },
    };

    check_packet(&packet, &player_inventory, Some(&inventory), &cursor_item)
        .expect("packet should be valid");
}

#[test]
fn click_slot_with_filled_cursor_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let inventory1 = Inventory::new(InventoryKind::Generic9x1);
    let mut inventory2 = Inventory::new(InventoryKind::Generic9x1);
    inventory2.set_slot(0, ItemStack::new(ItemKind::Diamond, 10, None));
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Diamond, 20, None));
    let packet1 = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::new(ItemKind::Diamond, 20, None),
        }]
        .into(),
        carried_item: ItemStack::EMPTY,
    };
    let packet2 = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::new(ItemKind::Diamond, 30, None),
        }]
        .into(),
        carried_item: ItemStack::EMPTY,
    };

    check_packet(&packet1, &player_inventory, Some(&inventory1), &cursor_item)
        .expect("packet should be valid");

    check_packet(&packet2, &player_inventory, Some(&inventory2), &cursor_item)
        .expect("packet should be valid");
}

#[test]
fn click_filled_slot_with_filled_cursor_stack_overflow_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let mut inventory = Inventory::new(InventoryKind::Generic9x1);
    inventory.set_slot(0, ItemStack::new(ItemKind::Diamond, 20, None));
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Diamond, 64, None));
    let packet = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::new(ItemKind::Diamond, 64, None),
        }]
        .into(),
        carried_item: ItemStack::new(ItemKind::Diamond, 20, None),
    };

    check_packet(&packet, &player_inventory, Some(&inventory), &cursor_item)
        .expect("packet should be valid");
}

#[test]
fn click_filled_slot_with_filled_cursor_different_item_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let mut inventory = Inventory::new(InventoryKind::Generic9x1);
    inventory.set_slot(0, ItemStack::new(ItemKind::IronIngot, 2, None));
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Diamond, 2, None));
    let packet = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::new(ItemKind::Diamond, 2, None),
        }]
        .into(),
        carried_item: ItemStack::new(ItemKind::IronIngot, 2, None),
    };

    check_packet(&packet, &player_inventory, Some(&inventory), &cursor_item)
        .expect("packet should be valid");
}

#[test]
fn click_slot_with_filled_cursor_failure() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let inventory1 = Inventory::new(InventoryKind::Generic9x1);
    let mut inventory2 = Inventory::new(InventoryKind::Generic9x1);
    inventory2.set_slot(0, ItemStack::new(ItemKind::Diamond, 10, None));
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Diamond, 20, None));
    let packet1 = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::new(ItemKind::Diamond, 22, None),
        }]
        .into(),
        carried_item: ItemStack::EMPTY,
    };
    let packet2 = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![SlotChange {
            idx: 0,
            stack: ItemStack::new(ItemKind::Diamond, 32, None),
        }]
        .into(),
        carried_item: ItemStack::EMPTY,
    };
    let packet3 = ClickSlotC2s {
        window_id: 1,
        button: 0,
        mode: ClickMode::Click,
        state_id: VarInt(0),
        slot_idx: 0,
        slot_changes: vec![
            SlotChange {
                idx: 0,
                stack: ItemStack::new(ItemKind::Diamond, 22, None),
            },
            SlotChange {
                idx: 1,
                stack: ItemStack::new(ItemKind::Diamond, 22, None),
            },
        ]
        .into(),
        carried_item: ItemStack::EMPTY,
    };

    check_packet(&packet1, &player_inventory, Some(&inventory1), &cursor_item)
        .expect_err("packet 1 should fail item duplication check");

    check_packet(&packet2, &player_inventory, Some(&inventory2), &cursor_item)
        .expect_err("packet 2 should fail item duplication check");

    check_packet(&packet3, &player_inventory, Some(&inventory1), &cursor_item)
        .expect_err("packet 3 should fail item duplication check");
}

#[test]
fn disallow_item_transmutation() {
    // no alchemy allowed - make sure that lead can't be turned into gold

    let mut player_inventory = Inventory::new(InventoryKind::Player);
    player_inventory.set_slot(9, ItemStack::new(ItemKind::Lead, 2, None));
    let cursor_item = CursorItem::default();

    let packets = vec![
        ClickSlotC2s {
            window_id: 0,
            button: 0,
            mode: ClickMode::ShiftClick,
            state_id: VarInt(0),
            slot_idx: 9,
            slot_changes: vec![
                SlotChange {
                    idx: 9,
                    stack: ItemStack::EMPTY,
                },
                SlotChange {
                    idx: 36,
                    stack: ItemStack::new(ItemKind::GoldIngot, 2, None),
                },
            ]
            .into(),
            carried_item: ItemStack::EMPTY,
        },
        ClickSlotC2s {
            window_id: 0,
            button: 0,
            mode: ClickMode::Hotbar,
            state_id: VarInt(0),
            slot_idx: 9,
            slot_changes: vec![
                SlotChange {
                    idx: 9,
                    stack: ItemStack::EMPTY,
                },
                SlotChange {
                    idx: 36,
                    stack: ItemStack::new(ItemKind::GoldIngot, 2, None),
                },
            ]
            .into(),
            carried_item: ItemStack::EMPTY,
        },
        ClickSlotC2s {
            window_id: 0,
            button: 0,
            mode: ClickMode::Click,
            state_id: VarInt(0),
            slot_idx: 9,
            slot_changes: vec![SlotChange {
                idx: 9,
                stack: ItemStack::EMPTY,
            }]
            .into(),
            carried_item: ItemStack::new(ItemKind::GoldIngot, 2, None),
        },
        ClickSlotC2s {
            window_id: 0,
            button: 0,
            mode: ClickMode::DropKey,
            state_id: VarInt(0),
            slot_idx: 9,
            slot_changes: vec![SlotChange {
                idx: 9,
                stack: ItemStack::new(ItemKind::GoldIngot, 1, None),
            }]
            .into(),
            carried_item: ItemStack::EMPTY,
        },
    ];

    for (i, packet) in packets.iter().enumerate() {
        check_packet(packet, &player_inventory, None, &cursor_item).expect_err(&format!(
            "packet {i} passed item duplication check when it should have failed"
        ));
    }
}

#[test]
fn allow_shift_click_overflow_to_new_stack() {
    let mut player_inventory = Inventory::new(InventoryKind::Player);
    player_inventory.set_slot(9, ItemStack::new(ItemKind::Diamond, 64, None));
    player_inventory.set_slot(36, ItemStack::new(ItemKind::Diamond, 32, None));
    let cursor_item = CursorItem::default();

    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: 9,
        button: 0,
        mode: ClickMode::ShiftClick,
        slot_changes: vec![
            SlotChange {
                idx: 37,
                stack: ItemStack::new(ItemKind::Diamond, 32, None),
            },
            SlotChange {
                idx: 36,
                stack: ItemStack::new(ItemKind::Diamond, 64, None),
            },
            SlotChange {
                idx: 9,
                stack: ItemStack::EMPTY,
            },
        ]
        .into(),
        carried_item: ItemStack::EMPTY,
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}

#[test]
fn allow_pickup_overfull_stack_click() {
    let mut player_inventory = Inventory::new(InventoryKind::Player);
    player_inventory.set_slot(9, ItemStack::new(ItemKind::Apple, 100, None));
    let cursor_item = CursorItem::default();

    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: 9,
        button: 0,
        mode: ClickMode::Click,
        slot_changes: vec![SlotChange {
            idx: 9,
            stack: ItemStack::EMPTY,
        }]
        .into(),
        carried_item: ItemStack::new(ItemKind::Apple, 100, None),
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}

#[test]
fn allow_place_overfull_stack_click() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Apple, 100, None));

    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: 9,
        button: 0,
        mode: ClickMode::Click,
        slot_changes: vec![SlotChange {
            idx: 9,
            stack: ItemStack::new(ItemKind::Apple, 64, None),
        }]
        .into(),
        carried_item: ItemStack::new(ItemKind::Apple, 36, None),
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}
#[test]
fn allow_clicking_outside_inventory_when_not_holding_anything_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Air, 0, None));

    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: -999, // -999 means outside inventory
        button: 0,
        mode: ClickMode::DropKey, // when not holding an item and clicking outside the user
        // interface the client sends this kind of packet
        slot_changes: vec![].into(),
        carried_item: ItemStack::new(ItemKind::Air, 0, None),
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}
#[test]
fn allow_clicking_outside_inventory_when_holding_something_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Air, 0, None));

    // This is in the notchian server a stack drop
    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: -999, // -999 means outside inventory
        button: 0,
        mode: ClickMode::Click, // when holding an item its a click
        slot_changes: vec![].into(),
        carried_item: ItemStack::new(ItemKind::Air, 0, None),
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}
#[test]
fn allow_clicking_on_the_margin_area_in_inventory_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Air, 0, None));

    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: -1, // -1 here means on the margin areas
        button: 0,
        mode: ClickMode::Click,
        slot_changes: vec![].into(),
        carried_item: ItemStack::new(ItemKind::Air, 0, None),
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}
#[test]
fn allow_clicking_on_an_empty_slot_with_empty_carried_item_success() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Air, 0, None));

    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: 3,
        button: 0,
        mode: ClickMode::Click,
        slot_changes: vec![].into(),
        carried_item: ItemStack::new(ItemKind::Air, 0, None),
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}
#[test]
fn allow_clicking_hotbar_keybinds_when_both_source_and_target_are_empty() {
    let player_inventory = Inventory::new(InventoryKind::Player);
    let cursor_item = CursorItem(ItemStack::new(ItemKind::Air, 0, None));

    let packet = ClickSlotC2s {
        window_id: 0,
        state_id: VarInt(2),
        slot_idx: 0,
        button: 0,
        mode: ClickMode::Hotbar,
        slot_changes: vec![].into(),
        carried_item: ItemStack::new(ItemKind::Air, 0, None),
    };

    check_packet(&packet, &player_inventory, None, &cursor_item).expect("packet should be valid");
}
