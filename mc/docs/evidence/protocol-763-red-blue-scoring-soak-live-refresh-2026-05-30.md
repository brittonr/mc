# Protocol-763 RED/BLUE scoring soak live refresh — 2026-05-30

## Summary

RED/BLUE scoring soak freshness replaces the historical target-only RED/BLUE scoring soak exception with fresh copied live evidence under `docs/evidence/`.

## Evidence

- Maintained RED command: `nix run .#mc-compat-valence-ctf-600s-soak`
- Maintained BLUE command: `nix run .#mc-compat-valence-ctf-blue-600s-soak`
- Summary receipt: `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.receipt.json`
- RED receipt: `docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.receipt.json`
- RED run log: `docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.run.log`
- RED client log: `docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.client.log`
- RED server log: `docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.server.log`
- BLUE receipt: `docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.receipt.json`
- BLUE run log: `docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.run.log`
- BLUE client log: `docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.client.log`
- BLUE server log: `docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.server.log`
- Checker record: `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.record`
- BLAKE3 manifest: `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.b3`
- Matrix receipt BLAKE3: `349b1b7dc84877dd56dce3344611d04ce86a74413738ebc3fdd2a2f720504bed`
- Child revisions: Valence `f57a325`, Stevenarella `1ab97d2`

## Required milestones

Scenario `multi-client-load-score`:

- client milestones: `multi_client_count`, `protocol_detected`, `join_game`, `render_tick`, `team_red`, `flag_pickup`, `flag_capture`, `score_red_1`
- server milestones: `server_client_a_seen`, `server_client_b_seen`, `server_flag_or_score`
- receipt status: `status=pass`, `mode=run`, `dry_run=false`, `missing_milestones=[]`, `forbidden_matches=[]`

Scenario `blue-flag-score`:

- client milestones: `protocol_detected`, `join_game`, `render_tick`, `team_blue`, `flag_pickup`, `flag_capture`, `score_blue_1`
- server milestones: `server_username_seen`, `server_flag_or_score`
- receipt status: `status=pass`, `mode=run`, `dry_run=false`, `missing_milestones=[]`, `forbidden_matches=[]`

## Scoped claim

fresh live RED/BLUE scoring soak refresh: the maintained RED multi-client score path and mirrored BLUE score path both passed under the bounded `600s` owned-local Valence CTF fixture with copied reviewable receipts and logs.

## Historical exception removed

The old mutable target-only receipt is no longer promoted by the acceptance matrix/current bundle row. It remains only review history through the historical oracle document.

## Explicit non-claims

No full CTF correctness. No production load. No public-server safety. No unbounded soak. No broad Minecraft compatibility. No all scoring races. No all CTF rule rows. No vanilla/reference parity.
