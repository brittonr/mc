# mc-compatibility Change Spec: canonicalize evidence gate wording

## Requirements

### Requirement: Canonical evidence gate wording catalog

r[mc_compatibility.canonical_evidence_gate_wording.catalog] The compatibility evidence gates MUST use canonical non-claim labels for row-specific evidence validation instead of compatibility aliases or duplicate legacy prose.

#### Scenario: Canonical non-claim labels are enforced

r[mc_compatibility.canonical_evidence_gate_wording.catalog.enforced]
- GIVEN a row-specific checker validates evidence docs, matrices, and bundles
- WHEN it checks adjacent non-claims
- THEN it requires the canonical row label used by the primary evidence row or matrix
- AND review docs do not need extra compatibility alias sentences solely to satisfy stale checker tokens.

### Requirement: Evidence checker token constants

r[mc_compatibility.canonical_evidence_gate_wording.checker_constants] Evidence gate checkers MUST name canonical wording tokens as constants before using them in validation token lists.

#### Scenario: Token drift remains visible

r[mc_compatibility.canonical_evidence_gate_wording.checker_constants.visible]
- GIVEN a canonical phrase changes intentionally
- WHEN reviewers inspect the checker diff
- THEN the changed phrase is visible at a named constant or row inventory entry
- AND positive and negative fixtures continue to fail closed for missing canonical evidence.

### Requirement: Aggregate row inventory consistency

r[mc_compatibility.canonical_evidence_gate_wording.row_inventory] Aggregate evidence gates MUST derive row-count expectations from their maintained required-row inventory.

#### Scenario: Promoted row additions update one inventory

r[mc_compatibility.canonical_evidence_gate_wording.row_inventory.derived]
- GIVEN a bounded compatibility row is promoted into an aggregate gate
- WHEN the required-row inventory is updated
- THEN the aggregate gate computes its expected row count from that inventory
- AND it fails if the evidence matrix has missing, extra, or unsupported rows.

### Requirement: Canonical wording validation evidence

r[mc_compatibility.canonical_evidence_gate_wording.validation] The change MUST record reviewable validation for focused row checkers, manifest freshness, aggregate maintained dry-runs, and Cairn lifecycle gates.

#### Scenario: Closeout evidence proves no claim broadening

r[mc_compatibility.canonical_evidence_gate_wording.validation.closeout]
- GIVEN canonical wording cleanup is complete
- WHEN the change is validated
- THEN focused WAN, CTF invalid-action, and full-survival gates pass
- AND maintained dry-runs, evidence manifest checks, Cairn proposal/design/tasks gates, sync/archive checks, and Cairn validation are recorded without broadening WAN, CTF, survival, protocol, public-server, or production claims.
