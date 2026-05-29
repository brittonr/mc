# Protocol 763 broad parser fixture oracle checkpoint

## Question

Do the external Stevenarella parser fixtures in commit `ba3ce751f04b4fecefe516e06dff3e40363d2e72` substantively support the four promoted broad-covered packet rows in the `mc` evidence bundle, given that three rows are byte-opaque raw consumers and one row is a structured channel parser?

## Inspected evidence

- `docs/evidence/protocol-763-broad-parser-fixtures-stevenarella-2026-05-28.md` snapshots the exact fixture source and assertions for review inside this repo.
- `docs/evidence/protocol-763-broad-coverage-ledger-gate-2026-05-28.run.log` records `cargo test --manifest-path stevenarella/protocol/Cargo.toml protocol_763_ -- --nocapture` passing all protocol-763 tests, including:
  - `protocol_763_high_risk_raw_parser_fixtures_accept_payloads`
  - `protocol_763_custom_payload_parser_fixture_accepts_brand_payload`
  - `protocol_763_custom_payload_parser_fixture_rejects_malformed_channel`
- `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` marks only these rows as `broad_covered`:
  - `play/clientbound/0x10 CommandTreeS2CPacket -> DeclareCommandsRaw`
  - `play/clientbound/0x43 ChunkDeltaUpdateS2CPacket -> ChunkDeltaUpdateRaw`
  - `play/clientbound/0x6d SynchronizeRecipesS2CPacket -> SynchronizeRecipesRaw`
  - `play/serverbound/0x0d CustomPayloadC2SPacket -> PluginMessageServerbound`
- `tools/check_protocol_coverage_ledger.py` validates that the fixture snapshot contains the named tests, packet keys, packet enum matches, byte-preservation assertions, and malformed custom-payload rejection markers before any row can be promoted.

## Decision

Decision: adequate for the narrow promoted packet-family claims.

The three clientbound rows are byte-opaque raw consumers. Their fixtures prove the protocol-763 wire IDs map to the expected Stevenarella raw packet variants and that parser output preserves all fixture bytes. This supports only raw packet acceptance, not command-tree, chunk-delta, or recipe semantic correctness. No malformed semantic rejection fixture is claimed for these raw consumers because every byte string is valid input to `Vec<u8>` by design; negative semantic fixtures require future semantic decoders.

The serverbound custom-payload row is structured enough to have malformed parser inputs. Its fixtures prove `minecraft:brand` channel decoding and payload preservation, plus rejection of invalid UTF-8 and oversized VarInt channel lengths without silent acceptance. This is the only promoted row with positive and negative parser-shape fixtures in the strict rejection sense.

Full protocol-763 compatibility, full Minecraft gameplay compatibility, and semantic correctness for commands/chunks/recipes/plugin messages remain non-claims.

## Owner

Owner: agent.

## Next action

Before promoting any additional structured packet row, add positive and negative parser-shape fixtures. Before promoting any additional byte-opaque raw row, add a positive byte-preservation fixture and an explicit semantic non-claim rationale. Then add a repo-local fixture source snapshot or live receipt/log evidence for that row, extend `tools/check_protocol_coverage_ledger.py` fixture validation, regenerate `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv`, and update the BLAKE3 manifests.
