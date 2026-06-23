# Tasks

- [ ] [serial] Inventory scenario-derived surfaces and classify each as generated, human-authored, or intentionally duplicated. r[mc_compatibility.manifest_surface_expansion.inventory]
- [ ] [depends:inventory] Extend generator fixtures to cover newly generated surface classes with positive cases and negative cases for duplicate names, unsafe paths, missing fields, unsupported states, and stale output. r[mc_compatibility.manifest_surface_expansion.generator]
- [ ] [depends:generator] Generate bounded app/check wrapper metadata or marked blocks from manifest data without changing runner runtime Nickel behavior. r[mc_compatibility.manifest_surface_expansion.wrappers]
- [ ] [depends:wrappers] Refresh README/index generated blocks only where machine ownership is explicit and human evidence interpretation remains outside markers. r[mc_compatibility.manifest_surface_expansion.docs]
- [ ] [depends:docs] Add or extend freshness checks so stale generated artifacts fail before evidence promotion. r[mc_compatibility.manifest_surface_expansion.freshness]
- [ ] [depends:freshness] Run manifest checks, generator tests, selected dry-run wrappers, maintained dry-run aggregate, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.manifest_surface_expansion.validation]
