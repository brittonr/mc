# Protocol-763 projectile travel/collision proof — updated 2026-06-27

## Scope

This checkpoint now promotes one focused protocol-763 projectile travel/collision live row in addition to the earlier projectile use/loadout and pinned damage-attribution rows.

The promoted travel/collision row is `bow_arrow_synthetic_use_to_hit`: Stevenarella emits client-visible bow-use, synthetic spawn-visible, swing, and synthetic travel-observed markers for `compatbota`; the Valence CTF fixture emits bounded synthetic server markers for use, travel sample, collision, and hit for projectile id `arrow_probe_sequence_303` targeting `compatbotb`. These markers are live owned-local fixture evidence, not spawned-arrow entity physics.

## Inventory

Existing covered projectile rows before this change:

- Projectile use/loadout rail: deterministic clients show projectile use/swing milestones and Valence projectile loadout correlation.
- Projectile damage attribution: pinned Valence row shows bounded projectile use/hit attribution and victim damage update causality.

Residual non-claims before row selection were projectile travel/collision simulation, spawned projectile entity physics, exact vanilla projectile parity, all projectile weapons, all collision surfaces, full combat correctness, public-server safety, and production readiness.

Candidate live signals inspected for this change were Stevenarella projectile-use, swing, spawn-visible, and travel-observed probe milestones plus Valence CTF fixture use, travel sample, collision, and hit markers. The selected row is `bow_arrow_synthetic_use_to_hit` because those signals can be correlated by attacker `compatbota`, target `compatbotb`, weapon representative `bow_like_projectile_probe`, and projectile id `arrow_probe_sequence_303` without depending on the pinned projectile-damage-attribution backend revision.

## Promoted matrix rows

| Seam | weapon representative | projectile representative | target/collision identity | ordered observation sequence | Receipt | BLAKE3 |
| --- | --- | --- | --- | --- | --- | --- |
| Projectile use/loadout rail | bow_like_projectile_probe | setup/loadout marker | remote_player_setup_no_damage_claim | client_projectile_use → client_projectile_swing → server_projectile_loadout | `docs/evidence/protocol-763-roi-03-projectile-hit-2026-05-27.receipt.json` | `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b` |
| Projectile travel/collision live rail | bow_like_projectile_probe | arrow_like_probe (`arrow_probe_sequence_303`) | target `compatbotb`, synthetic entity-hit collision | client_use → client_spawn_visible → client_swing → client_travel_observed; server_use → server_travel_sample → server_collision → server_hit | `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.receipt.json` | `addefc27887413ca3445950ee14fc17a6255df40d584e8e32205f489de593c1d` |
| Projectile damage attribution | bow_like_projectile_probe | damage attribution marker | remote_player_victim | client_projectile_use → client_projectile_swing → server_projectile_use → server_projectile_hit → victim_client_damage_update | `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json` | `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529` |

## Positive validation

The 2026-06-27 live rail evidence requires:

- receipt `mode=run`, `dry_run=false`, `status=pass`, protocol `763`, and `triage.suggested_boundary=none`;
- client milestones `projectile_use_sent`, `projectile_spawn_visible`, `projectile_swing_sent`, and `projectile_travel_observed` observed with no missing milestones;
- server milestones `server_projectile_use`, `server_projectile_travel_sample`, `server_projectile_collision`, and `server_projectile_hit` observed with no missing milestones;
- `projectile_travel_collision.passed=true`, empty `missing_steps`, empty `order_violations`, empty `identity_violations`, attacker `compatbota`, target `compatbotb`, and projectile id `arrow_probe_sequence_303`;
- non-claims include `not_full_projectile_physics`, `not_entity_spawn_or_ballistics`, `not_exact_vanilla_projectile_parity`, `not_public_server_safety`, and `not_production_readiness`.

Reviewable evidence:

- Receipt: `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.receipt.json` (`addefc27887413ca3445950ee14fc17a6255df40d584e8e32205f489de593c1d`).
- Comparator output: `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.comparator.kv` (`26ef274040b4649eb9b6530a9bbf2cc185a536ae8c49b0f09ce2068f43ccd846`).
- Run log: `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.run.log` (`c70bc21ab0ad8d2b3d458602754796869cf83a8a95ee330715ce30d13f349f25`).
- Typed-event log: `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.typed-events.log` (`2b3123dc0d6d5cbda6b7803735dbdbbea8c12ec2bf2686df22ffc2e35185a804`).
- Valence/client logs: `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.valence.log`, `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.client-compatbota.log`, and `docs/evidence/projectile-travel-collision-live-rail-2026-06-27.client-compatbotb.log`.

## Negative fixtures

Runner tests `projectile_travel_collision_fails_closed_for_bad_evidence` and `projectile_travel_collision_receipt_preserves_non_claims` reject:

- missing travel sample;
- missing collision result;
- hit/collision without prior travel;
- wrong target;
- wrong weapon;
- unordered server use/travel/collision/hit sequence;
- ambiguous projectile identity;
- overbroad exact-vanilla/full-physics parity claim markers;
- receipts that omit the scoped projectile non-claims.

## Promotion gate

Only the configured `bow_arrow_synthetic_use_to_hit` row is promoted for travel/collision. It claims a bounded owned-local fixture observation of ordered synthetic projectile markers, not continuous projectile entity simulation.

Future full projectile physics rows must record projectile entity spawn, tick/path evidence, collision surface identity, client-visible entity motion, reference/backend comparison, run log, BLAKE3 manifest, and positive/negative fixtures before promotion.

## Decision

- Question: Can a focused live row promote bounded projectile travel/collision markers without implying full projectile physics correctness?
- Inspected evidence: previous projectile use/loadout and pinned damage-attribution receipts, new live receipt/logs/comparator output, runner positive/negative fixtures, current acceptance/current-bundle docs, and generated scenario surfaces.
- Decision: Yes. Promote the single bounded `bow_arrow_synthetic_use_to_hit` row; exact vanilla projectile physics, spawned-arrow entity motion, all projectile weapons, all collision surfaces, and production PvP readiness remain non-claims.
- Owner: agent.
- Next action: add spawned-entity/path/physics rows only with a reference oracle and live entity-motion evidence.

## Non-claims

No continuous projectile travel simulation, spawned projectile entity physics, broad collision physics, all projectile weapons, bow/crossbow/trident variants, exact vanilla projectile physics, environmental collision, projectile drop/gravity timing, production PvP readiness, public-server safety, full combat correctness, full CTF correctness, or broad protocol coverage claim is made.
