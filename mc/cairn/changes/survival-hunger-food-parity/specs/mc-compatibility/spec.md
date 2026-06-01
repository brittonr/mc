# Delta: Survival hunger and food parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.survival_hunger_food.contract] The `hunger/food` row MUST define a bounded deterministic evidence contract before producing receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.survival_hunger_food.contract.scope]
- GIVEN `survival-hunger-food` work starts
- WHEN the evidence contract is reviewed
- THEN it names one deterministic hunger deficit, one configured food item, one consume action, one hunger/saturation delta, and one inventory decrement
- AND it states that all foods, starvation loops, regeneration balance, potion/status effects, exhaustion math, sprint/jump hunger drain, full survival compatibility, broad vanilla parity, and production readiness remain non-claims.

### Requirement: Checker

r[mc_compatibility.survival_hunger_food.checker] A deterministic checker MUST reject `hunger/food` promotion unless paired Paper and Valence evidence contains matching required metrics.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.survival_hunger_food.checker.rejects]
- GIVEN a receipt pair is missing or mismatches pre-consume hunger/saturation, consume start/finish, item decrement, post-consume hunger/saturation, optional health/regeneration observation when configured, and matching server-side food milestones
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Valence-only evidence is rejected

r[mc_compatibility.survival_hunger_food.checker.paired]
- GIVEN only a Valence receipt is present
- WHEN the checker runs for `hunger/food` reference parity
- THEN it refuses to promote the row and names the missing Paper/reference artifact.

### Requirement: Runner rail

r[mc_compatibility.survival_hunger_food.runner] The compatibility runner MUST expose a `survival-hunger-food` scenario that records client and server milestones without changing existing scenario semantics.

#### Scenario: Scenario is isolated

r[mc_compatibility.survival_hunger_food.runner.isolated]
- GIVEN existing survival and CTF scenarios
- WHEN `survival-hunger-food` is added
- THEN existing scenario milestone requirements and tests remain unchanged
- AND the new scenario has explicit required client and server milestones.

### Requirement: Fixtures

r[mc_compatibility.survival_hunger_food.fixtures] Paper and Valence fixtures MUST emit reviewable server-side milestones for `hunger/food` normalized metrics.

#### Scenario: Server milestones correlate with client milestones

r[mc_compatibility.survival_hunger_food.fixtures.correlate]
- GIVEN the client completes the configured `hunger/food` action
- WHEN both backend logs are reviewed
- THEN Paper and Valence report matching normalized metrics for the configured row
- AND each milestone is scoped to the configured username, scenario, and fixture state.

### Requirement: Receipts

r[mc_compatibility.survival_hunger_food.receipts] `hunger/food` evidence MUST include paired Paper and Valence receipts/logs copied under `docs/evidence/` with committed revision metadata or an oracle checkpoint.

#### Scenario: Evidence is reviewable locally

r[mc_compatibility.survival_hunger_food.receipts.reviewable]
- GIVEN a `hunger/food` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN all required receipts, client logs, server logs, run logs, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Matrix promotion

r[mc_compatibility.survival_hunger_food.matrix] The survival coverage matrix MUST promote only the `hunger/food` row after paired evidence passes.

#### Scenario: Broader survival claims remain non-claims

r[mc_compatibility.survival_hunger_food.matrix.nonclaims]
- GIVEN `hunger/food` evidence passes
- WHEN the matrix and current bundle are updated
- THEN only the `hunger/food` row is marked reference-parity covered
- AND all foods, starvation loops, regeneration balance, potion/status effects, exhaustion math, sprint/jump hunger drain, full survival compatibility, broad vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.survival_hunger_food.validation] The change MUST record checker, evidence manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_hunger_food.validation.log]
- GIVEN the `hunger/food` row is promoted
- WHEN the change is archived
- THEN repo-local logs record the row checker, survival coverage checker, acceptance matrix, current bundle, evidence manifests, task gate, and Cairn validation output.
