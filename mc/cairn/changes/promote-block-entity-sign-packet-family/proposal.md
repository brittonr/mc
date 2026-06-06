# Proposal: Promote block-entity sign packet-family evidence

## Why

The maintained survival sign block-entity persistence row already proves one bounded Paper/reference and Valence fixture: `compatbot` observes the configured `Sign` block entity at `28,64,0` with text `MC|Compat|Sign|Persist` before and after a controlled restart. However, the protocol packet inventory still lists the adjacent sign/block-entity packet rows as gaps, especially `BlockEntityUpdateS2CPacket`, `SignEditorOpenS2CPacket`, and `UpdateSignC2SPacket`.

The next high-value promotion is to turn the existing sign block-entity evidence into a narrow packet-family row without implying all block entities, arbitrary NBT parity, sign editing UI parity, or broad packet parser-shape coverage.

## What Changes

- Add a bounded `block-entity-sign-packet-family` promotion contract for the configured sign payload and exact protocol rows considered.
- Promote `BlockEntityUpdateS2CPacket` only if normalized evidence shows the configured sign block-entity payload is parsed/applied and paired with the existing Paper/reference and Valence survival block-entity receipts.
- Require `SignEditorOpenS2CPacket` and `UpdateSignC2SPacket` to remain explicit non-claims unless this change adds separate live sign-edit evidence and checker coverage for those rows.
- Fix the current-bundle inventory prose that still names drag transactions as a non-claim despite the now-promoted `inventory-drag-transactions` row.
- Update packet inventory, acceptance matrix/current bundle, row checker coverage, BLAKE3 manifests, and Cairn evidence only after deterministic positive/negative gates pass.

## Impact

- **Files**: packet inventory TSV, acceptance matrix/current bundle docs, sign/block-entity packet-family evidence docs/KV/receipts/manifests, pure checker source, Cairn specs/tasks, and possibly runner/client fixture metadata if fresh evidence is required.
- **Testing**: checker positive/negative fixtures, existing or refreshed survival block-entity paired checks, packet-inventory/current-bundle gates, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
