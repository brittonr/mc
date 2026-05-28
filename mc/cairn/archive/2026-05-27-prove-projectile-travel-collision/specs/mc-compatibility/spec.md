# Delta: Projectile travel and collision proof

## ADDED Requirements

### Requirement: Projectile matrix

r[mc_compatibility.prove_projectile_travel_collision.projectile_matrix] The projectile physics proof MUST define a matrix of projectile states, target types, weapon representatives, and required client/server evidence before promoting travel, collision, or variant breadth claims.

#### Scenario: Matrix scopes projectile physics claims

r[mc_compatibility.prove_projectile_travel_collision.projectile_matrix.scenario]
- GIVEN projectile physics behavior is being evaluated
- WHEN the matrix is reviewed
- THEN each row records weapon representative, projectile state sequence, target type, expected server evidence, expected client observation, and non-claim status
- AND unobserved travel/collision states remain non-claims

### Requirement: Positive projectile scenarios

r[mc_compatibility.prove_projectile_travel_collision.positive_projectile_scenarios] Selected projectile rows MUST have positive scenarios that correlate Stevenarella client projectile observations with Valence server projectile events.

#### Scenario: Projectile state sequence is correlated

r[mc_compatibility.prove_projectile_travel_collision.positive_projectile_scenarios.scenario]
- GIVEN a projectile matrix row is selected
- WHEN the scenario runs
- THEN the evidence correlates attacker, projectile sequence or entity, target, server event, and client observation for the claimed states
- AND the receipt records no missing projectile milestones for that row

### Requirement: Negative projectile scenarios

r[mc_compatibility.prove_projectile_travel_collision.negative_projectile_scenarios] Projectile verification MUST reject missing, mismatched, unordered, wrong-target, or wrong-weapon evidence.

#### Scenario: Weak projectile evidence fails

r[mc_compatibility.prove_projectile_travel_collision.negative_projectile_scenarios.scenario]
- GIVEN projectile evidence is missing travel/collision state or mismatches attacker, target, sequence, or weapon
- WHEN the runner evaluates the row
- THEN the row fails with explicit diagnostics
- AND no projectile travel/collision claim is promoted

### Requirement: Projectile promotion gate

r[mc_compatibility.prove_projectile_travel_collision.projectile_promotion_gate] Projectile travel, collision, and weapon variant breadth MUST remain non-claims until required rows have passing tests, live receipts, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Projectile promotion requires row evidence

r[mc_compatibility.prove_projectile_travel_collision.projectile_promotion_gate.scenario]
- GIVEN projectile travel/collision or variant breadth is proposed as covered
- WHEN any required projectile row lacks passing evidence
- THEN promotion is rejected
- AND exact vanilla projectile physics remains a separate non-claim unless proven by the vanilla parity package
