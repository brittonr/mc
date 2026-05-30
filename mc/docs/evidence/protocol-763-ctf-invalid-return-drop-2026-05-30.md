# Protocol-763 CTF invalid return/drop — 2026-05-30

## Scope

Seam: Invalid flag return/drop.

This row promotes one bounded Valence CTF negative rule: `compatbot` joins RED and attempts an own-base RED flag return/drop without carrier ownership. The expected containment is no flag state mutation, no capture, and no score change.

## Artifacts

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Contract | `docs/evidence/protocol-763-ctf-invalid-return-drop-contract-2026-05-30.md` | `dcc5ea3346029d63ebe52a2201aee5a4b776368c4492d84cde90462d8c43ebe0` |
| Receipt | `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.receipt.json` | `f0465c4ad154c051ee21bbe96bac939dad875ac3bdaaa785051cdb58636ba2ba` |
| Run log | `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.run.log` | `cc86c80c3fced7cc8f071fe046573a51f7d6b6cb5d97c5dacb5971494484d157` |
| Client log | `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.client.log` | `1e2ebd36e3c929430c5e913cf59dcc9cf8f137a04900798da5954c0f01e8a743` |
| Server log | `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.server.log` | `910285b631a87276b34b2d05941fd250fded234294abadcb6274e5f513200f63` |
| Checker record | `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.record` | `7f254d90909cb15d4a422bf44960919d1f440d6ec53ced536630c9458607c324` |

## Observed metrics

- Scenario: `ctf-invalid-return-drop`.
- Maintained command: `nix run .#mc-compat-valence-ctf-invalid-return-drop`.
- Receipt status: `status=pass`, `mode=run`, `dry_run=false`.
- Client observed milestones: `protocol_detected`, `join_game`, `render_tick`, `ctf_invalid_return_drop_attempted`, `ctf_invalid_return_drop_contained`.
- Server observed milestones: `server_username_seen`, `server_invalid_return_drop_rejected`.
- Client attempt log: `ctf_invalid_return_drop_attempted player_team=red flag_team=red pre_state=at_base action=own_base_return expected=no_flag_state_mutation_no_score`.
- Client containment log: `ctf_invalid_return_drop_contained player_team=red flag_team=red post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score`.
- Server rejection log: `invalid_flag_return_drop_rejected username=compatbot actor_team=Red flag_team=Red pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score`.
- Negative rail envelope: `invalid_action=own_base_return_without_carrier`, `target_scope=owned-local-loopback`, `observed_outcome=containment_observed`, `postcondition=ctf_invalid_return_drop_contained`, `preflight_passed=true`.
- Forbidden capture/score/state-mutation matches: `[]` for client and server.

## Child revisions

- Valence: `cc74aed6efd29047d7987f82800dc15446a772c2`, `git_status=clean`.
- Stevenarella: `2320afe4baefbd7625e7383b3258424b6e131ea4`, `git_status=clean`.

## Non-claims

No all invalid return/drop permutations. No all flag permutations. No full CTF correctness. No adversarial security. No production readiness. No broad Minecraft compatibility. No vanilla/reference parity claim.
