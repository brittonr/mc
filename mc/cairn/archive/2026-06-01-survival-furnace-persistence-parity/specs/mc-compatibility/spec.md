# Delta: Survival furnace persistence parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.survival_furnace_persistence.contract] The `furnace persistence` row MUST define a bounded deterministic evidence contract before producing receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.survival_furnace_persistence.contract.scope]
- GIVEN `survival-furnace-persistence` work starts
- WHEN the evidence contract is reviewed
- THEN it names one deterministic furnace block, one configured input stack, one fuel stack, one smelted output stack, and one reconnect/reopen observation within the same server process
- AND it states that all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, server restart/world persistence, full survival compatibility, broad vanilla parity, and production readiness remain non-claims.

### Requirement: Checker

r[mc_compatibility.survival_furnace_persistence.checker] A deterministic checker MUST reject `furnace persistence` promotion unless paired Paper and Valence evidence contains matching required metrics.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.survival_furnace_persistence.checker.rejects]
- GIVEN a receipt pair is missing or mismatches furnace open, input insert, fuel insert, burn/progress start, output availability, output collection, reconnect/reopen observation, and matching server-side furnace state milestones
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Valence-only evidence is rejected

r[mc_compatibility.survival_furnace_persistence.checker.paired]
- GIVEN only a Valence receipt is present
- WHEN the checker runs for `furnace persistence` reference parity
- THEN it refuses to promote the row and names the missing Paper/reference artifact.

### Requirement: Runner rail

r[mc_compatibility.survival_furnace_persistence.runner] The compatibility runner MUST expose a `survival-furnace-persistence` scenario that records client and server milestones without changing existing scenario semantics.

#### Scenario: Scenario is isolated

r[mc_compatibility.survival_furnace_persistence.runner.isolated]
- GIVEN existing survival and CTF scenarios
- WHEN `survival-furnace-persistence` is added
- THEN existing scenario milestone requirements and tests remain unchanged
- AND the new scenario has explicit required client and server milestones.

### Requirement: Fixtures

r[mc_compatibility.survival_furnace_persistence.fixtures] Paper and Valence fixtures MUST emit reviewable server-side milestones for `furnace persistence` normalized metrics.

#### Scenario: Server milestones correlate with client milestones

r[mc_compatibility.survival_furnace_persistence.fixtures.correlate]
- GIVEN the client completes the configured `furnace persistence` action
- WHEN both backend logs are reviewed
- THEN Paper and Valence report matching normalized metrics for the configured row
- AND each milestone is scoped to the configured username, scenario, and fixture state.

### Requirement: Receipts

r[mc_compatibility.survival_furnace_persistence.receipts] `furnace persistence` evidence MUST include paired Paper and Valence receipts/logs copied under `docs/evidence/` with committed revision metadata or an oracle checkpoint.

#### Scenario: Evidence is reviewable locally

r[mc_compatibility.survival_furnace_persistence.receipts.reviewable]
- GIVEN a `furnace persistence` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN all required receipts, client logs, server logs, run logs, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Matrix promotion

r[mc_compatibility.survival_furnace_persistence.matrix] The survival coverage matrix MUST promote only the `furnace persistence` row after paired evidence passes.

#### Scenario: Broader survival claims remain non-claims

r[mc_compatibility.survival_furnace_persistence.matrix.nonclaims]
- GIVEN `furnace persistence` evidence passes
- WHEN the matrix and current bundle are updated
- THEN only the `furnace persistence` row is marked reference-parity covered
- AND all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, server restart/world persistence, full survival compatibility, broad vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.survival_furnace_persistence.validation] The change MUST record checker, evidence manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_furnace_persistence.validation.log]
- GIVEN the `furnace persistence` row is promoted
- WHEN the change is archived
- THEN repo-local logs record the row checker, survival coverage checker, acceptance matrix, current bundle, evidence manifests, task gate, and Cairn validation output.
