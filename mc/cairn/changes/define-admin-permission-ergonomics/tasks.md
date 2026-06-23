# Tasks

- [ ] [serial] Compare Hyperion permission/admin command behavior with Valence command scopes and record integration scope. r[valence_hyperion_integration.admin_permissions.scope]
- [ ] [depends:scope] Define a pure permission evaluator over command metadata, roles/scopes, and context. r[valence_hyperion_integration.admin_permissions.evaluator]
- [ ] [depends:evaluator] Wire optional command visibility, denial diagnostics, and command-tree refresh into Valence's command system. r[valence_hyperion_integration.admin_permissions.command_integration]
- [ ] [depends:command_integration] Add optional storage adapters or explicit storage non-goals with invalid/missing-row fixtures if persistence exists. r[valence_hyperion_integration.admin_permissions.storage]
- [ ] [depends:storage] Add positive and negative tests for allowed commands, denied commands, missing permissions, stale command trees, invalid storage, and plugin-disabled behavior. r[valence_hyperion_integration.admin_permissions.tests]
- [ ] [depends:tests] Run permission tests, command integration tests, plugin-off regressions, Cairn gates, and Cairn validation. r[valence_hyperion_integration.admin_permissions.validation]
