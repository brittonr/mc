# Tasks

- [ ] [serial] Capture baseline receipt fixtures and identify schema fields, fallback evidence paths, legacy compatibility fields, and writer side effects. r[mc_compatibility.runner_modularity.receipt_model]
- [ ] [depends:receipt_model] Introduce typed receipt input and receipt model structs for scenario receipts without changing rendered schema output. r[mc_compatibility.runner_modularity.receipt_model]
- [ ] [depends:receipt_model] Split pure receipt building, deterministic JSON rendering, and filesystem writing/hashing into separate functions. r[mc_compatibility.runner_modularity.receipt_render_shell]
- [ ] [depends:receipt_render_shell] Add positive tests for passing, failing, dry-run, multi-client, projectile, MCP, typed-event, and failure-bundle receipt model/rendering paths. r[mc_compatibility.runner_modularity.receipt_positive_tests]
- [ ] [depends:receipt_positive_tests] Add negative tests for missing required evidence, duplicate fields, malformed artifact paths, invalid digests, and selected sections without supporting evidence. r[mc_compatibility.runner_modularity.receipt_negative_tests]
- [ ] [depends:receipt_negative_tests] Run receipt tests, receipt validation checks, runner tests, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_modularity.receipt_validation]
