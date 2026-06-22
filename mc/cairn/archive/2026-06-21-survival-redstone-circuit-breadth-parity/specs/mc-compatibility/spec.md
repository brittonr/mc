# Delta: Survival redstone circuit breadth parity

## Requirements

### Requirement: Redstone circuit breadth contract

r[mc_compatibility.survival_redstone_circuit_breadth_parity.contract] The `survival-redstone-circuit-breadth-parity` row MUST define a bounded circuit fixture before promotion.

#### Scenario: Contract names finite circuit scope

r[mc_compatibility.survival_redstone_circuit_breadth_parity.contract.scope]
- GIVEN redstone circuit work starts
- WHEN the contract is reviewed
- THEN it names one configured input, dust or repeater path, output block, bounded tick checkpoint sequence, optional stateful/mechanical component, and normalized powered-state metrics
- AND general redstone circuit parity, all tick-order semantics, clocks, farms, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Redstone circuit checker

r[mc_compatibility.survival_redstone_circuit_breadth_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence redstone circuit metrics before promotion.

#### Scenario: Weak redstone evidence fails closed

r[mc_compatibility.survival_redstone_circuit_breadth_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits powered-state checkpoints, mismatches the tick sequence, reports stale child revisions, or claims broad redstone parity
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid redstone metric.

### Requirement: Isolated redstone circuit rail

r[mc_compatibility.survival_redstone_circuit_breadth_parity.rail] The harness MUST expose an isolated circuit rail without changing the existing redstone toggle row.

#### Scenario: Existing redstone row remains unchanged

r[mc_compatibility.survival_redstone_circuit_breadth_parity.rail.isolated]
- GIVEN the existing lever/lamp row is promoted
- WHEN the circuit rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own circuit checkpoints.

### Requirement: Reviewable redstone circuit receipts

r[mc_compatibility.survival_redstone_circuit_breadth_parity.receipts] Paired redstone circuit receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_redstone_circuit_breadth_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow redstone circuit promotion

r[mc_compatibility.survival_redstone_circuit_breadth_parity.promotion] Matrix and bundle docs MUST promote only the bounded redstone circuit row after paired evidence passes.

#### Scenario: Broader redstone remains a non-claim

r[mc_compatibility.survival_redstone_circuit_breadth_parity.promotion.nonclaims]
- GIVEN paired redstone circuit evidence passes
- WHEN docs are updated
- THEN only the configured circuit row is marked covered
- AND general circuits, all tick-order semantics, pistons/observers/comparators breadth, clocks, farms, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Redstone circuit validation evidence

r[mc_compatibility.survival_redstone_circuit_breadth_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_redstone_circuit_breadth_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.
