# Protocol-763 survival coverage matrix — 2026-05-28

## Scope

This matrix tracks bounded survival evidence rows separately from full survival compatibility. Current evidence covers one paired Paper/Valence break/place/pickup parity row. Full survival compatibility, broader vanilla parity, persistence, and broad Minecraft survival behavior remain non-claims until every required row has reviewable evidence.

## Coverage rows

| Survival system | Status | Valence evidence | Reference evidence | Promotion requirement | Explicit non-claim | Next action |
| --- | --- | --- | --- | --- | --- | --- |
| break/place/pickup | reference_parity_covered | `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.receipt.json` | `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json` | Paired Paper/Valence comparator evidence: `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md`. | No full survival compatibility or broader vanilla parity. | create next missing survival row |
| crafting | missing | none | none | Add Valence and reference crafting receipts with inventory input/output metrics. | No crafting coverage. | create crafting rail |
| chest persistence | missing | none | none | Add open/store/reconnect/reopen receipts with slot persistence metrics. | No chest or persistence coverage. | create chest persistence rail |
| furnace persistence | missing | none | none | Add smelt/fuel/output/reconnect receipts with state persistence metrics. | No furnace coverage. | create furnace rail |
| hunger/food | missing | none | none | Add food consume, hunger, saturation, and health-loop receipts. | No hunger or food coverage. | create hunger rail |
| mob drops | missing | none | none | Add mob spawn/kill/drop pickup receipts with deterministic entity/drop metrics. | No mob AI or mob drop coverage. | create mob drop rail |
| redstone | missing | none | none | Add power/update receipts with deterministic block update metrics. | No redstone coverage. | create redstone rail |
| biome/dimension | missing | none | none | Add biome or dimension transition receipts with decoded environment/state metrics. | No biome or dimension coverage. | create biome/dimension rail |
| world persistence | missing | none | none | Add server restart or reload receipts with persisted block/inventory state metrics. | No world persistence coverage. | create persistence rail |

## Gate decision

`full_survival_compatibility` remains a non-claim because required rows beyond break/place/pickup are missing paired reference evidence. In plain terms: full_survival_compatibility remains a non-claim.

## Required evidence for row promotion

Each row needs:

- committed parent, Valence, and Stevenarella child revisions recorded in the receipt or an oracle checkpoint;
- reviewable Valence receipt/logs under `docs/evidence/`;
- BLAKE3 manifest entries for every cited artifact;
- paired reference receipt/logs when vanilla parity is claimed;
- positive and negative checker coverage for required normalized metrics;
- explicit non-claims for adjacent survival systems not covered by the row.
