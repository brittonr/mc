# Delta: Evidence manifest checker

## Requirements

### Requirement: Manifest Integrity

r[mc_compatibility.roi_06_evidence_manifest_checker.manifest_integrity] The repo MUST provide a maintained local checker that verifies every tracked `docs/evidence/*.b3` manifest references existing repo-local files with matching BLAKE3 digests.

#### Scenario: Manifest integrity is checked

r[mc_compatibility.roi_06_evidence_manifest_checker.manifest_integrity.scenario]
- GIVEN evidence manifests are committed under `docs/evidence`
- WHEN the evidence manifest checker runs
- THEN each manifest entry is parsed as a BLAKE3 digest plus repo-relative path
- AND each referenced file exists
- AND each digest matches the referenced file bytes

### Requirement: Stale Marker Guard

r[mc_compatibility.roi_06_evidence_manifest_checker.stale_marker_guard] The checker MUST reject stale receipt milestone names that are known to conflict with current maintained runner semantics.

#### Scenario: Stale marker names fail the gate

r[mc_compatibility.roi_06_evidence_manifest_checker.stale_marker_guard.scenario]
- GIVEN a tracked receipt contains a stale milestone name such as `equipment_packet_observed`
- WHEN the evidence manifest checker runs
- THEN the checker fails with an actionable stale-marker diagnostic
- AND current marker names such as `entity_equipment_update` remain allowed

### Requirement: Maintained Gate

r[mc_compatibility.roi_06_evidence_manifest_checker.maintained_gate] The evidence manifest checker MUST be documented and wired into the maintained local check set.

#### Scenario: Maintained gate is discoverable

r[mc_compatibility.roi_06_evidence_manifest_checker.maintained_gate.scenario]
- GIVEN an operator follows current evidence verification instructions
- WHEN they run the maintained local checks
- THEN the evidence manifest checker is included alongside matrix, bundle, and Cairn validation checks
