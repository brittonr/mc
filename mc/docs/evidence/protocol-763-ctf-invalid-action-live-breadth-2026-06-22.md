# Protocol-763 CTF invalid-action live breadth — 2026-06-22

## Question

Can the bounded row `opponent-base-return-drop-without-carrier` move from deterministic fixture coverage to owned-local live evidence without expanding the claim beyond one invalid return/drop permutation?

## Inspected evidence

- Live receipt: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.receipt.json`
- Row record: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.record`
- Live run log: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.run.log`
- Checker run log: `docs/evidence/protocol-763-ctf-invalid-action-live-breadth-checker-2026-06-22.run.log`
- Client log copy: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.client.log`
- Server log copy: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.server.log`
- Typed event log copy: `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-live-2026-06-22.typed-events.log`
- Focused BLAKE3 manifest: `docs/evidence/protocol-763-ctf-invalid-action-live-breadth-2026-06-22.b3`

## Observed live row

The selected seam is `CTF invalid-action breadth live` and the evidence mode is `bounded-owned-local-live`.

The row is `opponent-base-return-drop-without-carrier`: `compatbot` acts as red team against the blue flag at the opponent base while the blue flag is still `at_base`.

Required client attempt milestone:

```text
ctf_invalid_opponent_base_return_drop_attempted actor_team=red flag_team=blue pre_state=at_base base=opponent_base action=opponent_base_return_drop_without_carrier expected=no_flag_state_mutation_no_score
```

Required client postcondition milestone:

```text
ctf_invalid_opponent_base_return_drop_contained actor_team=red flag_team=blue post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score
```

Required server rejection milestone:

```text
invalid_opponent_base_return_drop_rejected username=compatbot actor_team=Red flag_team=Blue pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score
```

The runner receipt requires `server_invalid_opponent_base_return_drop_rejected`, empty missing-milestone lists, empty forbidden-match lists, `negative_live_rail.live_receipt=true`, `negative_live_rail.invalid_action=opponent_base_return_drop_without_carrier`, and `typed_event_oracle.contributes_to_pass_fail=true`.

## Non-claims

- No all invalid actions.
- No all flag permutations.
- No full CTF correctness.
- No adversarial security.
- No public-server safety.
- No production readiness.
- No broad Minecraft compatibility.
- No live CTF semantics breadth.
- No vanilla/reference parity.

## Decision

Promote only the bounded owned-local live row `opponent-base-return-drop-without-carrier`. The row demonstrates live containment for an opponent-base return/drop attempt without carrier and does not broaden the compatibility claim beyond that row.

## Owner

mc-compat Cairn change `ctf-invalid-action-live-breadth-proof`.

## Next action

Keep `tools/check_ctf_invalid_action_breadth.rs` and the focused BLAKE3 manifest as the reviewable guard for this row when updating acceptance-matrix and current-bundle summaries.
