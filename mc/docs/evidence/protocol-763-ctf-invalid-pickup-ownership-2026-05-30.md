# Protocol-763 CTF invalid pickup ownership — 2026-05-30

## Scope

Seam: Invalid flag pickup/ownership.

This row promotes one bounded Valence CTF negative rule: `compatbot` joins RED and attempts an own-flag RED pickup. The expected containment is no flag ownership transfer, no capture, and no score change.

## Artifacts

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Contract | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-contract-2026-05-30.md` | `206d23e588a80e64bbe7abe5b2de4561936758e40f8e76a45f0ee4f50f0aceef` |
| Receipt | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.receipt.json` | `64c353dc5f256526d4ecfb4078516e85491b42fc9da10adf8e91a7c2c166b8ac` |
| Run log | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.run.log` | `7c56924e95dc921dd2ebc1e91ba258842e4a8143f37a4f648de8417c6c90de02` |
| Client log | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.client.log` | `e65b37dbaae0675198a332d0ff85fae88691f3b9d89e24ee75055a5e055f960c` |
| Server log | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.server.log` | `13165dba1029b3464ac00aacc858b146328020edcc3cd438ada255cdd0adf780` |
| Checker record | `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.record` | `5af989d8045c7eaaf7038b5d7542b407136a8df838e57ed4f20ab798d561d675` |

## Observed metrics

- Scenario: `ctf-invalid-pickup-ownership`.
- Maintained command: `nix run .#mc-compat-valence-ctf-invalid-pickup-ownership`.
- Receipt status: `status=pass`, `mode=run`, `dry_run=false`.
- Client observed milestones: `protocol_detected`, `join_game`, `render_tick`, `ctf_invalid_pickup_attempted`, `ctf_invalid_pickup_contained`.
- Server observed milestones: `server_username_seen`, `server_invalid_pickup_rejected`.
- Client attempt log: `ctf_invalid_pickup_attempted player_team=red flag_team=red pre_owner=none action=own_flag_pickup expected=no_owner_transfer_no_score`.
- Client containment log: `ctf_invalid_pickup_contained player_team=red flag_team=red post_owner=none red_score=0 blue_score=0 outcome=no_owner_transfer_no_score`.
- Server rejection log: `invalid_flag_pickup_rejected username=compatbot player_team=Red flag_team=Red pre_owner=none post_owner=none red_score=0 blue_score=0 outcome=no_owner_transfer_no_score`.
- Negative rail envelope: `invalid_action=own_flag_pickup_without_ownership_transfer`, `target_scope=owned-local-loopback`, `observed_outcome=containment_observed`, `postcondition=ctf_invalid_pickup_contained`, `preflight_passed=true`.
- Forbidden capture/score/ownership matches: `[]` for client and server.

## Child revisions

- Valence: `4586eddda62057f120937ad5b5cac3b5883ca124`, `git_status=clean`.
- Stevenarella: `37a217f1c98923a379de236f8f27ce7fc798ba38`, `git_status=clean`.

## Non-claims

No all invalid actions. No all flag permutations. No full CTF correctness. No adversarial security. No production readiness. No broad Minecraft compatibility. No vanilla/reference parity claim.
