# mc-compatibility Change Spec: Block-entity sign packet-family promotion

## Requirements

### Requirement: Block-entity sign packet-family preflight

r[mc_compatibility.block_entity_sign_packet_family.preflight] Current bundle, acceptance matrix, and packet inventory prose MUST be internally consistent before block-entity sign packet-family coverage is promoted.

#### Scenario: Existing inventory prose matches promoted drag rows

r[mc_compatibility.block_entity_sign_packet_family.preflight.drag_consistency]
- GIVEN `inventory-drag-transactions` is already a promoted bounded row
- WHEN the current evidence bundle describes maintained inventory rows and explicit non-claims
- THEN the prose names the drag row as covered only within its configured bounds
- AND drag transactions are not listed as a broad non-claim except for unpromoted drag modes, distributions, windows, and semantics.

### Requirement: Block-entity sign packet-family contract

r[mc_compatibility.block_entity_sign_packet_family.contract] The `block-entity-sign-packet-family` row MUST define a bounded packet-family promotion contract before matrix, bundle, or packet-inventory coverage is claimed.

#### Scenario: Contract names exact packet rows and payload

r[mc_compatibility.block_entity_sign_packet_family.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names the configured actor `compatbot`, sign block-entity kind, position `28,64,0`, text payload `MC|Compat|Sign|Persist`, Paper/reference receipt, Valence receipt, child revisions, normalized metrics, and every protocol row considered
- AND `play/clientbound/0x08 BlockEntityUpdateS2CPacket` is promoted only for the configured sign NBT payload if checker evidence passes
- AND `play/clientbound/0x31 SignEditorOpenS2CPacket` plus `play/serverbound/0x2e UpdateSignC2SPacket` remain non-claims unless separate sign-edit live evidence and checker coverage are produced.

#### Scenario: Adjacent breadth remains non-claim

r[mc_compatibility.block_entity_sign_packet_family.contract.nonclaims]
- GIVEN the bounded packet-family row is promoted
- WHEN matrix, bundle, and packet inventory docs are reviewed
- THEN all block entities, arbitrary NBT parity, sign editing UI semantics without dedicated evidence, all sign text variants, all sign sides, all block-entity packet shapes, broad parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity sign packet-family checker

r[mc_compatibility.block_entity_sign_packet_family.checker] A deterministic Rust checker MUST validate normalized block-entity sign packet-family evidence before promotion.

#### Scenario: Valid block-entity sign packet evidence passes

r[mc_compatibility.block_entity_sign_packet_family.checker.valid]
- GIVEN normalized evidence names `block-entity-sign-packet-family`, the configured sign payload, the `BlockEntityUpdateS2CPacket` protocol row, clean child revisions, matching Paper/reference and Valence receipts, and sign-payload client observation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak block-entity sign packet evidence fails closed

r[mc_compatibility.block_entity_sign_packet_family.checker.rejects]
- GIVEN evidence is missing the row id, omits Paper/reference or Valence evidence, uses stale or unknown child revisions, names the wrong packet row, omits sign-payload observation, reports the wrong block entity kind, position, text payload, or backend, claims sign editing without sign-edit evidence, or claims broad block-entity/parser coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, overbroad, or mismatched metric.

### Requirement: Block-entity sign packet-family rail

r[mc_compatibility.block_entity_sign_packet_family.rail] The promotion MUST reuse or extend isolated sign/block-entity rails without changing existing survival, inventory, CTF, combat, network, or negative-live semantics.

#### Scenario: Existing sign persistence row remains separate

r[mc_compatibility.block_entity_sign_packet_family.rail.isolated]
- GIVEN the existing `survival-block-entity-persistence-parity` row already covers one sign payload across Paper/reference and Valence backends
- WHEN packet-family evidence is collected or normalized
- THEN the packet-family row records its own packet-row contract and checker output
- AND the survival block-entity persistence row remains a separate survival/reference-parity claim.

#### Scenario: Sign-edit rows require dedicated evidence

r[mc_compatibility.block_entity_sign_packet_family.rail.sign_edit]
- GIVEN `SignEditorOpenS2CPacket` or `UpdateSignC2SPacket` is considered for promotion
- WHEN the rail lacks a live sign-edit open/update interaction with client and server correlation
- THEN those rows stay explicit non-claims instead of inheriting coverage from sign block-entity persistence.

### Requirement: Reviewable block-entity sign packet-family artifacts

r[mc_compatibility.block_entity_sign_packet_family.artifacts] Review-critical block-entity sign packet-family artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.block_entity_sign_packet_family.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized KV inputs, checker output, BLAKE3 manifests, packet inventory updates, child revisions, and any oracle limitation checkpoint are present under `docs/evidence/` or tracked source paths.

### Requirement: Narrow block-entity sign packet-family matrix promotion

r[mc_compatibility.block_entity_sign_packet_family.matrix] Acceptance matrix, current-bundle docs, and packet inventory rows MUST promote only the configured block-entity sign packet-family scope after checker and evidence gates pass.

#### Scenario: Packet inventory stays exact

r[mc_compatibility.block_entity_sign_packet_family.matrix.inventory]
- GIVEN checker-backed block-entity sign packet evidence passes
- WHEN `protocol-763-packet-inventory-2026-05-28.tsv`, acceptance matrix, and current evidence bundle are updated
- THEN only the exact supported packet row or rows are marked covered
- AND unsupported sign editor, arbitrary block-entity, arbitrary NBT, broad parser-shape, full protocol, public-server, and production claims remain non-claims.

### Requirement: Block-entity sign packet-family validation evidence

r[mc_compatibility.block_entity_sign_packet_family.validation] The change MUST record checker, packet-inventory, matrix/bundle, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.block_entity_sign_packet_family.validation.log]
- GIVEN the block-entity sign packet-family row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, any runner/fixture checks, packet inventory or row contract checks, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
