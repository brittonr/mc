# Protocol-763 CTF rule ledger — 2026-05-27

## Scope

This checkpoint drains the CTF rule correctness Cairn by listing the bounded CTF rule clusters currently backed by evidence and by keeping full CTF correctness explicit as a non-claim.

full CTF correctness remains a non-claim.

## Promoted rule clusters

| Rule | Seam | Required client milestones | Required server milestones | Forbidden transitions | Evidence | BLAKE3 |
| --- | --- | --- | --- | --- | --- | --- |
| score_capture_red_blue_bounded | RED/BLUE scoring soak | flag_pickup, flag_capture, score_red_1, score_blue_1 | server_score_path | none in historical row | `docs/evidence/protocol-763-red-blue-soak-historical-oracle-2026-05-27.md` | `b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de` |
| flag_carrier_death_returns_flag_without_score | Flag-carrier death/return | flag_pickup, combat_attack_sent, combat_death_observed, respawn_request_sent, respawn_health_restored | server_flag_pickup, server_flag_carrier_death, server_flag_return | unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json` | `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4` |
| disconnect_returns_flag_and_reconnect_state_coherent | Reconnect flag-state | flag_pickup, reconnect_session | server_flag_pickup, server_flag_disconnect_return, server_reconnect_state_coherent | unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-reconnect-flag-state.matrix.receipt.json` | `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3` |
| invalid_pickup_rejected_without_ownership_transfer | Invalid flag pickup/ownership | ctf_invalid_pickup_attempted, ctf_invalid_pickup_contained | server_invalid_pickup_rejected | unexpected_flag_pickup_chat, unexpected_server_flag_pickup, unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.receipt.json` | `64c353dc5f256526d4ecfb4078516e85491b42fc9da10adf8e91a7c2c166b8ac` |
| invalid_return_drop_rejected_without_state_mutation | Invalid flag return/drop | ctf_invalid_return_drop_attempted, ctf_invalid_return_drop_contained | server_invalid_return_drop_rejected | unexpected_flag_return, unexpected_server_flag_pickup, unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.receipt.json` | `f0465c4ad154c051ee21bbe96bac939dad875ac3bdaaa785051cdb58636ba2ba` |

## Unpromoted rule clusters

| Rule family | Status | Next action |
| --- | --- | --- |
| invalid_pickup_accepted broad invalid-pickup breadth | Non-claim | Only one own-flag pickup rejection row is promoted; add more flag/team/owner permutations before claiming broader invalid pickup acceptance/rejection coverage. |
| invalid_return_accepted broad invalid return/drop breadth | Non-claim | Only one own-base return/drop rejection row is promoted; add more carrier/drop/return permutations before claiming broader invalid return/drop coverage. |
| score limit / win condition | Non-claim | Add bounded score-limit receipt. |
| simultaneous pickup/capture race | Non-claim | Add deterministic multi-client race fixture. |
| spawn/team balance/resource reset | Non-claim | Add scenario-specific receipts. |
| full CTF correctness | Non-claim | Requires all rule rows complete. |

## Positive validation

`tools/check_ctf_rule_ledger.py` requires:

- each receipt-backed row is `mode=run`, `dry_run=false`, `status=pass`;
- server protocol is `763`;
- required client and server milestones are present;
- missing milestone lists are empty;
- forbidden matches are empty;
- RED/BLUE scoring soak is backed by the historical oracle checkpoint with question, inspected evidence, decision, and digest;
- acceptance matrix and current bundle contain each rule-cluster digest;
- `tools/check_ctf_invalid_pickup_ownership.rs` validates the promoted invalid pickup row, including client containment, server rejection, no owner transfer, no score/capture, and BLAKE3-backed logs;
- `tools/check_ctf_invalid_return_drop.rs` validates the promoted invalid return/drop row, including client containment, server rejection, no flag state mutation, no score/capture, and BLAKE3-backed logs;
- full CTF correctness is still listed as a non-claim.

## Negative fixtures

`tools/check_ctf_rule_ledger.py --self-test` rejects:

- missing server flag return;
- protocol mismatch (`758` instead of `763`);
- unexpected score/capture evidence;
- missing historical scoring oracle;
- full CTF correctness overclaim in the matrix;
- missing invalid pickup server rejection, unexpected server flag pickup, missing checker record, and row-doc overclaims in `tools/check_ctf_invalid_pickup_ownership.rs --self-test`;
- missing invalid return/drop server rejection, unexpected flag return/state mutation, missing checker record, and row-doc overclaims in `tools/check_ctf_invalid_return_drop.rs --self-test`.

## Promotion gate

Only the five bounded rule clusters above are promoted. Full CTF correctness remains blocked until every rule-family row has live receipts, run logs, BLAKE3 manifests, matrix/current-bundle entries, and positive/negative checker coverage.

## Decision

- Question: Can current rule evidence be promoted without implying full CTF correctness?
- Inspected evidence: RED/BLUE historical oracle, flag-carrier death/return receipt, reconnect flag-state receipt, invalid pickup ownership receipt, invalid return/drop receipt, acceptance matrix, current bundle, and checker fixtures.
- Decision: Yes. Promote five bounded rule clusters and keep full CTF correctness as a non-claim.
- Owner: agent.
- Next action: add more invalid-action permutations and race-condition receipts before broadening rule correctness.

## Non-claims

No full CTF correctness, all invalid actions, all flag pickup/return permutations, score limit/win condition, simultaneous races, spawn/team balance, production gameplay readiness, or broad Minecraft compatibility claim is made.
