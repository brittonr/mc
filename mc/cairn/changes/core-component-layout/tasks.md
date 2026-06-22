# Tasks

- [ ] [serial] Define the final role-based source layout and transition map for core client, core server, and compatibility harness roots. r[mc_compatibility.core_component_layout.boundaries]
  Evidence: TBD
- [ ] [depends:boundaries] Implement a central typed layout resolver with positive fixtures for the final layout and transition layout, plus negative fixtures for missing roots, ambiguous duplicate roots, and nested Git directories under core components. r[mc_compatibility.core_component_layout.resolver]
  Evidence: TBD
- [ ] [depends:resolver] Move Stevenarella and Valence to their core role paths while preserving parent-owned source-tree semantics and path-scoped revision evidence. r[mc_compatibility.core_component_layout.core_moves]
  Evidence: TBD
- [ ] [depends:core_moves] Move or re-home compatibility harness source/config/generated surfaces only where the resolver and generated-surface checks keep wrapper, manifest, and documentation paths current. r[mc_compatibility.core_component_layout.compat_boundary]
  Evidence: TBD
- [ ] [depends:compat_boundary] Update README, AGENTS, architecture notes, and generated scenario index wording to describe clients, servers, and compat harnesses as core project components with documented upstream ancestry. r[mc_compatibility.core_component_layout.docs]
  Evidence: TBD
- [ ] [depends:docs] Run runner layout tests, scenario-manifest checks, generated-surface freshness checks, no-nested-git checks for core components, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.core_component_layout.validation]
  Evidence: TBD
