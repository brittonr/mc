# Protocol-763 CTF simultaneous pickup/capture race — 2026-06-01

Seam: Simultaneous pickup/capture race.

## Scope

This checkpoint promotes one bounded two-client same-flag race against owned-local Valence CTF protocol 763. Both clients join RED and contend for the BLUE flag. The deterministic oracle records one accepted pickup/capture transition and one rejected duplicate pickup in a 40-client-tick race window.

## Evidence

| Artifact | Path |
| --- | --- |
| Receipt | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.receipt.json` |
| Run log | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.run.log` |
| Client A log | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.client-a.log` |
| Client B log | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.client-b.log` |
| Server log | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.server.log` |
| Typed events | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.typed-events.log` |
| Normalized record | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.record` |
| Row contract kv | `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.kv` |

Receipt BLAKE3: `cc0b21579b8c5d99aa0d2bab04cc1ec3a34ecbdfceee2edc1ba0e497c11f977f`.

Child revisions:

- Valence: `979753084760ba646a5a2eb12574bc61ab112184`.
- Stevenarella: `d9caec597041b3443d894701591752d23772e5ae`.

## Normalized metrics

| Metric | Value |
| --- | --- |
| `client_identities` | `compatbota`, `compatbotb` |
| `team_roles` | both RED same-flag race contenders |
| `action_timestamps` | accepted `2026-06-02T04:36:31.196634Z`; rejected `2026-06-02T04:36:31.797059Z`; final `2026-06-02T04:36:32.097520Z` |
| `ordered_milestones` | accepted transition → rejected transition → final state |
| `accepted_transition` | `compatbotb` RED picks/captures BLUE flag |
| `rejected_transition` | `compatbota` duplicate BLUE flag pickup rejected with `reason=flag_already_held` |
| `final_flag_state` | BLUE flag at base; RED flag at base |
| `final_score` | RED `1`, BLUE `0` |
| `race_window_bounds` | 40 client ticks, owned-local loopback, no latency/jitter claim |

## Decision

The row can be promoted as a bounded race-window slice. The pass receipt observes client milestones `ctf_race_client_count`, `flag_pickup`, `flag_capture`, and `score_red_1`; server milestones `server_ctf_race_accepted_transition`, `server_ctf_race_rejected_transition`, and `server_ctf_race_final_state`; no `ctf_race_double_accept`, `RED: 2`, or `BLUE: 1` forbidden pattern.

## Non-claims

No all CTF concurrency, all race conditions, latency tolerance, network adversarial safety, unbounded concurrency, all flag pickup/capture permutations, all score races, full CTF correctness, production readiness, broad Minecraft compatibility, or vanilla/reference parity claim is made.
