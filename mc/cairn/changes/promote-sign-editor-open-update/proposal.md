# Proposal: Promote sign editor open/update evidence

## Why

`SignEditorOpenS2CPacket` and `UpdateSignC2SPacket` remain packet-inventory gaps. The existing sign block-entity persistence row proves server-authored sign payload observation, but it does not prove client sign editing UI open/update semantics.

## What Changes

- Add one bounded sign-edit rail where `compatbot` opens one configured sign editor and submits one configured four-line payload.
- Require client milestones for sign-editor open/update intent and Valence server correlation for the accepted sign update.
- Promote only the configured sign editor open/update packet rows, keeping all sign editing UI behavior, all sign variants, all text/NBT semantics, full protocol coverage, public-server safety, and production readiness as non-claims.

## Impact

- **Files**: Stevenarella probe, Valence fixture instrumentation, runner scenario metadata, packet inventory, evidence docs/manifests, checker, and Cairn tasks/specs.
- **Testing**: positive/negative checker fixtures, focused runner/client/server tests, packet-inventory and bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
