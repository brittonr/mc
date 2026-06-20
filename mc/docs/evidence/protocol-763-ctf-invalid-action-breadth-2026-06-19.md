# Protocol-763 CTF invalid-action breadth row — 2026-06-19

## Scope

This evidence promotes exactly one additional bounded invalid-action row: `opponent-base-return-drop-without-carrier` under the `CTF invalid-action breadth fixture` seam.

The row is deterministic fixture evidence, not a live gameplay receipt. It checks that the row family can name and validate an opponent-base return/drop rejection shape without changing the existing live `ctf-invalid-pickup-ownership` or `ctf-invalid-return-drop` rows.

## Artifacts

- Breadth matrix: `docs/evidence/protocol-763-ctf-invalid-action-breadth-matrix-2026-06-19.md`
- Fixture receipt: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.receipt.json`
- Row record: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.record`
- Validation log: `docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.run.log`
- Client fixture log: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.client.log`
- Server fixture log: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.server.log`
- BLAKE3 manifest: `docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.b3`
- Checker: `tools/check_ctf_invalid_action_breadth.rs`

## Selected row fields

| Field | Value |
| --- | --- |
| row_id | `opponent-base-return-drop-without-carrier` |
| seam | `CTF invalid-action breadth fixture` |
| scenario | `ctf-invalid-opponent-base-return-drop` |
| action_family | `return/drop` |
| actor_identity | `compatbot` |
| actor_team | `red` |
| flag_team | `blue` |
| base_state | `opponent_base` |
| pre_state | `at_base` |
| post_state | `at_base` |
| invalid_action | `opponent_base_return_drop_without_carrier` |
| expected_rejection | `no_flag_state_mutation_no_score` |
| postcondition | `ctf_invalid_opponent_base_return_drop_contained` |
| target_scope | `owned-local-loopback-fixture` |
| evidence_mode | `deterministic-fixture` |

## Normalized evidence

Client attempt log: `ctf_invalid_opponent_base_return_drop_attempted actor_team=red flag_team=blue pre_state=at_base base=opponent_base action=opponent_base_return_drop_without_carrier expected=no_flag_state_mutation_no_score`.

Client containment log: `ctf_invalid_opponent_base_return_drop_contained actor_team=red flag_team=blue post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score`.

Server rejection log: `invalid_opponent_base_return_drop_rejected username=compatbot actor_team=Red flag_team=Blue pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score`.

Forbidden transition evidence remains absent: `unexpected_flag_pickup`, `unexpected_flag_return`, `unexpected_flag_capture`, `unexpected_red_score`, and `unexpected_blue_score`.

## Non-claims

No all invalid actions. No all flag permutations. No full CTF correctness. No adversarial security. No public-server safety. No production readiness. No broad Minecraft compatibility. No live CTF semantics. No vanilla/reference parity. No claim that existing live CTF rows changed.
