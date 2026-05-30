# Protocol-763 CTF invalid pickup ownership contract — 2026-05-30

## Scope

Row: `ctf-invalid-pickup-ownership` / Invalid flag pickup/ownership.

This row covers exactly one owned-local Valence CTF negative action: player `compatbot` is on the RED team and attempts to pick up the RED flag (`player_team=red`, `flag_team=red`) by using the existing flag probe path against the own flag. The expected result is no ownership transfer and no score/capture promotion.

## Normalized metrics

| Metric | Expected value |
| --- | --- |
| scenario | `ctf-invalid-pickup-ownership` |
| player_team | `player_team=red` |
| flag_team | `flag_team=red` |
| pre_owner | `pre_owner=none` |
| invalid_action | `own_flag_pickup_without_ownership_transfer` |
| post_owner | `post_owner=none` |
| red_score | `red_score=0` |
| blue_score | `blue_score=0` |
| client attempt milestone | `ctf_invalid_pickup_attempted` |
| client postcondition milestone | `ctf_invalid_pickup_contained` |
| server rejection milestone | `server_invalid_pickup_rejected` / `invalid_flag_pickup_rejected` |
| containment outcome | `containment_observed` |

## Fail-closed rejection reasons

The row checker rejects missing or mismatched player team, flag identity, pre-owner state, invalid pickup action, post-owner state, score counters, forbidden capture/score patterns, missing server rejection, unexpected owner transfer, unexpected score/capture, missing BLAKE3-backed logs, or non-claim overreach.

## Non-claims

All invalid actions remain non-claims. all invalid actions remain non-claims. All flag permutations remain non-claims. Full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility remain non-claims.
