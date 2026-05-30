# Protocol 763 CTF score limit / win condition contract

Scenario: `ctf-score-limit-win-condition`.

## Bounded claim

One owned-local Valence CTF match starts one RED capture below configured score limit, then a single RED-team capture reaches the configured score limit and emits one win/end-state milestone.

Required normalized metrics:

- `score_limit=2`
- `red_score_before=1`
- `blue_score_before=0`
- `final_capture_team=Red`
- `carried_flag=Blue`
- `red_score_after=2`
- `blue_score_after=0`
- `winning_team=Red`
- `end_state=winner_declared`
- `win_emissions=1`
- `duplicate_win=false`
- `post_win_score_delta=0`

Required client milestone:

- `ctf_score_limit_win_seen score_limit=2 winning_team=red red_score=2 blue_score=0 end_state=winner_declared duplicate_win=false`

Required server milestones:

- `score_limit_pre_state score_limit=2 red_score=1 blue_score=0 next_capture_team=Red outcome=one_capture_before_win`
- `score_limit_final_capture username=compatbot capture_team=Red carried_flag=Blue score_limit=2 red_score_before=1 blue_score_before=0 red_score_after=2 blue_score_after=0`
- `score_limit_win_condition username=compatbot winning_team=Red score_limit=2 red_score=2 blue_score=0 end_state=winner_declared win_emissions=1 duplicate_win=false post_win_score_delta=0`

Forbidden telemetry:

- `score_limit_duplicate_win`
- `score_limit_post_win_score_mutation`
- `RED: 3`
- `BLUE: 1`

## Non-claims

Full CTF correctness remains a non-claim. full CTF correctness remains a non-claim. All score limits, all match settings, overtime/tiebreakers, scoreboard UI parity, all scoring races, production readiness, adversarial safety, and broad Minecraft compatibility remain non-claims.
