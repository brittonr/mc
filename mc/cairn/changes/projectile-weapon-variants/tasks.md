# Tasks

- [x] [serial] Define the bounded `projectile weapon variants` evidence contract and normalized metric names. r[mc_compatibility.projectile_weapon_variants.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `projectile weapon variants`. r[mc_compatibility.projectile_weapon_variants.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [ ] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.projectile_weapon_variants.evidence_standard]
- [ ] [depends:evidence_standard] Add `projectile-weapon-variants` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.projectile_weapon_variants.rail]
  - Detail: Define projectile weapon matrix schema.
  - Detail: Add first weapon-variant fixtures.
  - Detail: Add checker positives/negatives.
  - Detail: Promote only listed weapon rows.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.projectile_weapon_variants.artifacts]
- [ ] [depends:artifacts] Promote only the `projectile weapon variants` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.projectile_weapon_variants.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.projectile_weapon_variants.validation]
