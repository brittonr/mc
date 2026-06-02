# Protocol-763 survival coverage matrix — 2026-05-28

## Scope

This matrix tracks bounded survival evidence rows separately from full survival compatibility. Current evidence covers paired Paper/Valence break/place/pickup, crafting table, chest persistence, furnace persistence, and biome/dimension join-state parity rows. Full survival compatibility, broader vanilla parity, restart/world persistence, and broad Minecraft survival behavior remain non-claims until every required row has reviewable evidence.

## Coverage rows

| Survival system | Status | Valence evidence | Reference evidence | Promotion requirement | Explicit non-claim | Next action |
| --- | --- | --- | --- | --- | --- | --- |
| break/place/pickup | reference_parity_covered | `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.receipt.json` | `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md`. | No full survival compatibility or broader vanilla parity. | create next missing survival row |
| crafting | reference_parity_covered | `docs/evidence/protocol-763-survival-crafting-table-valence-2026-05-31.receipt.json` | `docs/evidence/protocol-763-survival-crafting-table-paper-2026-05-31.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/protocol-763-survival-crafting-table-2026-05-31.md`. | No full survival compatibility from crafting row; no furnace/hunger/mob/redstone/biome/dimension/world persistence coverage. | create next missing survival row |
| chest persistence | reference_parity_covered | `docs/evidence/protocol-763-survival-chest-persistence-valence-2026-05-29.receipt.json` | `docs/evidence/protocol-763-survival-chest-persistence-paper-2026-05-29.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.md`. | No full survival compatibility from chest persistence row; no all-container behavior, restart/world persistence, or broader vanilla parity. | create next missing survival row |
| furnace persistence | reference_parity_covered | `docs/evidence/survival-furnace-persistence-valence-2026-06-01.receipt.json` | `docs/evidence/survival-furnace-persistence-paper-2026-06-01.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-furnace-persistence-receipts-2026-06-01.md`; BLAKE3 manifest: `docs/evidence/survival-furnace-persistence-receipts-2026-06-01.b3`. | No full survival compatibility from furnace persistence row; no all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, restart/world persistence, or broader vanilla parity coverage. | create next missing survival row |
| hunger/food | missing | none | none | Add food consume, hunger, saturation, and health-loop receipts. | No hunger or food coverage. | create hunger rail |
| mob drops | missing | none | none | Add mob spawn/kill/drop pickup receipts with deterministic entity/drop metrics. | No mob AI or mob drop coverage. | create mob drop rail |
| redstone | missing | none | none | Add power/update receipts with deterministic block update metrics. | No redstone coverage. | create redstone rail |
| biome/dimension | reference_parity_covered | `docs/evidence/survival-biome-dimension-valence-2026-06-01.receipt.json` | `docs/evidence/survival-biome-dimension-paper-2026-06-01.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/survival-biome-dimension-receipts-2026-06-01.md`. | No full survival compatibility from biome/dimension row; no biome lookup semantics, dimension travel, or world persistence coverage. | create next missing survival row |
| world persistence | missing | none | none | Add server restart or reload receipts with persisted block/inventory state metrics. | No world persistence coverage. | create persistence rail |

## Gate decision

`full_survival_compatibility` remains a non-claim because required rows beyond break/place/pickup, crafting table, chest persistence, furnace persistence, and biome/dimension join state are missing paired reference evidence. In plain terms: full_survival_compatibility remains a non-claim.

## Required evidence for row promotion

Each row needs:

- committed parent, Valence, and Stevenarella child revisions recorded in the receipt or an oracle checkpoint;
- reviewable Valence receipt/logs under `docs/evidence/`;
- BLAKE3 manifest entries for every cited artifact;
- paired reference receipt/logs when vanilla parity is claimed;
- positive and negative checker coverage for required normalized metrics;
- explicit non-claims for adjacent survival systems not covered by the row.
