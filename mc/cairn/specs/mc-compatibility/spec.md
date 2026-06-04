# Mc Compatibility Specification

## Purpose

Defines the `mc-compatibility` capability for Minecraft client/server compatibility evidence, including observed Stevenarella protocol 763 boundary mappings against Valence.

## Requirements

### Requirement: Protocol 763 AdvancementUpdateS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.advancement_update.shape_reviewed] The system MUST review the Valence protocol 763 `AdvancementUpdateS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x69`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.advancement_update.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `AdvancementUpdateS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 AdvancementUpdateS2CPacket mapping is updated

r[mc_compatibility.protocol_763.advancement_update.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x69` to the reviewed Stevenarella semantic for `AdvancementUpdateS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.advancement_update.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x69` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `AdvancementUpdateS2CPacket`
- THEN the implementation no longer leaves `0x69` unmapped through the 758 fallback

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.advancement_update.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `AdvancementUpdateS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x69`

### Requirement: Protocol 763 AdvancementUpdateS2CPacket update is verified

r[mc_compatibility.protocol_763.advancement_update.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `AdvancementUpdateS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.advancement_update.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x69` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after AdvancementUpdateS2CPacket

r[mc_compatibility.protocol_763.advancement_update.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x69` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.advancement_update.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x69`

### Requirement: Protocol 763 AdvancementUpdateS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.advancement_update.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `AdvancementUpdateS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.advancement_update.evidence_recorded.non_overclaiming]
- GIVEN the `AdvancementUpdateS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x69` / `AdvancementUpdateS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

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

### Requirement: Protocol 763 ScoreboardDisplayS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.scoreboard_display.shape_reviewed] The system MUST review the Valence protocol 763 `ScoreboardDisplayS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x51`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.scoreboard_display.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `ScoreboardDisplayS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 ScoreboardDisplayS2CPacket mapping is updated

r[mc_compatibility.protocol_763.scoreboard_display.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x51` to the reviewed Stevenarella semantic for `ScoreboardDisplayS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.scoreboard_display.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x51` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `ScoreboardDisplayS2CPacket`
- THEN the implementation no longer treats `0x51` as `SetExperience`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.scoreboard_display.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `ScoreboardDisplayS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x51`

### Requirement: Protocol 763 ScoreboardDisplayS2CPacket update is verified

r[mc_compatibility.protocol_763.scoreboard_display.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `ScoreboardDisplayS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.scoreboard_display.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x51` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after ScoreboardDisplayS2CPacket

r[mc_compatibility.protocol_763.scoreboard_display.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x51` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.scoreboard_display.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x51`

### Requirement: Protocol 763 ScoreboardDisplayS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.scoreboard_display.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `ScoreboardDisplayS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.scoreboard_display.evidence_recorded.non_overclaiming]
- GIVEN the `ScoreboardDisplayS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x51` / `ScoreboardDisplayS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 ScoreboardPlayerUpdateS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.scoreboard_player.shape_reviewed] The system MUST review the Valence protocol 763 `ScoreboardPlayerUpdateS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x5b`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.scoreboard_player.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `ScoreboardPlayerUpdateS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 ScoreboardPlayerUpdateS2CPacket mapping is updated

r[mc_compatibility.protocol_763.scoreboard_player.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x5b` to the reviewed Stevenarella semantic for `ScoreboardPlayerUpdateS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.scoreboard_player.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x5b` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `ScoreboardPlayerUpdateS2CPacket`
- THEN the implementation no longer treats `0x5b` as `TitleTimes`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.scoreboard_player.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `ScoreboardPlayerUpdateS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x5b`

### Requirement: Protocol 763 ScoreboardPlayerUpdateS2CPacket update is verified

r[mc_compatibility.protocol_763.scoreboard_player.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `ScoreboardPlayerUpdateS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.scoreboard_player.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x5b` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after ScoreboardPlayerUpdateS2CPacket

r[mc_compatibility.protocol_763.scoreboard_player.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x5b` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.scoreboard_player.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x5b`

### Requirement: Protocol 763 ScoreboardPlayerUpdateS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.scoreboard_player.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `ScoreboardPlayerUpdateS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.scoreboard_player.evidence_recorded.non_overclaiming]
- GIVEN the `ScoreboardPlayerUpdateS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x5b` / `ScoreboardPlayerUpdateS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 UpdateSelectedSlotS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.selected_slot.shape_reviewed] The system MUST review the Valence protocol 763 `UpdateSelectedSlotS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x4d`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.selected_slot.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `UpdateSelectedSlotS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 UpdateSelectedSlotS2CPacket mapping is updated

r[mc_compatibility.protocol_763.selected_slot.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x4d` to the reviewed Stevenarella semantic for `UpdateSelectedSlotS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.selected_slot.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x4d` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `UpdateSelectedSlotS2CPacket`
- THEN the implementation no longer treats `0x4d` as `EntityMetadata`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.selected_slot.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `UpdateSelectedSlotS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x4d`

### Requirement: Protocol 763 UpdateSelectedSlotS2CPacket update is verified

r[mc_compatibility.protocol_763.selected_slot.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `UpdateSelectedSlotS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.selected_slot.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x4d` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after UpdateSelectedSlotS2CPacket

r[mc_compatibility.protocol_763.selected_slot.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x4d` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.selected_slot.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x4d`

### Requirement: Protocol 763 UpdateSelectedSlotS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.selected_slot.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `UpdateSelectedSlotS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.selected_slot.evidence_recorded.non_overclaiming]
- GIVEN the `UpdateSelectedSlotS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x4d` / `UpdateSelectedSlotS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 ScreenHandlerSlotUpdateS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.screen_handler_slot.shape_reviewed] The system MUST review the Valence protocol 763 `ScreenHandlerSlotUpdateS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x14`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.screen_handler_slot.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `ScreenHandlerSlotUpdateS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 ScreenHandlerSlotUpdateS2CPacket mapping is updated

r[mc_compatibility.protocol_763.screen_handler_slot.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x14` to the reviewed Stevenarella semantic for `ScreenHandlerSlotUpdateS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.screen_handler_slot.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x14` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `ScreenHandlerSlotUpdateS2CPacket`
- THEN the implementation no longer treats `0x14` as `WindowItems_StateCarry`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.screen_handler_slot.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `ScreenHandlerSlotUpdateS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x14`

### Requirement: Protocol 763 ScreenHandlerSlotUpdateS2CPacket update is verified

r[mc_compatibility.protocol_763.screen_handler_slot.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `ScreenHandlerSlotUpdateS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.screen_handler_slot.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x14` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after ScreenHandlerSlotUpdateS2CPacket

r[mc_compatibility.protocol_763.screen_handler_slot.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x14` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.screen_handler_slot.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x14`

### Requirement: Protocol 763 ScreenHandlerSlotUpdateS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.screen_handler_slot.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `ScreenHandlerSlotUpdateS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.screen_handler_slot.evidence_recorded.non_overclaiming]
- GIVEN the `ScreenHandlerSlotUpdateS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x14` / `ScreenHandlerSlotUpdateS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 PlayerListS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.player_list.shape_reviewed] The system MUST review the Valence protocol 763 `PlayerListS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x3a`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.player_list.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `PlayerListS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 PlayerListS2CPacket mapping is updated

r[mc_compatibility.protocol_763.player_list.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x3a` to the reviewed Stevenarella semantic for `PlayerListS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.player_list.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x3a` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `PlayerListS2CPacket`
- THEN the implementation no longer treats `0x3a` as `EntityDestroy`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.player_list.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `PlayerListS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x3a`

### Requirement: Protocol 763 PlayerListS2CPacket update is verified

r[mc_compatibility.protocol_763.player_list.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `PlayerListS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.player_list.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x3a` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after PlayerListS2CPacket

r[mc_compatibility.protocol_763.player_list.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x3a` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.player_list.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x3a`

### Requirement: Protocol 763 PlayerListS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.player_list.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `PlayerListS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.player_list.evidence_recorded.non_overclaiming]
- GIVEN the `PlayerListS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x3a` / `PlayerListS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 HealthUpdateS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.health_update.shape_reviewed] The system MUST review the Valence protocol 763 `HealthUpdateS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x57`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.health_update.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `HealthUpdateS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 HealthUpdateS2CPacket mapping is updated

r[mc_compatibility.protocol_763.health_update.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x57` to the reviewed Stevenarella semantic for `HealthUpdateS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.health_update.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x57` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `HealthUpdateS2CPacket`
- THEN the implementation no longer treats `0x57` as `UpdateSimulationDistance`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.health_update.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `HealthUpdateS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x57`

### Requirement: Protocol 763 HealthUpdateS2CPacket update is verified

r[mc_compatibility.protocol_763.health_update.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `HealthUpdateS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.health_update.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x57` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after HealthUpdateS2CPacket

r[mc_compatibility.protocol_763.health_update.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x57` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.health_update.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x57`

### Requirement: Protocol 763 HealthUpdateS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.health_update.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `HealthUpdateS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.health_update.evidence_recorded.non_overclaiming]
- GIVEN the `HealthUpdateS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x57` / `HealthUpdateS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 EntityTrackerUpdateS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.entity_tracker_update.shape_reviewed] The system MUST review the Valence protocol 763 `EntityTrackerUpdateS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x52`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.entity_tracker_update.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `EntityTrackerUpdateS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 EntityTrackerUpdateS2CPacket mapping is updated

r[mc_compatibility.protocol_763.entity_tracker_update.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x52` to the reviewed Stevenarella semantic for `EntityTrackerUpdateS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.entity_tracker_update.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x52` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `EntityTrackerUpdateS2CPacket`
- THEN the implementation no longer treats `0x52` as `UpdateHealth`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.entity_tracker_update.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `EntityTrackerUpdateS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x52`

### Requirement: Protocol 763 EntityTrackerUpdateS2CPacket update is verified

r[mc_compatibility.protocol_763.entity_tracker_update.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `EntityTrackerUpdateS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.entity_tracker_update.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x52` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after EntityTrackerUpdateS2CPacket

r[mc_compatibility.protocol_763.entity_tracker_update.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x52` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.entity_tracker_update.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x52`

### Requirement: Protocol 763 EntityTrackerUpdateS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.entity_tracker_update.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `EntityTrackerUpdateS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.entity_tracker_update.evidence_recorded.non_overclaiming]
- GIVEN the `EntityTrackerUpdateS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x52` / `EntityTrackerUpdateS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 EntityAttributesS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.entity_attributes.shape_reviewed] The system MUST review the Valence protocol 763 `EntityAttributesS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x6a`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.entity_attributes.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `EntityAttributesS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 EntityAttributesS2CPacket mapping is updated

r[mc_compatibility.protocol_763.entity_attributes.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x6a` to the reviewed Stevenarella semantic for `EntityAttributesS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.entity_attributes.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x6a` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `EntityAttributesS2CPacket`
- THEN the implementation no longer leaves `0x6a` unmapped through the 758 fallback

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.entity_attributes.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `EntityAttributesS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x6a`

### Requirement: Protocol 763 EntityAttributesS2CPacket update is verified

r[mc_compatibility.protocol_763.entity_attributes.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `EntityAttributesS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.entity_attributes.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x6a` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after EntityAttributesS2CPacket

r[mc_compatibility.protocol_763.entity_attributes.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x6a` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.entity_attributes.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x6a`

### Requirement: Protocol 763 EntityAttributesS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.entity_attributes.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `EntityAttributesS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.entity_attributes.evidence_recorded.non_overclaiming]
- GIVEN the `EntityAttributesS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x6a` / `EntityAttributesS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 EntityStatusS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.entity_status.shape_reviewed] The system MUST review the Valence protocol 763 `EntityStatusS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x1c`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.entity_status.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `EntityStatusS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 EntityStatusS2CPacket mapping is updated

r[mc_compatibility.protocol_763.entity_status.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x1c` to the reviewed Stevenarella semantic for `EntityStatusS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.entity_status.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x1c` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `EntityStatusS2CPacket`
- THEN the implementation no longer treats `0x1c` as `Explosion_VarInt`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.entity_status.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `EntityStatusS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x1c`

### Requirement: Protocol 763 EntityStatusS2CPacket update is verified

r[mc_compatibility.protocol_763.entity_status.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `EntityStatusS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.entity_status.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x1c` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after EntityStatusS2CPacket

r[mc_compatibility.protocol_763.entity_status.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x1c` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.entity_status.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x1c`

### Requirement: Protocol 763 EntityStatusS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.entity_status.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `EntityStatusS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.entity_status.evidence_recorded.non_overclaiming]
- GIVEN the `EntityStatusS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x1c` / `EntityStatusS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 PlayerAbilitiesS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.player_abilities.shape_reviewed] The system MUST review the Valence protocol 763 `PlayerAbilitiesS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x34`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.player_abilities.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `PlayerAbilitiesS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 PlayerAbilitiesS2CPacket mapping is updated

r[mc_compatibility.protocol_763.player_abilities.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x34` to the reviewed Stevenarella semantic for `PlayerAbilitiesS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.player_abilities.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x34` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `PlayerAbilitiesS2CPacket`
- THEN the implementation no longer treats `0x34` as `CombatEventEnter`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.player_abilities.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `PlayerAbilitiesS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x34`

### Requirement: Protocol 763 PlayerAbilitiesS2CPacket update is verified

r[mc_compatibility.protocol_763.player_abilities.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `PlayerAbilitiesS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.player_abilities.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x34` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after PlayerAbilitiesS2CPacket

r[mc_compatibility.protocol_763.player_abilities.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x34` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.player_abilities.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x34`

### Requirement: Protocol 763 PlayerAbilitiesS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.player_abilities.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `PlayerAbilitiesS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.player_abilities.evidence_recorded.non_overclaiming]
- GIVEN the `PlayerAbilitiesS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x34` / `PlayerAbilitiesS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 SynchronizeTagsS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.synchronize_tags.shape_reviewed] The system MUST review the Valence protocol 763 `SynchronizeTagsS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x6e`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.synchronize_tags.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `SynchronizeTagsS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 SynchronizeTagsS2CPacket mapping is updated

r[mc_compatibility.protocol_763.synchronize_tags.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x6e` to the reviewed Stevenarella semantic for `SynchronizeTagsS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.synchronize_tags.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x6e` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `SynchronizeTagsS2CPacket`
- THEN the implementation no longer leaves `0x6e` unmapped through the 758 fallback

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.synchronize_tags.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `SynchronizeTagsS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x6e`

### Requirement: Protocol 763 SynchronizeTagsS2CPacket update is verified

r[mc_compatibility.protocol_763.synchronize_tags.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `SynchronizeTagsS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.synchronize_tags.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x6e` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after SynchronizeTagsS2CPacket

r[mc_compatibility.protocol_763.synchronize_tags.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x6e` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.synchronize_tags.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x6e`

### Requirement: Protocol 763 SynchronizeTagsS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.synchronize_tags.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `SynchronizeTagsS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.synchronize_tags.evidence_recorded.non_overclaiming]
- GIVEN the `SynchronizeTagsS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x6e` / `SynchronizeTagsS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Protocol 763 ChunkDataS2CPacket shape is reviewed

r[mc_compatibility.protocol_763.chunk_data.shape_reviewed] The system MUST review the Valence protocol 763 `ChunkDataS2CPacket` packet shape and the Stevenarella internal packet/parser shape before changing `play/clientbound/0x24`.

#### Scenario: Boundary shape review is recorded

r[mc_compatibility.protocol_763.chunk_data.shape_reviewed.recorded]
- GIVEN the boundary is selected for implementation
- WHEN the operator inspects Valence and Stevenarella packet definitions
- THEN the implementation notes whether an existing Stevenarella internal packet is shape-compatible with `ChunkDataS2CPacket`
- AND the implementation does not use a packet-id alias when the packet shape is incompatible

### Requirement: Protocol 763 ChunkDataS2CPacket mapping is updated

r[mc_compatibility.protocol_763.chunk_data.mapping_updated] The system MUST map protocol 763 `play/clientbound/0x24` to the reviewed Stevenarella semantic for `ChunkDataS2CPacket` instead of relying on the inherited 758 fallback.

#### Scenario: Clientbound mapping resolves to the reviewed semantic

r[mc_compatibility.protocol_763.chunk_data.mapping_updated.clientbound]
- GIVEN protocol 763 packet translation is active
- WHEN `play/clientbound/0x24` is translated to Stevenarella's internal packet id
- THEN the result is the reviewed internal semantic for `ChunkDataS2CPacket`
- THEN the implementation no longer treats `0x24` as `Particle_f64`

#### Scenario: Reverse mapping stays stable

r[mc_compatibility.protocol_763.chunk_data.mapping_updated.reverse]
- GIVEN the reviewed internal semantic for `ChunkDataS2CPacket` is emitted under protocol 763
- WHEN Stevenarella translates it back to a wire id
- THEN the result is `play/clientbound/0x24`

### Requirement: Protocol 763 ChunkDataS2CPacket update is verified

r[mc_compatibility.protocol_763.chunk_data.tests_cover_mapping] The system MUST include focused positive and negative regression tests for the `ChunkDataS2CPacket` boundary.

#### Scenario: Focused tests cover the boundary

r[mc_compatibility.protocol_763.chunk_data.tests_cover_mapping.focused]
- GIVEN the mapping update is implemented
- WHEN the focused `steven_protocol` version tests run
- THEN the tests prove `0x24` maps to the reviewed semantic and does not resolve through the inherited fallback

### Requirement: Protocol 763 trace advances after ChunkDataS2CPacket

r[mc_compatibility.protocol_763.chunk_data.trace_advances] The system MUST rerun the Valence `ctf` trace or equivalent probe after updating `0x24` and identify the next unresolved boundary.

#### Scenario: Trace identifies the next boundary

r[mc_compatibility.protocol_763.chunk_data.trace_advances.next_boundary]
- GIVEN the focused tests pass
- WHEN the Valence `ctf` protocol 763 trace/probe is rerun
- THEN the trace confirms the prior mapped boundaries still resolve
- AND the trace records the next unresolved packet boundary after `0x24`

### Requirement: Protocol 763 ChunkDataS2CPacket evidence is recorded

r[mc_compatibility.protocol_763.chunk_data.evidence_recorded] The system MUST record deterministic parent `mc` evidence for the `ChunkDataS2CPacket` update without claiming full current-Valence or full Stevenarella protocol 763 support.

#### Scenario: Evidence receipt is non-overclaiming

r[mc_compatibility.protocol_763.chunk_data.evidence_recorded.non_overclaiming]
- GIVEN the `ChunkDataS2CPacket` update is verified
- WHEN the parent evidence receipt/check is created
- THEN the receipt claims only the `0x24` / `ChunkDataS2CPacket` boundary update
- AND the receipt keeps full-current-Valence and full-Stevenarella-763 support claims false

### Requirement: Compatibility testing

r[mc_compatibility.valence_compat_bot.safe_bounded_probe] Valence compatibility testing MUST provide a bounded owned bot/probe mode derived from the Hyperion bot pattern without enabling unbounded public stress traffic.

#### Scenario: Probe run is bounded

r[mc_compatibility.valence_compat_bot.safe_bounded_probe.scenario]

- GIVEN the operator selects the compat bot probe

- WHEN the probe is started against a local owned Valence example

- THEN the run uses bounded clients, duration, and target address controls

- AND the receipt records that it does not authorize external-server load testing

### Requirement: The compat bot probe

r[mc_compatibility.valence_compat_bot.semantic_receipt] The compat bot probe MUST emit deterministic semantic receipts for client-side milestones and non-claims.

#### Scenario: Receipt records milestones

r[mc_compatibility.valence_compat_bot.semantic_receipt.scenario]

- GIVEN the probe completes or fails

- WHEN the runner writes its receipt

- THEN the receipt records required, observed, and missing client milestones

- AND the receipt records target, duration/window, scenario, and explicit non-claims

### Requirement: The compat bot probe

r[mc_compatibility.valence_compat_bot.valence_gate] The compat bot probe MUST be covered by a focused deterministic gate before live acceptance evidence is claimed.

#### Scenario: Dry-run gate validates receipt shape

r[mc_compatibility.valence_compat_bot.valence_gate.scenario]

- GIVEN the probe surface exists

- WHEN the deterministic gate runs in Nix dry-run mode

- THEN the gate validates schema, scenario, milestone, and non-claim fields

### Requirement: Status ping response data

r[mc_compatibility.valence_status_response.resource_owned] Valence status ping response data MUST be configurable through a resource or equivalent public server setting.

#### Scenario: Configured status response is used

r[mc_compatibility.valence_status_response.resource_owned.scenario]

- GIVEN an example inserts configured status response data
- WHEN a client sends a status ping
- THEN the response uses the configured description/version/sample fields

### Requirement: The status response resource

r[mc_compatibility.valence_status_response.defaults_stable] The status response resource MUST preserve existing default behavior for examples that do not configure it.

#### Scenario: Default status response remains available

r[mc_compatibility.valence_status_response.defaults_stable.scenario]

- GIVEN an example does not configure custom status data
- WHEN a client sends a status ping
- THEN the response remains valid and compatible with the prior default behavior

### Requirement: Status response behavior

r[mc_compatibility.valence_status_response.test_oracle] Status response behavior MUST be testable as a deterministic compatibility oracle.

#### Scenario: Status probe asserts configured data

r[mc_compatibility.valence_status_response.test_oracle.scenario]

- GIVEN a test or smoke probe sets known status data
- WHEN the status-only probe runs
- THEN the receipt or assertion records the configured values that were observed

### Requirement: Compatibility testing packet capture

r[mc_compatibility.valence_packet_capture.headless_cli] Valence compatibility testing MUST provide a headless packet-capture oracle for local owned client/server runs.

#### Scenario: Capture starts for a local run

r[mc_compatibility.valence_packet_capture.headless_cli.scenario]

- GIVEN a local Valence example and client probe are selected
- WHEN the capture oracle is invoked
- THEN the oracle records direction, protocol state, packet id, decode status, and bounded timing metadata

### Requirement: Packet-capture evidence

r[mc_compatibility.valence_packet_capture.redacted_receipt] Packet-capture evidence MUST be normalized and redacted before it is recorded as durable evidence.

#### Scenario: Receipt omits raw sensitive traffic

r[mc_compatibility.valence_packet_capture.redacted_receipt.scenario]

- GIVEN packet capture data is converted into a receipt
- WHEN the receipt is written
- THEN the receipt includes normalized packet summaries and decode failures
- AND the receipt excludes raw payload dumps unless an explicit local debug artifact is requested

### Requirement: Packet-capture receipts

r[mc_compatibility.valence_packet_capture.triage_correlation] Packet-capture receipts MUST correlate with scenario triage when a compatibility run fails.

#### Scenario: Capture points at failing boundary

r[mc_compatibility.valence_packet_capture.triage_correlation.scenario]

- GIVEN a scenario receipt reports a protocol-runtime or client-probe failure
- WHEN the packet capture summary is attached or compared
- THEN the combined evidence identifies the first relevant packet/state boundary when available

### Requirement: Proxy compatibility work

r[mc_compatibility.valence_proxy_compat.boundary_documented] Valence proxy compatibility work MUST define direct, proxied, and deferred architecture boundaries before implementation.

#### Scenario: Boundary is explicit

r[mc_compatibility.valence_proxy_compat.boundary_documented.scenario]

- GIVEN the proxy compatibility seam is planned
- WHEN the proposal/design is reviewed
- THEN the plan records direct-vs-proxy claims and defers full Hyperion-style multi-proxy/mTLS unless separately requested

### Requirement: Proxy-path compatibility receipts

r[mc_compatibility.valence_proxy_compat.receipt_fields] Proxy-path compatibility receipts MUST record the route, forwarding mode, observed protocol/version, and non-claims.

#### Scenario: Proxy receipt identifies route

r[mc_compatibility.valence_proxy_compat.receipt_fields.scenario]

- GIVEN a proxy-path smoke or dry-run executes
- WHEN the receipt is written
- THEN the receipt records direct or proxied route, forwarding mode if known, observed protocol/version, and non-claims

### Requirement: Proxy compatibility

r[mc_compatibility.valence_proxy_compat.local_gate] Proxy compatibility MUST have a deterministic local gate before live proxy evidence is treated as durable.

#### Scenario: Dry-run proxy gate validates shape

r[mc_compatibility.valence_proxy_compat.local_gate.scenario]

- GIVEN the proxy compatibility receipt schema exists
- WHEN the dry-run gate runs
- THEN the gate validates route, forwarding-mode, protocol, and non-claim fields

### Requirement: Gameplay compatibility work

r[mc_compatibility.valence_gameplay_oracles.milestone_catalog] Valence compatibility work MUST catalog Hyperion-derived gameplay milestones before implementing new scenario claims.

#### Scenario: Milestones are mapped to Valence

r[mc_compatibility.valence_gameplay_oracles.milestone_catalog.scenario]

- GIVEN Hyperion Bedwars milestones are reviewed
- WHEN the Valence scenario plan is written
- THEN the plan maps each selected milestone to a Valence example/client/server evidence source

### Requirement: Gameplay scenario receipts

r[mc_compatibility.valence_gameplay_oracles.correlated_receipts] Valence gameplay scenario receipts MUST require correlated client and server evidence for semantic gameplay claims.

#### Scenario: Scenario requires both sides

r[mc_compatibility.valence_gameplay_oracles.correlated_receipts.scenario]

- GIVEN a non-smoke gameplay scenario runs
- WHEN the receipt is evaluated
- THEN the scenario passes only when required client milestones and server correlation are both observed

### Requirement: Gameplay oracle receipts

r[mc_compatibility.valence_gameplay_oracles.non_overclaiming] Valence gameplay oracle receipts MUST preserve explicit non-claims for unsupported gameplay and soak properties.

#### Scenario: Receipt states non-claims

r[mc_compatibility.valence_gameplay_oracles.non_overclaiming.scenario]

- GIVEN a gameplay scenario receipt is recorded
- WHEN the evidence is reviewed
- THEN the receipt states the exact supported claim and keeps full CTF, broad protocol, and unbounded soak claims false

### Requirement: Cross-file freshness

r[mc_compatibility.harden_evidence_freshness_gates.cross_file_freshness] Evidence freshness checks MUST verify consistency among acceptance matrix rows, current bundle rows, receipt copies, run logs, and BLAKE3 manifests.

#### Scenario: Freshness checker rejects drift

r[mc_compatibility.harden_evidence_freshness_gates.cross_file_freshness.scenario]
- GIVEN evidence rows are indexed for maintained compatibility claims
- WHEN the freshness checker runs
- THEN it verifies referenced artifacts exist, recorded BLAKE3 hashes match file contents, and matrix/current-bundle rows agree on scoped claims
- AND it fails on missing, stale, or contradictory evidence

### Requirement: Freshness fixtures

r[mc_compatibility.harden_evidence_freshness_gates.freshness_fixtures] The evidence freshness checker MUST include positive and negative fixtures for complete rows, stale hashes, missing matrix rows, missing bundle rows, missing run logs, and target-only receipts.

#### Scenario: Stale artifact fixtures fail closed

r[mc_compatibility.harden_evidence_freshness_gates.freshness_fixtures.scenario]
- GIVEN freshness fixtures are executed
- WHEN a fixture contains a stale hash, missing row, missing artifact, or target-only live receipt
- THEN the checker fails with explicit diagnostics
- AND no evidence promotion can rely on that fixture

### Requirement: Reviewable artifacts

r[mc_compatibility.harden_evidence_freshness_gates.reviewable_artifacts] Review-critical live receipts and logs MUST be copied under `docs/evidence/` with BLAKE3 manifests before they are cited by tasks, matrix rows, or bundle rows.

#### Scenario: Target-only evidence is rejected

r[mc_compatibility.harden_evidence_freshness_gates.reviewable_artifacts.scenario]
- GIVEN a task, matrix row, or bundle row cites a live receipt
- WHEN the cited artifact exists only under `target/`
- THEN the gate rejects the promotion
- AND the task must either copy the artifact under `docs/evidence/` or record an explicit historical/oracle decision

### Requirement: Promotion gate

r[mc_compatibility.harden_evidence_freshness_gates.promotion_gate] New or replaced compatibility evidence rows MUST run the documented promotion gate before claims are broadened or receipt hashes are updated.

#### Scenario: Promotion gate is required for row updates

r[mc_compatibility.harden_evidence_freshness_gates.promotion_gate.scenario]
- GIVEN a compatibility evidence row is added or its receipt hash changes
- WHEN the change is reviewed
- THEN the matrix checker, current-bundle checker, manifest checker, Cairn validation, and row-specific dry-run gate have tracked output or evidence
- AND promotion is blocked if any gate fails

### Requirement: Authorization preflight

r[mc_compatibility.prove_production_load_network_safety.authorization_preflight] Load and network safety proofs MUST fail before launch unless the target is owned local infrastructure or explicitly authorized for the bounded experiment.

#### Scenario: Unauthorized or public target is rejected

r[mc_compatibility.prove_production_load_network_safety.authorization_preflight.scenario]
- GIVEN a load or network experiment is requested
- WHEN the target is public, unowned, or lacks explicit authorization evidence
- THEN the runner rejects the experiment before starting clients or traffic
- AND the receipt records the authorization failure without claiming safety

### Requirement: Bounded envelopes

r[mc_compatibility.prove_production_load_network_safety.bounded_envelopes] Production/load/network claims MUST be expressed as bounded envelopes with explicit client count, duration, reconnect count, latency, jitter, packet-loss, and resource limits.

#### Scenario: Envelope parameters are explicit

r[mc_compatibility.prove_production_load_network_safety.bounded_envelopes.scenario]
- GIVEN a safety envelope is proposed
- WHEN the envelope is reviewed
- THEN every bound and unit is recorded in the plan and receipt
- AND unbounded or unspecified parameters fail the gate

### Requirement: Envelope fixtures

r[mc_compatibility.prove_production_load_network_safety.envelope_fixtures] Load/network envelope logic MUST include positive and negative fixtures for authorized bounded runs, missing authorization, public targets, unbounded parameters, and missing telemetry.

#### Scenario: Unsafe envelope fixture fails closed

r[mc_compatibility.prove_production_load_network_safety.envelope_fixtures.scenario]
- GIVEN envelope fixtures are executed
- WHEN authorization is missing, target scope is unsafe, bounds are unbounded, or telemetry is missing
- THEN the fixture fails with explicit diagnostics
- AND no production/load/network claim is promoted

### Requirement: Safety promotion gate

r[mc_compatibility.prove_production_load_network_safety.safety_promotion_gate] Public-server safety, production readiness, unbounded soak, unbounded reconnect, WAN, adversarial network, and packet-loss claims MUST remain non-claims unless an authorized bounded envelope has passing tests, live receipts, telemetry, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Promotion requires authorized bounded evidence

r[mc_compatibility.prove_production_load_network_safety.safety_promotion_gate.scenario]
- GIVEN a load or network safety claim is proposed
- WHEN any authorization, bound, telemetry, or receipt evidence is missing
- THEN promotion is rejected
- AND the current bundle keeps broader production and network safety as non-claims

### Requirement: Lifecycle model

r[mc_compatibility.prove_death_respawn_lifecycle.lifecycle_model] The death/respawn proof MUST define lifecycle states, allowed transitions, forbidden transitions, and server/client evidence requirements before claiming full lifecycle correctness.

#### Scenario: Lifecycle model scopes death and respawn claims

r[mc_compatibility.prove_death_respawn_lifecycle.lifecycle_model.scenario]
- GIVEN death/respawn correctness is being evaluated
- WHEN the lifecycle model is reviewed
- THEN each state and transition records expected server evidence, expected client observation, forbidden milestones, and non-claim status
- AND unmodeled transitions remain non-claims

### Requirement: Positive lifecycle scenarios

r[mc_compatibility.prove_death_respawn_lifecycle.positive_lifecycle_scenarios] Valid death/respawn transitions MUST have positive scenarios that correlate Valence lifecycle state with Stevenarella observations.

#### Scenario: Valid lifecycle transition is observed

r[mc_compatibility.prove_death_respawn_lifecycle.positive_lifecycle_scenarios.scenario]
- GIVEN a valid lifecycle transition row is selected
- WHEN the scenario runs
- THEN Valence records the authoritative death/respawn state and Stevenarella records the expected user-visible observation
- AND the receipt records no missing lifecycle milestones for that row

### Requirement: Negative lifecycle scenarios

r[mc_compatibility.prove_death_respawn_lifecycle.negative_lifecycle_scenarios] Lifecycle verification MUST reject duplicate, missing, forbidden, or out-of-order death/respawn evidence.

#### Scenario: Invalid lifecycle evidence fails

r[mc_compatibility.prove_death_respawn_lifecycle.negative_lifecycle_scenarios.scenario]
- GIVEN lifecycle evidence contains a duplicate, missing, forbidden, or out-of-order transition
- WHEN the runner evaluates the row
- THEN the row fails with explicit diagnostics
- AND no full death/respawn lifecycle claim is promoted

### Requirement: Lifecycle promotion gate

r[mc_compatibility.prove_death_respawn_lifecycle.lifecycle_promotion_gate] Full death/respawn lifecycle correctness MUST remain a non-claim until required lifecycle rows have passing deterministic tests, live receipts, BLAKE3 manifests, and evidence index updates.

#### Scenario: Lifecycle promotion requires row coverage

r[mc_compatibility.prove_death_respawn_lifecycle.lifecycle_promotion_gate.scenario]
- GIVEN full death/respawn lifecycle correctness is proposed
- WHEN any required lifecycle row lacks passing evidence
- THEN promotion is rejected
- AND the current bundle keeps lifecycle breadth as a non-claim

### Requirement: Inventory semantics matrix

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_matrix] The inventory proof MUST define a reviewable matrix of inventory windows, slot classes, click modes, carried-stack states, state-id freshness, and expected outcomes before promoting full inventory semantics.

#### Scenario: Matrix scopes inventory claims

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_matrix.scenario]
- GIVEN inventory semantics are being evaluated
- WHEN the matrix is reviewed
- THEN each row records the interaction shape, expected server outcome, expected client observation, evidence status, and non-claim status
- AND uncovered rows do not contribute to full inventory correctness claims

### Requirement: Positive inventory scenarios

r[mc_compatibility.prove_inventory_semantics_matrix.positive_inventory_scenarios] Valid inventory interactions MUST have positive scenarios that correlate Valence server state changes with Stevenarella client observations.

#### Scenario: Valid interaction preserves expected state

r[mc_compatibility.prove_inventory_semantics_matrix.positive_inventory_scenarios.scenario]
- GIVEN a valid inventory matrix row is selected
- WHEN the scenario performs the interaction
- THEN server before/after inventory state and client slot/window observations match the row expectation
- AND the receipt records no missing milestones for that row

### Requirement: Negative inventory scenarios

r[mc_compatibility.prove_inventory_semantics_matrix.negative_inventory_scenarios] Invalid inventory interactions MUST have negative scenarios that reject stale, malformed, invalid-slot, or invalid-carried-stack transitions without corrupting state.

#### Scenario: Invalid interaction fails closed

r[mc_compatibility.prove_inventory_semantics_matrix.negative_inventory_scenarios.scenario]
- GIVEN an invalid inventory interaction is injected
- WHEN the server and client evidence is evaluated
- THEN the invalid transition is rejected or restored according to the matrix
- AND the receipt fails if server state is corrupted or a forbidden client acceptance milestone appears

### Requirement: Inventory promotion gate

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_promotion_gate] Full inventory semantics MUST remain a non-claim until every required matrix row has passing deterministic tests, live receipt evidence, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Promotion is row-complete

r[mc_compatibility.prove_inventory_semantics_matrix.inventory_promotion_gate.scenario]
- GIVEN full inventory semantics are proposed as covered
- WHEN any required matrix row lacks passing evidence
- THEN the promotion is rejected
- AND the acceptance matrix keeps full inventory semantics as a non-claim

### Requirement: Equipment matrix

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_matrix] The equipment breadth proof MUST define a matrix of equipment slots, representative item types, empty/non-empty transitions, and update permutations before claiming all equipment update semantics.

#### Scenario: Matrix scopes equipment claims

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_matrix.scenario]
- GIVEN equipment update breadth is being evaluated
- WHEN the matrix is reviewed
- THEN each claimed slot/item/permutation row records expected server state, expected client observation, evidence status, and non-claim status
- AND untested rows remain non-claims

### Requirement: Positive equipment scenarios

r[mc_compatibility.prove_equipment_slot_item_matrix.positive_equipment_scenarios] Valid equipment matrix rows MUST have positive scenarios that correlate Valence equipment state with Stevenarella remote-entity equipment observations.

#### Scenario: Equipment update is observed for the intended entity

r[mc_compatibility.prove_equipment_slot_item_matrix.positive_equipment_scenarios.scenario]
- GIVEN a valid equipment matrix row is selected
- WHEN the equipment update occurs
- THEN Valence server state and Stevenarella client observation identify the same entity, slot, and item representative
- AND the receipt records no missing equipment milestones for that row

### Requirement: Negative equipment scenarios

r[mc_compatibility.prove_equipment_slot_item_matrix.negative_equipment_scenarios] Equipment verification MUST reject stale, missing, duplicate, wrong-entity, wrong-slot, or wrong-item evidence.

#### Scenario: Mismatched equipment evidence fails

r[mc_compatibility.prove_equipment_slot_item_matrix.negative_equipment_scenarios.scenario]
- GIVEN fixture or live evidence contains mismatched equipment milestones
- WHEN the runner evaluates the equipment row
- THEN the row fails with explicit diagnostics
- AND no acceptance matrix claim is promoted from the mismatched evidence

### Requirement: Equipment promotion gate

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_promotion_gate] All equipment slots/items/permutations MUST remain a non-claim until required matrix rows have deterministic tests, live receipts, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Equipment promotion is evidence complete

r[mc_compatibility.prove_equipment_slot_item_matrix.equipment_promotion_gate.scenario]
- GIVEN all equipment update semantics are proposed as covered
- WHEN any required matrix row lacks passing evidence
- THEN the promotion is rejected
- AND the current bundle continues to list equipment breadth as a non-claim

### Requirement: Modifier matrix

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_matrix] The armor/modifier proof MUST define a matrix of armor loadouts, armor materials, enchantment representatives, status-effect representatives, attack types, and expected damage deltas before promoting modifier breadth claims.

#### Scenario: Matrix scopes modifier claims

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_matrix.scenario]
- GIVEN armor, enchantment, or status-effect behavior is being evaluated
- WHEN the matrix is reviewed
- THEN each row records loadout, modifiers, attack type, expected server calculation, expected client health observation, evidence status, and non-claim status
- AND untested combinations remain non-claims

### Requirement: Positive modifier scenarios

r[mc_compatibility.prove_armor_enchantment_status_matrix.positive_modifier_scenarios] Selected armor/enchantment/status rows MUST have positive scenarios that correlate Valence modifier calculations with Stevenarella victim health observations.

#### Scenario: Modifier row has correlated damage evidence

r[mc_compatibility.prove_armor_enchantment_status_matrix.positive_modifier_scenarios.scenario]
- GIVEN a modifier matrix row is selected
- WHEN a bounded combat event occurs
- THEN server evidence records loadout, modifiers, raw damage, mitigated damage, and victim health delta
- AND Stevenarella records the matching victim health update

### Requirement: Negative modifier scenarios

r[mc_compatibility.prove_armor_enchantment_status_matrix.negative_modifier_scenarios] Modifier verification MUST reject wrong loadout, stale equipment, missing modifier attribution, or mismatched health evidence.

#### Scenario: Weak modifier evidence fails

r[mc_compatibility.prove_armor_enchantment_status_matrix.negative_modifier_scenarios.scenario]
- GIVEN evidence lacks the selected loadout or modifier attribution
- WHEN the row is evaluated
- THEN the row fails with explicit diagnostics
- AND no modifier breadth claim is promoted

### Requirement: Modifier promotion gate

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_promotion_gate] Armor loadout, enchantment, and status-effect breadth MUST remain a non-claim until required rows have passing tests, live receipts, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Modifier promotion is evidence complete

r[mc_compatibility.prove_armor_enchantment_status_matrix.modifier_promotion_gate.scenario]
- GIVEN modifier breadth is proposed as covered
- WHEN any required matrix row lacks passing evidence or a named oracle
- THEN the promotion is rejected
- AND exact vanilla parity remains a separate non-claim unless proven by its own oracle

### Requirement: Projectile matrix

r[mc_compatibility.prove_projectile_travel_collision.projectile_matrix] The projectile physics proof MUST define a matrix of projectile states, target types, weapon representatives, and required client/server evidence before promoting travel, collision, or variant breadth claims.

#### Scenario: Matrix scopes projectile physics claims

r[mc_compatibility.prove_projectile_travel_collision.projectile_matrix.scenario]
- GIVEN projectile physics behavior is being evaluated
- WHEN the matrix is reviewed
- THEN each row records weapon representative, projectile state sequence, target type, expected server evidence, expected client observation, and non-claim status
- AND unobserved travel/collision states remain non-claims

### Requirement: Positive projectile scenarios

r[mc_compatibility.prove_projectile_travel_collision.positive_projectile_scenarios] Selected projectile rows MUST have positive scenarios that correlate Stevenarella client projectile observations with Valence server projectile events.

#### Scenario: Projectile state sequence is correlated

r[mc_compatibility.prove_projectile_travel_collision.positive_projectile_scenarios.scenario]
- GIVEN a projectile matrix row is selected
- WHEN the scenario runs
- THEN the evidence correlates attacker, projectile sequence or entity, target, server event, and client observation for the claimed states
- AND the receipt records no missing projectile milestones for that row

### Requirement: Negative projectile scenarios

r[mc_compatibility.prove_projectile_travel_collision.negative_projectile_scenarios] Projectile verification MUST reject missing, mismatched, unordered, wrong-target, or wrong-weapon evidence.

#### Scenario: Weak projectile evidence fails

r[mc_compatibility.prove_projectile_travel_collision.negative_projectile_scenarios.scenario]
- GIVEN projectile evidence is missing travel/collision state or mismatches attacker, target, sequence, or weapon
- WHEN the runner evaluates the row
- THEN the row fails with explicit diagnostics
- AND no projectile travel/collision claim is promoted

### Requirement: Projectile promotion gate

r[mc_compatibility.prove_projectile_travel_collision.projectile_promotion_gate] Projectile travel, collision, and weapon variant breadth MUST remain non-claims until required rows have passing tests, live receipts, BLAKE3 manifests, and updated evidence indexes.

#### Scenario: Projectile promotion requires row evidence

r[mc_compatibility.prove_projectile_travel_collision.projectile_promotion_gate.scenario]
- GIVEN projectile travel/collision or variant breadth is proposed as covered
- WHEN any required projectile row lacks passing evidence
- THEN promotion is rejected
- AND exact vanilla projectile physics remains a separate non-claim unless proven by the vanilla parity package

### Requirement: Reference oracle

r[mc_compatibility.prove_vanilla_combat_parity.reference_oracle] Any vanilla combat parity proof MUST name the reference oracle, version, configuration, and limitations before accepting parity evidence.

#### Scenario: Oracle is pinned and reviewable

r[mc_compatibility.prove_vanilla_combat_parity.reference_oracle.scenario]
- GIVEN a vanilla parity claim is proposed
- WHEN the proof records its reference oracle
- THEN it identifies the oracle implementation, version, configuration, evidence path, decision owner, and known limitations
- AND it rejects Valence-only evidence as a parity oracle

### Requirement: Parity metrics

r[mc_compatibility.prove_vanilla_combat_parity.parity_metrics] The parity proof MUST define named metrics and tolerances for each claimed combat behavior before comparing Valence and reference evidence.

#### Scenario: Metrics define comparison boundaries

r[mc_compatibility.prove_vanilla_combat_parity.parity_metrics.scenario]
- GIVEN a combat parity row is selected
- WHEN the comparison is evaluated
- THEN the row names the metric, tolerance, unit, reference value, Valence value, and rationale
- AND metrics without tolerances remain non-claims

### Requirement: Parity fixtures

r[mc_compatibility.prove_vanilla_combat_parity.parity_fixtures] The parity comparison logic MUST include positive and negative fixtures for equal-within-tolerance, out-of-tolerance, missing-reference, wrong-version, and Valence-only evidence.

#### Scenario: Weak parity evidence fails closed

r[mc_compatibility.prove_vanilla_combat_parity.parity_fixtures.scenario]
- GIVEN parity fixtures are executed
- WHEN evidence is out of tolerance, lacks a reference run, uses the wrong version, or contains only Valence data
- THEN the fixture fails with explicit diagnostics
- AND no parity claim is promoted

### Requirement: Parity promotion gate

r[mc_compatibility.prove_vanilla_combat_parity.parity_promotion_gate] Exact vanilla combat parity MUST remain a non-claim until paired reference and Valence receipts satisfy the metric/tolerance gate and are tracked with BLAKE3 manifests.

#### Scenario: Parity promotion requires paired evidence

r[mc_compatibility.prove_vanilla_combat_parity.parity_promotion_gate.scenario]
- GIVEN a parity row is proposed for the acceptance matrix
- WHEN paired reference and Valence evidence is missing or stale
- THEN promotion is rejected
- AND the current bundle keeps exact vanilla parity as a non-claim

### Requirement: CTF rule ledger

r[mc_compatibility.prove_ctf_rule_correctness.rule_ledger] The CTF correctness proof MUST maintain a rule ledger that lists every rule or invariant required for the claimed Valence CTF correctness scope.

#### Scenario: Rule ledger identifies evidence status

r[mc_compatibility.prove_ctf_rule_correctness.rule_ledger.scenario]
- GIVEN full CTF correctness is being evaluated
- WHEN the rule ledger is reviewed
- THEN each rule records its evidence status, required client milestones, required server milestones, forbidden transitions, and next action
- AND uncovered rules remain explicit non-claims

### Requirement: Positive rule scenarios

r[mc_compatibility.prove_ctf_rule_correctness.positive_rule_scenarios] Legal CTF rule paths MUST have bounded positive scenarios with correlated Valence server and Stevenarella client evidence when client observation is part of the claim.

#### Scenario: Legal rule path is correlated

r[mc_compatibility.prove_ctf_rule_correctness.positive_rule_scenarios.scenario]
- GIVEN a legal CTF action is part of the promoted scope
- WHEN the scenario runs
- THEN required client milestones and server milestones are present for the same bounded game state
- AND the receipt records no missing milestones for that rule path

### Requirement: Negative rule scenarios

r[mc_compatibility.prove_ctf_rule_correctness.negative_rule_scenarios] Invalid CTF action rows MUST remain unpromoted until negative scenarios prove forbidden captures, pickups, returns, or scores do not occur.

#### Scenario: Invalid rule path is rejected

r[mc_compatibility.prove_ctf_rule_correctness.negative_rule_scenarios.scenario]
- GIVEN an invalid CTF action row is proposed for promotion
- WHEN live negative scenario evidence is missing or shows a forbidden server/client transition
- THEN the row remains a non-claim
- AND the receipt fails if the invalid action produces a rule-breaking state

### Requirement: Rule promotion gate

r[mc_compatibility.prove_ctf_rule_correctness.rule_promotion_gate] A CTF rule cluster MUST NOT be promoted as correct until its rule ledger row, dry-run fixture, live receipt, BLAKE3 manifest, and matrix entry agree on the scoped claim.

#### Scenario: Promotion requires complete rule evidence

r[mc_compatibility.prove_ctf_rule_correctness.rule_promotion_gate.scenario]
- GIVEN a CTF rule cluster is proposed for acceptance
- WHEN any required evidence artifact is missing or stale
- THEN the promotion is rejected
- AND the current bundle continues to list full CTF correctness as a non-claim

### Requirement: Protocol coverage ledger

r[mc_compatibility.prove_broad_protocol_763_coverage.coverage_ledger] The broad protocol-763 proof MUST maintain a reviewable coverage ledger that joins Valence packet metadata, Stevenarella mapping/parser status, and receipt-backed scenario evidence.

#### Scenario: Ledger records covered and uncovered protocol surfaces

r[mc_compatibility.prove_broad_protocol_763_coverage.coverage_ledger.scenario]
- GIVEN broad protocol-763 coverage is being evaluated
- WHEN the ledger is generated or reviewed
- THEN every Valence protocol-763 packet family considered by the claim has a status, evidence path or gap reason, owner, and next action
- AND uncovered rows remain explicit non-claims

### Requirement: Mapping and parser fixtures

r[mc_compatibility.prove_broad_protocol_763_coverage.mapping_parser_fixtures] Newly promoted protocol-763 packet families MUST have focused mapping/parser verification matching their implementation class before acceptance: structured parsers require positive and negative parser-shape fixtures; byte-opaque raw consumers require positive byte-preservation fixtures and explicit semantic non-claim rationale.

#### Scenario: Fixtures reject fallback aliases and malformed shapes

r[mc_compatibility.prove_broad_protocol_763_coverage.mapping_parser_fixtures.scenario]
- GIVEN a packet family is proposed for coverage promotion
- WHEN the focused verification runs
- THEN positive fixtures prove the reviewed mapping and parser behavior
- AND negative fixtures reject inherited fallback aliases and incompatible packet shapes
- AND malformed payload rejection is required for structured parsers
- AND byte-opaque raw consumers keep semantic malformed-payload rejection as a non-claim until semantic decoders exist

### Requirement: Live scenario gates

r[mc_compatibility.prove_broad_protocol_763_coverage.live_scenario_gates] Broad protocol coverage MUST be promoted only through bounded live scenario gates whose receipts name the exact scenario family and protocol surface being claimed.

#### Scenario: Live receipts scope protocol claims

r[mc_compatibility.prove_broad_protocol_763_coverage.live_scenario_gates.scenario]
- GIVEN mapping/parser fixtures pass for a scenario family
- WHEN a live receipt is produced
- THEN the receipt records client/server commits, scenario family, covered protocol surface, missing milestones, and BLAKE3-backed evidence paths
- AND it does not claim unrelated protocol families

### Requirement: Non-overclaiming gate

r[mc_compatibility.prove_broad_protocol_763_coverage.non_overclaiming_gate] Full Minecraft or full protocol-763 compatibility claims MUST remain blocked until the ledger, fixtures, live receipts, matrix, and current bundle all show complete coverage for the stated claim.

#### Scenario: Broad claim is blocked on any uncovered row

r[mc_compatibility.prove_broad_protocol_763_coverage.non_overclaiming_gate.scenario]
- GIVEN a broad/full compatibility claim is proposed
- WHEN any required ledger row lacks receipt-backed evidence or has failing verification
- THEN the claim is rejected
- AND the acceptance matrix keeps broad compatibility as a non-claim

### Requirement: Protocol inventory completeness

r[mc_compatibility.protocol_763_broad_coverage.inventory] Broad protocol-763 coverage MUST be based on a refreshed complete Valence packet inventory and Stevenarella mapping inventory.

#### Scenario: Inventory is complete

r[mc_compatibility.protocol_763_broad_coverage.inventory.complete]
- GIVEN broad protocol-763 coverage is evaluated
- WHEN the inventory checker runs
- THEN every Valence protocol-763 packet row is present
- AND every row has owner, mapping status, parser-shape status, and next action.

### Requirement: Mapping review

r[mc_compatibility.protocol_763_broad_coverage.mapping_review] Promoted protocol packet rows MUST use reviewed mappings and MUST NOT rely on fallback aliases.

#### Scenario: Fallback alias is rejected

r[mc_compatibility.protocol_763_broad_coverage.mapping_review.rejects]
- GIVEN a packet row uses an inherited fallback alias
- WHEN the row is promoted as covered
- THEN the checker fails and names the packet row.

### Requirement: Parser fixtures

r[mc_compatibility.protocol_763_broad_coverage.parser_fixtures] Promoted packet families MUST have parser-shape evidence matching the parser implementation class: structured parsers require positive and negative fixtures; byte-opaque raw consumers require positive byte-preservation fixtures plus explicit semantic non-claim rationale.

#### Scenario: Structured malformed fixture is rejected

r[mc_compatibility.protocol_763_broad_coverage.parser_fixtures.negative]
- GIVEN a malformed packet fixture for a promoted structured packet parser
- WHEN parser-shape tests run
- THEN the malformed packet is rejected without panic or silent acceptance.

#### Scenario: Raw consumer fixture is scoped

r[mc_compatibility.protocol_763_broad_coverage.parser_fixtures.raw_scope]
- GIVEN a promoted packet family is implemented as a byte-opaque raw consumer
- WHEN evidence is reviewed
- THEN a positive byte-preservation fixture is linked
- AND semantic malformed-payload rejection remains an explicit non-claim.

### Requirement: Packet family receipts

r[mc_compatibility.protocol_763_broad_coverage.receipts] Broad protocol coverage MUST include reviewable receipts or deterministic fixture evidence for promoted packet families.

#### Scenario: Receipt backs promoted family

r[mc_compatibility.protocol_763_broad_coverage.receipts.backing]
- GIVEN a packet family is marked covered
- WHEN evidence is reviewed
- THEN a receipt, fixture result, or BLAKE3-backed log is linked for that family.

### Requirement: Broad protocol non-claims

r[mc_compatibility.protocol_763_broad_coverage.nonclaims] Broad protocol-763 packet coverage MUST NOT claim full Minecraft gameplay compatibility.

#### Scenario: Full Minecraft remains separate

r[mc_compatibility.protocol_763_broad_coverage.nonclaims.separate]
- GIVEN broad protocol packet coverage is promoted
- WHEN the evidence doc is reviewed
- THEN full Minecraft compatibility, gameplay correctness, and production readiness remain separate non-claims.

### Requirement: Child revisions recorded

r[mc_compatibility.receipts.child_revisions.recorded] Live `mc-compat-runner` receipts MUST record resolved git revisions and cleanliness status for child repositories used to produce promoted evidence.

#### Scenario: Live receipt records child revisions

r[mc_compatibility.receipts.child_revisions.recorded.live]
- GIVEN a live Valence-backed scenario uses a Stevenarella client checkout
- WHEN the runner writes the receipt
- THEN the receipt includes the resolved Stevenarella commit hash
- AND the receipt includes the requested and resolved Valence commit hash
- AND the receipt includes clean/dirty status for both child repositories.

### Requirement: Child revision dry-run shape

r[mc_compatibility.receipts.child_revisions.dry_run] Dry-run receipts MUST include deterministic child revision placeholders without reading host git state.

#### Scenario: Dry-run remains deterministic

r[mc_compatibility.receipts.child_revisions.dry_run.shape]
- GIVEN a dry-run scenario is selected
- WHEN the runner writes the receipt
- THEN child revision fields use deterministic placeholder values
- AND no child git command is required.

### Requirement: Child revision gate

r[mc_compatibility.receipts.child_revisions.gated] Evidence checks MUST reject promoted non-legacy live receipts that cite child revisions unless the receipt records those revisions or an oracle checkpoint is explicitly linked.

#### Scenario: Missing child revision is rejected

r[mc_compatibility.receipts.child_revisions.gated.missing]
- GIVEN a promoted evidence row cites a child repository revision
- WHEN the receipt lacks a matching machine-readable child revision field
- THEN the evidence gate fails unless a linked oracle checkpoint explains the inspected evidence, decision, owner, and next action.

### Requirement: Child revision tests

r[mc_compatibility.receipts.child_revisions.verified] The child revision receipt behavior MUST have positive and negative tests.

#### Scenario: Tests cover clean and dirty child repos

r[mc_compatibility.receipts.child_revisions.verified.tests]
- GIVEN test fixtures for clean, dirty, and unavailable child repositories
- WHEN receipt construction is evaluated
- THEN clean repositories produce resolved revision fields
- AND dirty or unavailable repositories produce explicit diagnostics instead of silent omission.

### Requirement: Survival coverage rows

r[mc_compatibility.survival_coverage_matrix.rows] The repo MUST maintain a survival coverage matrix that separates covered survival rails from uncovered survival systems.

#### Scenario: Matrix names uncovered systems

r[mc_compatibility.survival_coverage_matrix.rows.uncovered]
- GIVEN the survival coverage matrix is reviewed
- WHEN a survival system has no live receipt
- THEN the matrix lists it as a non-claim
- AND the matrix includes crafting, furnace, chest, hunger, mob, redstone, biome, dimension, and persistence rows.

### Requirement: Survival row requirements

r[mc_compatibility.survival_coverage_matrix.row_requirements] Each survival matrix row MUST define the minimum evidence required for promotion.

#### Scenario: Row has promotion requirements

r[mc_compatibility.survival_coverage_matrix.row_requirements.present]
- GIVEN a survival row is proposed for promotion
- WHEN the matrix checker evaluates it
- THEN the row names required Valence receipts, reference receipts when parity is claimed, logs, hashes, and child revisions.

### Requirement: Full-survival gate

r[mc_compatibility.survival_coverage_matrix.gate] A deterministic checker MUST block full survival compatibility claims while required rows are missing evidence.

#### Scenario: Full claim is blocked

r[mc_compatibility.survival_coverage_matrix.gate.blocks]
- GIVEN any required survival row is missing live evidence
- WHEN documentation claims full survival compatibility
- THEN the checker fails and names the missing rows.

### Requirement: Survival non-claims

r[mc_compatibility.survival_coverage_matrix.nonclaims] Current evidence docs MUST point full-survival non-claims to the survival coverage matrix.

#### Scenario: Non-claim points to matrix

r[mc_compatibility.survival_coverage_matrix.nonclaims.linked]
- GIVEN the current evidence bundle discusses survival scope
- WHEN full survival compatibility is a non-claim
- THEN it links to the survival coverage matrix or checker output.

### Requirement: Survival parity metrics

r[mc_compatibility.survival_reference_parity.metrics] The survival parity rail MUST define normalized exact-match metrics before comparing Valence with a reference backend.

#### Scenario: Metrics are explicit

r[mc_compatibility.survival_reference_parity.metrics.explicit]
- GIVEN the break/place/pickup parity rail is evaluated
- WHEN the comparator reads the receipts
- THEN it compares explicit join, break, pickup/inventory, and placement fields
- AND it does not infer parity from raw log similarity alone.

### Requirement: Reference receipt

r[mc_compatibility.survival_reference_parity.reference_receipt] The rail MUST produce a reviewable local reference-server receipt for the same Stevenarella survival probe.

#### Scenario: Reference receipt is reviewable

r[mc_compatibility.survival_reference_parity.reference_receipt.reviewable]
- GIVEN the reference backend run completes
- WHEN evidence is promoted
- THEN the reference receipt and logs are copied under `docs/evidence/`
- AND BLAKE3 hashes are recorded.

### Requirement: Valence receipt

r[mc_compatibility.survival_reference_parity.valence_receipt] The rail MUST produce a matching Valence receipt from committed child revisions.

#### Scenario: Valence receipt is paired

r[mc_compatibility.survival_reference_parity.valence_receipt.paired]
- GIVEN the reference receipt exists
- WHEN the Valence receipt is generated
- THEN it uses the same scenario, username, target coordinates, and normalized metric names
- AND it records committed Valence and Stevenarella child revisions.

### Requirement: Parity comparator

r[mc_compatibility.survival_reference_parity.comparator] A deterministic checker MUST compare the paired receipts and fail on missing or mismatched metrics.

#### Scenario: Mismatch rejects parity

r[mc_compatibility.survival_reference_parity.comparator.rejects]
- GIVEN a paired receipt has a missing or changed normalized metric
- WHEN the comparator runs
- THEN it fails and names the mismatched metric.

### Requirement: Parity non-claims

r[mc_compatibility.survival_reference_parity.nonclaims] The paired break/place/pickup parity row MUST NOT claim full survival compatibility, broad vanilla parity, or production readiness.

#### Scenario: Non-claims remain explicit

r[mc_compatibility.survival_reference_parity.nonclaims.explicit]
- GIVEN the narrow parity row is promoted
- WHEN the evidence doc is reviewed
- THEN full survival compatibility and broad vanilla parity remain explicit non-claims.

### Requirement: Production network safety matrix

r[mc_compatibility.production_network_safety.matrix] The repo MUST maintain a matrix that separates owned-local load safety, public-server safety, WAN tolerance, and adversarial-network safety claims.

#### Scenario: Claims are separated

r[mc_compatibility.production_network_safety.matrix.separate]
- GIVEN network safety evidence is reviewed
- WHEN a claim is promoted
- THEN the matrix identifies exactly which safety claim is covered
- AND unrelated safety claims remain non-claims.

### Requirement: Production network safety gate

r[mc_compatibility.production_network_safety.gate] A deterministic checker MUST reject production/public/WAN/adversarial claims without required scope and telemetry fields.

#### Scenario: Missing safety fields reject claim

r[mc_compatibility.production_network_safety.gate.rejects]
- GIVEN an evidence row claims production or public network safety
- WHEN target ownership, authorization, bounds, telemetry, or non-claims are missing
- THEN the checker fails and names the missing fields.

### Requirement: Owned-local load evidence

r[mc_compatibility.production_network_safety.owned_local] Owned-local load safety MUST record client count, duration, hardware scope, telemetry, and failure criteria.

#### Scenario: Owned-local load is bounded

r[mc_compatibility.production_network_safety.owned_local.bounded]
- GIVEN an owned-local load receipt is produced
- WHEN it is reviewed
- THEN it records client count, duration, target ownership, telemetry, and configured upper bounds.

### Requirement: Public server authorized safety contract

r[mc_compatibility.public_server_authorized_safety.contract] The `public-server safety` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.public_server_authorized_safety.contract.scope]
- GIVEN `public-server-authorized-safety` work starts
- WHEN the evidence contract is reviewed
- THEN it names one explicitly authorized public or non-loopback target envelope with owner, written authorization reference, bounds, telemetry, and abort criteria
- AND it states that third-party target safety without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing remain non-claims.

### Requirement: Public server authorized safety checker

r[mc_compatibility.public_server_authorized_safety.checker] A deterministic checker MUST compare normalized metrics before the `public-server safety` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.public_server_authorized_safety.checker.rejects]
- GIVEN evidence is missing or mismatches target owner, authorization artifact, target scope, client count, duration, traffic limits, telemetry, abort criteria, redaction policy, and human checkpoint decision
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.public_server_authorized_safety.checker.standard]
- GIVEN the row requires human/oracle authorization checkpoint before live run plus deterministic receipt checks that reject missing fields
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Public server authorized safety rail

r[mc_compatibility.public_server_authorized_safety.rail] The harness MUST expose a `public-server-authorized-safety` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.public_server_authorized_safety.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `public-server-authorized-safety` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Public server authorized safety evidence

r[mc_compatibility.public_server_authorized_safety.evidence] `public-server safety` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.public_server_authorized_safety.evidence.reviewable]
- GIVEN the `public-server safety` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Public server authorized safety matrix promotion

r[mc_compatibility.public_server_authorized_safety.matrix] Acceptance matrix and current-bundle docs MUST promote only the `public-server safety` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.public_server_authorized_safety.matrix.nonclaims]
- GIVEN `public-server safety` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `public-server safety` row is marked covered
- AND third-party target safety without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing remain explicit non-claims.

### Requirement: Public server authorized safety validation

r[mc_compatibility.public_server_authorized_safety.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.public_server_authorized_safety.validation.log]
- GIVEN the `public-server safety` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.

### Requirement: WAN tolerance evidence

r[mc_compatibility.production_network_safety.wan] WAN tolerance claims MUST record perturbation mechanism, delay, jitter, loss, timeout, and fail-closed behavior.

#### Scenario: WAN tooling unavailable fails closed

r[mc_compatibility.production_network_safety.wan.fail_closed]
- GIVEN WAN perturbation tooling is unavailable
- WHEN a WAN tolerance receipt is requested
- THEN the receipt fails closed instead of silently claiming WAN safety.

### Requirement: WAN tolerance bounded telemetry contract

r[mc_compatibility.wan_tolerance_bounded_telemetry.contract] The `WAN tolerance` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.wan_tolerance_bounded_telemetry.contract.scope]
- GIVEN `wan-tolerance-bounded-telemetry` work starts
- WHEN the evidence contract is reviewed
- THEN it names one authorized owned-local perturbation envelope with configured delay, jitter, packet loss, timeout, duration, client count, and telemetry
- AND it states that public-server safety, internet-path safety, adversarial network safety, production readiness, unbounded soak/reconnect safety, and third-party target safety remain non-claims.

### Requirement: WAN tolerance bounded telemetry checker

r[mc_compatibility.wan_tolerance_bounded_telemetry.checker] A deterministic checker MUST compare normalized metrics before the `WAN tolerance` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.wan_tolerance_bounded_telemetry.checker.rejects]
- GIVEN evidence is missing or mismatches target ownership, authorization, delay, jitter, loss, timeout, duration, client count, reconnect count, telemetry samples, pass/fail criteria, and abort reason
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.wan_tolerance_bounded_telemetry.checker.standard]
- GIVEN the row requires fail-closed preflight plus live telemetry receipt and human/oracle checkpoint if tooling or target scope changes
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: WAN tolerance bounded telemetry rail

r[mc_compatibility.wan_tolerance_bounded_telemetry.rail] The harness MUST expose a `wan-tolerance-bounded-telemetry` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.wan_tolerance_bounded_telemetry.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `wan-tolerance-bounded-telemetry` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: WAN tolerance bounded telemetry reviewable evidence

r[mc_compatibility.wan_tolerance_bounded_telemetry.evidence] `WAN tolerance` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.wan_tolerance_bounded_telemetry.evidence.reviewable]
- GIVEN the `WAN tolerance` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: WAN tolerance bounded telemetry matrix promotion

r[mc_compatibility.wan_tolerance_bounded_telemetry.matrix] Acceptance matrix and current-bundle docs MUST promote only the `WAN tolerance` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.wan_tolerance_bounded_telemetry.matrix.nonclaims]
- GIVEN `WAN tolerance` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `WAN tolerance` row is marked covered
- AND public-server safety, internet-path safety, adversarial network safety, production readiness, unbounded soak/reconnect safety, and third-party target safety remain explicit non-claims.

### Requirement: WAN tolerance bounded telemetry validation

r[mc_compatibility.wan_tolerance_bounded_telemetry.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.wan_tolerance_bounded_telemetry.validation.log]
- GIVEN the `WAN tolerance` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.

### Requirement: Adversarial network evidence

r[mc_compatibility.production_network_safety.adversarial] Adversarial-network safety claims MUST require explicit oracle or human approval in addition to deterministic evidence.

#### Scenario: Human/oracle checkpoint required

r[mc_compatibility.production_network_safety.adversarial.oracle]
- GIVEN adversarial-network safety is proposed
- WHEN evidence is promoted
- THEN a checkpoint records the question, inspected evidence, decision, owner, and next action.

### Requirement: Production non-claims

r[mc_compatibility.production_network_safety.nonclaims] Existing loopback compatibility receipts MUST remain non-production evidence until the production/network safety matrix passes.

#### Scenario: Loopback receipts do not imply production

r[mc_compatibility.production_network_safety.nonclaims.loopback]
- GIVEN an owned-local loopback receipt passes
- WHEN documentation is generated
- THEN public-server safety, production readiness, WAN safety, and adversarial-network safety remain explicit non-claims.

### Requirement: Survival coverage matrix reflects reference parity

r[mc_compatibility.survival_coverage_matrix.reference_parity_synced] The survival coverage matrix MUST mark break/place/pickup as paired Paper/Valence reference parity covered when the promoted parity artifacts are present.

#### Scenario: Break/place/pickup row cites paired evidence

r[mc_compatibility.survival_coverage_matrix.reference_parity_synced.row]
- GIVEN the survival coverage matrix is reviewed
- WHEN the break/place/pickup row is present
- THEN it cites the Paper reference receipt
- AND it cites the Valence paired receipt
- AND it links the survival reference parity evidence doc.

### Requirement: Survival coverage checker blocks stale parity state

r[mc_compatibility.survival_coverage_matrix.reference_parity_gate] The survival coverage checker MUST reject stale break/place/pickup rows that claim Valence-only coverage or missing reference evidence after parity is promoted.

#### Scenario: Stale reference-missing row is rejected

r[mc_compatibility.survival_coverage_matrix.reference_parity_gate.rejects]
- GIVEN the break/place/pickup row still says reference evidence is missing
- WHEN the checker runs
- THEN it fails and names the stale row.

### Requirement: Survival breadth remains scoped

r[mc_compatibility.survival_coverage_matrix.reference_parity_nonclaims] Updating the break/place/pickup row MUST NOT claim broader survival compatibility.

#### Scenario: Missing survival rows remain non-claims

r[mc_compatibility.survival_coverage_matrix.reference_parity_nonclaims.rows]
- GIVEN break/place/pickup parity is marked covered
- WHEN the matrix is reviewed
- THEN crafting, chest persistence, furnace persistence, hunger/food, mob drops, redstone, biome/dimension, and world persistence remain missing non-claim rows.

### Requirement: Survival coverage sync validation

r[mc_compatibility.survival_coverage_matrix.reference_parity_validation] The reference parity sync MUST record checker and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_coverage_matrix.reference_parity_validation.log]
- GIVEN the matrix and checker are updated
- WHEN the change is archived
- THEN a run log records survival coverage checker, survival parity checker, current bundle, acceptance matrix, evidence manifest, task gate, and Cairn validation output.

### Requirement: Active task evidence closeout

r[mc_compatibility.task_evidence_gate.active_closeout] Completed active Cairn tasks MUST cite durable local evidence before archive.

#### Scenario: Completed active task cites durable evidence

r[mc_compatibility.task_evidence_gate.active_closeout.completed_task]
- GIVEN an active `cairn/changes/*/tasks.md` file contains a checked task
- WHEN the task-evidence gate evaluates the task
- THEN the task contains an evidence-labeled line
- AND the task cites at least one existing `docs/evidence/` artifact
- AND the task cites verification command output as a `docs/evidence/*.run.log` artifact
- AND the task cites either an existing `docs/evidence/*.b3` manifest or an inline BLAKE3 digest

### Requirement: Task evidence gate fails closed

r[mc_compatibility.task_evidence_gate.fail_closed] The task-evidence checker MUST fail closed for checked tasks that omit durable copied evidence, verification output, BLAKE3 evidence, or existing artifact paths.

#### Scenario: Missing evidence field fails

r[mc_compatibility.task_evidence_gate.fail_closed.missing_field]
- GIVEN a checked task lacks an evidence label, copied evidence path, run log path, BLAKE3 manifest or digest, or cites a missing artifact
- WHEN the task-evidence gate runs
- THEN the gate fails with a diagnostic naming the task location and missing field

### Requirement: Task evidence gate workflow

r[mc_compatibility.task_evidence_gate.flake_workflow] The task-evidence gate MUST be available through a focused flake check and included in the maintained compatibility aggregate.

#### Scenario: Operator runs closeout gate locally

r[mc_compatibility.task_evidence_gate.flake_workflow.local]
- GIVEN an operator is preparing to complete or archive a Cairn change
- WHEN they run the focused flake check or maintained aggregate
- THEN the gate self-tests its positive and negative fixtures
- AND the gate scans active Cairn tasks against copied repo evidence

### Requirement: Task evidence validation is recorded

r[mc_compatibility.task_evidence_gate.validation_evidence] Task-evidence gate work MUST record validation output before archive.

#### Scenario: Validation evidence is copied under docs/evidence

r[mc_compatibility.task_evidence_gate.validation_evidence.local]
- GIVEN the task-evidence gate is implemented
- WHEN the change is archived
- THEN checker self-test output, active scan output, flake output, Cairn validation, Cairn gate output, and evidence manifest validation are copied under `docs/evidence/` with BLAKE3 evidence

### Requirement: Evidence manifest source closure

r[mc_compatibility.evidence_manifest_source_closure.contract] Promoted evidence BLAKE3 manifests MUST cite files present in the parent repo source closure unless the row is explicitly represented by a reviewable oracle document.

#### Scenario: Manifest row is Nix-reviewable

r[mc_compatibility.evidence_manifest_source_closure.contract.reviewable]
- GIVEN a `docs/evidence/*.b3` manifest is promoted
- WHEN the repo is evaluated through Nix
- THEN every manifest path resolves to a file in the parent repo source closure
- AND child-repo or generated-output bytes are copied under `docs/evidence/` before being cited

### Requirement: External artifact closure preservation

r[mc_compatibility.evidence_manifest_source_closure.artifacts] Child-repo source files and generated artifacts referenced by evidence manifests MUST be preserved as durable copied artifacts without changing their BLAKE3 content identity.

#### Scenario: Copied artifact keeps digest

r[mc_compatibility.evidence_manifest_source_closure.artifacts.digest]
- GIVEN a manifest previously cited a nested repo file or `target/` artifact
- WHEN the artifact is copied into `docs/evidence/`
- THEN the manifest cites the copied path
- AND the BLAKE3 digest remains the digest of the copied artifact bytes

### Requirement: Accepted-spec digest refresh

r[mc_compatibility.evidence_manifest_source_closure.spec_digest] Evidence manifests that intentionally include accepted spec files MUST be refreshed after accepted spec edits.

#### Scenario: Spec digest rows match current accepted spec

r[mc_compatibility.evidence_manifest_source_closure.spec_digest.current]
- GIVEN accepted `cairn/specs/*/spec.md` content changes
- WHEN evidence manifest validation runs
- THEN every manifest row that cites the accepted spec records the current BLAKE3 digest or the row is removed in favor of immutable archive evidence

### Requirement: Source-closure validation evidence

r[mc_compatibility.evidence_manifest_source_closure.validation] Source-closure hardening MUST record local and Nix evidence-manifest validation before archive.

#### Scenario: Nix manifest check passes

r[mc_compatibility.evidence_manifest_source_closure.validation.nix]
- GIVEN source-closure hardening is complete
- WHEN validation evidence is recorded
- THEN local manifest self-test/full scan, Nix `mc-compat-evidence-manifests`, Cairn validation, and Cairn gates pass with output copied under `docs/evidence/`

### Requirement: Contract

r[mc_compatibility.adversarial_network_oracle_rail.contract] The `adversarial-network safety` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.adversarial_network_oracle_rail.contract.scope]
- GIVEN `adversarial-network-oracle` work starts
- WHEN the evidence contract is reviewed
- THEN it names one explicitly approved adversarial-network model with bounded packet mutation, target ownership, telemetry, and human/oracle decision record
- AND it states that general malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded adversarial robustness, and full protocol security remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.adversarial_network_oracle_rail.checker] A deterministic checker MUST compare normalized metrics before the `adversarial-network safety` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.adversarial_network_oracle_rail.checker.rejects]
- GIVEN evidence is missing or mismatches threat model id, mutation types, packet bounds, target ownership, authorization, telemetry, abort criteria, observed containment, and oracle decision
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.adversarial_network_oracle_rail.checker.standard]
- GIVEN the row requires human/oracle checkpoint plus deterministic evidence; no live adversarial claim without approval and bounded model
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.adversarial_network_oracle_rail.rail] The harness MUST expose a `adversarial-network-oracle` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.adversarial_network_oracle_rail.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `adversarial-network-oracle` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.adversarial_network_oracle_rail.evidence] `adversarial-network safety` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.adversarial_network_oracle_rail.evidence.reviewable]
- GIVEN the `adversarial-network safety` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.adversarial_network_oracle_rail.matrix] Acceptance matrix and current-bundle docs MUST promote only the `adversarial-network safety` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.adversarial_network_oracle_rail.matrix.nonclaims]
- GIVEN `adversarial-network safety` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `adversarial-network safety` row is marked covered
- AND general malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded adversarial robustness, and full protocol security remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.adversarial_network_oracle_rail.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.adversarial_network_oracle_rail.validation.log]
- GIVEN the `adversarial-network safety` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.

### Requirement: Contract

r[mc_compatibility.armor_loadout_enchantment_status_matrix.contract] The `armor/enchantment/status matrix` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.armor_loadout_enchantment_status_matrix.contract.scope]
- GIVEN `armor-loadout-enchantment-status-matrix` work starts
- WHEN the evidence contract is reviewed
- THEN it names a bounded table of configured armor loadout, enchantment, status-effect, attack type, and expected mitigation rows
- AND it states that all armor permutations, all enchantments, all status effects, exact vanilla balancing outside listed rows, production readiness, and full combat correctness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.armor_loadout_enchantment_status_matrix.checker] A deterministic checker MUST compare normalized metrics before the `armor/enchantment/status matrix` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.armor_loadout_enchantment_status_matrix.checker.rejects]
- GIVEN evidence is missing or mismatches loadout id, equipment slots, enchantment ids/levels, status effects, attack type, pre/post health, damage delta, mitigation delta, and tolerance fields
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.armor_loadout_enchantment_status_matrix.checker.standard]
- GIVEN the row requires matrix checker with positive and negative rows plus paired reference evidence for any vanilla-parity label
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.armor_loadout_enchantment_status_matrix.rail] The harness MUST expose a `armor-loadout-enchantment-status-matrix` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.armor_loadout_enchantment_status_matrix.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `armor-loadout-enchantment-status-matrix` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.armor_loadout_enchantment_status_matrix.evidence] `armor/enchantment/status matrix` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.armor_loadout_enchantment_status_matrix.evidence.reviewable]
- GIVEN the `armor/enchantment/status matrix` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.armor_loadout_enchantment_status_matrix.matrix] Acceptance matrix and current-bundle docs MUST promote only the `armor/enchantment/status matrix` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.armor_loadout_enchantment_status_matrix.matrix.nonclaims]
- GIVEN `armor/enchantment/status matrix` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `armor/enchantment/status matrix` row is marked covered
- AND all armor permutations, all enchantments, all status effects, exact vanilla balancing outside listed rows, production readiness, and full combat correctness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.armor_loadout_enchantment_status_matrix.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.armor_loadout_enchantment_status_matrix.validation.log]
- GIVEN the `armor/enchantment/status matrix` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.

### Requirement: Contract

r[mc_compatibility.equipment_slot_item_matrix_expansion.contract] The `equipment slot/item matrix` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.equipment_slot_item_matrix_expansion.contract.scope]
- GIVEN `equipment-slot-item-matrix-expansion` work starts
- WHEN the evidence contract is reviewed
- THEN it names a bounded matrix of configured equipment slots, item ids, counts, and remote observer update expectations
- AND it states that all equipment slots/items, equipment packet permutations, armor mitigation, enchantment/status effects, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.equipment_slot_item_matrix_expansion.checker] A deterministic checker MUST compare normalized metrics before the `equipment slot/item matrix` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.equipment_slot_item_matrix_expansion.checker.rejects]
- GIVEN evidence is missing or mismatches actor identity, observer identity, slot, item id, item count, update order, remote entity id, and client/server correlation ids
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.equipment_slot_item_matrix_expansion.checker.standard]
- GIVEN the row requires matrix checker with per-row client/server correlation and no broad slot/item claim outside listed rows
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.equipment_slot_item_matrix_expansion.rail] The harness MUST expose a `equipment-slot-item-matrix-expansion` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.equipment_slot_item_matrix_expansion.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `equipment-slot-item-matrix-expansion` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.equipment_slot_item_matrix_expansion.evidence] `equipment slot/item matrix` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.equipment_slot_item_matrix_expansion.evidence.reviewable]
- GIVEN the `equipment slot/item matrix` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.equipment_slot_item_matrix_expansion.matrix] Acceptance matrix and current-bundle docs MUST promote only the `equipment slot/item matrix` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.equipment_slot_item_matrix_expansion.matrix.nonclaims]
- GIVEN `equipment slot/item matrix` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `equipment slot/item matrix` row is marked covered
- AND all equipment slots/items, equipment packet permutations, armor mitigation, enchantment/status effects, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.equipment_slot_item_matrix_expansion.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.equipment_slot_item_matrix_expansion.validation.log]
- GIVEN the `equipment slot/item matrix` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.

### Requirement: Contract

r[mc_compatibility.ctf_invalid_pickup_ownership.contract] The `invalid flag pickup/ownership` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.ctf_invalid_pickup_ownership.contract.scope]
- GIVEN `ctf-invalid-pickup-ownership` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured invalid flag pickup attempt by the wrong team or invalid owner state with no ownership transfer and no score
- AND it states that all invalid actions, all flag permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.ctf_invalid_pickup_ownership.checker] A deterministic checker MUST compare normalized metrics before the `invalid flag pickup/ownership` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.ctf_invalid_pickup_ownership.checker.rejects]
- GIVEN evidence is missing or mismatches player team, flag identity, pre-owner state, invalid pickup action, post-owner state, score counters, forbidden capture/score patterns, and containment outcome
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.ctf_invalid_pickup_ownership.checker.standard]
- GIVEN the row requires live Valence CTF receipt with negative containment checker and BLAKE3-backed logs
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.ctf_invalid_pickup_ownership.rail] The harness MUST expose a `ctf-invalid-pickup-ownership` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.ctf_invalid_pickup_ownership.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `ctf-invalid-pickup-ownership` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.ctf_invalid_pickup_ownership.evidence] `invalid flag pickup/ownership` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.ctf_invalid_pickup_ownership.evidence.reviewable]
- GIVEN the `invalid flag pickup/ownership` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.ctf_invalid_pickup_ownership.matrix] Acceptance matrix and current-bundle docs MUST promote only the `invalid flag pickup/ownership` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.ctf_invalid_pickup_ownership.matrix.nonclaims]
- GIVEN `invalid flag pickup/ownership` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `invalid flag pickup/ownership` row is marked covered
- AND all invalid actions, all flag permutations, full CTF correctness, adversarial security, production readiness, and broad Minecraft compatibility remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.ctf_invalid_pickup_ownership.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.ctf_invalid_pickup_ownership.validation.log]
- GIVEN the `invalid flag pickup/ownership` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.

### Requirement: Stevenarella MCP control command contract

r[mc_compatibility.stevenarella_mcp_control.contract] Stevenarella MCP control MUST define a bounded, typed command contract before exposing automation tools.

#### Scenario: Command scope is explicit

r[mc_compatibility.stevenarella_mcp_control.contract.scope]
- GIVEN MCP control work starts
- WHEN the command contract is reviewed
- THEN it names status, connect, disconnect, key, look, mouse, use-item, attack, and chat as the initial supported actions
- AND it states that headless rendering, frame capture, public-server authorization, load testing, and semantic compatibility remain non-claims.

#### Scenario: Invalid commands fail closed

r[mc_compatibility.stevenarella_mcp_control.contract.invalid_commands]
- GIVEN an MCP request contains an unknown key name, unknown mouse button, malformed address, missing required field, or unsupported action
- WHEN command validation runs
- THEN it returns a structured error without mutating Stevenarella game state.

### Requirement: Stevenarella MCP transport safety

r[mc_compatibility.stevenarella_mcp_control.transport] Stevenarella MCP transport MUST be native-only and safe by default.

#### Scenario: Stdio remains JSON-RPC clean

r[mc_compatibility.stevenarella_mcp_control.transport.stdio_clean]
- GIVEN Stevenarella starts with `--mcp-stdio`
- WHEN ordinary client logs are emitted
- THEN stdout carries only MCP JSON-RPC bytes
- AND logs remain available through stderr where safe or through `client.log`.

#### Scenario: TCP bind fails closed

r[mc_compatibility.stevenarella_mcp_control.transport.tcp_auth]
- GIVEN Stevenarella is asked to bind MCP on a non-loopback address without an explicit token environment variable
- WHEN startup validates MCP options
- THEN startup rejects the bind before accepting control requests.

### Requirement: Stevenarella MCP main-thread command ownership

r[mc_compatibility.stevenarella_mcp_control.main_thread_queue] MCP control MUST preserve winit, GL, `Game`, and `Server` main-thread ownership.

#### Scenario: Worker thread only enqueues

r[mc_compatibility.stevenarella_mcp_control.main_thread_queue.worker_enqueues]
- GIVEN an MCP worker receives a valid command
- WHEN the command is accepted
- THEN the worker only enqueues a typed `ControlCommand`
- AND it does not directly mutate `Game`, `Server`, winit window state, or GL state.

#### Scenario: Commands drain at deterministic boundary

r[mc_compatibility.stevenarella_mcp_control.main_thread_queue.drain_boundary]
- GIVEN queued control commands exist
- WHEN the main loop enters the configured per-frame drain point
- THEN commands are applied before the server tick for that frame
- AND responses identify whether the command was applied, rejected, or deferred.

### Requirement: Stevenarella MCP tool behavior

r[mc_compatibility.stevenarella_mcp_control.tools] MCP tools MUST reuse Stevenarella internal control methods rather than host OS input synthesis.

#### Scenario: Movement uses internal key state

r[mc_compatibility.stevenarella_mcp_control.tools.key_state]
- GIVEN the client is connected and a player entity exists
- WHEN MCP sends a key command for a supported Stevenarella movement key
- THEN `Server::key_press` updates the corresponding `PlayerMovement` key state.

#### Scenario: Look updates player rotation

r[mc_compatibility.stevenarella_mcp_control.tools.look]
- GIVEN the client is connected and a player entity exists
- WHEN MCP sends a bounded look delta
- THEN the player rotation is updated with the same pitch limits as physical mouse input.

#### Scenario: Chat uses protocol packet path

r[mc_compatibility.stevenarella_mcp_control.tools.chat]
- GIVEN the client is connected
- WHEN MCP sends a chat message or slash command
- THEN Stevenarella sends it through the protocol `ChatMessage` serverbound path
- AND oversized or malformed messages are rejected before packet write.

### Requirement: Stevenarella MCP control validation evidence

r[mc_compatibility.stevenarella_mcp_control.validation] The MCP control plane MUST have positive and negative tests before any runner depends on it.

#### Scenario: Validation covers happy and sad paths

r[mc_compatibility.stevenarella_mcp_control.validation.tests]
- GIVEN MCP control implementation is complete
- WHEN focused tests run
- THEN they cover valid command parsing and application
- AND they cover invalid key names, invalid button names, disconnected operations, stdout contamination, and unsafe bind attempts.

### Requirement: Stevenarella MCP control evidence artifacts

r[mc_compatibility.stevenarella_mcp_control.artifacts] Review-critical MCP control evidence MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.stevenarella_mcp_control.artifacts.reviewable]
- GIVEN the control plane Cairn is ready to archive
- WHEN reviewers inspect the parent repo
- THEN focused test output, command-shape evidence, Cairn gate output, validation output, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Task evidence run logs are explicit

r[mc_compatibility.harness_coverage.task_evidence.run_log_status] The active Cairn task-evidence gate MUST reject completed tasks whose cited run logs lack explicit successful exit-status evidence.

#### Scenario: Completed task cites successful run output

r[mc_compatibility.harness_coverage.task_evidence.run_log_status.success]
- GIVEN a completed active Cairn task cites `docs/evidence/*.run.log`
- WHEN the task-evidence gate validates the task
- THEN the cited run log contains at least one `exit_status=` line
- AND every `exit_status=` line in that run log resolves to `0`.

#### Scenario: Missing or failed status fails closed

r[mc_compatibility.harness_coverage.task_evidence.run_log_status.rejects]
- GIVEN a completed active Cairn task cites a run log with no `exit_status=` line or a nonzero exit status
- WHEN the task-evidence gate validates the task
- THEN the gate fails and names the offending run log.

### Requirement: Task evidence paths are reviewable

r[mc_compatibility.harness_coverage.task_evidence.reviewable_paths] Completed Cairn tasks MUST NOT rely on transient build outputs or nested child-repo paths as review-critical evidence.

#### Scenario: Parent evidence copy is required

r[mc_compatibility.harness_coverage.task_evidence.reviewable_paths.parent_copy]
- GIVEN a completed active Cairn task references review-critical artifacts
- WHEN the task-evidence gate validates the task
- THEN review-critical artifact paths are parent-repo `docs/evidence/` paths
- AND path-like references rooted at `target/`, `stevenarella/`, `valence/`, `hyperion/`, or `Leafish/` are rejected.

### Requirement: Cited manifests cover cited run logs

r[mc_compatibility.harness_coverage.task_evidence.manifest_pairing] Completed Cairn tasks that cite `.b3` sidecars MUST cite a sidecar that covers each cited run log.

#### Scenario: Run log is in the manifest

r[mc_compatibility.harness_coverage.task_evidence.manifest_pairing.run_log]
- GIVEN a completed active Cairn task cites `docs/evidence/foo.run.log` and `docs/evidence/foo.b3`
- WHEN the task-evidence gate validates the task
- THEN at least one cited `.b3` manifest contains the cited run-log path
- OR the task contains an inline BLAKE3 digest for the run-log evidence.

### Requirement: Harness hardening evidence is durable

r[mc_compatibility.harness_coverage.validation] The harness hardening MUST include deterministic positive and negative fixtures plus repo-local validation evidence before archive.

#### Scenario: Validation covers happy and sad paths

r[mc_compatibility.harness_coverage.validation.fixtures]
- GIVEN the task-evidence gate is hardened
- WHEN checker self-tests run
- THEN they include positive completed-task fixtures
- AND negative fixtures for missing evidence label, missing docs evidence path, missing run log, missing BLAKE3, missing artifact, missing exit status, failed exit status, unrelated manifest, `target/` artifact path, and nested child-repo artifact path.

#### Scenario: Validation output is reviewable

r[mc_compatibility.harness_coverage.validation.reviewable]
- GIVEN the harness hardening is complete
- WHEN the change is archived
- THEN repo-local logs record checker self-tests, active task-evidence gate, evidence manifest check, Cairn gates, and Cairn validation under `docs/evidence/` with BLAKE3 sidecars.

### Requirement: Capture artifact contract

r[mc_compatibility.stevenarella_frame_capture.contract] Stevenarella frame capture MUST define bounded capture request, policy, and artifact metadata contracts before MCP capture tools are promoted.

#### Scenario: Capture scope is explicit

r[mc_compatibility.stevenarella_frame_capture.contract.scope]
- GIVEN frame capture work starts
- WHEN the capture contract is reviewed
- THEN it names single screenshot, latest frame, and bounded recording as the supported initial capture modes
- AND it states that visual-regression approval, semantic gameplay correctness, web capture, and headless EGL/OSMesa support remain non-claims.

#### Scenario: Invalid capture requests fail closed

r[mc_compatibility.stevenarella_frame_capture.contract.invalid]
- GIVEN a capture request uses an unsupported format, out-of-range fps, missing capture directory, path traversal, unbounded duration, or oversized output policy
- WHEN capture validation runs
- THEN it returns a structured error before GL readback or file write.

### Requirement: GL framebuffer readback

r[mc_compatibility.stevenarella_frame_capture.readback] Stevenarella frame capture MUST read pixels from Stevenarella's GL framebuffer rather than external host screenshot tools.

#### Scenario: Readback occurs after render

r[mc_compatibility.stevenarella_frame_capture.readback.after_render]
- GIVEN a native frame has completed `Renderer::tick(...)`
- WHEN a capture request is pending
- THEN Stevenarella reads RGBA pixels from the rendered framebuffer before native buffer swap
- AND the artifact metadata records the rendered dimensions and frame id.

#### Scenario: Origin is normalized

r[mc_compatibility.stevenarella_frame_capture.readback.origin]
- GIVEN GL returns pixels with bottom-left origin
- WHEN the screenshot buffer is encoded or digested
- THEN the buffer is normalized to top-left origin.

### Requirement: One-shot screenshot capture

r[mc_compatibility.stevenarella_frame_capture.screenshot] Stevenarella MUST support bounded one-shot screenshot capture through the control plane.

#### Scenario: Screenshot returns artifact metadata

r[mc_compatibility.stevenarella_frame_capture.screenshot.metadata]
- GIVEN an MCP client requests one screenshot
- WHEN the frame capture hook services the request
- THEN the response includes width, height, frame id, sequence id, format, UI inclusion status, and BLAKE3 digest
- AND it either includes bounded MCP image content or a contained artifact path.

### Requirement: Bounded frame recording

r[mc_compatibility.stevenarella_frame_capture.recording] Stevenarella frame recording MUST be bounded by policy.

#### Scenario: Recording cannot run unbounded

r[mc_compatibility.stevenarella_frame_capture.recording.bounded]
- GIVEN an MCP client requests frame recording
- WHEN the request lacks an explicit fps and duration or frame-count bound
- THEN recording is rejected before artifacts are written.

#### Scenario: Recording writes contained artifacts

r[mc_compatibility.stevenarella_frame_capture.recording.contained]
- GIVEN a valid bounded recording request and configured capture directory
- WHEN frames are captured
- THEN every written artifact stays under the configured capture directory
- AND every artifact has recorded BLAKE3 digest metadata.

### Requirement: MCP capture resources

r[mc_compatibility.stevenarella_frame_capture.mcp_resources] MCP capture tools/resources MUST expose screenshots and frames without forcing large unbounded JSON payloads.

#### Scenario: Large outputs are file-backed

r[mc_compatibility.stevenarella_frame_capture.mcp_resources.file_backed]
- GIVEN a capture output exceeds the configured inline response limit or belongs to a recording
- WHEN MCP returns the result
- THEN it returns artifact metadata and contained file paths rather than unbounded base64 content.

### Requirement: Capture validation evidence

r[mc_compatibility.stevenarella_frame_capture.validation] Frame capture MUST have positive and negative tests before promotion.

#### Scenario: Validation covers happy and sad paths

r[mc_compatibility.stevenarella_frame_capture.validation.tests]
- GIVEN frame capture implementation is complete
- WHEN focused tests run
- THEN they cover valid screenshot metadata and origin normalization
- AND they cover invalid format, path traversal, rate-limit rejection, and unbounded recording rejection.

### Requirement: Capture evidence artifacts

r[mc_compatibility.stevenarella_frame_capture.artifacts] Review-critical frame capture evidence MUST be copied under `docs/evidence/` before archive.

#### Scenario: Artifacts are durable

r[mc_compatibility.stevenarella_frame_capture.artifacts.reviewable]
- GIVEN the frame capture Cairn is ready to archive
- WHEN reviewers inspect the parent repo
- THEN focused test output, sample capture metadata, Cairn gate output, validation output, and BLAKE3 manifests are present under `docs/evidence/`.

### Requirement: Paired combat reference contract

r[mc_compatibility.vanilla_combat_reference_paired_receipts.contract] The `vanilla-combat-reference-parity` row MUST define a bounded paired-reference evidence contract before any combat reference-parity evidence is promoted.

#### Scenario: Contract names one bounded melee interaction

r[mc_compatibility.vanilla_combat_reference_paired_receipts.contract.scope]
- GIVEN the combat reference row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic attacker, victim, weapon, armor state, health delta, knockback metric, tolerance bound, reference oracle, reference version, Valence revision, and client revision
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Rust parity checker

r[mc_compatibility.vanilla_combat_reference_paired_receipts.checker] A Rust checker MUST compare normalized paired reference and Valence combat metrics through pure deterministic logic before the row is promoted.

#### Scenario: Valid paired evidence passes

r[mc_compatibility.vanilla_combat_reference_paired_receipts.checker.valid]
- GIVEN a Paper-reference record and a Valence record name the same row, attacker, victim, weapon, armor state, reference version, damage metric, knockback metric, and tolerance bounds
- WHEN the checker compares the records
- THEN it passes only if damage and knockback are within the configured bounds.

#### Scenario: Weak or mismatched evidence fails closed

r[mc_compatibility.vanilla_combat_reference_paired_receipts.checker.rejects]
- GIVEN evidence is missing the reference record, contains only Valence evidence, uses the wrong reference version, omits tolerance bounds, exceeds damage tolerance, exceeds knockback tolerance, reports a stale required revision, or mismatches weapon or armor state
- WHEN the checker compares the records
- THEN it fails and names the first missing or mismatched metric.

### Requirement: Isolated runner rail

r[mc_compatibility.vanilla_combat_reference_paired_receipts.rail] The harness MUST expose an isolated `vanilla-combat-reference-parity` rail for the bounded paired combat interaction without broadening existing scenario semantics.

#### Scenario: Existing combat rows remain unchanged

r[mc_compatibility.vanilla_combat_reference_paired_receipts.rail.isolated]
- GIVEN existing CTF combat, knockback, armor, projectile, and survival rows are maintained separately
- WHEN the paired combat reference rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the new rail records its own explicit client and server milestones.

### Requirement: Reference and Valence fixtures

r[mc_compatibility.vanilla_combat_reference_paired_receipts.fixtures] Paper-reference and Valence fixtures MUST record the same normalized server-side combat metrics for the configured interaction.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.vanilla_combat_reference_paired_receipts.fixtures.comparable]
- GIVEN the configured melee interaction runs against Paper-reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for attacker, victim, weapon, armor state, pre-health, post-health, damage delta, knockback metric, tolerance bounds, and backend identity
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable paired receipts

r[mc_compatibility.vanilla_combat_reference_paired_receipts.receipts] Paired combat reference evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.vanilla_combat_reference_paired_receipts.receipts.reviewable]
- GIVEN the paired row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper-reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and any oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow promotion

r[mc_compatibility.vanilla_combat_reference_paired_receipts.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `vanilla-combat-reference-parity` row after the paired comparator passes.

#### Scenario: Broad parity remains a non-claim

r[mc_compatibility.vanilla_combat_reference_paired_receipts.promotion.nonclaims]
- GIVEN the paired Paper-reference and Valence evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured combat reference-parity row is marked covered
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Validation and archive evidence

r[mc_compatibility.vanilla_combat_reference_paired_receipts.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.vanilla_combat_reference_paired_receipts.validation.log]
- GIVEN the paired combat reference row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record the checker self-tests, paired comparator, maintained dry-runs where applicable, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Armor combat reference contract

r[mc_compatibility.vanilla_combat_armor_reference_parity.contract] The `vanilla-combat-armor-reference-parity` row MUST define a bounded paired-reference evidence contract before any armor combat reference-parity evidence is promoted.

#### Scenario: Contract names one diamond-chestplate melee interaction

r[mc_compatibility.vanilla_combat_armor_reference_parity.contract.scope]
- GIVEN the armor combat reference row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic attacker, victim, iron-sword weapon, diamond-chestplate armor state, no enchantment, no status effect, health delta, knockback metric, tolerance bound, reference oracle, reference version, Valence revision, and client revision
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor loadouts, enchantments/status effects, modifier stacking, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Row-specific Rust parity checker

r[mc_compatibility.vanilla_combat_armor_reference_parity.checker] A Rust checker MUST compare normalized paired Paper-reference and Valence combat metrics through pure deterministic row-specific logic before the armor row is promoted.

#### Scenario: Valid armor paired evidence passes

r[mc_compatibility.vanilla_combat_armor_reference_parity.checker.valid]
- GIVEN a Paper-reference record and a Valence record name `vanilla-combat-armor-reference-parity`, `compatbota`, `compatbotb`, `iron_sword`, `diamond_chestplate`, `20.0`, `15.3`, `4.7`, the reference version, knockback metric, and tolerance bounds
- WHEN the checker compares the records
- THEN it passes only if damage, health, armor state, and knockback are within the configured row contract and bounds.

#### Scenario: Weak or mismatched armor evidence fails closed

r[mc_compatibility.vanilla_combat_armor_reference_parity.checker.rejects]
- GIVEN evidence is missing the reference record, contains only Valence evidence, uses an unknown row, uses the wrong reference version, omits tolerance bounds, reports a stale required revision, reports no armor for the armor row, mismatches weapon or armor state, or reports no-armor damage for the armor row
- WHEN the checker compares the records
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated armor runner rail

r[mc_compatibility.vanilla_combat_armor_reference_parity.rail] The harness MUST expose an isolated `vanilla-combat-armor-reference-parity` rail for the bounded paired armor interaction without broadening existing scenario semantics.

#### Scenario: Existing combat rows remain unchanged

r[mc_compatibility.vanilla_combat_armor_reference_parity.rail.isolated]
- GIVEN existing no-armor reference, CTF combat, knockback, armor mitigation, projectile, and survival rows are maintained separately
- WHEN the armor reference rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the new rail records its own explicit client and server milestones.

### Requirement: Armor reference and Valence fixtures

r[mc_compatibility.vanilla_combat_armor_reference_parity.fixtures] Paper-reference and Valence fixtures MUST record the same normalized server-side combat metrics for the configured diamond-chestplate interaction.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.vanilla_combat_armor_reference_parity.fixtures.comparable]
- GIVEN the configured armor melee interaction runs against Paper-reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for attacker, victim, weapon, armor state, pre-health, post-health, damage delta, knockback metric, tolerance bounds, backend identity, and row id
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable armor paired receipts

r[mc_compatibility.vanilla_combat_armor_reference_parity.receipts] Paired armor combat reference evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.vanilla_combat_armor_reference_parity.receipts.reviewable]
- GIVEN the armor row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper-reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow armor promotion

r[mc_compatibility.vanilla_combat_armor_reference_parity.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `vanilla-combat-armor-reference-parity` row after the paired comparator passes.

#### Scenario: Broad armor parity remains a non-claim

r[mc_compatibility.vanilla_combat_armor_reference_parity.promotion.nonclaims]
- GIVEN the paired Paper-reference and Valence armor evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured diamond-chestplate combat reference row is marked covered
- AND exact Mojang vanilla parity, broad combat balancing, all weapons, all armor loadouts, enchantments/status effects, modifier stacking, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Armor validation and archive evidence

r[mc_compatibility.vanilla_combat_armor_reference_parity.validation] The change MUST record checker, paired comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.vanilla_combat_armor_reference_parity.validation.log]
- GIVEN the armor combat reference row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record the checker self-tests, paired comparator, fixture and runner tests, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.
