# Protocol-763 CTF invalid return/drop contract — 2026-05-30

## Scope

Row: `ctf-invalid-return-drop`

This row covers one configured RED-player own-base return/drop attempt while the RED flag is already at base and unowned. It proves the attempted action is observed, server-side containment is logged, flag state remains unchanged, scores remain unchanged, and no capture/return/pickup side effect appears.

## Normalized metrics

- `player_team=red`
- `actor_team=Red`
- `flag_team=red` / `flag_team=Red`
- `pre_state=at_base`
- `post_state=at_base`
- `invalid_action=own_base_return_without_carrier`
- `postcondition=ctf_invalid_return_drop_contained`
- `red_score=0`
- `blue_score=0`
- `outcome=no_flag_state_mutation_no_score`
- `target_scope=owned-local-loopback`

## Required evidence

- Live Valence CTF receipt for `ctf-invalid-return-drop`.
- Client log with `ctf_invalid_return_drop_attempted` and `ctf_invalid_return_drop_contained`.
- Server log with `server_invalid_return_drop_rejected` / `invalid_flag_return_drop_rejected`.
- Forbidden-pattern scan showing no `flag_pickup`, `flag_return`, `flag_disconnect_return`, `flag_capture`, `RED: 1`, or `BLUE: 1` milestones.
- Row checker record and BLAKE3 manifest under `docs/evidence/`.

## Fail-closed cases

The row MUST fail on missing attempted-action evidence, flag state mutation, unexpected flag return/drop, unexpected score/capture, missing server containment, missing forbidden-pattern scan, or any claim that all invalid return/drop permutations are covered.

## Non-claims

No all invalid return/drop permutations, all flag permutations, full CTF correctness, adversarial security, production readiness, broad Minecraft compatibility, or vanilla/reference parity claim. Exact bounded-scope token: `all invalid return/drop permutations remain non-claims`.
