# mc-compatibility Change Spec: Targeted packet live parity

## Requirements

### Requirement: Targeted packet live parity selection

r[mc_compatibility.targeted_packet_live_parity.selection] The system MUST select a bounded subset of fixture-backed targeted packet rows before attempting live parity promotion.

#### Scenario: Selected rows have explicit live signals

r[mc_compatibility.targeted_packet_live_parity.selection.signals]
- GIVEN targeted packet rows have deterministic fixture evidence
- WHEN rows are selected for live parity promotion
- THEN each selected row records the packet identifier, fixture evidence source, intended live signal, applicable backend/client path, and non-claim scope
- AND rows not selected remain fixture-bounded.

### Requirement: Targeted packet live parity baseline

r[mc_compatibility.targeted_packet_live_parity.baseline] The change MUST run the existing fixture and evidence checks for selected packet rows before modifying live probes.

#### Scenario: Baseline separates existing fixture status from live promotion

r[mc_compatibility.targeted_packet_live_parity.baseline.recorded]
- GIVEN selected packet rows already have fixture evidence
- WHEN implementation begins
- THEN baseline logs show the existing targeted packet checks and evidence checks before live-probe changes are introduced.

### Requirement: Targeted packet live probes

r[mc_compatibility.targeted_packet_live_parity.probes] The runner SHOULD exercise selected packet behavior through live backend/client paths when local infrastructure can produce a deterministic signal.

#### Scenario: Live probe identifies packet behavior

r[mc_compatibility.targeted_packet_live_parity.probes.signal]
- GIVEN a selected packet row has an applicable live scenario
- WHEN the runner executes the live probe
- THEN the produced evidence identifies the scenario, backend path, client path, packet behavior, and observed milestone or log signal
- AND the probe does not claim full protocol 763 support.

### Requirement: Targeted packet live receipts

r[mc_compatibility.targeted_packet_live_parity.receipts] Live parity evidence MUST be recorded as reviewable logs and receipts before matrix promotion.

#### Scenario: Live receipts are non-overclaiming

r[mc_compatibility.targeted_packet_live_parity.receipts.non_overclaiming]
- GIVEN selected packet behavior is observed live
- WHEN receipts are written
- THEN they include packet row identifiers, scenario names, backend/client revisions when available, command/check context, and explicit false claims for full public-server safety, production readiness, broad gameplay semantics, and full protocol coverage.

### Requirement: Targeted packet matrix promotion

r[mc_compatibility.targeted_packet_live_parity.matrix] The acceptance matrix and current evidence bundle MUST promote only packet rows with passing live evidence.

#### Scenario: Matrix updates follow evidence

r[mc_compatibility.targeted_packet_live_parity.matrix.gated]
- GIVEN live evidence exists for some selected rows and not others
- WHEN the matrix and bundle are updated
- THEN only rows with passing live receipts move beyond fixture-bounded status
- AND unproven rows retain their prior evidence classification and non-claim notes.

### Requirement: Targeted packet live checker tests

r[mc_compatibility.targeted_packet_live_parity.tests] The change MUST include positive and negative targeted-packet checker tests for live promotion rules.

#### Scenario: Checker rejects unsafe promotion

r[mc_compatibility.targeted_packet_live_parity.tests.negative]
- GIVEN a packet row lacks live evidence, cites the wrong packet identifier, has a stale receipt digest, or claims full protocol coverage
- WHEN targeted packet checks run
- THEN the checks fail with explicit diagnostics instead of accepting the promotion.

### Requirement: Targeted packet live validation

r[mc_compatibility.targeted_packet_live_parity.validation] The change MUST record runner checks, targeted packet checks, evidence checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Live promotion evidence is reviewable

r[mc_compatibility.targeted_packet_live_parity.validation.logs]
- GIVEN live packet promotion work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, live probe checks or documented blockers for unpromoted rows, targeted packet checker tests, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, and Cairn validation passing.
