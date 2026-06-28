# Tasks

- [ ] [serial] Inventory scenario facts currently split across specs, behavior matches, typed-event graph edges, env wiring, and receipt selectors. r[mc_compatibility.runner_modularity.scenario_metadata]
- [ ] [depends:scenario_metadata] Extend scenario metadata or generated surfaces for run strategy, env intents, typed-event edges, evidence selectors, and non-claims where they can be declarative. r[mc_compatibility.runner_modularity.scenario_metadata]
- [ ] [depends:scenario_metadata] Replace representative match-heavy consumers with metadata lookups while preserving specialized explicit hooks for non-declarative behavior. r[mc_compatibility.runner_modularity.scenario_extension_path]
- [ ] [depends:scenario_extension_path] Add positive metadata fixtures for representative single-client, reconnect, multi-client, projectile, inventory, survival, CTF, and MCP scenarios. r[mc_compatibility.runner_modularity.scenario_metadata_positive_tests]
- [ ] [depends:scenario_metadata_positive_tests] Add negative metadata fixtures for missing run strategy, unknown env intent, invalid graph edge, duplicate alias, and unsupported handler references. r[mc_compatibility.runner_modularity.scenario_metadata_negative_tests]
- [ ] [depends:scenario_metadata_negative_tests] Run scenario-spec validation, generated-surface freshness checks, runner tests, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_modularity.scenario_metadata_validation]
