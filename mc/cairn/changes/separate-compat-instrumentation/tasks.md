# Tasks

- [ ] [serial] Inventory client probes, capture hooks, MCP control surfaces, server fixture milestones, and scenario-specific toggles. r[mc_compatibility.compat_instrumentation_boundary.inventory]
- [ ] [depends:inventory] Define explicit module/feature/entrypoint boundaries for compat instrumentation in client and server components. r[mc_compatibility.compat_instrumentation_boundary.contract]
- [ ] [depends:contract] Move or gate an initial instrumentation family while preserving typed-event and milestone vocabulary. r[mc_compatibility.compat_instrumentation_boundary.migration]
- [ ] [depends:migration] Add positive tests for enabled instrumentation and negative tests proving disabled/core paths do not emit harness-only behavior. r[mc_compatibility.compat_instrumentation_boundary.tests]
- [ ] [depends:tests] Update docs/AGENTS and evidence checker expectations if instrumentation paths or features change. r[mc_compatibility.compat_instrumentation_boundary.docs]
- [ ] [depends:docs] Run affected component tests, typed-event fixtures, selected dry-runs/live checks if required, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.compat_instrumentation_boundary.validation]
