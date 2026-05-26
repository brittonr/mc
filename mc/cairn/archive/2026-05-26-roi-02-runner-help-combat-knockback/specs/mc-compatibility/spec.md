# Delta: Runner help lists combat-knockback scenario

## Requirements

### Requirement: Help Surface

r[mc_compatibility.roi_02_runner_help_combat_knockback.help_surface] The runner usage/help text MUST list every supported maintained scenario, including `combat-knockback`.

#### Scenario: Help Surface evidence is required

r[mc_compatibility.roi_02_runner_help_combat_knockback.help_surface.scenario]
- GIVEN `Runner help lists combat-knockback scenario` is drained
- WHEN the evidence and checks are reviewed
- THEN `help_surface` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Help Test

r[mc_compatibility.roi_02_runner_help_combat_knockback.help_test] The runner tests MUST guard the help text against omitting supported scenarios.

#### Scenario: Help Test evidence is required

r[mc_compatibility.roi_02_runner_help_combat_knockback.help_test.scenario]
- GIVEN `Runner help lists combat-knockback scenario` is drained
- WHEN the evidence and checks are reviewed
- THEN `help_test` is satisfied by a tracked artifact or deterministic check
- AND scoped non-claims are preserved where relevant
