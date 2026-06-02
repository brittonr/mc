# Protocol-763 CTF spawn/team balance/resource reset — 2026-06-01

Seam: Spawn/team balance/resource reset.

## Scope

This checkpoint promotes one bounded two-client join/team-selection/spawn/resource/post-score reset sequence against owned-local Valence CTF protocol 763. `compatbota` is assigned RED, `compatbotb` is assigned BLUE, the server records balanced team counts, and a RED capture emits a coherent post-score resource/reset-state milestone.

## Evidence

| Artifact | Path |
| --- | --- |
| Receipt | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json` |
| Run log | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.run.log` |
| Client A log | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.client-a.log` |
| Client B log | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.client-b.log` |
| Server log | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.server.log` |
| Typed events | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.typed-events.log` |
| Normalized record | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.record` |
| Row contract kv | `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.kv` |

Receipt BLAKE3: `ce4ec8f61c956d5083d6701915a44b9e31c8e0adbfd018b25878774e516f2e6f`.

Child revisions:

- Valence: `f40d6d6d5aeee300bdaeb406c475e2607ac3b6a7`.
- Stevenarella: `d9caec597041b3443d894701591752d23772e5ae`.

## Normalized metrics

| Metric | Value |
| --- | --- |
| `team_counts` | RED `1`, BLUE `1` |
| `selected_teams` | `compatbota:Red`, `compatbotb:Blue` |
| `spawn_coordinates` | RED `-40.0,65.0,0.0`; BLUE `40.0,65.0,0.0` |
| `initial_resources` | slot36 `WoodenSword:1`; RED slot37 `RedWool:64`; BLUE slot37 `BlueWool:64` |
| `post_score_or_post_death_reset_state` | `scoreboard_flags_and_resources_coherent` after RED score |
| `inventory_resource_ids` | `WoodenSword`, `RedWool`, `BlueWool`, `TeamWool` |
| `server_correlation_ids` | `team-select-compatbota`, `team-select-compatbotb`, `score-reset-compatbota` |

## Decision

The row can be promoted as a bounded spawn/team/resource reset slice. The pass receipt observes client milestones `ctf_spawn_team_reset_client_count`, `team_red`, `team_blue`, `flag_capture`, and `score_red_1`; server milestones `server_ctf_spawn_red_assignment`, `server_ctf_spawn_blue_assignment`, `server_ctf_spawn_team_balance`, and `server_ctf_spawn_resource_reset`; no `ctf_spawn_team_imbalance`, `ctf_spawn_resource_stale_after_reset`, `RED: 2`, or `BLUE: 1` forbidden pattern.

## Non-claims

No all team balancing algorithms, all maps, all spawn rules, all resource loadouts, all reset triggers, matchmaking balance, full CTF correctness, production gameplay readiness, broad Minecraft compatibility, or vanilla/reference parity claim is made.
