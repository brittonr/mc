# Delta: Full CTF rule correctness proof

## ADDED Requirements

### Requirement: CTF rule ledger

r[mc_compatibility.prove_ctf_rule_correctness.rule_ledger] The CTF correctness proof MUST maintain a rule ledger that lists every rule or invariant required for the claimed Valence CTF correctness scope.

#### Scenario: Rule ledger identifies evidence status

r[mc_compatibility.prove_ctf_rule_correctness.rule_ledger.scenario]
- GIVEN full CTF correctness is being evaluated
- WHEN the rule ledger is reviewed
- THEN each rule records its evidence status, required client milestones, required server milestones, forbidden transitions, and next action
- AND uncovered rules remain explicit non-claims

### Requirement: Positive rule scenarios

r[mc_compatibility.prove_ctf_rule_correctness.positive_rule_scenarios] Legal CTF rule paths MUST have bounded positive scenarios with correlated Valence server and Stevenarella client evidence when client observation is part of the claim.

#### Scenario: Legal rule path is correlated

r[mc_compatibility.prove_ctf_rule_correctness.positive_rule_scenarios.scenario]
- GIVEN a legal CTF action is part of the promoted scope
- WHEN the scenario runs
- THEN required client milestones and server milestones are present for the same bounded game state
- AND the receipt records no missing milestones for that rule path

### Requirement: Negative rule scenarios

r[mc_compatibility.prove_ctf_rule_correctness.negative_rule_scenarios] Invalid CTF actions MUST be covered by negative scenarios that prove forbidden captures, pickups, returns, or scores do not occur.

#### Scenario: Invalid rule path is rejected

r[mc_compatibility.prove_ctf_rule_correctness.negative_rule_scenarios.scenario]
- GIVEN an invalid CTF action is attempted
- WHEN the scenario evaluates server and client evidence
- THEN forbidden milestones or score transitions are absent
- AND the receipt fails if the invalid action produces a rule-breaking state

### Requirement: Rule promotion gate

r[mc_compatibility.prove_ctf_rule_correctness.rule_promotion_gate] A CTF rule cluster MUST NOT be promoted as correct until its rule ledger row, dry-run fixture, live receipt, BLAKE3 manifest, and matrix entry agree on the scoped claim.

#### Scenario: Promotion requires complete rule evidence

r[mc_compatibility.prove_ctf_rule_correctness.rule_promotion_gate.scenario]
- GIVEN a CTF rule cluster is proposed for acceptance
- WHEN any required evidence artifact is missing or stale
- THEN the promotion is rejected
- AND the current bundle continues to list full CTF correctness as a non-claim
