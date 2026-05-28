# Delta: Production and network safety proof

## Requirements

### Requirement: Production network safety matrix

r[mc_compatibility.production_network_safety.matrix] The repo MUST maintain a matrix that separates owned-local load safety, public-server safety, WAN tolerance, and adversarial-network safety claims.

#### Scenario: Claims are separated

r[mc_compatibility.production_network_safety.matrix.separate]
- GIVEN network safety evidence is reviewed
- WHEN a claim is promoted
- THEN the matrix identifies exactly which safety claim is covered
- AND unrelated safety claims remain non-claims.

### Requirement: Production network safety gate

r[mc_compatibility.production_network_safety.gate] A deterministic checker MUST reject production/public/WAN/adversarial claims without required scope and telemetry fields.

#### Scenario: Missing safety fields reject claim

r[mc_compatibility.production_network_safety.gate.rejects]
- GIVEN an evidence row claims production or public network safety
- WHEN target ownership, authorization, bounds, telemetry, or non-claims are missing
- THEN the checker fails and names the missing fields.

### Requirement: Owned-local load evidence

r[mc_compatibility.production_network_safety.owned_local] Owned-local load safety MUST record client count, duration, hardware scope, telemetry, and failure criteria.

#### Scenario: Owned-local load is bounded

r[mc_compatibility.production_network_safety.owned_local.bounded]
- GIVEN an owned-local load receipt is produced
- WHEN it is reviewed
- THEN it records client count, duration, target ownership, telemetry, and configured upper bounds.

### Requirement: WAN tolerance evidence

r[mc_compatibility.production_network_safety.wan] WAN tolerance claims MUST record perturbation mechanism, delay, jitter, loss, timeout, and fail-closed behavior.

#### Scenario: WAN tooling unavailable fails closed

r[mc_compatibility.production_network_safety.wan.fail_closed]
- GIVEN WAN perturbation tooling is unavailable
- WHEN a WAN tolerance receipt is requested
- THEN the receipt fails closed instead of silently claiming WAN safety.

### Requirement: Adversarial network evidence

r[mc_compatibility.production_network_safety.adversarial] Adversarial-network safety claims MUST require explicit oracle or human approval in addition to deterministic evidence.

#### Scenario: Human/oracle checkpoint required

r[mc_compatibility.production_network_safety.adversarial.oracle]
- GIVEN adversarial-network safety is proposed
- WHEN evidence is promoted
- THEN a checkpoint records the question, inspected evidence, decision, owner, and next action.

### Requirement: Production non-claims

r[mc_compatibility.production_network_safety.nonclaims] Existing loopback compatibility receipts MUST remain non-production evidence until the production/network safety matrix passes.

#### Scenario: Loopback receipts do not imply production

r[mc_compatibility.production_network_safety.nonclaims.loopback]
- GIVEN an owned-local loopback receipt passes
- WHEN documentation is generated
- THEN public-server safety, production readiness, WAN safety, and adversarial-network safety remain explicit non-claims.
