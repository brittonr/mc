# Protocol-763 survival coverage matrix — 2026-05-28

## Scope

This matrix tracks bounded survival evidence rows separately from full survival compatibility. Current evidence covers paired Paper/Valence break/place/pickup, crafting table, chest persistence, furnace persistence, hunger/food, mob drops, redstone toggle, biome/dimension join-state, and bounded world-persistence restart parity rows. Full survival compatibility, broader vanilla parity, long-term durability, crash recovery, multi-chunk persistence, all block entities, concurrent saves, backups, and broad Minecraft survival behavior remain non-claims.

## Coverage rows

| Survival system | Status | Valence evidence | Reference evidence | Promotion requirement | Explicit non-claim | Next action |
| --- | --- | --- | --- | --- | --- | --- |
| break/place/pickup | reference_parity_covered | `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.receipt.json` | `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md`. | No full survival compatibility or broader vanilla parity. | create next missing survival row |
| crafting | reference_parity_covered | `docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.receipt.json` | `docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/protocol-763-survival-crafting-table-2026-05-31.md`. | No full survival compatibility from crafting row; no furnace/hunger/mob/redstone/biome/dimension/world persistence coverage. | create next missing survival row |
| chest persistence | reference_parity_covered | `docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.receipt.json` | `docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.md`. | No full survival compatibility from chest persistence row; no all-container behavior, restart/world persistence, or broader vanilla parity. | create next missing survival row |
| furnace persistence | reference_parity_covered | `docs/evidence/survival-furnace-persistence-valence-2026-06-01.receipt.json` | `docs/evidence/survival-furnace-persistence-paper-2026-06-01.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-furnace-persistence-receipts-2026-06-01.md`; BLAKE3 manifest: `docs/evidence/survival-furnace-persistence-receipts-2026-06-01.b3`. | No full survival compatibility from furnace persistence row; no all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, restart/world persistence, or broader vanilla parity coverage. | create next missing survival row |
| hunger/food | reference_parity_covered | `docs/evidence/survival-hunger-food-valence-2026-06-02.receipt.json` | `docs/evidence/survival-hunger-food-paper-2026-06-02.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-hunger-food-receipts-2026-06-02.md`; BLAKE3 manifest: `docs/evidence/survival-hunger-food-receipts-2026-06-02.b3`. | No full survival compatibility from hunger/food row; no all foods, exhaustion, regeneration/starvation, potion effects, offhand consumption, or broader vanilla parity coverage. | create next missing survival row |
| mob drops | reference_parity_covered | `docs/evidence/survival-mob-drop-valence-2026-06-02.receipt.json` | `docs/evidence/survival-mob-drop-paper-2026-06-02.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-mob-drop-receipts-2026-06-02.md`; BLAKE3 manifest: `docs/evidence/survival-mob-drop-receipts-2026-06-02.b3`. | No full survival compatibility from mob-drop row; no broad mob AI, loot-table distribution, all mob classes, pickup races, or broader vanilla parity coverage. | create next missing survival row |
| redstone | reference_parity_covered | `docs/evidence/survival-redstone-toggle-valence-2026-06-02.receipt.json` | `docs/evidence/survival-redstone-toggle-paper-2026-06-02.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-redstone-toggle-receipts-2026-06-02.md`; BLAKE3 manifest: `docs/evidence/survival-redstone-toggle-receipts-2026-06-02.b3`. | No full survival compatibility from redstone row; no general redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, or broad vanilla parity coverage. | create next missing survival row |
| biome/dimension | reference_parity_covered | `docs/evidence/survival-biome-dimension-valence-2026-06-01.receipt.json` | `docs/evidence/survival-biome-dimension-paper-2026-06-01.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-biome-dimension-receipts-2026-06-01.md`. | No full survival compatibility from biome/dimension row; no biome lookup semantics, dimension travel, or long-term world persistence durability coverage. | create next missing survival row |
| world persistence | reference_parity_covered | `docs/evidence/survival-world-persistence-valence-2026-06-02.receipt.json` | `docs/evidence/survival-world-persistence-paper-2026-06-02.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-world-persistence-receipts-2026-06-02.md`; BLAKE3 manifest: `docs/evidence/survival-world-persistence-receipts-2026-06-02.b3`. | No full survival compatibility from world persistence row; no long-term durability, crash recovery, multi-chunk persistence, all block entities, concurrent saves, backups, broad vanilla parity, or production readiness. | keep broad survival non-claims explicit |

## Gate decision

`full_survival_compatibility` remains a non-claim even after the bounded world-persistence restart row because the matrix still covers only named, deterministic evidence rows, not broad vanilla survival behavior. In plain terms: full_survival_compatibility remains a non-claim.

## Required evidence for row promotion

Each row needs:

- committed parent, Valence, and Stevenarella child revisions recorded in the receipt or an oracle checkpoint;
- reviewable Valence receipt/logs under `docs/evidence/`;
- BLAKE3 manifest entries for every cited artifact;
- paired reference receipt/logs when vanilla parity is claimed;
- positive and negative checker coverage for required normalized metrics;
- explicit non-claims for adjacent survival systems not covered by the row.
