# Delta: Survival chest persistence row

## Requirements

### Requirement: Chest persistence contract

r[mc_compatibility.survival_chest_persistence.contract] The chest persistence row MUST define a bounded, deterministic evidence contract before producing receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.survival_chest_persistence.contract.scope]
- GIVEN the chest persistence work starts
- WHEN the evidence contract is reviewed
- THEN it names exactly one chest block, one item stack, one chest slot, one close/reconnect/reopen sequence, and the normalized metrics required for promotion
- AND it states that all-container behavior, server restart persistence, world persistence, and broader vanilla parity remain non-claims.

### Requirement: Chest persistence checker

r[mc_compatibility.survival_chest_persistence.checker] A deterministic checker MUST reject chest persistence promotion unless paired Paper and Valence evidence contains matching required metrics.

#### Scenario: Missing or mismatched chest metrics fail closed

r[mc_compatibility.survival_chest_persistence.checker.rejects]
- GIVEN a receipt pair is missing the chest open, store, close, reconnect/reopen, persisted slot item, persisted count, or server chest-state metric
- WHEN the checker runs
- THEN it fails and names the missing metric.

#### Scenario: Valence-only evidence is rejected

r[mc_compatibility.survival_chest_persistence.checker.paired]
- GIVEN only a Valence receipt is present
- WHEN the checker runs
- THEN it refuses to promote the chest persistence row.

### Requirement: Chest persistence runner rail

r[mc_compatibility.survival_chest_persistence.runner] The compatibility runner MUST expose a `survival-chest-persistence` scenario that records client and server milestones without changing existing scenario semantics.

#### Scenario: Scenario is isolated

r[mc_compatibility.survival_chest_persistence.runner.isolated]
- GIVEN existing `survival-break-place-pickup` and `inventory-interaction` scenarios
- WHEN `survival-chest-persistence` is added
- THEN existing scenario milestone requirements and tests remain unchanged.

### Requirement: Chest persistence fixtures

r[mc_compatibility.survival_chest_persistence.fixtures] Paper and Valence fixtures MUST emit reviewable server-side milestones for chest open, store, close, reconnect/reopen, and persisted slot observation.

#### Scenario: Server milestones correlate with client milestones

r[mc_compatibility.survival_chest_persistence.fixtures.correlate]
- GIVEN the client stores the configured stack in the configured chest slot
- WHEN the chest is reopened after reconnect
- THEN both backends report the same persisted item and count for the configured slot.

### Requirement: Chest persistence receipts

r[mc_compatibility.survival_chest_persistence.receipts] Chest persistence evidence MUST include paired Paper and Valence receipts/logs copied under `docs/evidence/` with committed revision metadata or an oracle checkpoint.

#### Scenario: Evidence is reviewable locally

r[mc_compatibility.survival_chest_persistence.receipts.reviewable]
- GIVEN a chest persistence row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN all required receipts, client logs, server logs, run logs, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Chest persistence matrix promotion

r[mc_compatibility.survival_chest_persistence.matrix] The survival coverage matrix MUST promote only the `chest persistence` row after paired evidence passes.

#### Scenario: Broader survival claims remain non-claims

r[mc_compatibility.survival_chest_persistence.matrix.nonclaims]
- GIVEN chest persistence evidence passes
- WHEN the matrix and current bundle are updated
- THEN full survival compatibility, all-container behavior, server restart/world persistence, and broader vanilla parity remain explicit non-claims.

### Requirement: Chest persistence validation

r[mc_compatibility.survival_chest_persistence.validation] The change MUST record checker, evidence manifest, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_chest_persistence.validation.log]
- GIVEN the chest persistence row is promoted
- WHEN the change is archived
- THEN a repo-local run log records the chest checker, survival coverage checker, acceptance matrix, current bundle, evidence manifests, and Cairn validation output.
