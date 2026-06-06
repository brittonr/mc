# Block-entity sign packet-family promotion — 2026-06-06

This checkpoint promotes one bounded packet-family row by reusing the paired Paper/reference and Valence evidence from `survival-block-entity-persistence-parity` while keeping the survival persistence claim separate.

Evidence selected:

- Paper/reference receipt: `docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json`.
- Valence receipt: `docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json`.
- Normalized row evidence: `docs/evidence/survival-block-entity-persistence-paper-2026-06-04.evidence` and `docs/evidence/survival-block-entity-persistence-valence-2026-06-04.evidence`.
- Comparator output: `docs/evidence/survival-block-entity-persistence-row-parity-2026-06-04.run.log` with `exit_status=0`.
- Packet-family normalized receipt and KV input: `docs/evidence/block-entity-sign-packet-family-2026-06-06.receipt.json` and `docs/evidence/block-entity-sign-packet-family-2026-06-06.kv`.
- Deterministic checker: `tools/check_block_entity_sign_packet_family.rs`.

Covered packet row:

| State | Side | Wire ID | Packet | Scenario evidence | Coverage status | Parser-shape status |
| --- | --- | --- | --- | --- | --- | --- |
| `play` | `clientbound` | `0x08` | `BlockEntityUpdateS2CPacket` | `block_entity_sign_packet_family` | `scenario_bounded` | `shape_review_missing` |

Configured payload:

| Metric | Value |
| --- | --- |
| `actor` | `compatbot` |
| `kind` | `Sign` |
| `position` | `28,64,0` |
| `text_payload` | `MC\|Compat\|Sign\|Persist` |
| `paper_sign_payload_observed` | `ok` |
| `valence_sign_payload_observed` | `ok` |
| `paper_valence_row_parity` | `ok` |

Child revisions:

| Component | Revision | Status |
| --- | --- | --- |
| Stevenarella | `79d766c2439120ba2ec0217dd88ee9f708db9844` | `clean` |
| Valence | `f54e6d01b23a08a2977c7bf2d93301d589c65c4d` | `clean` |
| Paper fixture source | `864fedc1f1f645058b9ca061829d7c42e3da8d088e8cc394ab0c8abf6b2f5150` | BLAKE3 fixture source digest |

Scope:

- This promotes only `play/clientbound/0x08 BlockEntityUpdateS2CPacket` for the configured sign NBT payload observed by actor `compatbot` at `28,64,0` with text `MC|Compat|Sign|Persist`.
- The source survival row remains a separate reference-parity/persistence claim; this row only binds that evidence to the scenario-bounded packet-family inventory entry.
- `play/clientbound/0x31 SignEditorOpenS2CPacket` and `play/serverbound/0x2e UpdateSignC2SPacket` remain non-claims because no dedicated sign-edit open/update interaction evidence is promoted here.
- Non-claims remain: all block entities, arbitrary NBT parity, all sign text variants, all sign sides, all block-entity packet shapes, broad parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness.
