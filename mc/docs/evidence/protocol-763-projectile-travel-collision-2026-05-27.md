# Protocol-763 projectile travel/collision proof — 2026-05-27

## Scope

This checkpoint drains the projectile travel/collision Cairn by defining the bounded projectile states already promoted by live receipts and by keeping unobserved projectile physics explicit.

It promotes two protocol-763 rows: one projectile use/loadout rail and one pinned projectile damage-attribution rail. It does not prove projectile travel/collision simulation remains a non-claim, full projectile physics remains a non-claim, all projectile weapons, exact vanilla projectile physics, or production PvP readiness.

## Promoted matrix rows

| Seam | weapon representative | target type | projectile state sequence | Receipt | BLAKE3 |
| --- | --- | --- | --- | --- | --- |
| Projectile use/loadout rail | bow_like_projectile_probe | remote_player_setup_no_damage_claim | client_projectile_use → client_projectile_swing → server_projectile_loadout | `docs/evidence/protocol-763-roi-03-projectile-hit-2026-05-27.receipt.json` | `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b` |
| Projectile damage attribution | bow_like_projectile_probe | remote_player_victim | client_projectile_use → client_projectile_swing → server_projectile_use → server_projectile_hit → victim_client_damage_update | `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json` | `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529` |

## Positive validation

`tools/check_projectile_travel_collision.py` requires:

- receipts are `mode=run`, `dry_run=false`, `status=pass`;
- server protocol is `763`;
- required client/server projectile milestones are present;
- missing milestones and forbidden matches are empty;
- projectile damage attribution has attacker `compatbotb`, victim `compatbota`, no missing causality steps, and no order violations;
- acceptance matrix and current bundle contain the row digests;
- scoped non-claims include full projectile physics, all projectile weapons, broad Minecraft compatibility, production load, and full CTF correctness.

## Negative fixtures

`tools/check_projectile_travel_collision.py --self-test` rejects:

- missing projectile client evidence;
- server protocol mismatch;
- wrong target/victim evidence;
- missing server projectile hit/health-delta step;
- out-of-order causality steps;
- wrong_weapon_projectile_accepted;
- missing_projectile_use_accepted;
- wrong_attacker_projectile_accepted;
- wrong_target_projectile_accepted;
- out_of_order_projectile_accepted;
- missing non-claim/model text.

## Promotion gate

Only the two receipt-backed rows above are promoted. The second row claims bounded server projectile hit/damage attribution, not continuous projectile travel simulation. Future travel/collision simulation rows must record projectile entity/sequence IDs, path or tick evidence, target collision, client-visible observation, run log, BLAKE3 manifest, and positive/negative fixtures before promotion.

## Decision

- Question: Can existing projectile receipts be promoted as bounded projectile state rows without implying full physics correctness?
- Inspected evidence: projectile use/loadout receipt, pinned projectile damage attribution receipt, current acceptance matrix, current bundle, and checker fixtures.
- Decision: Yes. Promote the two bounded rows listed here; projectile travel/collision simulation remains a non-claim and full projectile physics remains a non-claim.
- Owner: agent.
- Next action: add separate rows for projectile entity travel, collision tick/path evidence, and weapon variants only with live receipts and manifests.

## Non-claims

No continuous projectile travel simulation, broad collision physics, all projectile weapons, bow/crossbow/trident variants, exact vanilla projectile physics, environmental collision, projectile drop/gravity timing, production PvP readiness, full combat correctness, full CTF correctness, or broad protocol coverage claim is made.
