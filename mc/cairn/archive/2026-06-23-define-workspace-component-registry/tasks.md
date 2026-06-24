# Tasks

- [x] [serial] Define the component registry contract with typed fields for path, role, owner, VCS boundary, commands, gate participation, and evidence policy. r[repository_layout.component_registry.contract]
  - Evidence: `docs/evidence/define-workspace-component-registry-nickel-export-2026-06-23.run.log` records contract-enforced Nickel export; BLAKE3 manifest `docs/evidence/define-workspace-component-registry-validation-2026-06-23.b3` covers it.
- [x] [depends:contract] Encode current workspace components and nested-repo exceptions without moving files. r[repository_layout.component_registry.current_inventory]
  - Evidence: `docs/evidence/define-workspace-component-registry-focused-checks-2026-06-23.run.log` records the current registry/root check; BLAKE3 manifest `docs/evidence/define-workspace-component-registry-validation-2026-06-23.b3` covers it.
- [x] [depends:current_inventory] Add positive and negative fixtures for valid rows, missing fields, duplicate roles, unsafe paths, and undocumented nested Git boundaries. r[repository_layout.component_registry.fixtures]
  - Evidence: `docs/evidence/define-workspace-component-registry-negative-fixtures-2026-06-23.run.log` records fail-closed fixture behavior; BLAKE3 manifest `docs/evidence/define-workspace-component-registry-validation-2026-06-23.b3` covers it.
- [x] [depends:fixtures] Generate or validate layout docs/checks from the registry while keeping runtime Nickel-free. r[repository_layout.component_registry.generated_surfaces]
  - Evidence: `docs/evidence/define-workspace-component-registry-flake-layout-boundaries-2026-06-23.run.log` records the focused registry/layout flake check; BLAKE3 manifest `docs/evidence/define-workspace-component-registry-validation-2026-06-23.b3` covers it.
- [x] [depends:generated_surfaces] Wire registry validation into focused checks without changing default compatibility behavior. r[repository_layout.component_registry.guard]
  - Evidence: `docs/evidence/define-workspace-component-registry-flake-layout-boundaries-2026-06-23.run.log` records the focused check wiring; BLAKE3 manifest `docs/evidence/define-workspace-component-registry-validation-2026-06-23.b3` covers it.
- [x] [depends:guard] Run registry checks, generated freshness checks if added, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.component_registry.validation]
  - Evidence: `docs/evidence/define-workspace-component-registry-post-implementation-gates-2026-06-23.run.log` records Cairn gates and validation; BLAKE3 manifest `docs/evidence/define-workspace-component-registry-validation-2026-06-23.b3` covers it.
