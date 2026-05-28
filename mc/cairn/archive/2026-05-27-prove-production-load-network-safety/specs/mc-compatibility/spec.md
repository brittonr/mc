# Delta: Production load and network safety proof

## ADDED Requirements

### Requirement: Authorization preflight

r[mc_compatibility.prove_production_load_network_safety.authorization_preflight] Load and network safety proofs MUST fail before launch unless the target is owned local infrastructure or explicitly authorized for the bounded experiment.

#### Scenario: Unauthorized or public target is rejected

r[mc_compatibility.prove_production_load_network_safety.authorization_preflight.scenario]
- GIVEN a load or network experiment is requested
- WHEN the target is public, unowned, or lacks explicit authorization evidence
- THEN the runner rejects the experiment before starting clients or traffic
- AND the receipt records the authorization failure without claiming safety

### Requirement: Bounded envelopes

r[mc_compatibility.prove_production_load_network_safety.bounded_envelopes] Production/load/network claims MUST be expressed as bounded envelopes with explicit client count, duration, reconnect count, latency, jitter, packet-loss, and resource limits.

#### Scenario: Envelope parameters are explicit

r[mc_compatibility.prove_production_load_network_safety.bounded_envelopes.scenario]
- GIVEN a safety envelope is proposed
- WHEN the envelope is reviewed
- THEN every bound and unit is recorded in the plan and receipt
- AND unbounded or unspecified parameters fail the gate

### Requirement: Envelope fixtures

r[mc_compatibility.prove_production_load_network_safety.envelope_fixtures] Load/network envelope logic MUST include positive and negative fixtures for authorized bounded runs, missing authorization, public targets, unbounded parameters, and missing telemetry.

#### Scenario: Unsafe envelope fixture fails closed

r[mc_compatibility.prove_production_load_network_safety.envelope_fixtures.scenario]
- GIVEN envelope fixtures are executed
- WHEN authorization is missing, target scope is unsafe, bounds are unbounded, or telemetry is missing
- THEN the fixture fails with explicit diagnostics
- AND no production/load/network claim is promoted

### Requirement: Safety promotion gate

r[mc_compatibility.prove_production_load_network_safety.safety_promotion_gate] Public-server safety, production readiness, unbounded soak, unbounded reconnect, WAN, adversarial network, and packet-loss claims MUST remain non-claims unless an authorized bounded envelope has passing tests, live receipts, telemetry, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Promotion requires authorized bounded evidence

r[mc_compatibility.prove_production_load_network_safety.safety_promotion_gate.scenario]
- GIVEN a load or network safety claim is proposed
- WHEN any authorization, bound, telemetry, or receipt evidence is missing
- THEN promotion is rejected
- AND the current bundle keeps broader production and network safety as non-claims
