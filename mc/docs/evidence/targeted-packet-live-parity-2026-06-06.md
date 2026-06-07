# Targeted packet live parity evidence — 2026-06-06

## Scope

This evidence supports Cairn change `promote-targeted-packet-live-parity`. The live-promotion attempt is intentionally conservative: selected rows were reviewed for owned-local live signals, but no row is promoted beyond its existing fixture-bounded status in this change.

## Selected live-parity candidates

| Candidate row | Packet identifier(s) | Existing fixture source | Intended live signal | Decision |
| --- | --- | --- | --- | --- |
| `resource-pack-status` | `play/clientbound/0x40 ResourcePackSendS2CPacket`; `play/serverbound/0x24 ResourcePackStatusC2SPacket` | `docs/evidence/resource-pack-status-2026-06-06.kv` | Owned-local server offers a local resource pack, Stevenarella declines without external fetch, and backend observes the status response. | Blocked: current runner has no deterministic owned-local resource-pack offer hook or asset-serving fixture that can prove no external fetch while preserving public-server safety non-claims. |
| `sign-editor-open-update` | `play/clientbound/0x31 SignEditorOpenS2CPacket`; `play/serverbound/0x2e UpdateSignC2SPacket` | `docs/evidence/sign-editor-open-update-2026-06-06.kv` | Backend opens a sign editor, client submits a four-line payload, and server accepts the update with sign-position correlation. | Blocked: existing sign persistence rail observes block-entity payloads but does not automate the sign-edit UI/open-update interaction. |
| `creative-inventory-action` | `play/serverbound/0x2b CreativeInventoryActionC2SPacket` | `docs/evidence/creative-inventory-action-2026-06-06.kv` | Owned-local creative-mode rail mutates one hotbar slot and backend records the accepted slot state. | Blocked: maintained runner rails do not currently provide a creative-mode fixture with deterministic server correlation and bounded non-claim fields. |

## Rows left fixture-bounded

All eight targeted packet promotion rows remain fixture-bounded unless a future change records passing live evidence:

- `block-entity-update-breadth` remains a deterministic non-sign block-entity update fixture; the existing live sign block-entity packet-family row does not prove non-sign breadth.
- `chat-command-containment` remains an owned-local fixture; existing MCP/chat observability is not a targeted chat/command containment receipt.
- `chunk-biome-data-packet` remains a byte-preservation fixture; existing chunk/biome live rows cover chunk-delta/join-state, not `ChunkBiomeDataS2CPacket` semantics.
- `entity-status-effect-packets` remains a status-effect apply/remove fixture; no maintained live rail applies/removes a bounded effect with client/server correlation.
- `recipe-book-client-settings` remains a settings fixture; existing crafting receipts do not prove recipe-book settings packet behavior.
- The selected `resource-pack-status`, `sign-editor-open-update`, and `creative-inventory-action` rows stay fixture-bounded for the blockers listed above.

## Checker changes

`tools/check_targeted_packet_promotions.rs` now has a fail-closed live evidence mode (`--live-evidence <kv>`). The self-test suite exercises:

- positive fixture validation for every targeted row;
- positive synthetic live evidence validation;
- missing live receipt rejection;
- packet-row mismatch rejection;
- stale receipt digest-status rejection;
- full-protocol overclaim rejection.

This adds the live-promotion guard before any matrix row can safely move beyond fixture-bounded status.

## Validation receipts

- Baseline fixture check before checker changes: `docs/evidence/targeted-packet-live-parity-baseline-2026-06-06.run.log` (`exit_status=0`).
- Checker validation after live-mode tests: `docs/evidence/targeted-packet-live-parity-checker-2026-06-06.run.log` (`exit_status=0`).
- Focused runner dry-run checks: `docs/evidence/targeted-packet-live-parity-runner-checks-2026-06-06.run.log` (`exit_status=0`).
- Cairn gates/validation: `docs/evidence/targeted-packet-live-parity-cairn-gates-2026-06-06.run.log`, `docs/evidence/targeted-packet-live-parity-cairn-validate-2026-06-06.run.log`, and `docs/evidence/targeted-packet-live-parity-post-sync-validate-2026-06-06.run.log` (`exit_status=0`).
- Spec sync and archive: `docs/evidence/targeted-packet-live-parity-sync-2026-06-06.run.log` and `docs/evidence/targeted-packet-live-parity-archive-2026-06-06.run.log` (`exit_status=0`).
- Evidence manifest refresh/checks and task-evidence gate: `docs/evidence/targeted-packet-live-parity-evidence-manifest-refresh-2026-06-06.run.log` and `docs/evidence/targeted-packet-live-parity-evidence-manifest-check-2026-06-06.run.log` (`exit_status=0`).

## Matrix/bundle decision

No acceptance-matrix or current-bundle row is promoted in this change. Existing targeted rows retain their fixture-bounded classifications and non-claim language.

## Non-claims

This evidence does not claim live Paper/Valence parity for any targeted packet row, full protocol 763 compatibility, broad Minecraft compatibility, public-server safety, production readiness, arbitrary gameplay semantics, or broad parser-shape coverage.
