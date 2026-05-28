# Protocol-763 survival reference parity metrics — 2026-05-28

## Status

reference_backend: paper-1.20.1-reference-harness
reference_version: minecraft-1.20.1-protocol-763
reference_receipt: docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json
valence_reference_pair: docs/evidence/protocol-763-survival-reference-valence-2026-05-28.receipt.json
decision_owner: agent

exact survival break/place/pickup parity is covered by paired Paper and Valence receipts. The checker rejects Valence-only survival evidence before any parity row can be promoted. full survival compatibility and broad vanilla parity remain non-claims.

## Normalized exact-match metrics

The comparator reads receipts and milestone logs, then compares explicit normalized fields. It does not infer parity from raw log similarity.

| Metric | Source | Meaning |
| --- | --- | --- |
| scenario.name | receipt | Scenario must be `survival-break-place-pickup`. |
| server.protocol | receipt | Protocol must be `763`. |
| client.username | receipt | Probe identity must match. |
| client.missing_milestones.empty | receipt | Client scenario must have no missing milestones. |
| client.forbidden_matches.empty | receipt | Client scenario must have no forbidden matches. |
| server.missing_milestones.empty | receipt | Server scenario must have no missing milestones. |
| server.forbidden_matches.empty | receipt | Server scenario must have no forbidden matches. |
| client.milestone.protocol_detected | receipt | Client detected the server protocol. |
| client.milestone.join_game | receipt | Client reached play join. |
| client.milestone.render_tick | receipt | Client rendered in-game. |
| client.milestone.survival_break_sent | receipt | Client sent fixed block break. |
| client.milestone.survival_break_update | receipt/log | Client observed break block update. |
| client.milestone.survival_pickup_seen | receipt/log | Client observed pickup/inventory transition. |
| client.milestone.survival_place_sent | receipt/log | Client sent fixed block placement. |
| client.milestone.survival_place_update | receipt/log | Client observed placement block update. |
| server.milestone.server_username_seen | receipt | Server saw the expected username. |
| server.milestone.server_survival_join | receipt/log | Server recorded survival join. |
| server.milestone.server_survival_break | receipt/log | Server recorded target block break. |
| server.milestone.server_survival_pickup | receipt/log | Server recorded pickup. |
| server.milestone.server_survival_place | receipt/log | Server recorded placement. |
| client.break.sent.location | client log | Fixed break target coordinate. |
| client.break.sent.status | client log | Break action status. |
| client.break.update.location | client log | Updated coordinate after break. |
| client.break.update.raw_id | client log | Resulting block raw id after break. |
| client.pickup.count | client log | Pickup count observed by client. |
| client.inventory.slot | client log | Updated inventory slot. |
| client.inventory.item_id | client log | Updated inventory item id. |
| client.inventory.count | client log | Updated inventory item count. |
| client.place.sent.location | client log | Placement target coordinate. |
| client.place.sent.face | client log | Placement face. |
| client.place.sent.hand | client log | Placement hand. |
| client.place.update.location | client log | Updated coordinate after placement. |
| client.place.update.raw_id | client log | Resulting block raw id after placement. |
| server.join.gamemode | server log | Server-side game mode. |
| server.join.target | server log | Server-side target coordinate. |
| server.break.item | server log | Server-side broken item/block material. |
| server.break.at | server log | Server-side break coordinate. |
| server.pickup.slot | server log | Server-side pickup slot. |
| server.pickup.item | server log | Server-side pickup item. |
| server.pickup.count | server log | Server-side pickup count. |
| server.place.item | server log | Server-side placed item. |
| server.place.from_slot | server log | Server-side placement source slot. |
| server.place.at | server log | Server-side placement coordinate. |

## Comparator fixtures

Checker: `tools/check_survival_reference_parity.py`.

Positive fixture:

- `paper` reference fixture and `valence` fixture pass when every normalized metric matches exactly.

Negative fixtures:

- `missing_reference`: comparison fails without a reference receipt/log bundle.
- `missing_metric`: comparison fails when a required log-derived field is absent.
- `mismatched_metric`: comparison fails and names the mismatched metric when a value changes.
- `wrong_backend`: comparison fails when the reference side is not a `paper` receipt.
- `missing_reference_client_log_arg` / related `missing_*_arg` diagnostics: partial paired-artifact CLI arguments fail instead of falling back to doc-only mode.

## Validation evidence

- Paper receipt/log bundle:
  - `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.receipt.json`
  - `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.client.log`
  - `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.server.log`
  - `docs/evidence/protocol-763-survival-reference-paper-2026-05-28.run.log`
- Valence receipt/log bundle:
  - `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.receipt.json`
  - `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.client.log`
  - `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.server.log`
  - `docs/evidence/protocol-763-survival-reference-valence-2026-05-28.run.log`
- Comparator log: `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.compare.log` (`survival reference parity comparison ok: 43 metrics`).
- Paper receipt BLAKE3: `a88fe547bfe2dd43fff3ac5bd967f0ebf5a3c539403211dd029865293130090b`.
- Pair manifest BLAKE3: `f2aee64638a7800b6b082988cc1efe876110507689961b51aed5e7b6d9cbd60a`.
- BLAKE3 manifest: `docs/evidence/protocol-763-survival-reference-pair-2026-05-28.b3`.
- Committed child revisions: Stevenarella `d758630ad77b444d80e4bd8dca8585b5507f556b`, Valence `7d13a242742347a05c9752501880a2e986819ae7`.
- Parent runner/fixture/comparator commit used for promoted evidence: `5d4973d`.
- Historical gate log: `docs/evidence/protocol-763-survival-reference-parity-gate-2026-05-28.run.log`; historical BLAKE3 manifest: `docs/evidence/protocol-763-survival-reference-parity-gate-2026-05-28.b3`.

## Paper fixture probe

`docs/evidence/protocol-763-survival-reference-paper-fixture-2026-05-28.md` records the first live Paper candidate blocker. That blocker is now resolved for this narrow rail by the Paper fixture plugin, the runner's `PAPER_PLUGIN_JAR` mount, and Stevenarella protocol-763 parser/probe updates. The final Paper bundle above is the promoted reference artifact.

## Next evidence needed

The paired break/place/pickup parity row is promoted. Future survival work must stay row-scoped: crafting, chest/furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence still need their own Valence and reference receipts before promotion.
