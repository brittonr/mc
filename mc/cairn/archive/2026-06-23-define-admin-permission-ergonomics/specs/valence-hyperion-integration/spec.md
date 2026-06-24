# valence-hyperion-integration Change Spec: Admin permission ergonomics

## Requirements

### Requirement: Admin permission scope

r[valence_hyperion_integration.admin_permissions.scope] The integration MUST compare Hyperion permission/admin behavior with Valence command scopes before adding permission ergonomics.

#### Scenario: Command-system ownership is clear

r[valence_hyperion_integration.admin_permissions.scope.ownership]
- GIVEN admin permission work is selected
- WHEN reviewers inspect the scope notes
- THEN they identify the Hyperion concepts referenced, the Valence command/scopes surfaces affected, and the decision not to introduce a parallel command framework.

### Requirement: Pure permission evaluator

r[valence_hyperion_integration.admin_permissions.evaluator] Permission decisions MUST be implemented as pure deterministic evaluation over command metadata, player roles/scopes, and explicit context.

#### Scenario: Denied command is deterministic

r[valence_hyperion_integration.admin_permissions.evaluator.denied]
- GIVEN a player lacks the required role or scope for a command
- WHEN the evaluator checks that command
- THEN it returns the documented denial result
- AND it does not inspect ECS world state, storage, clocks, or network state.

### Requirement: Command integration

r[valence_hyperion_integration.admin_permissions.command_integration] Optional permission ergonomics SHOULD integrate with Valence's existing command system for command visibility, execution denial, and command-tree refresh.

#### Scenario: Role change refreshes command visibility

r[valence_hyperion_integration.admin_permissions.command_integration.refresh]
- GIVEN a player's command permission context changes
- WHEN the command integration observes the change
- THEN the player's command tree visibility is refreshed according to the evaluator result
- AND commands outside the player's scope are hidden or denied according to the documented policy.

### Requirement: Permission storage boundary

r[valence_hyperion_integration.admin_permissions.storage] Permission persistence MAY be provided, but storage MUST be optional and separated from pure permission evaluation.

#### Scenario: Missing storage row uses documented default

r[valence_hyperion_integration.admin_permissions.storage.missing]
- GIVEN persistence is enabled and a player has no permission row
- WHEN the storage adapter loads permission context
- THEN it returns the documented default or diagnostic
- AND the evaluator receives an explicit context value.

### Requirement: Admin permission tests

r[valence_hyperion_integration.admin_permissions.tests] Admin permission work MUST include positive and negative tests for allowed commands, denied commands, missing permission data, stale command trees, invalid storage rows, and plugin-disabled behavior.

#### Scenario: Plugin disabled preserves commands

r[valence_hyperion_integration.admin_permissions.tests.disabled]
- GIVEN the optional admin permission plugin is disabled
- WHEN existing Valence command tests run
- THEN command registration, parsing, execution, and suggestions preserve their previous behavior.

### Requirement: Admin permission validation

r[valence_hyperion_integration.admin_permissions.validation] Admin permission work MUST record permission tests, command integration tests, plugin-off regressions, and Cairn gates before archive.

#### Scenario: Admin permission closeout is reviewable

r[valence_hyperion_integration.admin_permissions.validation.log]
- GIVEN admin permission work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show evaluator tests, negative denial/storage fixtures, command integration tests, plugin-off regressions, docs checks if present, and Cairn validation.
