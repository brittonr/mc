# Protocol-763 entity metadata packet-family row — 2026-06-01

## Scope

This checkpoint promotes only a narrow `entity metadata packet family` row for one scenario-observed metadata boundary:

- `play/clientbound/0x52 EntityTrackerUpdateS2CPacket -> EntityMetadata`
- bounded Valence `ctf` probe evidence showing the prior `EntityMetadata` `FromUtf8Error` boundary did not recur.

The packet inventory still marks this Valence row as `scenario_bounded` with parser shape missing. This row therefore does not claim broad parser-shape coverage, all entity metadata value types, mob AI semantics, malformed metadata rejection breadth, full protocol-763 compatibility, full Minecraft compatibility, or production readiness.

## Evidence basis

| Evidence | Path | BLAKE3 / status |
| --- | --- | --- |
| Row receipt | `docs/evidence/protocol-entity-metadata-family-2026-06-01.receipt.json` | `7e7fc51be2b85c1ceb37890494cf1dda76b28e91220993bc18d47fd2043fdebc` |
| Row contract kv | `docs/evidence/protocol-entity-metadata-family-2026-06-01.kv` | in this row manifest |
| Metadata boundary evidence doc | `docs/evidence/stevenarella-valence-763-entity-metadata-2026-05-23.md` | `50918b81d53bb0fa587dc7cbc01e4f48ebabf876a225821806ffa7c973b45e74` |
| Metadata boundary receipt | `docs/evidence/stevenarella-valence-763-entity-metadata-2026-05-23.receipt.json` | `72545f60abeb0a0d46fdb9859486c7924631498689fef60b56271a638ff2db59` |
| Observed-boundaries evidence doc | `docs/evidence/stevenarella-valence-763-observed-boundaries-drain-2026-05-23.md` | `72600dafe9db6ea904dbc947fbb5c7dc63562bebfe63875bc64fb35866559143` |
| Observed-boundaries receipt | `docs/evidence/stevenarella-valence-763-observed-boundaries-drain-2026-05-23.receipt.json` | `cc62a78238134a339205abaea4c003f82e89597cc89ce9e11c620274974886a2` |
| Packet inventory | `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` | `20de60f18fe7484e5ad393e07ce7ad8814e6f0150cda12a0440d8be9d55ecbf6` |

## Normalized metrics

| Metric | Value |
| --- | --- |
| `wire_id` | `play/clientbound/0x52` |
| `valence_packet_name` | `EntityTrackerUpdateS2CPacket` |
| `stevenarella_semantic` | `EntityMetadata` |
| `parser_fixture_id` | generic row-contract fixture `protocol-entity-metadata-family-coverage.kv` |
| `positive_payload_fixture` | bounded metadata boundary probe with no `FromUtf8Error`, no `UnexpectedEof`, no parser panic |
| `malformed_rejection_fixture_where_semantic_decoding_exists` | not promoted; broad malformed metadata semantics remain non-claim |
| `live_receipt_evidence_path` | `docs/evidence/protocol-entity-metadata-family-2026-06-01.receipt.json` |

## Decision

The row can be promoted as a bounded entity metadata packet-family slice because the old bounded Valence probe specifically targeted the `EntityMetadata` parser boundary, observed protocol `763`, reached join/chunk/render milestones, and recorded zero recurrence of the prior `FromUtf8Error`, `UnexpectedEof`, panic, or failed packet parse. The generic row-contract fixture proves the required evidence fields fail closed.

This is not broad parser-shape evidence. `EntityTrackerUpdateS2CPacket` remains scenario-bounded in the packet inventory until real positive and malformed parser-shape fixtures exist.

## Non-claims

No all-entity-metadata, all metadata value type, mob AI semantic, broad parser-shape, malformed metadata rejection breadth, full protocol-763 compatibility, full Minecraft compatibility, full gameplay semantic correctness, or production readiness claim is made.
