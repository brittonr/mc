# mc-compatibility Change Spec: CTF invalid-action live breadth proof

## Requirements

### Requirement: CTF invalid-action live breadth contract

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.contract] The system MUST define a bounded live evidence contract for `opponent-base-return-drop-without-carrier` before promoting fixture-only invalid-action breadth evidence to live evidence.

#### Scenario: Live invalid-action contract is bounded

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.contract.bounded]
- GIVEN the live invalid-action breadth row is selected
- WHEN reviewers inspect the contract
- THEN it names the exact row id, actor, team, target flag, attempted action, expected rejection, unchanged flag state, unchanged score state, forbidden mutations, owned-local authorization, tracked artifact requirements, and explicit non-claims.

### Requirement: CTF invalid-action live rail

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.live_rail] The runner MUST provide an owned-local live CTF rail for `opponent-base-return-drop-without-carrier` that records client attempt evidence and Valence server rejection/state evidence.

#### Scenario: Live invalid-action rail records containment

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.live_rail.contained]
- GIVEN the owned-local CTF live rail runs `opponent-base-return-drop-without-carrier`
- WHEN the actor attempts the invalid opponent-base return/drop without carrier ownership
- THEN the receipt records the client attempt, Valence rejection, unchanged flag state, unchanged score state, and absence of forbidden mutation, score, or capture events.

### Requirement: CTF invalid-action live checker

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.checker] The invalid-action breadth checker MUST reject live promotion when required live evidence, state containment, correlation, artifact identity, or non-claims are missing.

#### Scenario: Missing server rejection fails closed

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.checker.missing_rejection]
- GIVEN a live invalid-action row has client attempt evidence but no Valence rejection event
- WHEN the invalid-action breadth checker evaluates the row
- THEN the checker fails with a diagnostic naming the missing rejection.

#### Scenario: State mutation fails closed

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.checker.state_mutation]
- GIVEN a live invalid-action row records a score, flag ownership, or capture mutation after the invalid attempt
- WHEN the invalid-action breadth checker evaluates the row
- THEN the checker fails with a containment diagnostic.

#### Scenario: Wrong correlation fails closed

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.checker.correlation]
- GIVEN a live invalid-action row records the wrong actor, team, target flag, or attempted action for the selected row
- WHEN the invalid-action breadth checker evaluates the row
- THEN the checker fails with a correlation diagnostic.

### Requirement: CTF invalid-action live evidence promotion

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.evidence] The live row MUST have tracked receipt, typed-event log, run log, evidence doc, and BLAKE3 manifest artifacts under `docs/evidence/` before matrix or current-bundle promotion.

#### Scenario: Live evidence artifacts are reviewable

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.evidence.reviewable]
- GIVEN the live invalid-action rail has run
- WHEN reviewers inspect promoted evidence
- THEN receipt, typed-event log, run log, evidence document, and BLAKE3 manifest artifacts are tracked under `docs/evidence/` and the run log records successful exit status.

### Requirement: CTF invalid-action live docs and validation

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.docs] Matrix and current-bundle updates MUST promote only the bounded live invalid-action row and preserve all broad CTF and invalid-action non-claims.

#### Scenario: Matrix promotion stays bounded

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.docs.bounded]
- GIVEN the live invalid-action row is promoted
- WHEN the acceptance matrix and current evidence bundle are inspected
- THEN only `opponent-base-return-drop-without-carrier` changes from fixture-only to bounded live evidence
- AND full CTF correctness, all invalid actions, all flag permutations, adversarial security, public-server safety, production readiness, broad Minecraft compatibility, and vanilla/reference parity remain non-claims.

### Requirement: CTF invalid-action live validation

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.validation] The change MUST record reviewable evidence for live/dry-run validation, positive and negative checker fixtures, evidence manifests, matrix/current-bundle checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.ctf_invalid_action_live_breadth_proof.validation.log]
- GIVEN the live invalid-action breadth row is promoted
- WHEN reviewers inspect the task evidence
- THEN logs show live/dry-run validation, positive and negative checker fixtures, evidence manifest validation, matrix/current-bundle validation, Cairn proposal/design/tasks gates, and Cairn validation.
