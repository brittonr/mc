# Delta: Survival redstone toggle parity rail

## Requirements

### Requirement: Contract

r[mc_compatibility.survival_redstone_toggle.contract] The `redstone` row MUST define a bounded deterministic evidence contract before producing receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.survival_redstone_toggle.contract.scope]
- GIVEN `survival-redstone-toggle` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured input control, one configured powered output block, one on/off toggle sequence, and exact powered-state metrics
- AND it states that redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, block-update breadth, full survival compatibility, broad vanilla parity, and production readiness remain non-claims.

### Requirement: Checker

r[mc_compatibility.survival_redstone_toggle.checker] A deterministic checker MUST reject `redstone` promotion unless paired Paper and Valence evidence contains matching required metrics.

#### Scenario: Missing or mismatched metrics fail closed

r[mc_compatibility.survival_redstone_toggle.checker.rejects]
- GIVEN a receipt pair is missing or mismatches input interaction, server powered-state transition, client block/state update for the output, optional return-to-off transition when configured, and matching Paper/Valence powered-state observations
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Valence-only evidence is rejected

r[mc_compatibility.survival_redstone_toggle.checker.paired]
- GIVEN only a Valence receipt is present
- WHEN the checker runs for `redstone` reference parity
- THEN it refuses to promote the row and names the missing Paper/reference artifact.

### Requirement: Runner rail

r[mc_compatibility.survival_redstone_toggle.runner] The compatibility runner MUST expose a `survival-redstone-toggle` scenario that records client and server milestones without changing existing scenario semantics.

#### Scenario: Scenario is isolated

r[mc_compatibility.survival_redstone_toggle.runner.isolated]
- GIVEN existing survival and CTF scenarios
- WHEN `survival-redstone-toggle` is added
- THEN existing scenario milestone requirements and tests remain unchanged
- AND the new scenario has explicit required client and server milestones.

### Requirement: Fixtures

r[mc_compatibility.survival_redstone_toggle.fixtures] Paper and Valence fixtures MUST emit reviewable server-side milestones for `redstone` normalized metrics.

#### Scenario: Server milestones correlate with client milestones

r[mc_compatibility.survival_redstone_toggle.fixtures.correlate]
- GIVEN the client completes the configured `redstone` action
- WHEN both backend logs are reviewed
- THEN Paper and Valence report matching normalized metrics for the configured row
- AND each milestone is scoped to the configured username, scenario, and fixture state.

### Requirement: Receipts

r[mc_compatibility.survival_redstone_toggle.receipts] `redstone` evidence MUST include paired Paper and Valence receipts/logs copied under `docs/evidence/` with committed revision metadata or an oracle checkpoint.

#### Scenario: Evidence is reviewable locally

r[mc_compatibility.survival_redstone_toggle.receipts.reviewable]
- GIVEN a `redstone` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN all required receipts, client logs, server logs, run logs, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Matrix promotion

r[mc_compatibility.survival_redstone_toggle.matrix] The survival coverage matrix MUST promote only the `redstone` row after paired evidence passes.

#### Scenario: Broader survival claims remain non-claims

r[mc_compatibility.survival_redstone_toggle.matrix.nonclaims]
- GIVEN `redstone` evidence passes
- WHEN the matrix and current bundle are updated
- THEN only the `redstone` row is marked reference-parity covered
- AND redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, block-update breadth, full survival compatibility, broad vanilla parity, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.survival_redstone_toggle.validation] The change MUST record checker, evidence manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_redstone_toggle.validation.log]
- GIVEN the `redstone` row is promoted
- WHEN the change is archived
- THEN repo-local logs record the row checker, survival coverage checker, acceptance matrix, current bundle, evidence manifests, task gate, and Cairn validation output.
