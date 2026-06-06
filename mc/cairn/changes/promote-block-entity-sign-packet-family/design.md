# Design: Promote block-entity sign packet-family evidence

## Context

The current protocol-763 evidence set has a strong sign/block-entity survival row: paired Paper/reference and Valence receipts prove one configured sign block entity persists across controlled restart. Separately, the packet inventory still lists sign/block-entity packet rows as gaps. The promotion should bridge that evidence to a narrow packet-family claim without widening the survival parity row or claiming sign-edit UI behavior.

A small consistency cleanup is also needed: the current-bundle inventory checkpoint was written before the drag row was promoted and still describes drag transactions as a non-claim in prose even though the evidence table now includes `Inventory drag transactions`.

## Decisions

### 1. Promote the clientbound sign block-entity update first

**Choice:** Treat `play/clientbound/0x08 BlockEntityUpdateS2CPacket` for the configured sign NBT payload as the primary promotable packet row.

**Rationale:** Existing Stevenarella code parses sign block-entity updates and logs the configured sign payload. Existing Valence/Paper survival block-entity receipts already produce the configured `Sign` at `28,64,0` with text `MC|Compat|Sign|Persist`. This gives the highest evidence reuse and lowest implementation risk.

### 2. Keep sign-editor packets gated by their own evidence

**Choice:** `SignEditorOpenS2CPacket` and `UpdateSignC2SPacket` are considered in the contract but remain non-claims unless this change adds a separate bounded sign-edit open/update rail.

**Rationale:** Block-entity persistence does not prove sign-editor UI opening or client-authored sign text updates. The packet inventory should make that distinction explicit rather than inheriting coverage from a related sign payload observation.

### 3. Use a pure checker over normalized evidence

**Choice:** Implement or extend a deterministic Rust checker that evaluates normalized KV/receipt fields in memory. The shell only reads files, parses CLI arguments, and returns exit status.

**Rationale:** Packet-family scope is a documentation/evidence decision. It should be testable with positive and negative fixtures that do not start Paper, Valence, Stevenarella, Docker, or Nix.

### 4. Reuse paired survival receipts when still valid

**Choice:** Prefer using the existing `survival-block-entity-persistence-parity` receipt/log bundle if the child revisions and normalized metrics are sufficient. Rerun the paired scenario only if the checker contract needs fields not already present.

**Rationale:** Reusing durable evidence reduces churn and avoids unnecessary fixture changes. If reuse is insufficient, the change must produce fresh reviewable receipts under `docs/evidence/`.

### 5. Update packet inventory separately from gameplay matrices

**Choice:** Update `protocol-763-packet-inventory-2026-05-28.tsv` for packet-row coverage, and update acceptance/current-bundle docs only for the narrow packet-family claim.

**Rationale:** Packet-family coverage and survival gameplay/reference parity are distinct claims. Keeping both rows separate avoids implying broad block-entity semantics or full survival compatibility.

## Risks / Trade-offs

- Existing receipts may not machine-record a field that reviewers need for packet-family promotion. If so, produce a fresh receipt or a bounded oracle checkpoint instead of inferring from logs.
- Sign-edit packet rows are tempting to promote because they are adjacent in the packet inventory, but doing so without live sign-edit evidence would overclaim.
- Refreshing BLAKE3 manifests can cascade through nested manifests when shared docs or accepted specs change.
- Current bundle prose cleanup is small but important: stale non-claim text can confuse reviewers and should be fixed before adding another packet-family row.

## Out-of-scope high-value follow-up brainstorm

1. **Sign editor open/update rail** — promote `SignEditorOpenS2CPacket` and `UpdateSignC2SPacket` with one bounded sign-edit interaction and server correlation.
2. **Creative inventory action row** — bounded creative-mode `CreativeInventoryActionC2SPacket` slot set/drop evidence, keeping all creative semantics as non-claims.
3. **Chunk biome data packet row** — targeted `ChunkBiomeDataS2CPacket` parser/receipt fixture to reduce the remaining chunk/biome packet-family gap.
4. **Block-entity update breadth row** — one additional non-sign block entity, such as chest or skull, to avoid treating sign as representative of all block entities.
5. **Entity status-effect packet row** — bounded apply/remove status effect evidence to cover `EntityStatusEffectS2CPacket` and `RemoveEntityStatusEffectS2CPacket` without claiming all effects.
6. **Recipe-book client settings row** — bounded `RecipeBookDataC2SPacket`/recipe-book state evidence, separate from crafting-table recipe execution.
7. **Chat/command execution containment row** — bounded signed/unsigned chat or command execution path with explicit no-public-server and no-security-breadth claims.
8. **Movement packet family row** — deterministic position/look/on-ground packet correlation in Valence with anti-cheat/security breadth left as non-claims.
9. **Player list/team/scoreboard packet family row** — bounded scoreboard/team packet evidence from CTF rows, with UI parity and all scoreboard semantics out of scope.
10. **Resource-pack status row** — bounded server resource-pack offer/status response fixture, with no asset-loading, trust, or public-server safety claim.
