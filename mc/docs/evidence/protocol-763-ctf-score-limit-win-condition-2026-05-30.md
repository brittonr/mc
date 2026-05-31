# Protocol 763 CTF score limit / win condition

Score limit / win condition

## Summary

This row promotes only the bounded `score limit / win condition` seam for protocol 763 Valence CTF. The fixture starts RED one capture below the configured score limit, has `compatbot` capture the BLUE flag once, and records exactly one win/end-state milestone.

## Evidence

- Maintained command: `nix run .#mc-compat-valence-ctf-score-limit-win-condition`
- Scenario: `ctf-score-limit-win-condition`
- Contract: `docs/evidence/protocol-763-ctf-score-limit-win-condition-contract-2026-05-30.md`
- Receipt: `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.receipt.json`
- Run log: `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.run.log`
- Client log: `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.client.log`
- Server log: `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.server.log`
- Checker record: `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.record`
- BLAKE3 manifest: `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.b3`
- Matrix receipt BLAKE3: `7c0d7805e54d95f2768f0164f1b4e62f59f57d524f3a61c3205eb0d611e89e02`
- Child revisions: Valence `f57a325`, Stevenarella `1ab97d2`

## Required milestones

Client evidence:

- `ctf_score_limit_win_seen score_limit=2 winning_team=red red_score=2 blue_score=0 end_state=winner_declared duplicate_win=false`

Server evidence:

- `score_limit_pre_state score_limit=2 red_score=1 blue_score=0 next_capture_team=Red outcome=one_capture_before_win`
- `score_limit_final_capture username=compatbot capture_team=Red carried_flag=Blue score_limit=2 red_score_before=1 blue_score_before=0 red_score_after=2 blue_score_after=0`
- `score_limit_win_condition username=compatbot winning_team=Red score_limit=2 red_score=2 blue_score=0 end_state=winner_declared win_emissions=1 duplicate_win=false post_win_score_delta=0`

Receipt expectations:

- `status=pass`
- `mode=run`
- `dry_run=false`
- `protocol=763`
- `forbidden_matches=[]`
- `expected_summary_packets=[login_success, play_join_game, flag_pickup, flag_capture, score_limit_win_condition]`
- `claims_correctness=false`
- `claims_semantic_equivalence=false`

## Scoped claim

Bounded near-limit RED capture: RED score starts at `1`, BLUE score starts at `0`, RED captures BLUE flag, RED score reaches configured score limit `2`, winner is `Red`, end state is `winner_declared`, `win_emissions=1`, `duplicate_win=false`, and `post_win_score_delta=0`.

## Explicit non-claims

No all score limits. No all match settings. No overtime/tiebreakers. No scoreboard UI parity. No all scoring races. No full CTF correctness. No adversarial security. No production readiness. No broad Minecraft compatibility. No vanilla/reference parity.
