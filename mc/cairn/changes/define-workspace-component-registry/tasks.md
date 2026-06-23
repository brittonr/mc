# Tasks

- [ ] [serial] Define the component registry contract with typed fields for path, role, owner, VCS boundary, commands, gate participation, and evidence policy. r[repository_layout.component_registry.contract]
- [ ] [depends:contract] Encode current workspace components and nested-repo exceptions without moving files. r[repository_layout.component_registry.current_inventory]
- [ ] [depends:current_inventory] Add positive and negative fixtures for valid rows, missing fields, duplicate roles, unsafe paths, and undocumented nested Git boundaries. r[repository_layout.component_registry.fixtures]
- [ ] [depends:fixtures] Generate or validate layout docs/checks from the registry while keeping runtime Nickel-free. r[repository_layout.component_registry.generated_surfaces]
- [ ] [depends:generated_surfaces] Wire registry validation into focused checks without changing default compatibility behavior. r[repository_layout.component_registry.guard]
- [ ] [depends:guard] Run registry checks, generated freshness checks if added, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.component_registry.validation]
