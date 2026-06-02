# Protocol-763 CTF rule ledger — 2026-05-27

## Scope

This checkpoint drains the CTF rule correctness Cairn by listing the bounded CTF rule clusters currently backed by evidence and by keeping full CTF correctness explicit as a non-claim.

full CTF correctness remains a non-claim.

## Promoted rule clusters

| Rule | Seam | Required client milestones | Required server milestones | Forbidden transitions | Evidence | BLAKE3 |
| --- | --- | --- | --- | --- | --- | --- |
| score_capture_red_blue_bounded | RED/BLUE scoring soak | multi-client-load-score, blue-flag-score, flag_pickup, flag_capture, score_red_1, score_blue_1 | server_client_a_seen, server_client_b_seen, server_username_seen, server_flag_or_score | panic, unexpected_eof, protocol_mismatch, decode_error | `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.receipt.json` | `349b1b7dc84877dd56dce3344611d04ce86a74413738ebc3fdd2a2f720504bed` |
| flag_carrier_death_returns_flag_without_score | Flag-carrier death/return | flag_pickup, combat_attack_sent, combat_death_observed, respawn_request_sent, respawn_health_restored | server_flag_pickup, server_flag_carrier_death, server_flag_return | unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json` | `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4` |
| disconnect_returns_flag_and_reconnect_state_coherent | Reconnect flag-state | flag_pickup, reconnect_session | server_flag_pickup, server_flag_disconnect_return, server_reconnect_state_coherent | unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-reconnect-flag-state.matrix.receipt.json` | `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3` |
| invalid_pickup_rejected_without_ownership_transfer | Invalid flag pickup/ownership | ctf_invalid_pickup_attempted, ctf_invalid_pickup_contained | server_invalid_pickup_rejected | unexpected_flag_pickup_chat, unexpected_server_flag_pickup, unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.receipt.json` | `64c353dc5f256526d4ecfb4078516e85491b42fc9da10adf8e91a7c2c166b8ac` |
| invalid_return_drop_rejected_without_state_mutation | Invalid flag return/drop | ctf_invalid_return_drop_attempted, ctf_invalid_return_drop_contained | server_invalid_return_drop_rejected | unexpected_flag_return, unexpected_server_flag_pickup, unexpected_flag_capture, unexpected_red_score, unexpected_blue_score | `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.receipt.json` | `f0465c4ad154c051ee21bbe96bac939dad875ac3bdaaa785051cdb58636ba2ba` |
| score_limit_win_emits_once_without_post_win_mutation | Score limit / win condition | ctf_score_limit_win_seen | server_score_limit_pre_state, server_score_limit_final_capture, server_score_limit_win_condition | score_limit_duplicate_win, score_limit_post_win_score_mutation, unexpected_red_score_3, unexpected_blue_score_1 | `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.receipt.json` | `7c0d7805e54d95f2768f0164f1b4e62f59f57d524f3a61c3205eb0d611e89e02` |
| simultaneous_pickup_capture_race_one_accept_one_reject | Simultaneous pickup/capture race | ctf_race_client_count, flag_pickup, flag_capture, score_red_1 | server_ctf_race_accepted_transition, server_ctf_race_rejected_transition, server_ctf_race_final_state | ctf_race_double_accept, unexpected_red_score_2, unexpected_blue_score_1 | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.receipt.json` | `cc0b21579b8c5d99aa0d2bab04cc1ec3a34ecbdfceee2edc1ba0e497c11f977f` |

## Unpromoted rule clusters

| Rule family | Status | Next action |
| --- | --- | --- |
| invalid_pickup_accepted broad invalid-pickup breadth | Non-claim | Only one own-flag pickup rejection row is promoted; add more flag/team/owner permutations before claiming broader invalid pickup acceptance/rejection coverage. |
| invalid_return_accepted broad invalid return/drop breadth | Non-claim | Only one own-base return/drop rejection row is promoted; add more carrier/drop/return permutations before claiming broader invalid return/drop coverage. |
| score_limit_variants broad score-limit breadth | Non-claim | Only one near-limit RED capture to configured score limit `2` is promoted; add more score limits/settings/race fixtures before claiming broader win-condition coverage. |
| simultaneous race breadth | Non-claim | Only one deterministic two-client same-flag race window is promoted; add latency, jitter, alternate team/flag, and larger-concurrency fixtures before claiming broader race coverage. |
| spawn/team balance/resource reset | Non-claim | Add scenario-specific receipts. |
| full CTF correctness | Non-claim | Requires all rule rows complete. |

## Positive validation

`tools/check_ctf_rule_ledger.py` requires:

- each receipt-backed row is `mode=run`, `dry_run=false`, `status=pass`;
- server protocol is `763`;
- required client and server milestones are present;
- missing milestone lists are empty;
- forbidden matches are empty;
- RED/BLUE scoring soak is backed by fresh copied `multi-client-load-score` and `blue-flag-score` receipts/logs plus the live-refresh summary receipt: `docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.receipt.json`, `docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.receipt.json`, and `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.receipt.json`;
- acceptance matrix and current bundle contain each rule-cluster digest;
- `tools/check_ctf_invalid_pickup_ownership.rs` validates the promoted invalid pickup row, including client containment, server rejection, no owner transfer, no score/capture, and BLAKE3-backed logs;
- `tools/check_ctf_invalid_return_drop.rs` validates the promoted invalid return/drop row, including client containment, server rejection, no flag state mutation, no score/capture, and BLAKE3-backed logs;
- `tools/check_ctf_score_limit_win_condition.rs` validates the promoted score-limit row, including near-limit pre-state, final capture metrics, one win/end-state emission, no duplicate-win, no post-win score mutation, and BLAKE3-backed logs;
- `tools/check_mc_compat_row_contracts.rs --row ctf-simultaneous-pickup-capture-race` validates the promoted simultaneous pickup/capture race row, including client identities, team roles, ordered accepted/rejected/final milestones, final score, race-window bounds, and BLAKE3-backed logs;
- full CTF correctness is still listed as a non-claim.

## Negative fixtures

`tools/check_ctf_rule_ledger.py --self-test` rejects:

- missing server flag return;
- protocol mismatch (`758` instead of `763`);
- unexpected score/capture evidence;
- missing historical scoring oracle;
- full CTF correctness overclaim in the matrix;
- missing invalid pickup server rejection, unexpected server flag pickup, missing checker record, and row-doc overclaims in `tools/check_ctf_invalid_pickup_ownership.rs --self-test`;
- missing invalid return/drop server rejection, unexpected flag return/state mutation, missing checker record, and row-doc overclaims in `tools/check_ctf_invalid_return_drop.rs --self-test`;
- duplicate score-limit win, post-win score mutation, wrong final score, missing checker record, and row-doc overclaims in `tools/check_ctf_score_limit_win_condition.rs --self-test`;
- dry-run RED/BLUE receipt, target-only historical receipt, missing BLUE score milestone, missing server evidence, missing checker record, and row-doc overclaims in `tools/check_red_blue_scoring_soak_live_refresh.rs --self-test`.

## Promotion gate

Only the seven bounded rule clusters above are promoted. Full CTF correctness remains blocked until every rule-family row has live receipts, run logs, BLAKE3 manifests, matrix/current-bundle entries, and positive/negative checker coverage.

## Decision

- Question: Can current rule evidence be promoted without implying full CTF correctness?
- Inspected evidence: RED/BLUE live-refresh receipts/logs, flag-carrier death/return receipt, reconnect flag-state receipt, invalid pickup ownership receipt, invalid return/drop receipt, score-limit win-condition receipt, simultaneous pickup/capture race receipt, acceptance matrix, current bundle, and checker fixtures.
- Decision: Yes. Promote seven bounded rule clusters and keep full CTF correctness as a non-claim.
- Owner: agent.
- Next action: add more invalid-action permutations, broader race-condition receipts, and spawn/team balance receipts before broadening rule correctness.

## Non-claims

No full CTF correctness, all invalid actions, all flag pickup/return permutations, all score limits/settings, overtime/tiebreakers, scoreboard UI parity, all simultaneous races, latency/jitter race tolerance, spawn/team balance, production gameplay readiness, or broad Minecraft compatibility claim is made.
