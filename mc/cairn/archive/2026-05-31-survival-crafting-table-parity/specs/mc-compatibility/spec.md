# Delta: Survival crafting table parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.survival_crafting_table.contract] The `crafting` row MUST define a bounded deterministic evidence contract before producing receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.survival_crafting_table.contract.scope]
- GIVEN `survival-crafting-table` work starts
- WHEN the evidence contract is reviewed
- THEN it names one deterministic crafting table, one configured recipe, one configured input stack set, one result stack, and exact inventory/result metrics
- AND it states that full crafting coverage, all recipes, recipe-book behavior, shift-click matrices, all container transaction modes, full survival compatibility, broad vanilla parity, and production readiness remain non-claims.

### Requirement: Checker

r[mc_compatibility.survival_crafting_table.checker] A deterministic checker MUST reject `crafting` promotion unless paired Paper and Valence evidence contains matching required metrics.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.survival_crafting_table.checker.rejects]
- GIVEN a receipt pair is missing or mismatches crafting-table open, configured input-slot placement, result-slot availability, result collection, inventory decrement/increment, and matching server-side recipe/result milestones
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Valence-only evidence is rejected

r[mc_compatibility.survival_crafting_table.checker.paired]
- GIVEN only a Valence receipt is present
- WHEN the checker runs for `crafting` reference parity
- THEN it refuses to promote the row and names the missing Paper/reference artifact.

### Requirement: Runner rail

r[mc_compatibility.survival_crafting_table.runner] The compatibility runner MUST expose a `survival-crafting-table` scenario that records client and server milestones without changing existing scenario semantics.

#### Scenario: Scenario is isolated

r[mc_compatibility.survival_crafting_table.runner.isolated]
- GIVEN existing survival and CTF scenarios
- WHEN `survival-crafting-table` is added
- THEN existing scenario milestone requirements and tests remain unchanged
- AND the new scenario has explicit required client and server milestones.

### Requirement: Fixtures

r[mc_compatibility.survival_crafting_table.fixtures] Paper and Valence fixtures MUST emit reviewable server-side milestones for `crafting` normalized metrics.

#### Scenario: Server milestones correlate with client milestones

r[mc_compatibility.survival_crafting_table.fixtures.correlate]
- GIVEN the client completes the configured `crafting` action
- WHEN both backend logs are reviewed
- THEN Paper and Valence report matching normalized metrics for the configured row
- AND each milestone is scoped to the configured username, scenario, and fixture state.

### Requirement: Receipts

r[mc_compatibility.survival_crafting_table.receipts] `crafting` evidence MUST include paired Paper and Valence receipts/logs copied under `docs/evidence/` with committed revision metadata or an oracle checkpoint.

#### Scenario: Evidence is reviewable locally

r[mc_compatibility.survival_crafting_table.receipts.reviewable]
- GIVEN a `crafting` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN all required receipts, client logs, server logs, run logs, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Matrix promotion

r[mc_compatibility.survival_crafting_table.matrix] The survival coverage matrix MUST promote only the `crafting` row after paired evidence passes.

#### Scenario: Broader survival claims remain non-claims

r[mc_compatibility.survival_crafting_table.matrix.nonclaims]
- GIVEN `crafting` evidence passes
- WHEN the matrix and current bundle are updated
- THEN only the `crafting` row is marked reference-parity covered
- AND full crafting coverage, all recipes, recipe-book behavior, shift-click matrices, all container transaction modes, full survival compatibility, broad vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.survival_crafting_table.validation] The change MUST record checker, evidence manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_crafting_table.validation.log]
- GIVEN the `crafting` row is promoted
- WHEN the change is archived
- THEN repo-local logs record the row checker, survival coverage checker, acceptance matrix, current bundle, evidence manifests, task gate, and Cairn validation output.
