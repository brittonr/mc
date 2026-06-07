## Context

The targeted-packet live-parity drain selected `creative-inventory-action` as a small owned-local candidate but left it fixture-bounded because no maintained creative rail existed. The row is useful as a first live promotion because it can be scoped to one slot mutation and one server correlation.

## Goals / Non-Goals

Goals:
- Create a deterministic creative-mode rail for one configured slot/item/count mutation.
- Keep pure scenario definitions and live-evidence contract validation separate from process/client/server orchestration.
- Promote only the `creative-inventory-action` row if the live evidence checker passes.

Non-goals:
- Proving broad creative inventory semantics, all slots/items, pick-block behavior, survival inventory parity, public-server safety, production readiness, or full protocol 763 compatibility.
- Promoting any other targeted packet row.

## Design

1. Define a narrow creative live contract in pure data: scenario id, actor, game-mode precondition, semantic slot, wire slot, item id, item count, expected packet row, expected backend correlation, and non-claims.
2. Add or extend an isolated runner path that applies the creative-mode precondition and drives one client mutation through the existing Stevenarella/Valence or owned-local backend path.
3. Record normalized evidence as KV and receipt/log artifacts under `docs/evidence/`.
4. Validate the normalized evidence through `tools/check_targeted_packet_promotions.rs --live-evidence <kv>` before any matrix/bundle/inventory update.
5. Update the acceptance matrix, current bundle, and packet inventory only for `creative-inventory-action` when the checker passes.

## Risks

- Client automation may not reliably emit `CreativeInventoryActionC2SPacket`; if so, record a blocker and leave the row fixture-bounded.
- Backend correlation must distinguish creative slot mutation from survival inventory actions.

## Validation

- Run targeted packet checks before changes as a baseline.
- Run runner tests/dry-runs covering the new scenario path.
- Run the live-evidence checker against positive creative evidence and negative stale/wrong-row/overclaim fixtures.
- Run matrix, current-bundle, packet-inventory, evidence-manifest, task-evidence, Cairn gates, Cairn sync, archive, and post-archive validation.
