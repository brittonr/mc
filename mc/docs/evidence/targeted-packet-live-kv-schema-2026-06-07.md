# Targeted packet live evidence KV schema — 2026-06-07

## Scope

This note documents the shared KV schema used by `tools/check_targeted_packet_promotions.rs --live-evidence <kv>` before any targeted packet row moves beyond fixture-bounded status. It is schema/workflow evidence only; it does not promote a packet row by itself.

## Common promotion keys

Every live-promotion KV MUST include:

| Key | Meaning |
| --- | --- |
| `row.id` | Targeted packet row id from the checker row table. |
| `live.promotion.status=passed` | Fail-closed live promotion marker. Blocker/selection notes are not accepted as promotion evidence. |
| `live.evidence.mode=owned-local-live` | Evidence came from an owned-local live path, not a broad/public target. |
| `live.packet.row` | Exact packet row being promoted; must match one of the row's configured packet identifiers. |
| `live.scenario` | Runner scenario or fixture rail that produced the signal. |
| `live.backend` | One of the supported backend labels checked by the schema. |
| `live.client.path` / `live.backend.path` | Client and backend path used for the live signal. |
| `live.client.revision` / `live.backend.revision` / `live.revision.status=current` | Revision metadata required for promotion. |
| `live.receipt` / `live.receipt.blake3` / `live.receipt.digest_status=current` | Durable receipt path under `docs/evidence/`, BLAKE3 digest, and currentness marker. |
| `live.row_extension.kind` | Row-specific extension selected by the checker. |
| `nonclaim.*=true` | Common and row-specific non-claims required by the targeted packet row. |

## Row extension hooks

The checker validates common keys first, then row-specific extension fields and metrics. Current extension kinds cover:

- `block-entity-update-breadth`
- `chat-command-containment`
- `chunk-biome-data-packet`
- `creative-inventory-action`
- `entity-status-effect-packets`
- `recipe-book-client-settings`
- `resource-pack-status`
- `sign-editor-open-update`

Examples of row-specific fields include creative slot/item/count metrics, resource-pack local/no-external-fetch fields, sign position/payload fields, and backend correlation metrics.

## Workflow for future live rails

1. Select a row and record its intended live signal, backend/client path, and non-claims.
2. Produce a durable KV/receipt/log set under `docs/evidence/`.
3. Run `tools/check_targeted_packet_promotions.rs --live-evidence <kv>` through the repo flake check or an equivalent compiled checker.
4. Promote matrix/current-bundle/packet-inventory docs only after the checker passes.
5. Keep blocker notes and selection docs separate from live-promotion KV; they document why rows remain fixture-bounded but are not promotion evidence.

## Validation evidence

- Baseline targeted packet check before schema changes: `docs/evidence/targeted-packet-live-kv-schema-baseline-2026-06-07.run.log` (`exit_status=0`).
- Schema checker validation after implementation: `docs/evidence/targeted-packet-live-kv-schema-checker-2026-06-07.run.log` (`exit_status=0`).
- Cairn gates: `docs/evidence/targeted-packet-live-kv-schema-cairn-gates-2026-06-07.run.log` (`exit_status=0`).
- Evidence manifest refresh/checks and task-evidence gate: `docs/evidence/targeted-packet-live-kv-schema-evidence-manifest-refresh-2026-06-07.run.log` and `docs/evidence/targeted-packet-live-kv-schema-evidence-checks-2026-06-07.run.log` (`exit_status=0`).
- Sync/archive/post-archive validation: `docs/evidence/targeted-packet-live-kv-schema-sync-2026-06-07.run.log`, `docs/evidence/targeted-packet-live-kv-schema-post-sync-validate-2026-06-07.run.log`, `docs/evidence/targeted-packet-live-kv-schema-archive-2026-06-07.run.log`, and `docs/evidence/targeted-packet-live-kv-schema-post-archive-checks-2026-06-07.run.log` (`exit_status=0`).

## Non-claims

This schema does not claim live parity for any packet row, full protocol 763 compatibility, broad Minecraft compatibility, public-server safety, production readiness, or arbitrary gameplay semantics.
