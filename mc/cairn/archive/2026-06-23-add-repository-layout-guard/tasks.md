# Tasks

- [x] [serial] Define the layout guard scope, diagnostics, waiver model, and non-claims. r[repository_layout.layout_guard.contract]
  Evidence: docs/evidence/add-repository-layout-guard-2026-06-23-validation.run.log; docs/evidence/add-repository-layout-guard-2026-06-23-validation.b3
- [x] [depends:contract] Implement a pure guard core over an in-memory tree/registry model and a thin filesystem shell. r[repository_layout.layout_guard.core]
  Evidence: docs/evidence/add-repository-layout-guard-2026-06-23-validation.run.log; docs/evidence/add-repository-layout-guard-2026-06-23-validation.b3
- [x] [depends:core] Add positive and negative fixtures for valid layout, undocumented root dir, surprise nested Git, root transient artifact, missing subtree docs, and generated marker drift. r[repository_layout.layout_guard.fixtures]
  Evidence: docs/evidence/add-repository-layout-guard-2026-06-23-validation.run.log; docs/evidence/add-repository-layout-guard-2026-06-23-validation.b3
- [x] [depends:fixtures] Wire the guard into a focused flake check or documented command, initially advisory where active cleanup Cairns own known findings. r[repository_layout.layout_guard.wiring]
  Evidence: docs/evidence/add-repository-layout-guard-2026-06-23-validation.run.log; docs/evidence/add-repository-layout-guard-2026-06-23-validation.b3
- [x] [depends:wiring] Connect guard inputs to the component registry and artifact-boundary rules when those exist, without duplicating source-of-truth data. r[repository_layout.layout_guard.registry_integration]
  Evidence: docs/evidence/add-repository-layout-guard-2026-06-23-validation.run.log; docs/evidence/add-repository-layout-guard-2026-06-23-validation.b3
- [x] [depends:registry_integration] Run guard tests, focused flake check, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.layout_guard.validation]
  Evidence: docs/evidence/add-repository-layout-guard-2026-06-23-validation.run.log; docs/evidence/add-repository-layout-guard-2026-06-23-validation.b3
