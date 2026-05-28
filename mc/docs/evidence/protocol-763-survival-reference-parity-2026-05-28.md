# Protocol-763 survival reference parity metrics — 2026-05-28

## Status

reference_backend: paper-1.20.1-reference-harness
reference_version: minecraft-1.20.1-protocol-763
reference_receipt: none
valence_reference_pair: none
decision_owner: agent

exact survival break/place/pickup parity remains a non-claim. The current Valence-only survival rail is useful evidence, but paired reference evidence is still missing. The checker rejects Valence-only survival evidence before any parity row can be promoted.

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

- Run log: `docs/evidence/protocol-763-survival-reference-parity-gate-2026-05-28.run.log`.
- BLAKE3 manifest: `docs/evidence/protocol-763-survival-reference-parity-gate-2026-05-28.b3`.
- The gate self-test passed, the doc/non-claim check passed, and the current Valence receipt used as both sides was rejected with `wrong_backend:reference:valence`.

## Paper fixture probe

`docs/evidence/protocol-763-survival-reference-paper-fixture-2026-05-28.md` records the first live Paper candidate probe. Dry-run matrix wiring now passes for protocol 763, but the live Paper reference receipt is blocked: Stevenarella panics on vanilla/Paper clientbound play packet `0x6b`, and plain Paper does not emit the server-side `server_survival_*` fixture milestones required by the comparator.

## Next evidence needed

1. Add a Paper/reference fixture plugin or equivalent harness for this exact probe, including server-side `server_survival_*` metrics.
2. Patch or extend Stevenarella's Paper 1.20.1 parser path for packet `0x6b`.
3. Produce reference receipt/log artifacts under `docs/evidence/`.
3. Produce a matching Valence receipt/log bundle from committed child revisions.
4. Run `python3 tools/check_survival_reference_parity.py --reference-receipt ... --reference-client-log ... --reference-server-log ... --valence-receipt ... --valence-client-log ... --valence-server-log ...`.
5. Only then update the acceptance matrix for the narrow break/place/pickup parity row; full survival compatibility and broad vanilla parity stay non-claims.
