# Protocol-763 RED/BLUE scoring soak live-refresh contract — 2026-05-30

## Scope

RED/BLUE scoring soak freshness covers one fresh live rerun of both maintained scoring soak rails:

- Scenario `multi-client-load-score` through `nix run .#mc-compat-valence-ctf-600s-soak`
- Scenario `blue-flag-score` through `nix run .#mc-compat-valence-ctf-blue-600s-soak`

Each run must use protocol `763`, version `1.20.1`, `duration_secs=600`, `timeout_secs=600`, local Stevenarella and Valence checkouts, and copied receipts/logs under `docs/evidence/` with BLAKE3 manifests.

## Required normalized metrics

RED run:

- `status=pass`, `mode=run`, `dry_run=false`
- required and observed `multi_client_count`, `team_red`, `flag_pickup`, `flag_capture`, and `score_red_1`
- server correlation for `server_client_a_seen`, `server_client_b_seen`, and `server_flag_or_score`
- empty `missing_milestones` and `forbidden_matches`

BLUE run:

- `status=pass`, `mode=run`, `dry_run=false`
- required and observed `team_blue`, `flag_pickup`, `flag_capture`, and `score_blue_1`
- server correlation for `server_username_seen` and `server_flag_or_score`
- empty `missing_milestones` and `forbidden_matches`

Both runs must record clean child revisions and must not cite mutable `target/` receipts as promoted evidence.

## Non-claims

full CTF correctness remains a non-claim. production load remains a non-claim. Public-server safety remains a non-claim. unbounded soak remains a non-claim. Broad Minecraft compatibility remains a non-claim. Unrelated CTF rule rows remain non-claims.
