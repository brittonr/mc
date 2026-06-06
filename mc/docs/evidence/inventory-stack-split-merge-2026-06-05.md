# Inventory stack split/merge bounded evidence

## Scope

Promotes only `inventory-stack-split-merge`: one owned-local protocol-763 Valence CTF run where `compatbot` splits a `RedWool x64` stack in player inventory slot `37` into `32/32`, places the split half into slot `38`, picks that half back up, and merges it back into slot `37`.

## Evidence

- Receipt: `docs/evidence/inventory-stack-split-merge-2026-06-05.json` (`status=pass`, scenario milestones complete, forbidden patterns empty).
- Run log: `docs/evidence/inventory-stack-split-merge-2026-06-05.run.log` (`exit_status=0`).
- Client log: `docs/evidence/inventory-stack-split-merge-2026-06-05.client.log` records initial slot `37` count `64`, split pickup/place, destination count `32`, merge pickup/place, and final source count `64`.
- Server log: `docs/evidence/inventory-stack-split-merge-2026-06-05.server.log` records four Valence `ClickSlotEvent` correlations and the bounded split/merge summaries.
- Typed events: `docs/evidence/inventory-stack-split-merge-2026-06-05.typed-events.log` records 18 ordered client/server events for the scenario.
- Normalized KV: `docs/evidence/inventory-stack-split-merge-2026-06-05.kv`.
- Deterministic checker: `tools/check_inventory_stack_split_merge_evidence.rs`; validation log `docs/evidence/inventory-stack-split-merge-checker-2026-06-05.run.log` has `exit_status=0`.

## Child revisions

The receipt machine-records clean child revisions:

- Stevenarella `722d9503f2c31cf8e20ed0bb27092e9df9bdf206`, `git_status=clean`.
- Valence `a8ff748446aab2ecabb7674aad5c5d42208857de`, `git_status=clean`.

## Claim

Bounded player-inventory stack split/merge works for this one fixture: actor `compatbot`, item `RedWool`, source slot `37`, destination slot `38`, counts `64 -> 32/32 -> 64/0`, accepted state-id sequence `1,1,1,1`, and Valence server `ClickSlotEvent` split/merge correlation.

## Non-claims

No drag transactions, creative inventory, all windows, all click modes, all slot mappings, broad inventory semantics, broad parser-shape coverage, full protocol-763 compatibility, full Minecraft compatibility, CTF correctness, public-server safety, or production readiness.
