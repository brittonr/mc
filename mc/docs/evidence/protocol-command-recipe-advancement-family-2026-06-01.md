# Protocol-763 command/recipe/advancement packet-family row — 2026-06-01

## Scope

This checkpoint promotes only a narrow `command/recipe/advancement packet family` row for two already-reviewed protocol-763 raw packet mappings plus one live recipe feature receipt:

- `play/clientbound/0x10 CommandTreeS2CPacket -> DeclareCommandsRaw`
- `play/clientbound/0x6d SynchronizeRecipesS2CPacket -> SynchronizeRecipesRaw`
- live feature evidence from the paired Paper and Valence `survival-crafting-table` receipts.

No advancement packet row is promoted here. The family label stays narrow: command-tree and recipe raw byte preservation plus one live crafting recipe feature only.

## Evidence basis

| Evidence | Path | BLAKE3 / status |
| --- | --- | --- |
| Row receipt | `docs/evidence/protocol-command-recipe-advancement-family-2026-06-01.receipt.json` | in this row manifest |
| Row contract kv | `docs/evidence/protocol-command-recipe-advancement-family-2026-06-01.kv` | in this row manifest |
| Parser ledger | `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-28.md` | `fa6e946063ea721d447cefbd6867ee934c99b253ee27cfec742e460728c2e3cb` |
| Packet inventory | `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` | `20de60f18fe7484e5ad393e07ce7ad8814e6f0150cda12a0440d8be9d55ecbf6` |
| Parser fixture snapshot | `docs/evidence/protocol-763-broad-parser-fixtures-stevenarella-2026-05-28.md` | `d4e47d929a9f0a8bba15abe5c72160c1cc64a2dcb1ee517957ada7399d542266` |
| Parser oracle | `docs/evidence/protocol-763-broad-parser-fixture-oracle-2026-05-28.md` | `fc92fc5a65cee89b7de66616822671a2bd1a9fac51a624e7f4977c19be7fcc51` |
| Parser gate log | `docs/evidence/protocol-763-broad-coverage-ledger-gate-2026-05-28.run.log` | `aa712d34bdd843831918f9aa38fad0253b0a82862b873ae64e7773f3f1513475` |
| Survival crafting evidence doc | `docs/evidence/protocol-763-survival-crafting-table-2026-05-31.md` | `264ea8807fb36032ce08b088c6a55759e1d1fa8b895beb7002a4314e71ea2268` |
| Paper crafting receipt | `docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.receipt.json` | `710f64a04451a62604d17a78cc84f3e2db84ec3d7034b7feaa149b1e8af57a15` |
| Valence crafting receipt | `docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.receipt.json` | `59a44542ccae0bb2af696227b79c4bbc3e7dc696bc026a44cd14c04e6d0e0c61` |

## Normalized metrics

| Metric | Value |
| --- | --- |
| `packet_family` | `command_tree_raw`, `recipe_book_raw` |
| `wire_id` | `play/clientbound/0x10`, `play/clientbound/0x6d` |
| `semantic_fixture_id` | `protocol_763_high_risk_raw_parser_fixtures_accept_payloads` |
| `parser_fixture_result` | raw payload bytes preserved for `DeclareCommandsRaw` and `SynchronizeRecipesRaw` |
| `malformed_fixture_status` | semantic malformed rejection remains a non-claim for byte-opaque raw consumers |
| `live_scenario_feature` | `survival-crafting-table` with `Stick x4` result on Paper and Valence |
| `receipt_path` | `docs/evidence/protocol-command-recipe-advancement-family-2026-06-01.receipt.json` |
| `digest` | BLAKE3 of the row receipt, recorded in the row manifest |

## Decision

The row can be promoted as a bounded protocol-family slice because the packet inventory already marks `CommandTreeS2CPacket` and `SynchronizeRecipesS2CPacket` as `broad_covered`, the parser snapshot/oracle prove raw byte preservation for the two raw consumers, and the survival crafting receipts provide a live recipe feature path with reviewable Paper and Valence artifacts.

This does not prove command execution semantics, recipe-book semantics, semantic command/recipe decoding, malformed semantic rejection for raw consumers, any advancement packet row, full protocol-763 compatibility, full Minecraft compatibility, or production readiness.

## Non-claims

No all-command, all-recipe, all-advancement, recipe-book, command-execution, advancement-update, malformed semantic rejection for byte-opaque raw consumers, full protocol-763 compatibility, full Minecraft compatibility, full survival compatibility, broad vanilla parity, or production readiness claim is made.
