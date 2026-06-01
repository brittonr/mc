# Protocol-763 inventory transaction packet-family row — 2026-06-01

## Scope

This checkpoint promotes only a narrow `inventory transaction packet family` row for one scenario-bounded inventory transaction packet path plus existing live inventory receipts:

- `play/serverbound/0x0b ClickSlotC2SPacket -> ClickWindow_StateBeforeSlot`
- live `inventory-interaction` matrix rows for player-inventory and open-container clicks.
- bounded negative containment receipts for stale state id and invalid slot/window click.

The packet inventory still marks `ClickSlotC2SPacket` as `scenario_bounded` with parser shape missing. This row therefore does not claim broad parser-shape coverage, full inventory semantics, all window types, all click modes, stack split/merge, drag transactions, or creative inventory.

## Evidence basis

| Evidence | Path | BLAKE3 / status |
| --- | --- | --- |
| Row receipt | `docs/evidence/protocol-inventory-transaction-family-2026-06-01.receipt.json` | `df02262c86a9b1966e613a8485464ea234aa31c06acd06230f3de3123e349d61` |
| Row contract kv | `docs/evidence/protocol-inventory-transaction-family-2026-06-01.kv` | in this row manifest |
| Inventory matrix doc | `docs/evidence/protocol-763-inventory-semantics-matrix-2026-05-27.md` | `25fde948a009275aa36e7bdf7c33828d2c71cd26148c49fcabbdcb81836997b5` |
| Player-inventory click receipt | `docs/evidence/protocol-763-click.matrix.receipt.json` | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` |
| Open-container click receipt | `docs/evidence/protocol-763-open-container.matrix.receipt.json` | `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3` |
| Negative live rails doc | `docs/evidence/protocol-763-negative-live-rails-2026-05-29.md` | `b677dd67b023c3f2eb0c0bf20b9f8d85208d446b6cc76ee290a95e2a6bd00ffb` |
| Stale state containment receipt | `docs/evidence/negative-live-rails-inventory-stale-state-2026-05-29.json` | `8d11184e0350be47b7cf742b23d120e13e60b3c38bfae8791dfbd1e87176f1d8` |
| Invalid click containment receipt | `docs/evidence/negative-live-rails-inventory-invalid-click-2026-05-29.json` | `4f0d1bf8de944db0604b9fff42a322b67783eee9619619c1e88cace77124c9c3` |
| Packet inventory | `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` | `20de60f18fe7484e5ad393e07ce7ad8814e6f0150cda12a0440d8be9d55ecbf6` |

## Normalized metrics

| Metric | Value |
| --- | --- |
| `transaction_packet_name` | `ClickSlotC2SPacket` |
| `state_side` | `play/serverbound` |
| `wire_id` | `0x0b` |
| `slot_window_state_id_fields` | `window_id`, `slot`, `button`, `mode`, `state_id`, `carried_stack` |
| `parser_fixture_id` | generic row-contract fixture `protocol-inventory-transaction-family-coverage.kv` |
| `malformed_fixture_status` | bounded negative live stale-state and invalid-click containment; broad malformed-click semantics remain non-claims |
| `live_scenario` | `inventory-interaction` |
| `receipt_digest` | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` |

## Decision

The row can be promoted as a bounded inventory transaction packet-family slice because the inventory semantics matrix proves live click/container-click rows with protocol-763 receipts, and negative live rails prove containment for one stale state id and one invalid slot/window click. The generic row-contract fixture proves required evidence fields fail closed.

This is not broad parser-shape evidence. `ClickSlotC2SPacket` remains scenario-bounded in the packet inventory until a real parser-shape fixture exists.

## Non-claims

No full inventory semantics, all window types, all click modes, all slot mappings, carried-stack breadth, stack split/merge, drag transaction, creative inventory, broad parser-shape coverage, malformed-click breadth, slot restoration, full item lifecycle, full protocol-763 compatibility, full Minecraft compatibility, full CTF correctness, or production readiness claim is made.
