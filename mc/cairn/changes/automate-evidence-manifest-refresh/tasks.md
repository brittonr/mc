# Tasks

- [ ] [serial] Define the manifest refresh contract, CLI modes, scoped path policy, and non-claiming evidence workflow. r[mc_compatibility.evidence_manifest_refresh.contract]
- [ ] [depends:contract] Implement a pure manifest parser/planner that reports current, stale, malformed, missing-file, and outside-root rows without filesystem mutation. r[mc_compatibility.evidence_manifest_refresh.planner]
- [ ] [depends:planner] Add the imperative refresh/check shell that applies deterministic fixpoint rewrites only when refresh mode is requested. r[mc_compatibility.evidence_manifest_refresh.refresh_mode]
- [ ] [depends:refresh_mode] Wire the helper into flake app/check surfaces and document the preferred Cairn evidence workflow. r[mc_compatibility.evidence_manifest_refresh.integration]
- [ ] [depends:integration] Add positive tests for unchanged and stale manifests plus negative tests for malformed rows, missing files, outside-root paths, and non-converging fixture graphs. r[mc_compatibility.evidence_manifest_refresh.tests]
- [ ] [depends:tests] Run focused helper tests, existing evidence-manifest/task-evidence checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.evidence_manifest_refresh.validation]
