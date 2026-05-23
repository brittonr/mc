# Mc Compatibility Delta: ScoreboardObjectiveUpdateS2CPacket protocol 763 boundary

## Requirements

### Requirement: Protocol 763 ScoreboardObjectiveUpdateS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.scoreboard_objective.shape_reviewed] The system MUST review the Valence protocol 763 `ScoreboardObjectiveUpdateS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x58`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.scoreboard_objective.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `ScoreboardObjectiveUpdateS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 ScoreboardObjectiveUpdateS2CPacket mapping is updated

r[mc_compatibility.protocol_763.scoreboard_objective.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x58` to the reviewed Stevenarella semantic for `ScoreboardObjectiveUpdateS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.scoreboard_objective.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x58` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `ScoreboardObjectiveUpdateS2CPacket`
- THEN the implementation no longer treats `0x58` as `TitleSubtitle`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.scoreboard_objective.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `ScoreboardObjectiveUpdateS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x58`

### Requirement: Protocol 763 ScoreboardObjectiveUpdateS2CPacket update is verified

r[mc_compatibility.protocol_763.scoreboard_objective.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `ScoreboardObjectiveUpdateS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.scoreboard_objective.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x58` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after ScoreboardObjectiveUpdateS2CPacket

r[mc_compatibility.protocol_763.scoreboard_objective.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x58` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.scoreboard_objective.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x58`

### Requirement: Protocol 763 ScoreboardObjectiveUpdateS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.scoreboard_objective.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `ScoreboardObjectiveUpdateS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.scoreboard_objective.evidence_recorded.non_overclaiming]
- GIVEN the `ScoreboardObjectiveUpdateS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x58` / `ScoreboardObjectiveUpdateS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false
