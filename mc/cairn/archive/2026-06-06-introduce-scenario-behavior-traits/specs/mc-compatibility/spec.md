# mc-compatibility Change Spec: Scenario behavior traits

## Requirements

### Requirement: Scenario behavior trait contract

r[mc_compatibility.scenario_behavior_traits.contract] The runner MUST define a scenario behavior contract before moving scenario names, aliases, milestones, forbidden patterns, or probe hooks out of the existing match blocks.

#### Scenario: Stable scenario identity is preserved

r[mc_compatibility.scenario_behavior_traits.contract.identity]
- GIVEN scenario behavior is represented by specs and traits
- WHEN CLI, config, receipt, manifest, and checker surfaces name a scenario
- THEN the existing `Scenario` enum remains the stable identity
- AND canonical names, accepted aliases, and receipt scenario strings remain unchanged unless a separate compatibility change explicitly renames them.

### Requirement: Static scenario specs

r[mc_compatibility.scenario_behavior_traits.specs] Simple scenario behavior SHOULD be represented as static `ScenarioSpec` data rather than repeated open-coded matches.

#### Scenario: Milestone tables are reviewable

r[mc_compatibility.scenario_behavior_traits.specs.reviewable]
- GIVEN a reviewer inspects a scenario
- WHEN the static spec is read
- THEN the reviewer can see the canonical name, accepted aliases, client milestones, server milestones, forbidden patterns, and non-default evaluation notes in one bounded location
- AND unchanged scenarios retain the same milestone and forbidden-pattern strings as before the refactor.

### Requirement: Explicit exceptional hooks

r[mc_compatibility.scenario_behavior_traits.hooks] Scenario behavior that cannot be represented as static data MUST use explicit trait hooks with safe defaults.

#### Scenario: Exceptional behavior is not hidden in string comparisons

r[mc_compatibility.scenario_behavior_traits.hooks.explicit]
- GIVEN a scenario needs dynamic projectile-health needles, MCP control behavior, restart/persistence log enrichment, multi-client expectations, or probe environment setup
- WHEN the behavior is implemented
- THEN it is exposed through a named `ScenarioBehavior` method
- AND the default implementation performs no side effects and makes no claim changes.

### Requirement: Scenario match migration

r[mc_compatibility.scenario_behavior_traits.migration] Existing scenario parse/name/milestone/forbidden-pattern/probe paths MUST migrate to spec and behavior lookups only after parity fixtures exist.

#### Scenario: Runner output remains stable

r[mc_compatibility.scenario_behavior_traits.migration.parity]
- GIVEN the migration is complete
- WHEN existing dry-run and evidence-evaluation code paths run for each scenario
- THEN canonical names, required milestone IDs, server milestone IDs, forbidden-match IDs, missing/observed calculations, and receipt fields match the pre-refactor contract.

### Requirement: Scenario behavior tests

r[mc_compatibility.scenario_behavior_traits.tests] The change MUST include positive and negative tests for scenario behavior parity and fail-closed lookup.

#### Scenario: Every existing scenario passes parity tests

r[mc_compatibility.scenario_behavior_traits.tests.positive]
- GIVEN the static specs and behavior hooks cover all existing scenarios
- WHEN tests enumerate the scenario set
- THEN each scenario has a unique canonical name, preserves accepted aliases, preserves client/server milestone tables, preserves forbidden patterns, and exposes any special hook intentionally.

#### Scenario: Invalid scenario definitions fail closed

r[mc_compatibility.scenario_behavior_traits.tests.negative]
- GIVEN a scenario name is unknown, an alias is missing, a canonical name is duplicated, required milestones are absent, forbidden-pattern defaults are unsupported, or a special hook is missing
- WHEN scenario lookup or validation runs
- THEN the runner or test fixture fails with an explicit diagnostic before producing compatibility evidence.

### Requirement: Scenario behavior validation

r[mc_compatibility.scenario_behavior_traits.validation] The change MUST record focused runner tests, scenario manifest checks, relevant evidence checker output, and Cairn gates before archive.

#### Scenario: Validation covers behavior parity

r[mc_compatibility.scenario_behavior_traits.validation.log]
- GIVEN scenario behavior traits are implemented
- WHEN the change is archived
- THEN successful logs show scenario parity tests, unknown-scenario rejection tests, manifest checks, relevant evidence checkers, Cairn proposal/design/tasks gates, and Cairn validation.