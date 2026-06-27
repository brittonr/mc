# mc-compatibility Change Spec: Biome/dimension state correlation

## Requirements

### Requirement: Biome/dimension state inventory

r[mc_compatibility.biome_dimension_state_correlation.inventory] The change MUST inventory current `survival-biome-dimension-state` fallback behavior, fixture configuration, client observation, server state source, and non-claim boundaries before migration.

#### Scenario: Join-state scope is reviewable

r[mc_compatibility.biome_dimension_state_correlation.inventory.reviewable]
- GIVEN biome/dimension state correlation work begins
- WHEN reviewers inspect the inventory
- THEN it names the selected scenario, current evidence, fixture state, observed client fields, server state source, and explicit non-claims.

### Requirement: Typed join-state contract

r[mc_compatibility.biome_dimension_state_correlation.contract] The selected row MUST define typed receipt fields for scenario identity, protocol context, client-observed join-state data, server-configured state, correlation result, and non-claim labels.

#### Scenario: Client and server state are correlated

r[mc_compatibility.biome_dimension_state_correlation.contract.correlated]
- GIVEN the join-state scenario runs
- WHEN the receipt is evaluated
- THEN it records client-observed state and server-configured state under the same scenario and protocol context
- AND it states that dimension travel, all biome semantics, and full survival compatibility remain non-claims.

### Requirement: Pure join-state validator

r[mc_compatibility.biome_dimension_state_correlation.validator] Join-state validation MUST be a pure deterministic core over normalized receipt fields and MUST fail closed for client-only evidence, server/client mismatches, missing protocol context, or overbroad claims.

#### Scenario: Matching join-state evidence passes

r[mc_compatibility.biome_dimension_state_correlation.validator.positive]
- GIVEN client-observed state and server-configured state match for the selected scenario and protocol context
- WHEN the validator evaluates the normalized record
- THEN validation passes with stable diagnostics.

#### Scenario: Weak join-state evidence fails

r[mc_compatibility.biome_dimension_state_correlation.validator.negative]
- GIVEN a receipt lacks server state, mismatches client and server state, omits protocol context, or claims dimension travel
- WHEN the validator evaluates the normalized record
- THEN validation fails and names the missing, mismatched, or overbroad field.

### Requirement: Join-state wiring

r[mc_compatibility.biome_dimension_state_correlation.wiring] Runner, client, and server fixture shells MUST emit the typed join-state fields without changing the existing scenario name, wrapper shape, or bounded evidence claim.

#### Scenario: Structured fields replace substring fallback

r[mc_compatibility.biome_dimension_state_correlation.wiring.structured]
- GIVEN the typed join-state fields are emitted
- WHEN scenario validation runs
- THEN validation uses structured receipt fields rather than substring fallback
- AND raw logs remain supplemental review evidence.

### Requirement: Join-state manifest migration

r[mc_compatibility.biome_dimension_state_correlation.manifest] Scenario manifest migration state, fallback budget baseline, generated scenario surfaces, and evidence docs MUST update only after typed validation passes.

#### Scenario: Manifest shows typed readiness

r[mc_compatibility.biome_dimension_state_correlation.manifest.ready]
- GIVEN typed join-state validation passes
- WHEN generated scenario surfaces are refreshed
- THEN `survival-biome-dimension-state` is marked typed-event-ready and removed from approved fallback debt
- AND its non-claim fields remain visible.

### Requirement: Biome/dimension state closeout

r[mc_compatibility.biome_dimension_state_correlation.closeout] The change MUST record focused scenario checks, scenario manifest checks, generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.biome_dimension_state_correlation.closeout.log]
- GIVEN biome/dimension state correlation has migrated
- WHEN reviewers inspect task evidence
- THEN logs show positive and negative validator fixtures, focused scenario checks, scenario manifest checks, generated-surface freshness, evidence manifest validation, task-evidence validation, Cairn gates, and Cairn validation.
