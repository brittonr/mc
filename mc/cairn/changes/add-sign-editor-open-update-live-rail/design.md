## Context

Existing block-entity/sign evidence proves a bounded sign payload/persistence path, but not the client sign editor UI open/update packet pair. The targeted packet live-parity drain therefore kept `sign-editor-open-update` fixture-bounded.

## Goals / Non-Goals

Goals:
- Exercise one configured sign editor open/update interaction with client and backend correlation.
- Keep sign editor packet evidence separate from sign block-entity persistence evidence.
- Promote only `sign-editor-open-update` if row-specific live evidence passes.

Non-goals:
- Proving all sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, public-server safety, production readiness, or full protocol 763 compatibility.

## Design

1. Define a pure sign-edit contract: actor, sign position, initial sign state, submitted four-line payload, expected client open milestone, expected update action, expected server accepted-update correlation, packet rows, and non-claims.
2. Add or extend an isolated runner scenario that triggers the editor open and drives one update through Stevenarella automation or a deterministic fixture path.
3. Emit normalized KV/receipt/log evidence with the packet rows `SignEditorOpenS2CPacket` and `UpdateSignC2SPacket`.
4. Validate with targeted packet live-evidence checker positive and negative fixtures.
5. Update docs only after checker and evidence gates pass.

## Risks

- Headless UI automation may not support deterministic sign text entry.
- Server-side accepted-update correlation must distinguish sign editor updates from block-entity persistence observations.

## Validation

Run baseline checks, sign-editor rail or blocker checks, targeted packet checker positives/negatives, matrix/current-bundle/packet-inventory checks, evidence-manifest/task-evidence checks, Cairn gates/sync/archive, and post-archive validation.
