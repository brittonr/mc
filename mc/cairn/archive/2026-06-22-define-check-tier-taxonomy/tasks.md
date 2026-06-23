# Tasks

- [x] [serial] Define the check-tier names, scope, required evidence, and non-claims. r[repository_layout.check_tiers.taxonomy]
  Evidence: docs/evidence/check-tier-taxonomy-2026-06-22.run.log; docs/evidence/check-tier-taxonomy-2026-06-22.b3
- [x] [depends:taxonomy] Inventory existing flake checks, apps, manual commands, component tests, evidence gates, and Cairn gates into the tier map. r[repository_layout.check_tiers.inventory]
  Evidence: docs/evidence/check-tier-taxonomy-2026-06-22.run.log; docs/evidence/check-tier-taxonomy-2026-06-22.b3
- [x] [depends:inventory] Add or update docs that tell developers which tier to run for docs-only, generated, runner, component, evidence, and archive changes. r[repository_layout.check_tiers.docs]
  Evidence: docs/evidence/check-tier-taxonomy-2026-06-22.run.log; docs/evidence/check-tier-taxonomy-2026-06-22.b3
- [x] [depends:docs] Add optional wrapper outputs or generated indexes for common tier entrypoints while preserving existing check names. r[repository_layout.check_tiers.entrypoints]
  Evidence: No wrapper output was added; `docs/check-tiers.md` is the checked index and preserves existing check names. See docs/evidence/check-tier-taxonomy-2026-06-22.run.log; docs/evidence/check-tier-taxonomy-2026-06-22.b3
- [x] [depends:entrypoints] Add dry-run/evaluation checks so tier docs and wrapper inventories stay fresh. r[repository_layout.check_tiers.freshness]
  Evidence: docs/evidence/check-tier-taxonomy-2026-06-22.run.log; docs/evidence/check-tier-taxonomy-2026-06-22.b3
- [x] [depends:freshness] Run tier wrapper dry-runs/evaluation, docs checks, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.check_tiers.validation]
  Evidence: docs/evidence/check-tier-taxonomy-2026-06-22.run.log; docs/evidence/check-tier-taxonomy-2026-06-22.b3
