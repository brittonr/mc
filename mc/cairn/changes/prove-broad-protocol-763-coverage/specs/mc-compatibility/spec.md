# Delta: Broad protocol 763 coverage proof

## ADDED Requirements

### Requirement: Protocol coverage ledger

r[mc_compatibility.prove_broad_protocol_763_coverage.coverage_ledger] The broad protocol-763 proof MUST maintain a reviewable coverage ledger that joins Valence packet metadata, Stevenarella mapping/parser status, and receipt-backed scenario evidence.

#### Scenario: Ledger records covered and uncovered protocol surfaces

r[mc_compatibility.prove_broad_protocol_763_coverage.coverage_ledger.scenario]
- GIVEN broad protocol-763 coverage is being evaluated
- WHEN the ledger is generated or reviewed
- THEN every Valence protocol-763 packet family considered by the claim has a status, evidence path or gap reason, owner, and next action
- AND uncovered rows remain explicit non-claims

### Requirement: Mapping and parser fixtures

r[mc_compatibility.prove_broad_protocol_763_coverage.mapping_parser_fixtures] Newly promoted protocol-763 packet families MUST have focused positive and negative mapping/parser verification before acceptance.

#### Scenario: Fixtures reject fallback aliases and malformed shapes

r[mc_compatibility.prove_broad_protocol_763_coverage.mapping_parser_fixtures.scenario]
- GIVEN a packet family is proposed for coverage promotion
- WHEN the focused verification runs
- THEN positive fixtures prove the reviewed semantic mapping and parser shape
- AND negative fixtures reject inherited fallback aliases, incompatible packet shapes, and malformed payloads

### Requirement: Live scenario gates

r[mc_compatibility.prove_broad_protocol_763_coverage.live_scenario_gates] Broad protocol coverage MUST be promoted only through bounded live scenario gates whose receipts name the exact scenario family and protocol surface being claimed.

#### Scenario: Live receipts scope protocol claims

r[mc_compatibility.prove_broad_protocol_763_coverage.live_scenario_gates.scenario]
- GIVEN mapping/parser fixtures pass for a scenario family
- WHEN a live receipt is produced
- THEN the receipt records client/server commits, scenario family, covered protocol surface, missing milestones, and BLAKE3-backed evidence paths
- AND it does not claim unrelated protocol families

### Requirement: Non-overclaiming gate

r[mc_compatibility.prove_broad_protocol_763_coverage.non_overclaiming_gate] Full Minecraft or full protocol-763 compatibility claims MUST remain blocked until the ledger, fixtures, live receipts, matrix, and current bundle all show complete coverage for the stated claim.

#### Scenario: Broad claim is blocked on any uncovered row

r[mc_compatibility.prove_broad_protocol_763_coverage.non_overclaiming_gate.scenario]
- GIVEN a broad/full compatibility claim is proposed
- WHEN any required ledger row lacks receipt-backed evidence or has failing verification
- THEN the claim is rejected
- AND the acceptance matrix keeps broad compatibility as a non-claim
