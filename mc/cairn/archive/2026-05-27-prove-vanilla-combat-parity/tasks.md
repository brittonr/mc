# Tasks

- [x] [serial] Select and document the vanilla/reference oracle. r[mc_compatibility.prove_vanilla_combat_parity.reference_oracle]
- [x] [serial] Define parity metrics and tolerances. r[mc_compatibility.prove_vanilla_combat_parity.parity_metrics]
- [x] [serial] Add positive and negative parity comparison fixtures. r[mc_compatibility.prove_vanilla_combat_parity.parity_fixtures]
- [x] [serial] Promote only paired Valence/reference parity receipts. r[mc_compatibility.prove_vanilla_combat_parity.parity_promotion_gate]

## Progress

- Reference-oracle contract and current no-promotion decision are documented in `docs/evidence/protocol-763-vanilla-combat-parity-2026-05-27.md`.
- Parity metrics and tolerances are defined for damage delta, knockback velocity, armor mitigation, and projectile damage.
- Positive and negative fixtures in `tools/check_vanilla_combat_parity.py --self-test` cover within-tolerance comparisons, missing reference, wrong version, out-of-tolerance, and Valence-only evidence.
- `tools/check_vanilla_combat_parity.py` promotes no rows because paired Valence/reference receipts are absent; exact vanilla combat parity remains a non-claim.
