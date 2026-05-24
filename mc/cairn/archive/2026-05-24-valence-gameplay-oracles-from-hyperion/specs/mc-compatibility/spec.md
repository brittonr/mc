# Mc Compatibility Delta: Valence gameplay oracles from Hyperion

## Requirements

### Requirement: Compatibility work

r[mc_compatibility.valence_gameplay_oracles.milestone_catalog] Valence compatibility work MUST catalog Hyperion-derived gameplay milestones before implementing new scenario claims.

#### Scenario: Milestones are mapped to Valence

r[mc_compatibility.valence_gameplay_oracles.milestone_catalog.scenario]

- GIVEN Hyperion Bedwars milestones are reviewed

- WHEN the Valence scenario plan is written

- THEN the plan maps each selected milestone to a Valence example/client/server evidence source

### Requirement: Gameplay scenario receipts

r[mc_compatibility.valence_gameplay_oracles.correlated_receipts] Valence gameplay scenario receipts MUST require correlated client and server evidence for semantic gameplay claims.

#### Scenario: Scenario requires both sides

r[mc_compatibility.valence_gameplay_oracles.correlated_receipts.scenario]

- GIVEN a non-smoke gameplay scenario runs

- WHEN the receipt is evaluated

- THEN the scenario passes only when required client milestones and server correlation are both observed

### Requirement: Gameplay oracle receipts

r[mc_compatibility.valence_gameplay_oracles.non_overclaiming] Valence gameplay oracle receipts MUST preserve explicit non-claims for unsupported gameplay and soak properties.

#### Scenario: Receipt states non-claims

r[mc_compatibility.valence_gameplay_oracles.non_overclaiming.scenario]

- GIVEN a gameplay scenario receipt is recorded

- WHEN the evidence is reviewed

- THEN the receipt states the exact supported claim and keeps full CTF, broad protocol, and unbounded soak claims false
