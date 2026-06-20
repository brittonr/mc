# Protocol-763 survival coverage matrix — 2026-05-28

## Scope

This matrix tracks bounded survival evidence rows separately from full survival compatibility. Current evidence covers paired Paper/Valence break/place/pickup, crafting table, chest persistence, furnace persistence, hunger/food, mob drops, redstone toggle, biome/dimension join-state, bounded world-persistence restart, bounded crash-recovery parity, and bounded sign block-entity persistence rows. Full survival compatibility, broader vanilla parity, long-term durability, arbitrary crash consistency, multi-chunk persistence, all block entities, arbitrary NBT parity, sign editing UI parity, concurrent saves, backups, and broad Minecraft survival behavior remain non-claims.

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
| world persistence | reference_parity_covered | `docs/evidence/survival-world-persistence-valence-2026-06-02.receipt.json` | `docs/evidence/survival-world-persistence-paper-2026-06-02.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-world-persistence-receipts-2026-06-02.md`; BLAKE3 manifest: `docs/evidence/survival-world-persistence-receipts-2026-06-02.b3`. | No full survival compatibility from world persistence row; no long-term durability, arbitrary crash consistency, multi-chunk persistence, all block entities, concurrent saves, backups, broad vanilla parity, or production readiness. | keep broad survival non-claims explicit |
| crash recovery | reference_parity_covered | `docs/evidence/survival-crash-recovery-valence-2026-06-04.receipt.json` | `docs/evidence/survival-crash-recovery-paper-2026-06-04.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-crash-recovery-receipts-2026-06-04.md`; BLAKE3 manifest: `docs/evidence/survival-crash-recovery-receipts-2026-06-04.b3`. | No full survival compatibility from crash recovery row; no long-term durability, arbitrary crash consistency, multi-chunk persistence, all block entities, concurrent saves, backups, broad vanilla parity, or production readiness. | keep broad survival non-claims explicit |
| sign block entity | reference_parity_covered | `docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json` | `docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-block-entity-persistence-receipts-2026-06-04.md`; BLAKE3 manifest: `docs/evidence/survival-block-entity-persistence-receipts-2026-06-04.b3`. | No full survival compatibility from sign block-entity row; no all block-entity parity, arbitrary NBT parity, sign editing UI parity, multi-chunk persistence, broad vanilla parity, public-server safety, or production readiness. | keep broad survival and all-block-entity non-claims explicit |

## Gate decision

`full_survival_compatibility` remains a non-claim even after the bounded world-persistence restart, crash-recovery, and sign block-entity rows because the matrix still covers only named, deterministic evidence rows, not broad vanilla survival behavior. In plain terms: full_survival_compatibility remains a non-claim.

## Aggregate survival claim boundary

The focused gate `mc-compat-survival-aggregate-claim-boundary` is required before any future full survival compatibility or broad vanilla survival parity wording can be used. Current evidence remains row-scoped reference parity for each bounded survival row; aggregate survival claim blocked is the only allowed aggregate status until every prerequisite below has paired Paper/reference and Valence evidence, comparator output, fresh BLAKE3 manifest linkage, and an aggregate evidence bundle. Each pending breadth prerequisite is a bounded survival row family, not a broad claim.

| Survival aggregate prerequisite | Status | Valence evidence | Reference evidence | Comparator/evidence doc | Manifest | Claim vocabulary |
| --- | --- | --- | --- | --- | --- | --- |
| crafting recipe breadth | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| furnace smelting breadth | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| hunger health cycle | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| mob AI loot breadth | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| redstone circuit breadth | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| biome dimension travel | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| world multichunk durability | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| container block entity breadth | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |
| sign editing live parity | pending_breadth_evidence | none | none | none | none | aggregate survival claim blocked; non-claim |

## Required evidence for row promotion

Each row needs:

- committed parent, Valence, and Stevenarella child revisions recorded in the receipt or an oracle checkpoint;
- reviewable Valence receipt/logs under `docs/evidence/`;
- BLAKE3 manifest entries for every cited artifact;
- paired reference receipt/logs when vanilla parity is claimed;
- positive and negative checker coverage for required normalized metrics;
- explicit non-claims for adjacent survival systems not covered by the row.
