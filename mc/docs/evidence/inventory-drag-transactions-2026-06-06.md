# Inventory drag transactions bounded evidence

## Scope

Promotes only `inventory-drag-transactions`: one owned-local protocol-763 Valence CTF run where `compatbot` picks up a `RedWool x64` stack from player inventory slot `37`, performs a left-drag over slots `38` and `39`, and ends with `0/32/32` across source/targets.

## Evidence

- Receipt: `docs/evidence/inventory-drag-transactions-2026-06-06.json` (`status=pass`, scenario milestones complete, forbidden patterns empty).
- Run log: `docs/evidence/inventory-drag-transactions-2026-06-06.run.log` (`exit_status=0`).
- Client log: `docs/evidence/inventory-drag-transactions-2026-06-06.client.log` records initial slot `37` count `64`, pickup/source-empty, drag start, target slots `38`/`39`, drag end, and final `0/32/32` distribution.
- Server log: `docs/evidence/inventory-drag-transactions-2026-06-06.server.log` records five Valence `ClickSlotEvent` correlations for pickup, drag start, target A, target B, and drag end.
- Typed events: `docs/evidence/inventory-drag-transactions-2026-06-06.typed-events.log` records 18 ordered client/server events for the scenario.
- Normalized KV: `docs/evidence/inventory-drag-transactions-2026-06-06.kv`.
- Deterministic checker: `tools/check_inventory_drag_transactions_evidence.rs`; validation log `docs/evidence/inventory-drag-transactions-checker-2026-06-06.run.log` has `exit_status=0`.

## Child revisions

The receipt machine-records clean child revisions:

- Stevenarella `aae205baa896b66cd4738ba28b410acea93642fd`, `git_status=clean`.
- Valence `ab5e244d5222e8dadfbc31e62c8c2ce9516ea779`, `git_status=clean`.

## Claim

Bounded player-inventory drag transactions work for this one fixture: actor `compatbot`, item `RedWool`, source slot `37`, target slots `38` and `39`, counts `64 -> 0/32/32`, accepted state-id sequence `1,1,1,1,1`, and Valence server `ClickSlotEvent` drag correlation.

## Non-claims

No creative inventory, all windows, all click modes, all slot mappings, all drag distributions, broad inventory semantics, broad parser-shape coverage, full protocol-763 compatibility, full Minecraft compatibility, CTF correctness, public-server safety, or production readiness.
