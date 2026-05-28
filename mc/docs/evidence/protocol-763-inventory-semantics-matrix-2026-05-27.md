# Protocol-763 inventory semantics matrix proof — 2026-05-27

## Scope

This checkpoint drains the inventory semantics matrix Cairn by defining the finite inventory rows currently promoted by the maintained `inventory-interaction` receipts and by keeping every unobserved inventory behavior as an explicit non-claim.

It does not claim full inventory semantics, all window types, state-id rejection behavior from a live invalid packet, stack splitting/merging, drag transactions, creative-mode inventory, or full item lifecycle correctness. Full inventory semantics remains a non-claim.

## Slot mapping table

| Slot name | Protocol/window scope | Valence evidence | Stevenarella evidence | Claim status |
| --- | --- | --- | --- | --- |
| `hotbar_weapon_slot_36` | player inventory hotbar slot `36` | `server_inventory_hotbar_select`, `server_inventory_drop`, `server_inventory_pickup` | `inventory_sword_slot`, `inventory_drop_sent`, `inventory_pickup_seen` | Bounded row coverage only. |
| `hotbar_wool_slot_37` | player inventory hotbar slot `37` | `server_inventory_click`, `server_block_place` | `inventory_wool_slot`, `inventory_click_sent`, `inventory_block_place_sent` | Bounded row coverage only. |
| `container_slot_0` | `generic_3x3_container` slot `0` | `server_inventory_open_container`, `server_inventory_container_click` | `inventory_open_container_seen`, `inventory_container_click_sent` | Bounded row coverage only. |

## Promoted matrix rows

| Seam | window_kind | slot_class | click_mode | carried_stack | state_id | expected_outcome | Receipt | BLAKE3 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Inventory/drop | player_inventory | hotbar_weapon_slot_36 | drop_item_action | none | fresh_observed | server_inventory_drop | `docs/evidence/protocol-763-inventory-drop.matrix.receipt.json` | `4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46` |
| Pickup semantics | player_inventory | hotbar_weapon_slot_36 | pickup_entity_collect | none | server_authoritative | server_inventory_pickup | `docs/evidence/protocol-763-pickup.matrix.receipt.json` | `bcac4aab63857cf0d3b6dd148455324e7f0368dd3e57cfd26841ae7fc1b5ffe8` |
| Player-inventory click/container click | player_inventory | hotbar_wool_slot_37 | left_click | red_wool_63 | fresh_observed | server_inventory_click | `docs/evidence/protocol-763-click.matrix.receipt.json` | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` |
| Open-container semantics | generic_3x3_container | container_slot_0 | left_click | red_wool_63_to_empty | fresh_observed | server_inventory_container_click | `docs/evidence/protocol-763-open-container.matrix.receipt.json` | `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3` |
| Block placement / use-item-on-block | player_inventory | hotbar_wool_slot_37 | use_item_on_block | red_wool_63 | fresh_observed | server_block_place | `docs/evidence/protocol-763-block-place.matrix.receipt.json` | `9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122` |

## Positive validation

`tools/check_inventory_semantics_matrix.py` requires each promoted row to have:

- `mode=run`, `dry_run=false`, `status=pass`;
- server protocol `763`;
- scenario `inventory-interaction`;
- no missing client/server milestones;
- no forbidden matches;
- row-specific client milestones and server milestones;
- scoped non-claims for full CTF correctness, broad Minecraft compatibility, unbounded soak, and production load;
- matching acceptance-matrix and current-bundle digest entries.

## Negative fixtures

`tools/check_inventory_semantics_matrix.py --self-test` rejects these malformed fixtures:

- missing client milestone for a promoted row;
- missing Valence server milestone for a promoted row;
- protocol mismatch (`758` instead of `763`);
- stale_state_accepted;
- invalid_slot_accepted;
- malformed_click_accepted;
- state_corruption;
- missing matrix/non-claim model text.

## Promotion gate

Only the five receipt-backed rows above are promoted. Invalid/stale/malformed behavior is covered by deterministic checker fixtures, not by a live invalid-packet receipt; therefore invalid transition runtime semantics remain a non-claim until a future live rail injects those packets and records server restoration/rejection evidence.

A future row must provide a protocol-763 live receipt, run log, BLAKE3 manifest, acceptance-matrix row, current-bundle row, and positive/negative checker coverage before promotion.

## Decision

- Question: Can the maintained inventory interaction receipts be promoted as a reviewable inventory matrix without claiming full inventory semantics?
- Inspected evidence: five reviewable matrix receipt copies, `docs/evidence/stevenarella-valence-763-inventory-interaction-2026-05-25.md`, acceptance matrix rows, current evidence bundle rows, and checker positive/negative fixtures.
- Decision: Yes. Promote the five bounded rows listed here; full inventory semantics remains a non-claim.
- Owner: agent.
- Next action: add new matrix rows only when state-id freshness/rejection, invalid slots, malformed clicks, carried-stack edge cases, or extra window types have their own protocol-763 live receipts and manifests.

## Non-claims

No full inventory semantics, all slot mappings, all click modes, all container/window types, stale state-id rejection, invalid slot rejection, malformed click rejection, stack splitting/merging, drag transactions, creative inventory, slot restoration, full item lifecycle, production load, full CTF correctness, or broad protocol coverage claim is made.
