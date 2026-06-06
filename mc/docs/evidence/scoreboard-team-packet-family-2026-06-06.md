# Scoreboard/team packet-family bounded evidence — 2026-06-06

## Scope

This checkpoint promotes one bounded packet-family row: `play/clientbound/0x5a TeamS2CPacket -> Teams_VarInt` in the owned-local `ctf-spawn-team-balance-reset` context. The row is a packet-family evidence claim, separate from the gameplay claim already covered by `Spawn/team balance/resource reset`.

## Evidence inputs

- Source live receipt: `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json`.
- Normalized packet-family receipt: `docs/evidence/scoreboard-team-packet-family-2026-06-06.receipt.json`.
- Normalized checker input: `docs/evidence/scoreboard-team-packet-family-2026-06-06.kv`.
- Checker: `tools/check_scoreboard_team_packet_family.rs`.
- Checker output: `docs/evidence/scoreboard-team-packet-family-checker-2026-06-06.run.log` (`exit_status=0`).
- Focused validation output: `docs/evidence/scoreboard-team-packet-family-validation-2026-06-06.run.log` (`exit_status=0`).
- Cairn gate output: `docs/evidence/scoreboard-team-packet-family-cairn-gates-2026-06-06.run.log` (`exit_status=0`).
- Packet inventory row: `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv` records `TeamS2CPacket` as `reviewed_override_no_shape_claim`, internal `Teams_VarInt`, `shape_review_missing`, and `scenario_bounded`.

## Normalized metrics

| Metric | Value |
| --- | --- |
| Scenario | `ctf-spawn-team-balance-reset` |
| Packet row | `play/clientbound/0x5a TeamS2CPacket -> Teams_VarInt` |
| RED user | `compatbota` |
| BLUE user | `compatbotb` |
| Team counts | `red=1,blue=1` |
| Client observations | `team_red`, `team_blue` |
| Server correlation | `server_ctf_spawn_red_assignment`, `server_ctf_spawn_blue_assignment`, `server_ctf_spawn_team_balance` |
| Stevenarella revision | `d9caec597041b3443d894701591752d23772e5ae` (`clean`) |
| Valence revision | `f40d6d6d5aeee300bdaeb406c475e2607ac3b6a7` (`clean`) |

## Decision

Promote only the configured `TeamS2CPacket` row for this bounded two-client team assignment/team-balance context. `ScoreboardObjectiveUpdateS2CPacket`, `ScoreboardDisplayS2CPacket`, `ScoreboardPlayerUpdateS2CPacket`, scoreboard UI parity, all scoreboards, all team rules, all objective/display/score variants, full CTF correctness, full protocol-763 compatibility, public-server safety, and production readiness remain non-claims.
