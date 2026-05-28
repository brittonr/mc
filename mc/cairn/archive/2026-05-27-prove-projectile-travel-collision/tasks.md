# Tasks

- [x] [serial] Define projectile travel/collision states and variant matrix. r[mc_compatibility.prove_projectile_travel_collision.projectile_matrix]
- [x] [serial] Add positive bounded projectile scenarios for currently promoted states. r[mc_compatibility.prove_projectile_travel_collision.positive_projectile_scenarios]
- [x] [serial] Add negative mismatched/out-of-order projectile scenarios. r[mc_compatibility.prove_projectile_travel_collision.negative_projectile_scenarios]
- [x] [serial] Promote only receipt-backed projectile physics rows. r[mc_compatibility.prove_projectile_travel_collision.projectile_promotion_gate]

## Progress

- Projectile matrix is documented in `docs/evidence/protocol-763-projectile-travel-collision-2026-05-27.md`.
- Positive validation promotes two protocol-763 live receipt-backed rows: projectile use/loadout and pinned projectile damage attribution; continuous travel, miss, obstacle collision, and variant breadth remain non-claims.
- Negative fixtures in `tools/check_projectile_travel_collision.py --self-test` reject missing projectile evidence, protocol mismatch, wrong target/victim evidence, missing server hit step, out-of-order causality, wrong-weapon acceptance, and missing non-claim/model text.
- `tools/check_projectile_travel_collision.py` promotes only receipt-backed projectile rows and keeps continuous projectile travel/collision simulation, all weapons, and full projectile physics as non-claims.
