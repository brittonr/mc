# Delta: Equipment update observation rail

## Requirements

### Requirement: Equipment Packet Observation

r[mc_compatibility.roi_02_equipment_update_rail.equipment_packet_observation] The repo MUST add a bounded protocol-763 rail for client-observed entity equipment updates independent of combat mitigation math.

#### Scenario: Equipment Packet Observation evidence is required

r[mc_compatibility.roi_02_equipment_update_rail.equipment_packet_observation.scenario]
- GIVEN `Equipment update observation rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `equipment_packet_observation` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Server Equipment Milestone

r[mc_compatibility.roi_02_equipment_update_rail.server_equipment_milestone] Valence or the maintained target MUST emit deterministic equipment setup/update milestones that can be correlated with the client observation.

#### Scenario: Server Equipment Milestone evidence is required

r[mc_compatibility.roi_02_equipment_update_rail.server_equipment_milestone.scenario]
- GIVEN `Equipment update observation rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `server_equipment_milestone` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Receipt And Gate

r[mc_compatibility.roi_02_equipment_update_rail.receipt_and_gate] The rail MUST have a live receipt, BLAKE3 evidence doc, and deterministic dry-run check that prevents stale or missing equipment-update milestone coverage.

#### Scenario: Receipt And Gate evidence is required

r[mc_compatibility.roi_02_equipment_update_rail.receipt_and_gate.scenario]
- GIVEN `Equipment update observation rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `receipt_and_gate` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant
