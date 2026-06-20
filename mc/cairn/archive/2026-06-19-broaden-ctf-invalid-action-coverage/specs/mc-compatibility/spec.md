# mc-compatibility Change Spec: broaden CTF invalid-action coverage

## Requirements

### Requirement: CTF invalid-action breadth matrix

r[mc_compatibility.ctf_invalid_action_breadth.matrix] The CTF compatibility evidence set MUST define a maintained invalid-action breadth matrix before promoting additional invalid pickup or invalid return/drop permutations.

#### Scenario: Matrix names each bounded permutation

r[mc_compatibility.ctf_invalid_action_breadth.matrix.scope]
- GIVEN additional invalid pickup or invalid return/drop coverage is proposed
- WHEN reviewers inspect the breadth matrix
- THEN each candidate row names action family, actor identity, actor team, flag team, base or carrier pre-state, expected rejection, postcondition, required client milestones, required server milestones, forbidden transitions, evidence status, and non-claims
- AND all invalid actions, all flag permutations, full CTF correctness, adversarial security, public-server safety, production readiness, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: Parameterized invalid-action checker

r[mc_compatibility.ctf_invalid_action_breadth.checker] A deterministic checker MUST validate invalid-action rows from matrix-defined expectations instead of relying on unstructured prose alone.

#### Scenario: Valid row evidence passes narrowly

r[mc_compatibility.ctf_invalid_action_breadth.checker.valid]
- GIVEN a row record, receipt, client log, server log, rule ledger entry, acceptance matrix row, and current-bundle section match one matrix-defined invalid-action permutation
- WHEN the checker evaluates the row
- THEN it passes only for the named bounded permutation and records no broader invalid-action claim.

#### Scenario: Weak row evidence fails closed

r[mc_compatibility.ctf_invalid_action_breadth.checker.rejects]
- GIVEN evidence is missing the row id, mismatches actor team, flag team, owner state, base state, expected rejection, postcondition, required milestones, forbidden transition absence, canonical non-claims, or BLAKE3-backed artifact linkage
- WHEN the checker evaluates the row
- THEN it fails with deterministic diagnostics naming the missing, mismatched, or overbroad metric.

### Requirement: Additional bounded invalid-action row

r[mc_compatibility.ctf_invalid_action_breadth.additional_row] The change MUST add at least one additional bounded invalid-action row beyond the currently promoted own-flag pickup and own-base return/drop rows before broadening any matrix claim.

#### Scenario: New row remains narrow

r[mc_compatibility.ctf_invalid_action_breadth.additional_row.narrow]
- GIVEN one additional invalid pickup or invalid return/drop permutation has passing evidence
- WHEN acceptance matrix, current-bundle, and CTF rule ledger docs are updated
- THEN only that named permutation is marked covered
- AND all invalid actions, all flag permutations, full CTF correctness, adversarial security, public-server safety, production readiness, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: Invalid-action runner or fixture rail

r[mc_compatibility.ctf_invalid_action_breadth.rail] The runner or fixture layer MUST emit normalized invalid-action row evidence for the selected additional permutation without changing existing CTF row semantics.

#### Scenario: Existing rows stay stable

r[mc_compatibility.ctf_invalid_action_breadth.rail.isolated]
- GIVEN existing CTF invalid pickup and invalid return/drop rows already pass
- WHEN the new invalid-action rail or fixture runs
- THEN existing scenario names, milestones, receipts, and claims remain compatible
- AND the new row has separate normalized evidence fields and artifact paths.

### Requirement: Invalid-action breadth validation evidence

r[mc_compatibility.ctf_invalid_action_breadth.validation] The change MUST record reviewable baseline, checker, runner or fixture, manifest, matrix/bundle, task-evidence, Cairn gate, sync/archive, and final validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.ctf_invalid_action_breadth.validation.closeout]
- GIVEN invalid-action breadth work is complete
- WHEN the change is archived
- THEN repo-local evidence logs show baseline CTF checks, checker positive and negative tests, selected row evidence validation, evidence-manifest checks, CTF rule ledger/current-bundle checks, maintained dry-runs, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and final Cairn validation passing.
