# valence-hyperion-integration Change Spec: Optional anti-cheat statistics plugin

## Requirements

### Requirement: Anti-cheat statistics scope

r[valence_hyperion_integration.anticheat_stats.scope] The integration MUST audit Hyperion statistics behavior and Valence event sources before adding an anti-cheat statistics plugin.

#### Scenario: Metric scope is bounded

r[valence_hyperion_integration.anticheat_stats.scope.bounded]
- GIVEN anti-cheat statistics work is selected
- WHEN reviewers inspect the scope notes
- THEN the selected metrics, event sources, sampling windows, non-goals, and no-default-enforcement boundary are recorded.

### Requirement: Stable statistics core

r[valence_hyperion_integration.anticheat_stats.core] Statistics calculations MUST be implemented first as a pure stable Rust core over explicit samples and sample-window settings.

#### Scenario: Empty sample window is handled

r[valence_hyperion_integration.anticheat_stats.core.empty]
- GIVEN the statistics core receives an empty sample window
- WHEN it computes selected metrics
- THEN it returns the documented empty-window result
- AND it does not panic, divide by zero, read clocks, or mutate global state.

### Requirement: Statistics fixture coverage

r[valence_hyperion_integration.anticheat_stats.fixtures] Anti-cheat statistics work MUST include positive and negative fixtures for normal samples and invalid/boundary inputs.

#### Scenario: Invalid sample window fails closed

r[valence_hyperion_integration.anticheat_stats.fixtures.invalid_window]
- GIVEN a metric config has an invalid sample window
- WHEN the fixture validator runs
- THEN it returns a deterministic diagnostic
- AND the plugin does not emit a misleading score for that metric.

### Requirement: Optional statistics plugin

r[valence_hyperion_integration.anticheat_stats.plugin] Valence MAY expose an optional statistics plugin that samples explicit event streams and emits observations, but it MUST NOT enforce kicks, bans, or gameplay mutations by default.

#### Scenario: Plugin disabled has no effect

r[valence_hyperion_integration.anticheat_stats.plugin.disabled]
- GIVEN the statistics plugin is not enabled
- WHEN existing Valence gameplay and networking tests run
- THEN no anti-cheat components, metrics, or enforcement behavior are added.

### Requirement: Statistics documentation

r[valence_hyperion_integration.anticheat_stats.docs] Statistics plugin documentation SHOULD describe metric meanings, false-positive risks, data retention, and non-claims.

#### Scenario: Docs warn about enforcement limits

r[valence_hyperion_integration.anticheat_stats.docs.limits]
- GIVEN statistics docs are published
- WHEN reviewers inspect them
- THEN they state that metrics are advisory signals unless a separate policy plugin consumes them.

### Requirement: Statistics validation

r[valence_hyperion_integration.anticheat_stats.validation] Anti-cheat statistics work MUST record statistics tests, negative fixtures, plugin-off regressions, sampling smoke tests, and Cairn gates before archive.

#### Scenario: Statistics closeout is reviewable

r[valence_hyperion_integration.anticheat_stats.validation.log]
- GIVEN statistics plugin work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show pure statistics tests, negative invalid-input tests, plugin-off regressions, sampling smoke tests, docs checks if present, and Cairn validation.
