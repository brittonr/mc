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
