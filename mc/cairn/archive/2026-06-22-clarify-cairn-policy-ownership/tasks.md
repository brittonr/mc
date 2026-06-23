# Tasks

- [x] [serial] Document current `cairn-policy/` owner, generation command, schema version constraints, and why it is top-level. r[repository_layout.cairn_policy_ownership.contract]
  Evidence: `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.run.log`; `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.b3`.
- [x] [depends:contract] Inventory all code, flake, docs, and validation references to `cairn-policy/` paths. r[repository_layout.cairn_policy_ownership.references]
  Evidence: `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.run.log`; `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.b3`.
- [x] [depends:references] Decide whether to keep the top-level path or migrate under `cairn/`, with compatibility evidence for any move. r[repository_layout.cairn_policy_ownership.path_decision]
  Evidence: `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.run.log`; `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.b3`.
- [x] [depends:path_decision] Add or update freshness/schema checks for generated Cairn policy artifacts. r[repository_layout.cairn_policy_ownership.freshness]
  Evidence: `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.run.log`; `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.b3`.
- [x] [depends:freshness] Update README/architecture/agent notes with policy ownership and regeneration guidance. r[repository_layout.cairn_policy_ownership.docs]
  Evidence: `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.run.log`; `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.b3`.
- [x] [depends:docs] Run Cairn validation, policy freshness/schema checks, Cairn gates, and any path-compatibility checks with reviewable logs. r[repository_layout.cairn_policy_ownership.validation]
  Evidence: `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.run.log`; `docs/evidence/clarify-cairn-policy-ownership-validation-2026-06-22.b3`; post-archive `docs/evidence/clarify-cairn-policy-ownership-post-archive-2026-06-22.run.log`; `docs/evidence/clarify-cairn-policy-ownership-post-archive-2026-06-22.b3`.
