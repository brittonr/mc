# Protocol-763 equipment permutation packet-family row — 2026-06-01

## Scope

This checkpoint promotes only a narrow `equipment permutation packet family` row for one scenario-bounded equipment-update packet path plus one live observer receipt:

- `play/clientbound/0x55 EntityEquipmentUpdateS2CPacket -> EntityEquipment_Array`
- live `equipment-update-observation` evidence for entity `4`, `slot4/main_hand_remote_entity`, item id `829`, count `1`.

The packet inventory still marks this Valence row as `scenario_bounded` with parser shape missing. This row therefore does not claim broad parser-shape coverage or all equipment packet permutations.

## Evidence basis

| Evidence | Path | BLAKE3 / status |
| --- | --- | --- |
| Row receipt | `docs/evidence/protocol-equipment-permutation-family-2026-06-01.receipt.json` | `bb6bf5c18357e9ebb26ff43d72a094923b0c7dbe7726e52f65186fadd36def02` |
| Row contract kv | `docs/evidence/protocol-equipment-permutation-family-2026-06-01.kv` | in this row manifest |
| Equipment expansion doc | `docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.md` | `e5d995e8e4ff6b8f2f79f38fd558632f02ff6fc4ca3bc7aa69a67aae4aa46abd` |
| Equipment matrix doc | `docs/evidence/protocol-763-equipment-slot-item-matrix-2026-05-27.md` | `5a1acbd36920e0ab617c8c5f8d76eb1090813638840148447150ac75d2513e17` |
| Live observer receipt | `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json` | `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d` |
| Live observer run log | `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.run.log` | `184223509605c1dc9b89db3d8c77ff0b8b7a1c103bec8031de75515d4777dc16` |
| Normalized row record | `docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.record` | `10df069435475a161078ef19ebc61110113a4317b47d5da12af7518e97e0c1c8` |
| Expansion checker log | `docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.run.log` | `17d3cee2a1b2d95ff784a249c4df5d6b04f55abac13764894572626124e9d931` |
| Packet inventory | `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` | `20de60f18fe7484e5ad393e07ce7ad8814e6f0150cda12a0440d8be9d55ecbf6` |

## Normalized metrics

| Metric | Value |
| --- | --- |
| `equipment_packet_name` | `EntityEquipmentUpdateS2CPacket` |
| `wire_id` | `play/clientbound/0x55` |
| `entity_id` | `4` |
| `slot` | `slot4/main_hand_remote_entity` |
| `item_id` | `829` |
| `count` | `1` |
| `parser_fixture_id` | generic row-contract fixture `protocol-equipment-permutation-family-coverage.kv` |
| `live_observer_receipt` | `docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json` |
| `digest` | BLAKE3 of the row receipt, recorded in the row manifest |

## Decision

The row can be promoted as a bounded equipment packet-family slice because the existing equipment slot/item matrix proves one live remote-player equipment update with client/server correlation, and the generic row-contract fixture proves the required evidence fields fail closed. This is not a broad parser fixture: `EntityEquipmentUpdateS2CPacket` remains scenario-bounded in the packet inventory until a real parser-shape fixture exists.

## Non-claims

No all-equipment-permutation, all-equipment-slot, all-item-type, all-count, empty/non-empty transition breadth, repeated update ordering, broad parser-shape coverage, armor mitigation, combat balancing, exact vanilla/reference parity, full protocol-763 compatibility, full Minecraft compatibility, full combat correctness, full CTF correctness, or production readiness claim is made.
