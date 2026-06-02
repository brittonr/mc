# Delta: Survival mob drop parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.survival_mob_drop.contract] The `mob drops` row MUST define a bounded deterministic evidence contract before producing receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.survival_mob_drop.contract.scope]
- GIVEN `survival-mob-drop` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured mob, one bounded kill interaction, one configured drop stack, one pickup, and exact entity/drop metrics
- AND it states that mob AI parity, pathfinding, all entities, all loot tables, combat balancing, experience drops, farms/spawners, full survival compatibility, broad vanilla parity, and production readiness remain non-claims.

### Requirement: Checker

r[mc_compatibility.survival_mob_drop.checker] A deterministic checker MUST reject `mob drops` promotion unless paired Paper and Valence evidence contains matching required metrics.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.survival_mob_drop.checker.rejects]
- GIVEN a receipt pair is missing or mismatches mob spawn, target acquisition or fixed placement, client attack, server damage/death, drop spawn item/count, client collect/pickup observation, inventory increment, and matching server-side drop milestones
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Valence-only evidence is rejected

r[mc_compatibility.survival_mob_drop.checker.paired]
- GIVEN only a Valence receipt is present
- WHEN the checker runs for `mob drops` reference parity
- THEN it refuses to promote the row and names the missing Paper/reference artifact.

### Requirement: Runner rail

r[mc_compatibility.survival_mob_drop.runner] The compatibility runner MUST expose a `survival-mob-drop` scenario that records client and server milestones without changing existing scenario semantics.

#### Scenario: Scenario is isolated

r[mc_compatibility.survival_mob_drop.runner.isolated]
- GIVEN existing survival and CTF scenarios
- WHEN `survival-mob-drop` is added
- THEN existing scenario milestone requirements and tests remain unchanged
- AND the new scenario has explicit required client and server milestones.

### Requirement: Fixtures

r[mc_compatibility.survival_mob_drop.fixtures] Paper and Valence fixtures MUST emit reviewable server-side milestones for `mob drops` normalized metrics.

#### Scenario: Server milestones correlate with client milestones

r[mc_compatibility.survival_mob_drop.fixtures.correlate]
- GIVEN the client completes the configured `mob drops` action
- WHEN both backend logs are reviewed
- THEN Paper and Valence report matching normalized metrics for the configured row
- AND each milestone is scoped to the configured username, scenario, and fixture state.

### Requirement: Receipts

r[mc_compatibility.survival_mob_drop.receipts] `mob drops` evidence MUST include paired Paper and Valence receipts/logs copied under `docs/evidence/` with committed revision metadata or an oracle checkpoint.

#### Scenario: Evidence is reviewable locally

r[mc_compatibility.survival_mob_drop.receipts.reviewable]
- GIVEN a `mob drops` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN all required receipts, client logs, server logs, run logs, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Matrix promotion

r[mc_compatibility.survival_mob_drop.matrix] The survival coverage matrix MUST promote only the `mob drops` row after paired evidence passes.

#### Scenario: Broader survival claims remain non-claims

r[mc_compatibility.survival_mob_drop.matrix.nonclaims]
- GIVEN `mob drops` evidence passes
- WHEN the matrix and current bundle are updated
- THEN only the `mob drops` row is marked reference-parity covered
- AND mob AI parity, pathfinding, all entities, all loot tables, combat balancing, experience drops, farms/spawners, full survival compatibility, broad vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.survival_mob_drop.validation] The change MUST record checker, evidence manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_mob_drop.validation.log]
- GIVEN the `mob drops` row is promoted
- WHEN the change is archived
- THEN repo-local logs record the row checker, survival coverage checker, acceptance matrix, current bundle, evidence manifests, task gate, and Cairn validation output.
