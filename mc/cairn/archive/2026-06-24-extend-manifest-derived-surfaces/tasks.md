# Tasks

- [x] [serial] Inventory scenario-derived surfaces and classify each as generated, human-authored, or intentionally duplicated. r[mc_compatibility.manifest_surface_expansion.inventory]
  Evidence: docs/evidence/extend-manifest-derived-surfaces-implemented-checks-2026-06-23.run.log; docs/evidence/extend-manifest-derived-surfaces-2026-06-23.b3
- [x] [depends:inventory] Extend generator fixtures to cover newly generated surface classes with positive cases and negative cases for duplicate names, unsafe paths, missing fields, unsupported states, and stale output. r[mc_compatibility.manifest_surface_expansion.generator]
  Evidence: docs/evidence/extend-manifest-derived-surfaces-implemented-checks-2026-06-23.run.log; docs/evidence/extend-manifest-derived-surfaces-2026-06-23.b3
- [x] [depends:generator] Generate bounded app/check wrapper metadata or marked blocks from manifest data without changing runner runtime Nickel behavior. r[mc_compatibility.manifest_surface_expansion.wrappers]
  Evidence: docs/evidence/extend-manifest-derived-surfaces-implemented-checks-2026-06-23.run.log; docs/evidence/extend-manifest-derived-surfaces-2026-06-23.b3
- [x] [depends:wrappers] Refresh README/index generated blocks only where machine ownership is explicit and human evidence interpretation remains outside markers. r[mc_compatibility.manifest_surface_expansion.docs]
  Evidence: docs/evidence/extend-manifest-derived-surfaces-implemented-checks-2026-06-23.run.log; docs/evidence/extend-manifest-derived-surfaces-2026-06-23.b3
- [x] [depends:docs] Add or extend freshness checks so stale generated artifacts fail before evidence promotion. r[mc_compatibility.manifest_surface_expansion.freshness]
  Evidence: docs/evidence/extend-manifest-derived-surfaces-implemented-checks-2026-06-23.run.log; docs/evidence/extend-manifest-derived-surfaces-2026-06-23.b3
- [x] [depends:freshness] Run manifest checks, generator tests, selected dry-run wrappers, maintained dry-run aggregate, Cairn gates, and Cairn validation with reviewable logs. Evidence: docs/evidence/extend-manifest-derived-surfaces-closeout-2026-06-24.run.log (`overall_exit_status=0`); docs/evidence/extend-manifest-derived-surfaces-manifest-refresh-2026-06-24.run.log (`exit_status=0`); docs/evidence/extend-manifest-derived-surfaces-2026-06-23.b3. r[mc_compatibility.manifest_surface_expansion.validation]
