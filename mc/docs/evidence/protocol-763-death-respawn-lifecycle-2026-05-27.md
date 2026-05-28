# Protocol-763 death/respawn lifecycle proof — 2026-05-27

## Scope

This checkpoint drains the death/respawn lifecycle Cairn by turning the maintained flag-carrier death/return receipt into an explicit lifecycle model with deterministic positive and negative validation fixtures.

It proves one bounded Valence CTF lifecycle row: flag carrier alive → lethal damage observed → dead → respawn requested → respawned → post_respawn_playable enough to observe restored health and continued server state coherence; full death/respawn lifecycle remains a non-claim.

## Lifecycle states

| State | Entry evidence | Exit evidence | Forbidden evidence | Claim status |
| --- | --- | --- | --- | --- |
| `alive` | protocol 763 detected, joined game, rendered, team selected | flag pickup or remote-player combat setup | decode/runtime forbidden patterns | Covered only for maintained two-client CTF rail. |
| `lethal_damage_observed` | client `combat_attack_sent`; Valence flag carrier damage/death path | client `combat_death_observed`; server `server_flag_carrier_death` | score/capture milestones during death edge | Covered for flag-carrier death edge. |
| `dead` | client death health observation | client `respawn_request_sent` | duplicate death before respawn request | Covered only for the observed bounded row. |
| `respawn_requested` | client respawn request | client `respawn_health_restored` | missing respawn or out-of-order restoration | Covered only for the observed bounded row. |
| `respawned` | restored health | server flag returned/reset | unexpected score/capture | Covered for flag return/reset correlation. |
| `post_respawn_playable` | no missing scenario/server milestones and no forbidden matches | future seam-specific evidence | full recovery/inventory/reconnect claims | Non-claim beyond this row. |

## Positive row

Promoted bounded row:

- Seam: `Flag-carrier death/return`.
- Maintained command: `nix run .#mc-compat-valence-ctf-flag-carrier-death-return`.
- Reviewable receipt: `docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json`.
- BLAKE3: `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4`.
- Evidence index: `docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.md`.

Required client transition order:

1. `protocol_detected`
2. `join_game`
3. `render_tick`
4. `team_blue`
5. `flag_pickup`
6. `remote_player_spawn`
7. `combat_attack_sent`
8. `combat_death_observed`
9. `respawn_request_sent`
10. `respawn_health_restored`

Required server transition order:

1. `server_client_a_seen`
2. `server_client_b_seen`
3. `server_flag_pickup`
4. `server_flag_carrier_death`
5. `server_flag_return`

## Negative fixtures

`tools/check_death_respawn_lifecycle.py --self-test` rejects these malformed fixtures:

- missing respawn: `respawn_health_restored` absent;
- duplicate death: repeated `combat_death_observed`;
- out-of-order lifecycle: `respawn_request_sent` before `combat_death_observed`;
- forbidden score/capture: score or capture milestone appears during the death/return row;
- protocol mismatch: server protocol is not `763`;
- missing model text: lifecycle evidence doc no longer names required states/non-claims.

## Promotion gate

A lifecycle row is promotable only when all of these are true:

- receipt is `mode=run`, `dry_run=false`, `status=pass`;
- server protocol is `763`;
- scenario and server milestones are complete;
- scenario and server forbidden matches are empty;
- lifecycle transitions are present once and in order;
- receipt carries scoped non-claims for full CTF correctness, broad Minecraft compatibility, unbounded soak, and production load;
- acceptance matrix and current bundle cite the receipt digest;
- BLAKE3 manifests cover the reviewable receipt and gate logs.

## Verification

- Lifecycle checker: `python3 tools/check_death_respawn_lifecycle.py --self-test` and `python3 tools/check_death_respawn_lifecycle.py`.
- Load/protocol review-gap checker: `python3 tools/check_load_network_safety.py --self-test` and `python3 tools/check_load_network_safety.py`.
- Gate log: `docs/evidence/protocol-763-death-respawn-lifecycle-gate-2026-05-27.run.log`.
- BLAKE3 manifest: `docs/evidence/protocol-763-death-respawn-lifecycle-gate-2026-05-27.b3`.

## Decision

- Question: Can death/respawn lifecycle claims be promoted from the maintained flag-carrier death/return receipt without implying full lifecycle correctness?
- Inspected evidence: `docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json`, `docs/evidence/valence-ctf-flag-carrier-death-return.md`, acceptance matrix row, current evidence bundle row, and checker positive/negative fixtures.
- Decision: Yes, one bounded flag-carrier death/return lifecycle row is promoted. Full death/respawn lifecycle remains a non-claim until additional ordinary death, repeated death, reconnect-during-death, inventory reset, and invalid-respawn rows have live receipts.
- Owner: agent.
- Next action: add separate lifecycle rows only when each has a protocol-763 live receipt, run log, BLAKE3 manifest, and updated matrix/current-bundle indexes.

## Non-claims

No full death/respawn lifecycle, death-message semantics, ordinary-death breadth, repeated-death safety, invalid-respawn timing, inventory reset correctness, reconnect-during-death semantics, full CTF correctness, production load, or broad Minecraft compatibility claim is made.
