# Delta: Flag-carrier death and flag-return compatibility rail

## Requirements

### Requirement: Scenario Owned

r[mc_compatibility.flag_carrier_death_return.scenario_owned] The system MUST provide a bounded protocol-763 Valence CTF scenario that causes a flag carrier to die or be eliminated before capture.

#### Scenario: Scenario Owned evidence is required

r[mc_compatibility.flag_carrier_death_return.scenario_owned.scenario]
- GIVEN the `Flag-carrier death and flag-return compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.flag_carrier_death_return.scenario_owned`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Server Correlation

r[mc_compatibility.flag_carrier_death_return.server_correlation] The system MUST record server-side correlation for carrier username, opposing attacker username, held flag team, death or lethal-health transition, and flag return/reset semantics.

#### Scenario: Server Correlation evidence is required

r[mc_compatibility.flag_carrier_death_return.server_correlation.scenario]
- GIVEN the `Flag-carrier death and flag-return compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.flag_carrier_death_return.server_correlation`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Client Evidence

r[mc_compatibility.flag_carrier_death_return.client_evidence] The system MUST record client-side Stevenarella milestones for flag pickup, opposing-player combat/death observation, and respawn or restored gameplay state after the flag-carrier death edge.

#### Scenario: Client Evidence evidence is required

r[mc_compatibility.flag_carrier_death_return.client_evidence.scenario]
- GIVEN the `Flag-carrier death and flag-return compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.flag_carrier_death_return.client_evidence`
- AND the receipt or documentation states scoped non-claims where the proof is bounded

### Requirement: Receipt Gate

r[mc_compatibility.flag_carrier_death_return.receipt_gate] The system MUST expose a maintained runner/flaked dry-run gate and a live receipt that include required/missing milestones, forbidden accidental-score/capture evidence, log hygiene, and BLAKE3 evidence documentation.

#### Scenario: Receipt Gate evidence is required

r[mc_compatibility.flag_carrier_death_return.receipt_gate.scenario]
- GIVEN the `Flag-carrier death and flag-return compatibility rail` change is being drained
- WHEN the implementation and evidence are reviewed
- THEN the evidence satisfies `mc_compatibility.flag_carrier_death_return.receipt_gate`
- AND the receipt or documentation states scoped non-claims where the proof is bounded
