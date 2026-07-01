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

r[mc_compatibility.valence_compat_bot.safe_bounded_probe] Valence compatibility testing MUST provide a bounded owned bot/probe mode derived from the historical Hyperion bot pattern without enabling unbounded public stress traffic.

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
- THEN the plan records direct-vs-proxy claims and defers full Hyperion-style multi-proxy/mTLS unless separately requested with reviewable source evidence

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

r[mc_compatibility.valence_gameplay_oracles.milestone_catalog] Valence compatibility work MUST catalog historical Hyperion-derived gameplay milestones before implementing new scenario claims.

#### Scenario: Milestones are mapped to Valence

r[mc_compatibility.valence_gameplay_oracles.milestone_catalog.scenario]

- GIVEN archived or source-snapshot Hyperion Bedwars milestones are reviewed
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

### Requirement: Stack split/merge promotion contract

r[mc_compatibility.inventory_stack_split_merge_promotion.contract] The `inventory-stack-split-merge` row MUST define a bounded promotion contract before any matrix or current-bundle coverage is claimed.

#### Scenario: Promotion scope is exact

r[mc_compatibility.inventory_stack_split_merge_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one deterministic actor, item, source slot, destination slot, initial count, split action, carried count, merge action, final counts, state-id sequence, Valence server correlation, child revisions, and comparator/checker metrics
- AND drag transactions, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Stack split/merge checker

r[mc_compatibility.inventory_stack_split_merge_promotion.checker] A deterministic Rust checker MUST validate normalized stack split/merge evidence before promotion.

#### Scenario: Valid row evidence passes

r[mc_compatibility.inventory_stack_split_merge_promotion.checker.valid]
- GIVEN a receipt or normalized KV record names `inventory-stack-split-merge`, clean child revisions, the configured actor/item/source slot/destination slot/counts, state-id sequence, and Valence server `ClickSlot` correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak row evidence fails closed

r[mc_compatibility.inventory_stack_split_merge_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale or unknown child revisions, omits state-id data, mismatches source/destination slot counts, records the wrong item, lacks server `ClickSlot` correlation, contains Valence-only unreviewable target output, or claims broad inventory coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated stack split/merge rail

r[mc_compatibility.inventory_stack_split_merge_promotion.rail] The harness MUST expose an isolated `inventory-stack-split-merge` rail without changing existing inventory, survival, CTF, protocol, combat, or negative-live semantics.

#### Scenario: Existing rows stay separate

r[mc_compatibility.inventory_stack_split_merge_promotion.rail.isolated]
- GIVEN existing maintained inventory rows cover drop, pickup, player-inventory click, open-container click, and block placement/use-item-on-block
- WHEN the stack split/merge rail is added
- THEN existing scenario milestones and non-claims remain unchanged
- AND the new row records its own client and server milestones for split, merge, state-id, slot counts, and server correlation.

### Requirement: Reviewable stack split/merge artifacts

r[mc_compatibility.inventory_stack_split_merge_promotion.artifacts] Review-critical stack split/merge artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.inventory_stack_split_merge_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized comparator or KV inputs, checker output, BLAKE3 manifests, child revisions, and any oracle limitation checkpoint are present under `docs/evidence/`.

### Requirement: Narrow stack split/merge matrix promotion

r[mc_compatibility.inventory_stack_split_merge_promotion.matrix] Acceptance matrix and current-bundle docs MUST promote only the configured `inventory-stack-split-merge` row after checker and evidence gates pass.

#### Scenario: Broader inventory remains a non-claim

r[mc_compatibility.inventory_stack_split_merge_promotion.matrix.nonclaims]
- GIVEN stack split/merge evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured stack split/merge row is marked covered
- AND drag transactions, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Stack split/merge validation evidence

r[mc_compatibility.inventory_stack_split_merge_promotion.validation] The change MUST record checker, runner, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.inventory_stack_split_merge_promotion.validation.log]
- GIVEN the stack split/merge row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Drag transaction promotion contract

r[mc_compatibility.inventory_drag_transactions_promotion.contract] The `inventory-drag-transactions` row MUST define a bounded promotion contract before any matrix or current-bundle coverage is claimed.

#### Scenario: Promotion scope is exact

r[mc_compatibility.inventory_drag_transactions_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one deterministic actor, item, source slot, target slots, drag phase sequence, carried-stack state, final source and target counts, state-id sequence, Valence server quick-craft correlation, child revisions, and comparator/checker metrics
- AND all drag modes, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, stack split/merge outside its dedicated row, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Drag transaction checker

r[mc_compatibility.inventory_drag_transactions_promotion.checker] A deterministic Rust checker MUST validate normalized drag transaction evidence before promotion.

#### Scenario: Valid row evidence passes

r[mc_compatibility.inventory_drag_transactions_promotion.checker.valid]
- GIVEN a receipt or normalized KV record names `inventory-drag-transactions`, clean child revisions, the configured actor/item/source slot/target slots/final counts, drag start/add/end phase sequence, state-id sequence, and Valence server quick-craft correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak row evidence fails closed

r[mc_compatibility.inventory_drag_transactions_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale or unknown child revisions, omits state-id data, omits or reorders drag phases, mismatches source or target slot counts, records the wrong item, lacks server quick-craft correlation, contains Valence-only unreviewable target output, or claims all drag or broad inventory coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated drag transaction rail

r[mc_compatibility.inventory_drag_transactions_promotion.rail] The harness MUST expose an isolated `inventory-drag-transactions` rail without changing existing inventory, survival, CTF, protocol, combat, stack split/merge, or negative-live semantics.

#### Scenario: Existing rows stay separate

r[mc_compatibility.inventory_drag_transactions_promotion.rail.isolated]
- GIVEN existing maintained inventory rows cover drop, pickup, player-inventory click, open-container click, block placement/use-item-on-block, and one stack split/merge fixture
- WHEN the drag transaction rail is added
- THEN existing scenario milestones and non-claims remain unchanged
- AND the new row records its own client and server milestones for drag phases, state-id, slot counts, carried-stack state, and server correlation.

### Requirement: Reviewable drag transaction artifacts

r[mc_compatibility.inventory_drag_transactions_promotion.artifacts] Review-critical drag transaction artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.inventory_drag_transactions_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized comparator or KV inputs, checker output, BLAKE3 manifests, child revisions, and any oracle limitation checkpoint are present under `docs/evidence/`.

### Requirement: Narrow drag transaction matrix promotion

r[mc_compatibility.inventory_drag_transactions_promotion.matrix] Acceptance matrix and current-bundle docs MUST promote only the configured `inventory-drag-transactions` row after checker and evidence gates pass.

#### Scenario: Broader inventory remains a non-claim

r[mc_compatibility.inventory_drag_transactions_promotion.matrix.nonclaims]
- GIVEN drag transaction evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured drag transaction row is marked covered
- AND all drag modes, creative inventory, all windows, all click modes, all inventory transactions, all inventory semantics, stack split/merge outside its dedicated row, broad inventory parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Drag transaction validation evidence

r[mc_compatibility.inventory_drag_transactions_promotion.validation] The change MUST record checker, runner, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.inventory_drag_transactions_promotion.validation.log]
- GIVEN the drag transaction row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.

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

### Requirement: Projectile travel/collision live rail inventory

r[mc_compatibility.projectile_travel_collision_live_rail.inventory] The change MUST inventory existing projectile use/loadout evidence, projectile damage-attribution evidence, residual projectile non-claims, and candidate live signals before selecting a travel/collision row.

#### Scenario: Existing projectile boundaries are visible

r[mc_compatibility.projectile_travel_collision_live_rail.inventory.reviewable]
- GIVEN projectile travel/collision work begins
- WHEN reviewers inspect the inventory
- THEN it names covered projectile rows, residual non-claims, candidate travel/collision signals, and the selected bounded row.

### Requirement: Focused projectile travel/collision matrix row

r[mc_compatibility.projectile_travel_collision_live_rail.matrix] The selected live rail MUST define one focused projectile matrix row with weapon representative, projectile representative, attacker identity, target or collision identity, ordered observations, comparison rule, and explicit non-claims.

#### Scenario: Row scopes the projectile claim

r[mc_compatibility.projectile_travel_collision_live_rail.matrix.scoped]
- GIVEN the projectile matrix row is reviewed
- WHEN the selected row is inspected
- THEN it names the projectile representative, weapon representative, attacker, target or collision identity, required server events, required client observations, and non-claim labels
- AND unselected projectile weapons and collision surfaces remain non-claims.

### Requirement: Pure projectile travel/collision comparator

r[mc_compatibility.projectile_travel_collision_live_rail.comparator] Projectile travel/collision comparison MUST be a pure deterministic core over normalized records and MUST fail closed for missing travel, missing collision, wrong target, wrong weapon, unordered sequence, ambiguous projectile identity, or overbroad parity claims.

#### Scenario: Complete projectile row passes

r[mc_compatibility.projectile_travel_collision_live_rail.comparator.positive]
- GIVEN normalized server and client records contain the selected projectile launch, ordered travel observation, collision or hit result, attacker, target, and weapon representative
- WHEN the comparator evaluates the row
- THEN comparison passes with stable diagnostics.

#### Scenario: Weak projectile row fails

r[mc_compatibility.projectile_travel_collision_live_rail.comparator.negative]
- GIVEN projectile records are missing travel observation, missing collision result, unordered, wrong-target, wrong-weapon, ambiguous, or claiming broad vanilla physics
- WHEN the comparator evaluates the row
- THEN comparison fails and names the missing or invalid evidence.

### Requirement: Projectile travel/collision live rail wiring

r[mc_compatibility.projectile_travel_collision_live_rail.wiring] Runner, Stevenarella probe, and Valence fixture shells MUST emit typed projectile metrics for the selected owned-local live row while preserving existing combat scenario names, receipt behavior, and non-claims.

#### Scenario: Existing combat rails remain stable

r[mc_compatibility.projectile_travel_collision_live_rail.wiring.compatible]
- GIVEN the new projectile rail is wired
- WHEN existing combat damage, knockback, projectile use/loadout, and damage-attribution scenarios run or dry-run
- THEN their scenario names, wrapper behavior, and bounded non-claim fields remain compatible.

### Requirement: Projectile travel/collision live evidence

r[mc_compatibility.projectile_travel_collision_live_rail.evidence] The selected live rail MUST produce reviewable receipts, client logs, server logs, comparator output, evidence docs, and BLAKE3 manifests under `docs/evidence/` before promotion.

#### Scenario: Evidence bundle is reviewable

r[mc_compatibility.projectile_travel_collision_live_rail.evidence.reviewable]
- GIVEN the live projectile rail passes
- WHEN evidence is promoted
- THEN the receipt, logs, comparator output, evidence note, and BLAKE3 manifest are copied under `docs/evidence/`
- AND the evidence note states the bounded row and adjacent non-claims.

### Requirement: Projectile travel/collision row promotion boundary

r[mc_compatibility.projectile_travel_collision_live_rail.promotion] Acceptance/current-bundle docs MUST promote only the configured projectile travel/collision row after comparator and manifest validation pass.

#### Scenario: Adjacent projectile claims remain blocked

r[mc_compatibility.projectile_travel_collision_live_rail.promotion.bounded]
- GIVEN the selected row is promoted
- WHEN reviewers inspect acceptance and current-bundle docs
- THEN only the configured row is marked covered
- AND exact vanilla projectile physics, all projectile weapons, all collision surfaces, full combat correctness, public-server safety, and production readiness remain non-claims.

### Requirement: Projectile rail closeout

r[mc_compatibility.projectile_travel_collision_live_rail.closeout] The change MUST record focused live validation, comparator fixtures, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.projectile_travel_collision_live_rail.closeout.log]
- GIVEN the projectile travel/collision rail is ready to archive
- WHEN reviewers inspect task evidence
- THEN logs show positive and negative comparator fixtures, focused live validation, evidence manifest validation, task-evidence validation, Cairn proposal/design/tasks gates, and Cairn validation.

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

### Requirement: Valence CTF fixture module boundaries

r[mc_compatibility.valence_fixture_modularity.ctf_boundaries] The Valence CTF compatibility fixture SHOULD expose cohesive module boundaries for runtime config, arena setup, team and flag rules, scoring rules, inventory probes, combat and projectile probes, schedule contracts, and milestone formatting.

#### Scenario: CTF responsibility has one owner

r[mc_compatibility.valence_fixture_modularity.ctf_boundaries.ownership]
- GIVEN a CTF fixture responsibility is reviewed
- WHEN maintainers inspect the fixture module tree
- THEN that responsibility is owned by a focused module or pure fixture-core component
- AND unrelated CTF responsibilities are not added back to the root example shell.

### Requirement: Valence CTF fixture functional core

r[mc_compatibility.valence_fixture_modularity.ctf_functional_core] Non-trivial CTF rule and probe decisions SHOULD live in pure deterministic cores that return explicit decisions for Bevy system shells to apply.

#### Scenario: CTF rule is testable without ECS

r[mc_compatibility.valence_fixture_modularity.ctf_functional_core.testable]
- GIVEN CTF logic decides flag ownership, scoring, inventory probe state, combat evidence, projectile evidence, team reset state, or milestone text
- WHEN the logic is extracted
- THEN the decision can be tested with in-memory inputs
- AND Bevy ECS queries, resource mutation, packet/event emission, filesystem reads, and logging remain in shells.

### Requirement: Valence CTF fixture parity

r[mc_compatibility.valence_fixture_modularity.ctf_parity] CTF fixture modularization MUST preserve existing env flags, milestone vocabulary, schedule contracts, dry-run and live evidence boundaries, and non-claims.

#### Scenario: CTF evidence boundary remains stable

r[mc_compatibility.valence_fixture_modularity.ctf_parity.stable]
- GIVEN a supported pre-refactor CTF fixture probe input
- WHEN the modularized fixture processes the same input
- THEN the emitted milestones, schedule contract behavior, and non-claim boundaries remain equivalent
- AND no new CTF correctness or broad compatibility claim is promoted.

### Requirement: Valence CTF fixture positive tests

r[mc_compatibility.valence_fixture_modularity.ctf_positive_tests] The change MUST include positive tests for representative flag, score, team-balance, inventory, combat, projectile, runtime-config, and milestone decisions.

#### Scenario: Supported CTF decisions pass

r[mc_compatibility.valence_fixture_modularity.ctf_positive_tests.coverage]
- GIVEN representative supported CTF fixture inputs
- WHEN extracted CTF cores process them
- THEN tests prove the expected decisions, state transitions, or milestone text are produced.

### Requirement: Valence CTF fixture negative tests

r[mc_compatibility.valence_fixture_modularity.ctf_negative_tests] The change MUST include negative tests for disabled probes, invalid runtime config, stale flag ownership, duplicate scoring or win emission, malformed inventory events, and unsupported arrow policy input.

#### Scenario: Invalid CTF decisions fail closed

r[mc_compatibility.valence_fixture_modularity.ctf_negative_tests.fail_closed]
- GIVEN invalid or unsupported CTF fixture inputs
- WHEN extracted CTF cores process them
- THEN tests prove the inputs are rejected, ignored, or contained according to current fixture behavior without corrupting state.

### Requirement: Valence CTF fixture validation

r[mc_compatibility.valence_fixture_modularity.ctf_validation] The change MUST record focused Valence/example tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_fixture_modularity.ctf_validation.logs]
- GIVEN CTF fixture modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative fixture-core tests plus affected dry-runs and Cairn gates passing.

### Requirement: Survival fixture module boundaries

r[mc_compatibility.valence_fixture_modularity.survival_boundaries] The Valence survival compatibility fixture SHOULD expose cohesive module boundaries for runtime config, arena setup, containers, crafting, furnace, hunger and health, mob drops, redstone, persistence, block entities, biome and dimension behavior, sign editing, breadth fixtures, and milestone formatting.

#### Scenario: Survival responsibility has one owner

r[mc_compatibility.valence_fixture_modularity.survival_boundaries.ownership]
- GIVEN a survival fixture responsibility is reviewed
- WHEN maintainers inspect the fixture module tree
- THEN that responsibility is owned by a focused module or pure fixture-core component
- AND unrelated survival responsibilities are not added back to the root example shell.

### Requirement: Survival fixture functional core

r[mc_compatibility.valence_fixture_modularity.survival_functional_core] Non-trivial survival fixture predicates, classifications, transitions, and milestone construction SHOULD live in pure deterministic cores that return explicit decisions for Bevy system shells to apply.

#### Scenario: Survival fixture decision is testable without ECS

r[mc_compatibility.valence_fixture_modularity.survival_functional_core.testable]
- GIVEN survival logic decides item or slot classification, container clicks, hunger changes, mob-drop pickup, redstone state, persistence phases, biome/dimension identity, sign text, or milestone text
- WHEN the logic is extracted
- THEN the decision can be tested with in-memory inputs
- AND Bevy ECS mutation, packet/event emission, marker-file writes, and logging remain in shells.

### Requirement: Survival fixture parity

r[mc_compatibility.valence_fixture_modularity.survival_parity] Survival fixture modularization MUST preserve existing env flags, fixture semantics, milestone vocabulary, persistence phases, evidence boundaries, and non-claims.

#### Scenario: Survival evidence boundary remains stable

r[mc_compatibility.valence_fixture_modularity.survival_parity.stable]
- GIVEN a supported pre-refactor survival fixture probe input
- WHEN the modularized fixture processes the same input
- THEN the emitted milestones, persistence phase behavior, and non-claim boundaries remain equivalent
- AND no full-survival or broad compatibility claim is promoted.

### Requirement: Survival fixture positive tests

r[mc_compatibility.valence_fixture_modularity.survival_positive_tests] The change MUST include positive tests for representative chest, crafting, furnace, hunger, mob-drop, redstone, persistence, block-entity, biome/dimension, sign, and breadth decisions.

#### Scenario: Supported survival decisions pass

r[mc_compatibility.valence_fixture_modularity.survival_positive_tests.coverage]
- GIVEN representative supported survival fixture inputs
- WHEN extracted survival cores process them
- THEN tests prove the expected decisions, state transitions, marker plans, or milestone text are produced.

### Requirement: Survival fixture negative tests

r[mc_compatibility.valence_fixture_modularity.survival_negative_tests] The change MUST include negative tests for disabled fixtures, invalid runtime config, wrong slots or items, malformed clicks, missing marker paths, invalid persistence phase, and unsupported environment IDs.

#### Scenario: Invalid survival decisions fail closed

r[mc_compatibility.valence_fixture_modularity.survival_negative_tests.fail_closed]
- GIVEN invalid or unsupported survival fixture inputs
- WHEN extracted survival cores process them
- THEN tests prove the inputs are rejected, ignored, or contained according to current fixture behavior without corrupting state or evidence boundaries.

### Requirement: Survival fixture validation

r[mc_compatibility.valence_fixture_modularity.survival_validation] The change MUST record focused Valence/example tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_fixture_modularity.survival_validation.logs]
- GIVEN survival fixture modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative fixture-core tests plus affected dry-runs and Cairn gates passing.

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

### Requirement: Hunger health-cycle contract

r[mc_compatibility.survival_hunger_health_cycle_parity.contract] The `survival-hunger-health-cycle-parity` row MUST define a bounded health-cycle evidence contract before promotion.

#### Scenario: Contract names controlled cycle

r[mc_compatibility.survival_hunger_health_cycle_parity.contract.scope]
- GIVEN hunger-health work starts
- WHEN the contract is reviewed
- THEN it names starting health, food, saturation, exhaustion trigger, regeneration checkpoint, starvation or low-food checkpoint, inventory consumption, and normalized comparison metrics
- AND all foods, all exhaustion sources, potion/effect interactions, full survival compatibility, and broad vanilla parity remain non-claims.

### Requirement: Hunger health-cycle checker

r[mc_compatibility.survival_hunger_health_cycle_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence hunger-health metrics before promotion.

#### Scenario: Weak hunger evidence fails closed

r[mc_compatibility.survival_hunger_health_cycle_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits food/health/saturation metrics, mismatches named checkpoints, reports stale child revisions, or claims broad hunger mechanics
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the invalid health-cycle metric.

### Requirement: Isolated hunger health-cycle rail

r[mc_compatibility.survival_hunger_health_cycle_parity.rail] The harness MUST expose an isolated hunger-health rail without changing the existing Bread consumption row.

#### Scenario: Existing hunger row remains unchanged

r[mc_compatibility.survival_hunger_health_cycle_parity.rail.isolated]
- GIVEN the existing Bread consumption row is promoted
- WHEN the hunger-health rail is added
- THEN existing row milestones and non-claims remain unchanged
- AND the new row records its own health-cycle checkpoints.

### Requirement: Reviewable hunger health-cycle receipts

r[mc_compatibility.survival_hunger_health_cycle_parity.receipts] Paired hunger-health receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are reviewable

r[mc_compatibility.survival_hunger_health_cycle_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect `docs/evidence/`
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are present.

### Requirement: Narrow hunger health-cycle promotion

r[mc_compatibility.survival_hunger_health_cycle_parity.promotion] Matrix and bundle docs MUST promote only the bounded hunger-health cycle row after paired evidence passes.

#### Scenario: Broader hunger remains a non-claim

r[mc_compatibility.survival_hunger_health_cycle_parity.promotion.nonclaims]
- GIVEN paired hunger-health evidence passes
- WHEN docs are updated
- THEN only the configured hunger-health row is marked covered
- AND all foods, all exhaustion sources, potion/effect interactions, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Hunger health-cycle validation evidence

r[mc_compatibility.survival_hunger_health_cycle_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_hunger_health_cycle_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.

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

### Requirement: Stevenarella MCP module boundaries

r[mc_compatibility.stevenarella_mcp.module_boundaries] Stevenarella MCP code SHOULD expose cohesive module boundaries for JSON-RPC protocol handling, auth validation, transport runtime, control queue adaptation, tool and resource registry, and capture-tool adaptation.

#### Scenario: MCP responsibility has one owner

r[mc_compatibility.stevenarella_mcp.module_boundaries.ownership]
- GIVEN an MCP responsibility is reviewed
- WHEN maintainers inspect the MCP module tree
- THEN the responsibility is owned by a focused module
- AND unrelated transport, tool, auth, and capture concerns are not reintroduced into one root module.

### Requirement: Stevenarella MCP protocol core

r[mc_compatibility.stevenarella_mcp.protocol_core] MCP request routing and JSON-RPC response rendering SHOULD be pure over in-memory request values, explicit auth state, and explicit tool adapter outcomes.

#### Scenario: MCP request routing is testable without transport

r[mc_compatibility.stevenarella_mcp.protocol_core.testable]
- GIVEN an MCP JSON-RPC request line and explicit adapter outcomes
- WHEN the protocol core handles the request
- THEN the result can be tested without stdio, TCP sockets, threads, capture waits, game state, or channel side effects
- AND transport/runtime shells remain responsible for those side effects.

### Requirement: Stevenarella MCP parity

r[mc_compatibility.stevenarella_mcp.parity] MCP modularization MUST preserve endpoint opt-in behavior, auth semantics, tool and resource names, JSON-RPC error codes, response shapes, capture and control outcomes, and evidence non-claims.

#### Scenario: MCP public surface remains stable

r[mc_compatibility.stevenarella_mcp.parity.stable]
- GIVEN a supported pre-refactor MCP request or startup option
- WHEN the modularized MCP surface processes the same input
- THEN the endpoint behavior, auth result, JSON-RPC response, tool/resource vocabulary, and non-claim boundaries remain equivalent
- AND no new game-control capability is enabled by default.

### Requirement: Stevenarella MCP positive tests

r[mc_compatibility.stevenarella_mcp.positive_tests] The change MUST include positive tests for tools/list, resources/list, control calls, capture calls, resource reads, auth success, stdio options, and TCP endpoint validation.

#### Scenario: Supported MCP paths pass

r[mc_compatibility.stevenarella_mcp.positive_tests.coverage]
- GIVEN representative supported MCP requests and startup inputs
- WHEN extracted MCP modules process them
- THEN tests prove the expected responses, adapter calls, and validation outcomes are produced.

### Requirement: Stevenarella MCP negative tests

r[mc_compatibility.stevenarella_mcp.negative_tests] The change MUST include negative tests for malformed JSON, unknown methods, missing tools, unauthorized requests, invalid capture arguments, closed queues, empty token env names, and invalid TCP auth.

#### Scenario: Invalid MCP paths fail closed

r[mc_compatibility.stevenarella_mcp.negative_tests.fail_closed]
- GIVEN invalid or unauthorized MCP requests or startup inputs
- WHEN extracted MCP modules process them
- THEN tests prove the inputs are rejected with the expected JSON-RPC error, validation diagnostic, or containment outcome.

### Requirement: Stevenarella MCP validation

r[mc_compatibility.stevenarella_mcp.validation] The change MUST record focused Stevenarella MCP tests, affected mc-compat MCP dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_mcp.validation.logs]
- GIVEN MCP modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative MCP tests plus affected dry-runs and Cairn gates passing.

### Requirement: Stevenarella control core

r[mc_compatibility.stevenarella_control.control_core] Stevenarella control command validation, normalization, response classification, key/look/mouse payload checks, and command capability facts SHOULD be pure over explicit command inputs.

#### Scenario: Control command decision is testable without game state

r[mc_compatibility.stevenarella_control.control_core.testable]
- GIVEN a control command input
- WHEN the control core validates or classifies it
- THEN the result can be tested without MCP transport, game mutation, capture queues, network sends, or logging.

### Requirement: Stevenarella control shell boundary

r[mc_compatibility.stevenarella_control.shell_boundary] Control-core extraction MUST keep MCP transport, game-state mutation, capture queues, network sends, and logging outside pure control cores.

#### Scenario: Control side effects remain in shell

r[mc_compatibility.stevenarella_control.shell_boundary.effects]
- GIVEN the control core returns a command decision
- WHEN MCP or game shell applies that decision
- THEN only the shell mutates game state, sends packets, queues capture, handles transport, or logs diagnostics.

### Requirement: Stevenarella control parity

r[mc_compatibility.stevenarella_control.parity] Control-core extraction MUST preserve JSON/control schema, response vocabulary, validation behavior, command names, and evidence non-claims.

#### Scenario: Control behavior remains stable

r[mc_compatibility.stevenarella_control.parity.stable]
- GIVEN a supported pre-refactor control command
- WHEN the extracted control core and shell process the same input
- THEN schema behavior, validation result, command name, response vocabulary, and non-claim boundary remain equivalent.

### Requirement: Stevenarella control positive tests

r[mc_compatibility.stevenarella_control.positive_tests] The change MUST include positive tests for status, connect, disconnect, key, look, mouse, use, attack, chat, resource-pack, sign-editor, and capture command validation.

#### Scenario: Supported control paths pass

r[mc_compatibility.stevenarella_control.positive_tests.coverage]
- GIVEN representative supported control commands
- WHEN extracted control cores process them
- THEN tests prove the expected validation and classification outcomes are produced.

### Requirement: Stevenarella control negative tests

r[mc_compatibility.stevenarella_control.negative_tests] The change MUST include negative tests for malformed commands, invalid keys, out-of-range look values, invalid mouse deltas, missing payloads, unsupported commands, and schema mismatches.

#### Scenario: Invalid control paths fail closed

r[mc_compatibility.stevenarella_control.negative_tests.fail_closed]
- GIVEN invalid control commands
- WHEN extracted control cores process them
- THEN tests prove the inputs are rejected or diagnosed according to current behavior.

### Requirement: Stevenarella control validation

r[mc_compatibility.stevenarella_control.validation] The change MUST record focused control/MCP tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_control.validation.logs]
- GIVEN control-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative control tests plus affected checks and Cairn gates passing.

### Requirement: Stevenarella game shell boundaries

r[mc_compatibility.stevenarella_game.shell_boundaries] Stevenarella game startup and runtime shell code SHOULD expose cohesive boundaries for startup options, game lifecycle, MCP control application, capture startup, connection orchestration, ticking, and window events.

#### Scenario: Game shell responsibility has one owner

r[mc_compatibility.stevenarella_game.shell_boundaries.ownership]
- GIVEN a client shell responsibility is reviewed
- WHEN maintainers inspect the game shell modules
- THEN the responsibility is owned by a focused module
- AND unrelated startup, control, capture, connection, tick, and window concerns are not reintroduced into one root module.

### Requirement: Stevenarella control core

r[mc_compatibility.stevenarella_game.control_core] Non-trivial MCP control and startup decisions SHOULD be pure over explicit state summaries and return explicit shell actions or responses.

#### Scenario: Control decision is testable without live game state

r[mc_compatibility.stevenarella_game.control_core.testable]
- GIVEN a control command, startup option, or capture startup input
- WHEN the pure game-shell core evaluates it
- THEN the resulting response or shell action can be tested without renderer, window, filesystem, or network side effects.

### Requirement: Stevenarella game shell parity

r[mc_compatibility.stevenarella_game.parity] Game-shell modularization MUST preserve existing CLI flags, MCP response vocabulary, capture behavior, connection behavior, window behavior, and evidence non-claims.

#### Scenario: Game shell behavior remains stable

r[mc_compatibility.stevenarella_game.parity.stable]
- GIVEN a supported pre-refactor startup option, control command, or window event
- WHEN the modularized game shell processes the same input
- THEN the selected action, response message, side-effect boundary, and non-claim behavior remain equivalent.

### Requirement: Stevenarella game shell positive tests

r[mc_compatibility.stevenarella_game.positive_tests] The change MUST include positive tests for status, connect, disconnect, key, look, mouse, use-item, attack, chat, sign-editor, capture, and startup option plans.

#### Scenario: Supported game-shell paths pass

r[mc_compatibility.stevenarella_game.positive_tests.coverage]
- GIVEN representative supported game-shell inputs
- WHEN extracted game-shell cores process them
- THEN tests prove the expected responses or action plans are produced.

### Requirement: Stevenarella game shell negative tests

r[mc_compatibility.stevenarella_game.negative_tests] The change MUST include negative tests for disconnected commands, missing player state, invalid sign editor state, invalid capture request, unavailable queues, invalid startup recording options, and out-of-range look input.

#### Scenario: Invalid game-shell paths fail closed

r[mc_compatibility.stevenarella_game.negative_tests.fail_closed]
- GIVEN invalid game-shell inputs
- WHEN extracted game-shell cores process them
- THEN tests prove the inputs are rejected, deferred, or contained with the expected response before unintended side effects occur.

### Requirement: Stevenarella game shell validation

r[mc_compatibility.stevenarella_game.validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_game.validation.logs]
- GIVEN game-shell modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative game-shell tests plus affected dry-runs and Cairn gates passing.

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

### Requirement: Survival crash-recovery contract

r[mc_compatibility.survival_crash_recovery_parity.contract] The `survival-crash-recovery-parity` row MUST define a bounded paired-reference evidence contract before any crash-recovery survival evidence is promoted.

#### Scenario: Contract names one crash-recovered block mutation

r[mc_compatibility.survival_crash_recovery_parity.contract.scope]
- GIVEN the crash-recovery row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic actor, block, position, isolated storage scope, ungraceful stop method, backend restart, reconnect, post-crash observation, Paper/reference backend, Valence backend, child revisions, and comparator metrics
- AND long-term durability, arbitrary crash consistency, multi-chunk persistence, all block entities, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Crash row parity checker

r[mc_compatibility.survival_crash_recovery_parity.checker] A deterministic Rust checker MUST compare normalized paired Paper/reference and Valence crash-recovery metrics before the row is promoted.

#### Scenario: Valid crash-recovery paired evidence passes

r[mc_compatibility.survival_crash_recovery_parity.checker.valid]
- GIVEN Paper and Valence records name `survival-crash-recovery-parity`, clean child revisions, `Dirt`, position `24,64,0`, forced stop, isolated storage, crash-recovery restart, reconnect, post-crash observation, and server recovery state
- WHEN the checker compares the records
- THEN it passes only if every configured metric is present and equal across Paper and Valence.

#### Scenario: Weak crash-recovery evidence fails closed

r[mc_compatibility.survival_crash_recovery_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, uses an unknown row, omits a configured crash metric, reports a stale required revision, lacks child revision metadata, uses a graceful shutdown metric, or mismatches the post-crash block state
- WHEN the checker compares the records
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated crash-recovery runner rail

r[mc_compatibility.survival_crash_recovery_parity.rail] The harness MUST expose an isolated `survival-crash-recovery-parity` rail without broadening existing survival or graceful world-persistence semantics.

#### Scenario: Graceful and crash rows stay separate

r[mc_compatibility.survival_crash_recovery_parity.rail.isolated]
- GIVEN existing survival rows and the `survival-world-persistence-restart` graceful row are maintained separately
- WHEN the crash-recovery rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the crash row records its own explicit client and server milestones for forced stop, crash-recovery restart, reconnect, and post-crash observation.

### Requirement: Crash-reference and Valence fixtures

r[mc_compatibility.survival_crash_recovery_parity.fixtures] Paper/reference and Valence fixtures MUST record comparable crash-recovery server metrics for the configured mutation.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.survival_crash_recovery_parity.fixtures.comparable]
- GIVEN the configured crash-recovery interaction runs against Paper/reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for actor, block, position, isolated storage, ungraceful stop, backend restart, reconnect, post-crash observation, backend identity, and row id
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable crash-recovery paired receipts

r[mc_compatibility.survival_crash_recovery_parity.receipts] Paired crash-recovery evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.survival_crash_recovery_parity.receipts.reviewable]
- GIVEN the crash-recovery row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper/reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow crash-recovery promotion

r[mc_compatibility.survival_crash_recovery_parity.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `survival-crash-recovery-parity` row after the paired comparator passes.

#### Scenario: Broad durability remains a non-claim

r[mc_compatibility.survival_crash_recovery_parity.promotion.nonclaims]
- GIVEN paired crash-recovery evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured crash-recovery row is marked covered
- AND long-term durability, arbitrary crash consistency, multi-chunk persistence, all block entities, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Crash-recovery validation and archive evidence

r[mc_compatibility.survival_crash_recovery_parity.validation] The change MUST record checker, paired comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.survival_crash_recovery_parity.validation.log]
- GIVEN the crash-recovery row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival block-entity persistence contract

r[mc_compatibility.survival_block_entity_persistence_parity.contract] The `survival-block-entity-persistence-parity` row MUST define a bounded paired-reference evidence contract before any block-entity persistence survival evidence is promoted.

#### Scenario: Contract names one sign block entity payload

r[mc_compatibility.survival_block_entity_persistence_parity.contract.scope]
- GIVEN the block-entity persistence row is prepared
- WHEN the contract is reviewed
- THEN it names one deterministic actor, block entity kind, sign text payload, position, restart method, reconnect, post-restart observation, Paper/reference backend, Valence backend, child revisions, and comparator metrics
- AND all block entities, arbitrary NBT parity, sign editing UI semantics, multi-chunk persistence, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity row parity checker

r[mc_compatibility.survival_block_entity_persistence_parity.checker] A deterministic Rust checker MUST compare normalized paired Paper/reference and Valence sign block-entity persistence metrics before the row is promoted.

#### Scenario: Valid block-entity paired evidence passes

r[mc_compatibility.survival_block_entity_persistence_parity.checker.valid]
- GIVEN Paper and Valence records name `survival-block-entity-persistence-parity`, clean child revisions, the configured sign block entity kind, position, text payload, restart method, reconnect, post-restart observation, and server persistence state
- WHEN the checker compares the records
- THEN it passes only if every configured metric is present and equal across Paper and Valence.

#### Scenario: Weak block-entity evidence fails closed

r[mc_compatibility.survival_block_entity_persistence_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, uses an unknown row, omits a configured sign metric, reports a stale required revision, lacks child revision metadata, uses the wrong block entity kind, mismatches the post-restart text payload, or reports the wrong position
- WHEN the checker compares the records
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Isolated block-entity persistence runner rail

r[mc_compatibility.survival_block_entity_persistence_parity.rail] The harness MUST expose an isolated `survival-block-entity-persistence-parity` rail without broadening existing survival, graceful world-persistence, or crash-recovery semantics.

#### Scenario: Existing persistence rows stay separate

r[mc_compatibility.survival_block_entity_persistence_parity.rail.isolated]
- GIVEN existing survival rows, the graceful ordinary-block world-persistence row, and the crash-recovery row are maintained separately
- WHEN the block-entity persistence rail is added
- THEN their required milestones and non-claims remain unchanged
- AND the block-entity row records its own explicit client and server milestones for sign mutation, restart, reconnect, and post-restart sign observation.

### Requirement: Block-entity reference and Valence fixtures

r[mc_compatibility.survival_block_entity_persistence_parity.fixtures] Paper/reference and Valence fixtures MUST record comparable sign block-entity persistence server metrics for the configured payload.

#### Scenario: Fixture metrics are comparable

r[mc_compatibility.survival_block_entity_persistence_parity.fixtures.comparable]
- GIVEN the configured sign block-entity interaction runs against Paper/reference and Valence backends
- WHEN fixture logs are produced
- THEN both logs use the same metric keys for actor, block entity kind, text payload, position, restart method, reconnect, post-restart observation, backend identity, and row id
- AND backend-specific details stay outside the pure comparison decision.

### Requirement: Reviewable block-entity paired receipts

r[mc_compatibility.survival_block_entity_persistence_parity.receipts] Paired sign block-entity persistence evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Receipts include revisions and manifests

r[mc_compatibility.survival_block_entity_persistence_parity.receipts.reviewable]
- GIVEN the block-entity persistence row is ready for promotion
- WHEN reviewers inspect the repository
- THEN Paper/reference and Valence receipts, client/server logs, checker output, BLAKE3 manifests, child revisions, and oracle limitations are present under `docs/evidence/`
- AND Valence-only or target-only evidence is rejected.

### Requirement: Narrow block-entity promotion

r[mc_compatibility.survival_block_entity_persistence_parity.promotion] Acceptance matrix and current-bundle docs MUST promote only the configured `survival-block-entity-persistence-parity` row after the paired comparator passes.

#### Scenario: Broad block-entity parity remains a non-claim

r[mc_compatibility.survival_block_entity_persistence_parity.promotion.nonclaims]
- GIVEN paired sign block-entity evidence passes
- WHEN matrix and bundle docs are updated
- THEN only the configured sign block-entity row is marked covered
- AND all block entities, arbitrary NBT parity, sign editing UI semantics, multi-chunk persistence, concurrent saves, backups, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity validation and archive evidence

r[mc_compatibility.survival_block_entity_persistence_parity.validation] The change MUST record checker, paired comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.survival_block_entity_persistence_parity.validation.log]
- GIVEN the block-entity persistence row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, runner/fixture checks, scenario manifest check, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Scoreboard/team packet-family contract

r[mc_compatibility.scoreboard_team_packet_family_promotion.contract] The `scoreboard-team-packet-family` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names exact scoreboard or team packet rows

r[mc_compatibility.scoreboard_team_packet_family_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one scenario context, exact packet row or rows, normalized team/objective/display/score fields, client observation or fixture evidence, server correlation, child revisions if live, and checker metrics
- AND scoreboard UI parity, all team rules, all objective/display/score variants, full CTF correctness, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Scoreboard/team packet-family checker

r[mc_compatibility.scoreboard_team_packet_family_promotion.checker] A deterministic Rust checker MUST validate normalized scoreboard/team packet evidence before promotion.

#### Scenario: Valid scoreboard/team evidence passes

r[mc_compatibility.scoreboard_team_packet_family_promotion.checker.valid]
- GIVEN normalized evidence names `scoreboard-team-packet-family`, the configured packet row or rows, normalized fields, client observation or fixture evidence, server correlation, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak scoreboard/team evidence fails closed

r[mc_compatibility.scoreboard_team_packet_family_promotion.checker.rejects]
- GIVEN evidence is missing the row id, names an unsupported packet row, omits normalized fields, lacks client/fixture or server correlation, uses stale revisions, or claims UI/full-CTF/scoreboard breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Scoreboard/team packet-family rail

r[mc_compatibility.scoreboard_team_packet_family_promotion.rail] The harness MUST expose or select isolated scoreboard/team packet evidence without changing existing CTF, survival, inventory, combat, network, or negative-live semantics.

#### Scenario: Gameplay and packet-family claims stay separate

r[mc_compatibility.scoreboard_team_packet_family_promotion.rail.isolated]
- GIVEN existing CTF rows cover bounded gameplay transitions
- WHEN scoreboard/team packet evidence is added
- THEN existing CTF claims remain unchanged
- AND the packet-family row records separate packet metrics and checker output.

### Requirement: Scoreboard/team packet artifacts

r[mc_compatibility.scoreboard_team_packet_family_promotion.artifacts] Review-critical scoreboard/team packet artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and exact packet rows

r[mc_compatibility.scoreboard_team_packet_family_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts or fixtures, logs, normalized inputs, checker output, BLAKE3 manifests, packet inventory updates, child revisions if live, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow scoreboard/team packet matrix promotion

r[mc_compatibility.scoreboard_team_packet_family_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured scoreboard/team packet row after checker and evidence gates pass.

#### Scenario: Broader scoreboard/team remains a non-claim

r[mc_compatibility.scoreboard_team_packet_family_promotion.matrix.nonclaims]
- GIVEN scoreboard/team packet evidence passes
- WHEN docs are updated
- THEN only the configured packet row or rows are marked covered
- AND UI parity, all scoreboards, all team rules, full CTF correctness, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Scoreboard/team packet validation evidence

r[mc_compatibility.scoreboard_team_packet_family_promotion.validation] The change MUST record checker, fixture or runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.scoreboard_team_packet_family_promotion.validation.log]
- GIVEN the scoreboard/team packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, fixture/runner checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Movement packet-family contract

r[mc_compatibility.movement_packet_family_promotion.contract] The `movement-packet-family` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one movement transition

r[mc_compatibility.movement_packet_family_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, start position, target position, look fields if applicable, on-ground state, packet row or rows, server correlation, tolerance if any, child revisions, and checker metrics
- AND movement physics, collision, anti-cheat, latency tolerance, malicious-client resilience, all movement packet variants, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Movement packet-family checker

r[mc_compatibility.movement_packet_family_promotion.checker] A deterministic Rust checker MUST validate normalized movement packet evidence before promotion.

#### Scenario: Valid movement evidence passes

r[mc_compatibility.movement_packet_family_promotion.checker.valid]
- GIVEN normalized evidence names `movement-packet-family`, clean child revisions, the configured movement fields, client action milestone, Valence server correlation, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and within configured tolerance.

#### Scenario: Weak movement evidence fails closed

r[mc_compatibility.movement_packet_family_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale revisions, names the wrong packet variant, omits movement fields, exceeds tolerance, lacks server correlation, or claims physics/anti-cheat/security breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Movement packet-family rail

r[mc_compatibility.movement_packet_family_promotion.rail] The harness MUST expose an isolated movement packet rail without changing existing CTF, survival, inventory, combat, network, or negative-live semantics.

#### Scenario: Existing gameplay movement stays implicit

r[mc_compatibility.movement_packet_family_promotion.rail.isolated]
- GIVEN existing gameplay rows may move clients incidentally
- WHEN the movement packet rail is added
- THEN existing rows remain unchanged
- AND the new row records explicit normalized movement metrics.

### Requirement: Movement packet artifacts

r[mc_compatibility.movement_packet_family_promotion.artifacts] Review-critical movement packet artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.movement_packet_family_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow movement packet matrix promotion

r[mc_compatibility.movement_packet_family_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured movement row after checker and evidence gates pass.

#### Scenario: Broader movement correctness remains a non-claim

r[mc_compatibility.movement_packet_family_promotion.matrix.nonclaims]
- GIVEN movement packet evidence passes
- WHEN docs are updated
- THEN only the configured movement row is marked covered
- AND physics, anti-cheat, collision, latency, malicious-client resilience, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Movement packet validation evidence

r[mc_compatibility.movement_packet_family_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.movement_packet_family_promotion.validation.log]
- GIVEN the movement packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Minecraft protocol I/O trait contract

r[mc_compatibility.minecraft_protocol_io_traits.contract] The runner MUST define a narrow Minecraft protocol I/O trait contract before replacing direct packet, string, or VarInt helpers.

#### Scenario: Contract is runner-local

r[mc_compatibility.minecraft_protocol_io_traits.contract.scope]
- GIVEN protocol helper traits are introduced
- WHEN reviewers inspect the contract
- THEN the scope is limited to runner-local status/query and packet-framing helpers
- AND it does not claim a complete Minecraft protocol implementation, Valence protocol parity, public-server compatibility, or new packet coverage.

### Requirement: Pure protocol helper core

r[mc_compatibility.minecraft_protocol_io_traits.core] VarInt, string, and packet framing helpers MUST separate deterministic wire-format logic from network I/O.

#### Scenario: In-memory protocol helpers are deterministic

r[mc_compatibility.minecraft_protocol_io_traits.core.pure]
- GIVEN in-memory byte buffers and explicit string or packet inputs
- WHEN VarInt, string, and packet helper functions run
- THEN they produce or parse the documented bytes deterministically
- AND they do not open sockets, read files, spawn processes, inspect environment, use clocks, or mutate external state.

#### Scenario: Wire-format constants are named

r[mc_compatibility.minecraft_protocol_io_traits.core.constants]
- GIVEN VarInt encoding or decoding uses masks, continuation bits, shift widths, or maximum byte counts
- WHEN reviewers inspect the helper implementation
- THEN those numeric values are named constants
- AND tests cover boundary values that depend on them.

### Requirement: Protocol helper migration

r[mc_compatibility.minecraft_protocol_io_traits.migration] Existing runner status/query and packet-write call sites MUST migrate to protocol I/O traits without changing wire bytes or readiness behavior.

#### Scenario: Status behavior remains stable

r[mc_compatibility.minecraft_protocol_io_traits.migration.parity]
- GIVEN the runner waits for a server status response or writes a protocol packet
- WHEN the migrated helper path executes
- THEN packet framing, VarInt encoding, string encoding, timeout/error mapping, and success diagnostics match the pre-refactor contract.

### Requirement: Protocol helper tests

r[mc_compatibility.minecraft_protocol_io_traits.tests] The protocol helper refactor MUST include positive and negative tests that run against in-memory readers and writers.

#### Scenario: Valid wire-format fixtures pass

r[mc_compatibility.minecraft_protocol_io_traits.tests.positive]
- GIVEN valid VarInt values, strings, packet IDs, and payloads
- WHEN protocol helper tests run through in-memory cursors
- THEN values round-trip and packet bytes match expected status/query fixture bytes.

#### Scenario: Invalid wire-format fixtures fail closed

r[mc_compatibility.minecraft_protocol_io_traits.tests.negative]
- GIVEN input ends early, a packet is truncated, a VarInt exceeds the supported maximum byte count, or a string length is invalid for the fixture
- WHEN protocol helper tests run
- THEN the helper returns an explicit error
- AND no caller treats malformed input as a successful status or packet decode.

### Requirement: Protocol I/O evidence

r[mc_compatibility.minecraft_protocol_io_traits.evidence] Review-critical protocol I/O logs MUST be copied under `docs/evidence/` when task closeout cites status/query behavior beyond unit tests.

#### Scenario: Evidence scope is narrow

r[mc_compatibility.minecraft_protocol_io_traits.evidence.reviewable]
- GIVEN closeout cites protocol I/O behavior
- WHEN reviewers inspect evidence
- THEN logs identify the checked status/query or dry-run path and state that no complete protocol, Valence parity, or public-server compatibility claim is made.

### Requirement: Protocol I/O validation

r[mc_compatibility.minecraft_protocol_io_traits.validation] The change MUST record protocol helper tests, relevant runner/status regression checks, and Cairn gates before archive.

#### Scenario: Protocol I/O closeout is reviewable

r[mc_compatibility.minecraft_protocol_io_traits.validation.log]
- GIVEN Minecraft protocol I/O traits are implemented
- WHEN the change is archived
- THEN successful logs show positive protocol helper tests, negative malformed-input tests, relevant runner/status regression checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Server backend runtime trait contract

r[mc_compatibility.server_backend_runtime_traits.contract] The compatibility runner MUST define a bounded server-backend runtime trait contract before replacing backend-specific enum matches.

#### Scenario: Contract preserves public backend identity

r[mc_compatibility.server_backend_runtime_traits.contract.identity]
- GIVEN backend behavior is moved behind traits
- WHEN reviewers inspect the contract
- THEN `ServerBackend` remains the stable CLI/config/receipt identity for `valence` and `paper`
- AND the trait contract names backend name, default port, lifecycle operations, log label, log read behavior, dry-run behavior, and error reporting responsibilities.

### Requirement: Backend runtime boundary

r[mc_compatibility.server_backend_runtime_traits.boundary] Valence and Paper backend implementations MUST separate pure backend facts from imperative lifecycle operations.

#### Scenario: Pure facts are side-effect free

r[mc_compatibility.server_backend_runtime_traits.boundary.pure]
- GIVEN a caller requests a backend name, default port, or log label
- WHEN the runtime implementation answers
- THEN the result is deterministic from the backend and config inputs
- AND no process, filesystem, container, clock, or environment operation occurs.

#### Scenario: Lifecycle shells preserve existing operations

r[mc_compatibility.server_backend_runtime_traits.boundary.shell]
- GIVEN a caller starts, stops, force-stops, or reads logs for a backend
- WHEN the runtime implementation handles the request
- THEN it delegates to the existing Valence or Paper operation shape
- AND it does not change command arguments, container names, pid-file behavior, dry-run behavior, or log-source semantics.

### Requirement: Backend match migration

r[mc_compatibility.server_backend_runtime_traits.migration] Existing runner paths SHOULD route backend behavior through the runtime trait dispatch instead of open-coded backend matches once parity tests exist.

#### Scenario: Receipt and matrix behavior remains stable

r[mc_compatibility.server_backend_runtime_traits.migration.parity]
- GIVEN the backend runtime migration is complete
- WHEN existing dry-run, run-matrix, compare-receipt, cleanup, and status code paths execute
- THEN backend names, default ports, receipt server fields, matrix backend ordering, and log labels match the pre-refactor contract.

### Requirement: Backend runtime tests

r[mc_compatibility.server_backend_runtime_traits.tests] The migration MUST include positive and negative tests that prove backend trait parity and fail-closed parsing.

#### Scenario: Known backends pass parity checks

r[mc_compatibility.server_backend_runtime_traits.tests.positive]
- GIVEN the Valence and Paper runtimes are constructed through the stable dispatch
- WHEN tests inspect names, default ports, matrix config defaults, dry-run lifecycle behavior, and log-source selection
- THEN Valence and Paper match the documented compatibility runner behavior.

#### Scenario: Unknown backend names fail closed

r[mc_compatibility.server_backend_runtime_traits.tests.negative]
- GIVEN config, CLI, or receipt comparison input names an unsupported backend
- WHEN parsing or validation runs
- THEN the runner rejects the value with an explicit diagnostic
- AND no runtime implementation is selected by fallback or string guessing.

### Requirement: Backend runtime evidence

r[mc_compatibility.server_backend_runtime_traits.evidence] Review-critical backend runtime evidence MUST be promoted under `docs/evidence/` when the refactor claims behavior parity beyond local tests.

#### Scenario: Evidence names parity scope

r[mc_compatibility.server_backend_runtime_traits.evidence.reviewable]
- GIVEN backend runtime parity is claimed in tasks or closeout notes
- WHEN reviewers inspect evidence
- THEN focused logs or receipts identify the checked Valence/Paper paths and state that no broader backend/plugin or public-server behavior is claimed.

### Requirement: Backend runtime validation

r[mc_compatibility.server_backend_runtime_traits.validation] The change MUST run focused runner tests and Cairn gates before archive.

#### Scenario: Closeout validation is complete

r[mc_compatibility.server_backend_runtime_traits.validation.log]
- GIVEN the backend runtime trait refactor is complete
- WHEN the change is archived
- THEN runner tests, any relevant checker output, Cairn proposal/design/tasks gates, and Cairn validation are recorded with successful exit status.

### Requirement: Evidence matcher contract

r[mc_compatibility.evidence_matcher_traits.contract] Scenario evaluation MUST use an explicit evidence matcher contract before dynamic milestone matching is moved out of ad hoc string-name branches.

#### Scenario: Matcher contract separates ID from behavior

r[mc_compatibility.evidence_matcher_traits.contract.identity]
- GIVEN a milestone rule is evaluated
- WHEN the rule is reported as observed, missing, or forbidden
- THEN the stable milestone ID remains separate from the matcher behavior
- AND receipt-facing `ScenarioEvidence` and `ServerScenarioEvidence` output shapes remain unchanged.

### Requirement: Pure matcher core

r[mc_compatibility.evidence_matcher_traits.core] Evidence matchers MUST be pure deterministic functions over in-memory evidence text and explicit context.

#### Scenario: Matcher core has no side effects

r[mc_compatibility.evidence_matcher_traits.core.pure]
- GIVEN matcher evaluation receives client output, server output, normalized text, username context, and scenario context
- WHEN a literal, case-insensitive, dynamic username, dynamic client-suffix, or any-of matcher runs
- THEN it returns only a boolean presence decision
- AND it does not read files, spawn commands, inspect environment, use clocks, perform network access, or mutate external state.

### Requirement: Explicit milestone rules

r[mc_compatibility.evidence_matcher_traits.rules] Client, server, and forbidden milestone tables SHOULD attach explicit matcher values rather than relying on milestone-name string comparisons.

#### Scenario: Dynamic server checks are visible

r[mc_compatibility.evidence_matcher_traits.rules.dynamic]
- GIVEN a server milestone checks the configured username, client A username, client B username, or flag-or-score fallback
- WHEN reviewers inspect the milestone rule
- THEN the dynamic matcher kind is visible in the rule definition
- AND the stable milestone ID remains the same as the pre-refactor output ID.

### Requirement: Matcher migration

r[mc_compatibility.evidence_matcher_traits.migration] Existing scenario evaluation MUST migrate to matcher evaluation without changing receipt schemas or milestone pass/fail semantics.

#### Scenario: Evaluation parity is preserved

r[mc_compatibility.evidence_matcher_traits.migration.parity]
- GIVEN existing client output and server log fixtures
- WHEN old and new evaluation expectations are compared
- THEN observed milestones, missing milestones, forbidden matches, and pass/fail booleans match the documented pre-refactor behavior.

### Requirement: Matcher tests

r[mc_compatibility.evidence_matcher_traits.tests] The matcher core MUST include positive and negative tests for each supported matcher kind and scenario parity.

#### Scenario: Supported matchers pass valid fixtures

r[mc_compatibility.evidence_matcher_traits.tests.positive]
- GIVEN fixtures contain literal, lowercase-normalized, dynamic username, dynamic client-suffix, and any-of evidence
- WHEN matcher tests run
- THEN each matcher reports the expected observed milestone without requiring a live client or server.

#### Scenario: Missing or forbidden evidence fails closed

r[mc_compatibility.evidence_matcher_traits.tests.negative]
- GIVEN fixtures omit required evidence, contain only differently cased text where case-sensitive matching is required, use the wrong dynamic username, or contain forbidden patterns
- WHEN matcher and scenario evaluation tests run
- THEN missing and forbidden IDs are reported explicitly
- AND pass/fail booleans remain fail-closed.

### Requirement: Matcher validation

r[mc_compatibility.evidence_matcher_traits.validation] The change MUST record focused matcher tests, scenario evaluation tests, relevant evidence checker output, and Cairn gates before archive.

#### Scenario: Matcher closeout is reviewable

r[mc_compatibility.evidence_matcher_traits.validation.log]
- GIVEN evidence matcher traits are implemented
- WHEN the change is archived
- THEN successful logs show matcher positive tests, matcher negative tests, scenario evaluation parity, relevant checker execution, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Checker framework contract

r[mc_compatibility.checker_framework_traits.contract] Shared evidence-checker infrastructure MUST define a minimal trait contract before standalone checker code is migrated onto it.

#### Scenario: Checker-specific semantics remain explicit

r[mc_compatibility.checker_framework_traits.contract.scope]
- GIVEN a checker adopts the shared framework
- WHEN reviewers inspect the checker
- THEN row-specific required fields, expected values, overclaim policy, and evidence contract remain visible in checker-owned code or constants
- AND the shared framework only owns common parsing, diagnostics, shell orchestration, and fixture mechanics.

### Requirement: Pure checker framework core

r[mc_compatibility.checker_framework_traits.core] Shared checker helpers MUST be pure deterministic functions over in-memory arguments, text, records, and expected contracts.

#### Scenario: Key/value parsing diagnostics are deterministic

r[mc_compatibility.checker_framework_traits.core.kv]
- GIVEN a key/value evidence record contains valid rows, malformed rows, empty keys, duplicate keys, comments, and blank lines
- WHEN the shared parser evaluates the text
- THEN it returns a deterministic record or diagnostics naming malformed rows, empty keys, and duplicates
- AND it performs no filesystem, process, environment, clock, network, or stdout/stderr operations.

### Requirement: Thin checker shell

r[mc_compatibility.checker_framework_traits.shell] The checker framework shell MUST isolate argument parsing, file reads, stdout/stderr formatting, and exit-code handling from validation cores.

#### Scenario: CLI behavior remains compatible

r[mc_compatibility.checker_framework_traits.shell.compatible]
- GIVEN a migrated checker is invoked with `--self-test`, valid evidence, invalid evidence, unknown arguments, or missing values
- WHEN the shell handles the invocation
- THEN success and failure exit status, summary text, and diagnostic text remain compatible with the pre-migration checker contract unless a separate change updates that contract.

### Requirement: Exemplar checker migration

r[mc_compatibility.checker_framework_traits.migration] The first checker-framework adoption SHOULD migrate a small exemplar pair of repeated evidence checkers before broader checker migration.

#### Scenario: Exemplar migration proves parity

r[mc_compatibility.checker_framework_traits.migration.parity]
- GIVEN the exemplar checkers are migrated
- WHEN their existing positive and negative self-tests run
- THEN valid fixtures still pass, invalid fixtures still fail with useful diagnostics, and no evidence row gains or loses coverage claims.

### Requirement: Checker framework tests

r[mc_compatibility.checker_framework_traits.tests] The framework and migrated checkers MUST include positive and negative tests for parsing, validation helpers, shell behavior, and overclaim rejection.

#### Scenario: Valid framework fixtures pass

r[mc_compatibility.checker_framework_traits.tests.positive]
- GIVEN valid key/value records, valid token expectations, clean child revisions, and valid checker arguments
- WHEN framework and migrated checker tests run
- THEN parsing, helper validation, shell orchestration, and checker-specific validation all pass.

#### Scenario: Invalid framework fixtures fail closed

r[mc_compatibility.checker_framework_traits.tests.negative]
- GIVEN records have malformed lines, duplicate keys, empty keys, missing required fields, wrong values, stale revisions, missing negative fixtures, or truthy broad overclaims
- WHEN framework and migrated checker tests run
- THEN diagnostics identify the invalid input
- AND no checker reports success for weak or overbroad evidence.

### Requirement: Checker framework validation

r[mc_compatibility.checker_framework_traits.validation] The change MUST record migrated checker self-tests, framework tests, task-evidence gates, relevant evidence checks, and Cairn gates before archive.

#### Scenario: Checker framework closeout is reviewable

r[mc_compatibility.checker_framework_traits.validation.log]
- GIVEN the framework and exemplar migration are complete
- WHEN the change is archived
- THEN successful logs show framework positive tests, framework negative tests, migrated checker self-tests, task-evidence checks, relevant evidence manifest checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Shared checker framework components

r[mc_compatibility.checker_framework.shared_components] mc compatibility checker scripts SHOULD use shared framework components for repository layout, path safety, JSON or receipt extraction, BLAKE3 and evidence manifest handling, diagnostics, fixtures, and self-test helpers when those concerns are common.

#### Scenario: Checker uses shared owner for common concern

r[mc_compatibility.checker_framework.shared_components.owner]
- GIVEN a checker needs a common path, receipt, evidence, diagnostic, fixture, or self-test behavior
- WHEN the checker is created or substantially changed
- THEN it uses the shared checker framework component for that behavior or documents a focused exception
- AND the behavior is not duplicated ad hoc without tests.

### Requirement: Checker parity

r[mc_compatibility.checker_framework.checker_parity] Checker framework consolidation MUST preserve existing checker CLI flags, exit behavior, diagnostics relied on by evidence, flake check wiring, evidence boundaries, and non-claims unless a checker-specific Cairn changes them.

#### Scenario: Existing checker invocation remains stable

r[mc_compatibility.checker_framework.checker_parity.stable]
- GIVEN a supported pre-refactor checker invocation
- WHEN the consolidated checker or framework-backed checker receives the same inputs
- THEN the pass/fail result, reviewable diagnostic intent, and evidence boundary remain equivalent
- AND no compatibility evidence is promoted by the refactor.

### Requirement: Python checker migration policy

r[mc_compatibility.checker_framework.python_migration] Any Python checker substantially changed by this consolidation SHOULD be migrated to Rust or Steel, while untouched Python checkers MAY remain until their next owner-driven change.

#### Scenario: Touched Python checker has explicit outcome

r[mc_compatibility.checker_framework.python_migration.outcome]
- GIVEN a Python checker is selected for extension during checker framework consolidation
- WHEN the change is implemented
- THEN the checker is migrated to Rust or Steel, or the change records why the Python checker remained untouched and out of scope.

### Requirement: Checker framework positive tests

r[mc_compatibility.checker_framework.positive_tests] The change MUST include positive tests for framework path handling, receipt parsing, evidence manifest loading, fixture success, diagnostics, and representative migrated checker behavior.

#### Scenario: Framework-supported checker paths pass

r[mc_compatibility.checker_framework.positive_tests.coverage]
- GIVEN representative valid checker inputs and fixtures
- WHEN framework-backed checker code processes them
- THEN tests prove the expected diagnostics, parsed values, manifest results, and checker pass outcomes are produced.

### Requirement: Checker framework negative tests

r[mc_compatibility.checker_framework.negative_tests] The change MUST include negative tests for unsafe paths, malformed JSON or receipts, stale manifests, missing fixtures, duplicate diagnostics, and checker misuse of framework contracts.

#### Scenario: Invalid checker inputs fail closed

r[mc_compatibility.checker_framework.negative_tests.fail_closed]
- GIVEN invalid checker inputs, stale evidence, or misuse of framework contracts
- WHEN framework-backed checker code processes them
- THEN tests prove the inputs are rejected with actionable diagnostics before false evidence pass results are emitted.

### Requirement: Checker framework validation

r[mc_compatibility.checker_framework.validation] The change MUST record focused checker tests, affected flake checks, evidence-manifest checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.checker_framework.validation.logs]
- GIVEN checker framework consolidation is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative framework tests plus affected checker gates and Cairn gates passing.

### Requirement: Block-entity sign packet-family preflight

r[mc_compatibility.block_entity_sign_packet_family.preflight] Current bundle, acceptance matrix, and packet inventory prose MUST be internally consistent before block-entity sign packet-family coverage is promoted.

#### Scenario: Existing inventory prose matches promoted drag rows

r[mc_compatibility.block_entity_sign_packet_family.preflight.drag_consistency]
- GIVEN `inventory-drag-transactions` is already a promoted bounded row
- WHEN the current evidence bundle describes maintained inventory rows and explicit non-claims
- THEN the prose names the drag row as covered only within its configured bounds
- AND drag transactions are not listed as a broad non-claim except for unpromoted drag modes, distributions, windows, and semantics.

### Requirement: Block-entity sign packet-family contract

r[mc_compatibility.block_entity_sign_packet_family.contract] The `block-entity-sign-packet-family` row MUST define a bounded packet-family promotion contract before matrix, bundle, or packet-inventory coverage is claimed.

#### Scenario: Contract names exact packet rows and payload

r[mc_compatibility.block_entity_sign_packet_family.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names the configured actor `compatbot`, sign block-entity kind, position `28,64,0`, text payload `MC|Compat|Sign|Persist`, Paper/reference receipt, Valence receipt, child revisions, normalized metrics, and every protocol row considered
- AND `play/clientbound/0x08 BlockEntityUpdateS2CPacket` is promoted only for the configured sign NBT payload if checker evidence passes
- AND `play/clientbound/0x31 SignEditorOpenS2CPacket` plus `play/serverbound/0x2e UpdateSignC2SPacket` remain non-claims unless separate sign-edit live evidence and checker coverage are produced.

#### Scenario: Adjacent breadth remains non-claim

r[mc_compatibility.block_entity_sign_packet_family.contract.nonclaims]
- GIVEN the bounded packet-family row is promoted
- WHEN matrix, bundle, and packet inventory docs are reviewed
- THEN all block entities, arbitrary NBT parity, sign editing UI semantics without dedicated evidence, all sign text variants, all sign sides, all block-entity packet shapes, broad parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity sign packet-family checker

r[mc_compatibility.block_entity_sign_packet_family.checker] A deterministic Rust checker MUST validate normalized block-entity sign packet-family evidence before promotion.

#### Scenario: Valid block-entity sign packet evidence passes

r[mc_compatibility.block_entity_sign_packet_family.checker.valid]
- GIVEN normalized evidence names `block-entity-sign-packet-family`, the configured sign payload, the `BlockEntityUpdateS2CPacket` protocol row, clean child revisions, matching Paper/reference and Valence receipts, and sign-payload client observation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak block-entity sign packet evidence fails closed

r[mc_compatibility.block_entity_sign_packet_family.checker.rejects]
- GIVEN evidence is missing the row id, omits Paper/reference or Valence evidence, uses stale or unknown child revisions, names the wrong packet row, omits sign-payload observation, reports the wrong block entity kind, position, text payload, or backend, claims sign editing without sign-edit evidence, or claims broad block-entity/parser coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, overbroad, or mismatched metric.

### Requirement: Block-entity sign packet-family rail

r[mc_compatibility.block_entity_sign_packet_family.rail] The promotion MUST reuse or extend isolated sign/block-entity rails without changing existing survival, inventory, CTF, combat, network, or negative-live semantics.

#### Scenario: Existing sign persistence row remains separate

r[mc_compatibility.block_entity_sign_packet_family.rail.isolated]
- GIVEN the existing `survival-block-entity-persistence-parity` row already covers one sign payload across Paper/reference and Valence backends
- WHEN packet-family evidence is collected or normalized
- THEN the packet-family row records its own packet-row contract and checker output
- AND the survival block-entity persistence row remains a separate survival/reference-parity claim.

#### Scenario: Sign-edit rows require dedicated evidence

r[mc_compatibility.block_entity_sign_packet_family.rail.sign_edit]
- GIVEN `SignEditorOpenS2CPacket` or `UpdateSignC2SPacket` is considered for promotion
- WHEN the rail lacks a live sign-edit open/update interaction with client and server correlation
- THEN those rows stay explicit non-claims instead of inheriting coverage from sign block-entity persistence.

### Requirement: Reviewable block-entity sign packet-family artifacts

r[mc_compatibility.block_entity_sign_packet_family.artifacts] Review-critical block-entity sign packet-family artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.block_entity_sign_packet_family.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized KV inputs, checker output, BLAKE3 manifests, packet inventory updates, child revisions, and any oracle limitation checkpoint are present under `docs/evidence/` or tracked source paths.

### Requirement: Narrow block-entity sign packet-family matrix promotion

r[mc_compatibility.block_entity_sign_packet_family.matrix] Acceptance matrix, current-bundle docs, and packet inventory rows MUST promote only the configured block-entity sign packet-family scope after checker and evidence gates pass.

#### Scenario: Packet inventory stays exact

r[mc_compatibility.block_entity_sign_packet_family.matrix.inventory]
- GIVEN checker-backed block-entity sign packet evidence passes
- WHEN `protocol-763-packet-inventory-2026-05-28.tsv`, acceptance matrix, and current evidence bundle are updated
- THEN only the exact supported packet row or rows are marked covered
- AND unsupported sign editor, arbitrary block-entity, arbitrary NBT, broad parser-shape, full protocol, public-server, and production claims remain non-claims.

### Requirement: Block-entity sign packet-family validation evidence

r[mc_compatibility.block_entity_sign_packet_family.validation] The change MUST record checker, packet-inventory, matrix/bundle, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.block_entity_sign_packet_family.validation.log]
- GIVEN the block-entity sign packet-family row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, any runner/fixture checks, packet inventory or row contract checks, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Block-entity update breadth contract

r[mc_compatibility.block_entity_update_breadth_promotion.contract] The `block-entity-update-breadth` row MUST define a bounded non-sign block-entity promotion contract before coverage is claimed.

#### Scenario: Contract names one non-sign payload

r[mc_compatibility.block_entity_update_breadth_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor or fixture, non-sign block entity kind, position, normalized payload metric, packet row, backend evidence, child revisions if live, and checker metrics
- AND all block entities, arbitrary NBT parity, persistence breadth, sign editing, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity update breadth checker

r[mc_compatibility.block_entity_update_breadth_promotion.checker] A deterministic Rust checker MUST validate normalized non-sign block-entity update evidence before promotion.

#### Scenario: Valid non-sign block-entity evidence passes

r[mc_compatibility.block_entity_update_breadth_promotion.checker.valid]
- GIVEN normalized evidence names `block-entity-update-breadth`, the configured kind, position, payload metric, packet row, backend evidence, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak non-sign block-entity evidence fails closed

r[mc_compatibility.block_entity_update_breadth_promotion.checker.rejects]
- GIVEN evidence is missing the row id, names the wrong kind, position, packet row, or payload, omits backend evidence, lacks required revision metadata, or claims arbitrary NBT/all-block-entity coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, unexpected, or mismatched metric.

### Requirement: Block-entity update breadth rail

r[mc_compatibility.block_entity_update_breadth_promotion.rail] The harness MUST expose or select an isolated non-sign block-entity update rail without changing existing sign persistence, survival, inventory, CTF, combat, or network semantics.

#### Scenario: Sign and non-sign rows stay separate

r[mc_compatibility.block_entity_update_breadth_promotion.rail.isolated]
- GIVEN sign block-entity evidence already exists
- WHEN non-sign block-entity evidence is added
- THEN sign claims remain unchanged
- AND the non-sign row records its own fixture and checker output.

### Requirement: Block-entity update breadth artifacts

r[mc_compatibility.block_entity_update_breadth_promotion.artifacts] Review-critical non-sign block-entity artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and payload source

r[mc_compatibility.block_entity_update_breadth_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts or fixtures, logs, normalized inputs, checker output, BLAKE3 manifests, revision metadata, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow block-entity update breadth matrix promotion

r[mc_compatibility.block_entity_update_breadth_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured non-sign block-entity row after checker and evidence gates pass.

#### Scenario: Broader block-entity coverage remains a non-claim

r[mc_compatibility.block_entity_update_breadth_promotion.matrix.nonclaims]
- GIVEN non-sign block-entity evidence passes
- WHEN docs are updated
- THEN only the configured row is marked covered
- AND arbitrary NBT, all block entities, persistence breadth, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Block-entity update breadth validation evidence

r[mc_compatibility.block_entity_update_breadth_promotion.validation] The change MUST record checker, fixture or runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.block_entity_update_breadth_promotion.validation.log]
- GIVEN the non-sign block-entity row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, fixture/runner checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Chat/command containment contract

r[mc_compatibility.chat_command_containment_promotion.contract] The `chat-command-containment` row MUST define a bounded owned-local promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one harmless payload

r[mc_compatibility.chat_command_containment_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, packet row or rows, harmless payload, owned-local target scope, server receipt or rejection metric, redaction policy, child revisions, and checker metrics
- AND all chat signing/security, all commands, command permissions, moderation, public-server safety, adversarial resilience, full protocol-763 compatibility, and production readiness remain explicit non-claims.

### Requirement: Chat/command containment checker

r[mc_compatibility.chat_command_containment_promotion.checker] A deterministic Rust checker MUST validate normalized chat/command containment evidence before promotion.

#### Scenario: Valid chat/command containment evidence passes

r[mc_compatibility.chat_command_containment_promotion.checker.valid]
- GIVEN normalized evidence names `chat-command-containment`, clean child revisions, owned-local scope, harmless payload identity, server receipt or rejection metric, redaction policy, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak chat/command containment evidence fails closed

r[mc_compatibility.chat_command_containment_promotion.checker.rejects]
- GIVEN evidence is missing the row id, lacks owned-local scope, uses stale revisions, names the wrong payload or packet row, omits server correlation, lacks redaction policy, or claims public-server/security/command breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Chat/command containment rail

r[mc_compatibility.chat_command_containment_promotion.rail] The harness MUST expose an isolated chat/command containment rail without changing existing CTF, survival, inventory, combat, network, or negative-live semantics.

#### Scenario: Safety scope is isolated

r[mc_compatibility.chat_command_containment_promotion.rail.isolated]
- GIVEN existing network/public-server rows have separate safety contracts
- WHEN the chat/command containment rail is added
- THEN existing safety claims remain unchanged
- AND the new row records only owned-local fixture evidence.

### Requirement: Chat/command containment artifacts

r[mc_compatibility.chat_command_containment_promotion.artifacts] Review-critical chat/command containment artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and redaction policy

r[mc_compatibility.chat_command_containment_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, redaction policy, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow chat/command matrix promotion

r[mc_compatibility.chat_command_containment_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured chat/command containment row after checker and evidence gates pass.

#### Scenario: Broader chat/command safety remains a non-claim

r[mc_compatibility.chat_command_containment_promotion.matrix.nonclaims]
- GIVEN chat/command containment evidence passes
- WHEN docs are updated
- THEN only the configured owned-local row is marked covered
- AND public-server safety, security, all commands, chat signing, moderation, full protocol, and production claims remain explicit non-claims.

### Requirement: Chat/command containment validation evidence

r[mc_compatibility.chat_command_containment_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.chat_command_containment_promotion.validation.log]
- GIVEN the chat/command containment row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Chunk biome data packet contract

r[mc_compatibility.chunk_biome_data_packet_promotion.contract] The `chunk-biome-data-packet` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one packet fixture

r[mc_compatibility.chunk_biome_data_packet_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names the packet row `ChunkBiomeDataS2CPacket`, fixture source, protocol version, payload identity or hash, parser expectations, optional live context receipt, and checker metrics
- AND all biome semantics, all chunk semantics, all worldgen packets, dimension travel, Nether/End behavior, full protocol-763 compatibility, broad Minecraft compatibility, and production readiness remain explicit non-claims.

### Requirement: Chunk biome data packet checker

r[mc_compatibility.chunk_biome_data_packet_promotion.checker] A deterministic Rust checker MUST validate normalized chunk biome data packet evidence before promotion.

#### Scenario: Valid chunk biome evidence passes

r[mc_compatibility.chunk_biome_data_packet_promotion.checker.valid]
- GIVEN normalized evidence names `chunk-biome-data-packet`, the configured packet row, fixture payload identity, parser result, protocol version, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak chunk biome evidence fails closed

r[mc_compatibility.chunk_biome_data_packet_promotion.checker.rejects]
- GIVEN evidence is missing the row id, names the wrong packet, lacks fixture identity, omits parser result, mismatches protocol, or claims broad biome/chunk/worldgen semantics
- WHEN the checker evaluates the record
- THEN it fails and names the missing, unexpected, or mismatched metric.

### Requirement: Chunk biome data packet rail

r[mc_compatibility.chunk_biome_data_packet_promotion.rail] The promotion MUST use isolated packet fixture or live-context rails without changing existing survival, chunk/biome, inventory, CTF, combat, or network semantics.

#### Scenario: Existing chunk/biome rows remain separate

r[mc_compatibility.chunk_biome_data_packet_promotion.rail.isolated]
- GIVEN existing chunk/biome rows cover chunk-delta and overworld environment context
- WHEN chunk biome data evidence is added
- THEN existing rows remain unchanged
- AND the new row records its own packet fixture and checker output.

### Requirement: Chunk biome reviewable artifacts

r[mc_compatibility.chunk_biome_data_packet_promotion.artifacts] Review-critical chunk biome packet artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and fixture source

r[mc_compatibility.chunk_biome_data_packet_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN fixture payloads or hashes, normalized inputs, checker output, BLAKE3 manifests, optional live receipts, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow chunk biome packet matrix promotion

r[mc_compatibility.chunk_biome_data_packet_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured chunk biome data packet row after checker and evidence gates pass.

#### Scenario: Broader chunk/biome remains a non-claim

r[mc_compatibility.chunk_biome_data_packet_promotion.matrix.nonclaims]
- GIVEN chunk biome data evidence passes
- WHEN docs are updated
- THEN only the configured packet row is marked covered
- AND all broader biome, chunk, worldgen, dimension, full protocol, and production claims remain explicit non-claims.

### Requirement: Chunk biome packet validation evidence

r[mc_compatibility.chunk_biome_data_packet_promotion.validation] The change MUST record checker, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.chunk_biome_data_packet_promotion.validation.log]
- GIVEN the chunk biome data packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, fixture or runner checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Creative inventory action contract

r[mc_compatibility.creative_inventory_action_promotion.contract] The `creative-inventory-action` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one creative slot mutation

r[mc_compatibility.creative_inventory_action_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, creative game-mode precondition, packet row `CreativeInventoryActionC2SPacket`, semantic slot, wire slot, item, count, server acceptance metric, final slot state, child revisions, and checker metrics
- AND all creative inventory semantics, all slots, all items, all game-mode transitions, all pick-block behavior, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Creative inventory action checker

r[mc_compatibility.creative_inventory_action_promotion.checker] A deterministic Rust checker MUST validate normalized creative inventory evidence before promotion.

#### Scenario: Valid creative action evidence passes

r[mc_compatibility.creative_inventory_action_promotion.checker.valid]
- GIVEN normalized evidence names `creative-inventory-action`, clean child revisions, creative-mode precondition, configured slot/item/count, client action milestone, Valence acceptance, and final slot state
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak creative action evidence fails closed

r[mc_compatibility.creative_inventory_action_promotion.checker.rejects]
- GIVEN evidence is missing the row id, lacks creative game mode, uses stale revisions, names the wrong slot/item/count, omits server acceptance, mismatches final state, or claims broad creative inventory coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Creative inventory action rail

r[mc_compatibility.creative_inventory_action_promotion.rail] The harness MUST expose an isolated creative inventory rail without changing existing survival/player-inventory, CTF, combat, network, or negative-live semantics.

#### Scenario: Survival inventory rows stay separate

r[mc_compatibility.creative_inventory_action_promotion.rail.isolated]
- GIVEN existing inventory rows cover survival/player-inventory actions
- WHEN the creative rail is added
- THEN existing inventory scenario milestones and non-claims remain unchanged
- AND the creative row records its own game-mode, packet, and slot-state evidence.

### Requirement: Creative inventory reviewable artifacts

r[mc_compatibility.creative_inventory_action_promotion.artifacts] Review-critical creative inventory artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.creative_inventory_action_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow creative inventory matrix promotion

r[mc_compatibility.creative_inventory_action_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured creative inventory action row after checker and evidence gates pass.

#### Scenario: Broader creative inventory remains a non-claim

r[mc_compatibility.creative_inventory_action_promotion.matrix.nonclaims]
- GIVEN creative inventory action evidence passes
- WHEN docs are updated
- THEN only the configured creative row is marked covered
- AND all broader creative inventory, all slots/items, game-mode breadth, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Creative inventory validation evidence

r[mc_compatibility.creative_inventory_action_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.creative_inventory_action_promotion.validation.log]
- GIVEN the creative inventory action row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Entity status-effect packet contract

r[mc_compatibility.entity_status_effect_packets_promotion.contract] The `entity-status-effect-packets` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one effect apply/remove scope

r[mc_compatibility.entity_status_effect_packets_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor or target entity, effect id or name, amplifier, duration, packet row or rows, server correlation, child revisions, and checker metrics
- AND all effects, stacking, particles/UI, gameplay modifiers, combat balancing, survival parity, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Entity status-effect checker

r[mc_compatibility.entity_status_effect_packets_promotion.checker] A deterministic Rust checker MUST validate normalized entity status-effect evidence before promotion.

#### Scenario: Valid status-effect evidence passes

r[mc_compatibility.entity_status_effect_packets_promotion.checker.valid]
- GIVEN normalized evidence names `entity-status-effect-packets`, clean child revisions, the configured effect metrics, client apply and optional remove observations, and Valence server correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak status-effect evidence fails closed

r[mc_compatibility.entity_status_effect_packets_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale revisions, names the wrong entity/effect/amplifier/duration, omits required apply or remove correlation, or claims broad effect or modifier semantics
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Entity status-effect rail

r[mc_compatibility.entity_status_effect_packets_promotion.rail] The harness MUST expose an isolated status-effect rail without changing existing combat, survival, CTF, inventory, network, or negative-live semantics.

#### Scenario: Effect packet row stays separate from modifier claims

r[mc_compatibility.entity_status_effect_packets_promotion.rail.isolated]
- GIVEN existing combat and survival rows have their own scoped claims
- WHEN the status-effect rail is added
- THEN existing rows remain unchanged
- AND the status-effect row records only packet observation/correlation metrics.

### Requirement: Entity status-effect artifacts

r[mc_compatibility.entity_status_effect_packets_promotion.artifacts] Review-critical status-effect artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.entity_status_effect_packets_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow entity status-effect matrix promotion

r[mc_compatibility.entity_status_effect_packets_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured status-effect row after checker and evidence gates pass.

#### Scenario: Broader effect mechanics remain a non-claim

r[mc_compatibility.entity_status_effect_packets_promotion.matrix.nonclaims]
- GIVEN status-effect packet evidence passes
- WHEN docs are updated
- THEN only the configured effect packet row is marked covered
- AND all broader effect, modifier, combat, survival, full protocol, and production claims remain explicit non-claims.

### Requirement: Entity status-effect validation evidence

r[mc_compatibility.entity_status_effect_packets_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.entity_status_effect_packets_promotion.validation.log]
- GIVEN the status-effect packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Recipe-book client settings contract

r[mc_compatibility.recipe_book_client_settings_promotion.contract] The `recipe-book-client-settings` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one settings transition

r[mc_compatibility.recipe_book_client_settings_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, packet row `RecipeBookDataC2SPacket`, configured recipe-book state fields, client action metric, Valence server correlation, child revisions, and checker metrics
- AND recipe-book UI behavior, all recipe categories, recipe discovery, all recipes, full crafting coverage, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Recipe-book client settings checker

r[mc_compatibility.recipe_book_client_settings_promotion.checker] A deterministic Rust checker MUST validate normalized recipe-book settings evidence before promotion.

#### Scenario: Valid recipe-book settings evidence passes

r[mc_compatibility.recipe_book_client_settings_promotion.checker.valid]
- GIVEN normalized evidence names `recipe-book-client-settings`, clean child revisions, configured settings fields, client action metric, and Valence server correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak recipe-book settings evidence fails closed

r[mc_compatibility.recipe_book_client_settings_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale revisions, names the wrong settings fields, omits client or server correlation, or claims broad recipe-book/crafting coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Recipe-book client settings rail

r[mc_compatibility.recipe_book_client_settings_promotion.rail] The harness MUST expose an isolated recipe-book settings rail without changing existing crafting, survival, inventory, CTF, combat, or network semantics.

#### Scenario: Crafting rows stay separate

r[mc_compatibility.recipe_book_client_settings_promotion.rail.isolated]
- GIVEN existing crafting evidence covers one crafting-table recipe path
- WHEN the recipe-book settings rail is added
- THEN existing crafting claims remain unchanged
- AND the new row records only settings packet evidence.

### Requirement: Recipe-book settings artifacts

r[mc_compatibility.recipe_book_client_settings_promotion.artifacts] Review-critical recipe-book settings artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.recipe_book_client_settings_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow recipe-book settings matrix promotion

r[mc_compatibility.recipe_book_client_settings_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured recipe-book settings row after checker and evidence gates pass.

#### Scenario: Broader recipe-book semantics remain a non-claim

r[mc_compatibility.recipe_book_client_settings_promotion.matrix.nonclaims]
- GIVEN recipe-book settings evidence passes
- WHEN docs are updated
- THEN only the configured settings row is marked covered
- AND recipe-book UI, discovery, all recipes, crafting breadth, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Recipe-book settings validation evidence

r[mc_compatibility.recipe_book_client_settings_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.recipe_book_client_settings_promotion.validation.log]
- GIVEN the recipe-book settings row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Resource-pack status contract

r[mc_compatibility.resource_pack_status_promotion.contract] The `resource-pack-status` row MUST define a bounded local promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one local offer/status exchange

r[mc_compatibility.resource_pack_status_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, local fixture offer metadata, packet row or rows, configured client status response, server correlation, no-external-fetch guarantee, redaction policy, child revisions, and checker metrics
- AND asset download/application, trust/security validation, all resource-pack statuses, public-server safety, full protocol-763 compatibility, and production readiness remain explicit non-claims.

### Requirement: Resource-pack status checker

r[mc_compatibility.resource_pack_status_promotion.checker] A deterministic Rust checker MUST validate normalized resource-pack status evidence before promotion.

#### Scenario: Valid resource-pack status evidence passes

r[mc_compatibility.resource_pack_status_promotion.checker.valid]
- GIVEN normalized evidence names `resource-pack-status`, clean child revisions, local fixture scope, configured offer/status metrics, server correlation, no-external-fetch guarantee, redaction policy, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak resource-pack status evidence fails closed

r[mc_compatibility.resource_pack_status_promotion.checker.rejects]
- GIVEN evidence is missing the row id, lacks local scope, uses stale revisions, names the wrong offer/status, omits server correlation, lacks no-external-fetch or redaction fields, or claims asset/trust/public-server breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Resource-pack status rail

r[mc_compatibility.resource_pack_status_promotion.rail] The harness MUST expose an isolated resource-pack status rail without changing existing CTF, survival, inventory, combat, network, or public-server semantics.

#### Scenario: External fetch is not required

r[mc_compatibility.resource_pack_status_promotion.rail.isolated]
- GIVEN the resource-pack status row runs in an owned-local fixture
- WHEN the rail is executed
- THEN it records offer/status packet evidence without requiring external resource downloads
- AND existing public-server and production-safety claims remain unchanged.

### Requirement: Resource-pack status artifacts

r[mc_compatibility.resource_pack_status_promotion.artifacts] Review-critical resource-pack status artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and redaction policy

r[mc_compatibility.resource_pack_status_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, no-external-fetch/redaction metadata, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow resource-pack status matrix promotion

r[mc_compatibility.resource_pack_status_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured resource-pack status row after checker and evidence gates pass.

#### Scenario: Broader resource-pack safety remains a non-claim

r[mc_compatibility.resource_pack_status_promotion.matrix.nonclaims]
- GIVEN resource-pack status evidence passes
- WHEN docs are updated
- THEN only the configured local status row is marked covered
- AND asset loading, trust/security, all statuses, public-server safety, full protocol, and production claims remain explicit non-claims.

### Requirement: Resource-pack status validation evidence

r[mc_compatibility.resource_pack_status_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.resource_pack_status_promotion.validation.log]
- GIVEN the resource-pack status row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Sign editor open/update contract

r[mc_compatibility.sign_editor_open_update_promotion.contract] The `sign-editor-open-update` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one sign edit

r[mc_compatibility.sign_editor_open_update_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, sign position, initial sign state, submitted four-line payload, `SignEditorOpenS2CPacket`, `UpdateSignC2SPacket`, server acceptance metric, child revisions, and checker metrics
- AND all sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Sign editor open/update checker

r[mc_compatibility.sign_editor_open_update_promotion.checker] A deterministic Rust checker MUST validate normalized sign editor open/update evidence before promotion.

#### Scenario: Valid sign edit evidence passes

r[mc_compatibility.sign_editor_open_update_promotion.checker.valid]
- GIVEN normalized evidence names `sign-editor-open-update`, clean child revisions, the configured sign position and payload, client open/update milestones, and Valence server accepted-update correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak sign edit evidence fails closed

r[mc_compatibility.sign_editor_open_update_promotion.checker.rejects]
- GIVEN evidence is missing the row id, omits open or update correlation, uses stale or unknown child revisions, reports the wrong sign position or payload, lacks server acceptance, or claims broad sign editing or block-entity coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Sign editor open/update rail

r[mc_compatibility.sign_editor_open_update_promotion.rail] The harness MUST expose an isolated sign editor open/update rail without changing existing sign persistence, survival, inventory, CTF, combat, network, or negative-live semantics.

#### Scenario: Existing sign persistence remains separate

r[mc_compatibility.sign_editor_open_update_promotion.rail.isolated]
- GIVEN existing sign block-entity persistence evidence is already promoted
- WHEN the sign editor rail is added
- THEN existing persistence claims remain unchanged
- AND the new row records separate open/update packet evidence.

### Requirement: Sign editor reviewable artifacts

r[mc_compatibility.sign_editor_open_update_promotion.artifacts] Review-critical sign editor artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.sign_editor_open_update_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow sign editor matrix promotion

r[mc_compatibility.sign_editor_open_update_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured sign editor row after checker and evidence gates pass.

#### Scenario: Broader sign editing remains a non-claim

r[mc_compatibility.sign_editor_open_update_promotion.matrix.nonclaims]
- GIVEN sign editor evidence passes
- WHEN docs are updated
- THEN only the configured sign editor open/update row is marked covered
- AND broad sign editing, arbitrary sign text, all sign variants, block-entity breadth, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Sign editor validation evidence

r[mc_compatibility.sign_editor_open_update_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.sign_editor_open_update_promotion.validation.log]
- GIVEN the sign editor row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Evidence manifest refresh contract

r[mc_compatibility.evidence_manifest_refresh.contract] The repository MUST define a deterministic contract for checking and refreshing reviewable BLAKE3 evidence manifests.

#### Scenario: Manifest refresh scope is explicit

r[mc_compatibility.evidence_manifest_refresh.contract.scope]
- GIVEN an operator refreshes Cairn evidence manifests
- WHEN the manifest helper is invoked with default settings
- THEN it operates on reviewable `docs/evidence/*.b3` manifests inside the repository
- AND it does not claim new compatibility behavior or alter receipt semantics.

### Requirement: Evidence manifest planner

r[mc_compatibility.evidence_manifest_refresh.planner] Manifest parsing and refresh planning MUST be implemented as deterministic core logic over explicit inputs.

#### Scenario: Stale rows are classified without mutation

r[mc_compatibility.evidence_manifest_refresh.planner.classifies]
- GIVEN a manifest row with an old digest, a current digest, a missing file, malformed text, or an outside-root path
- WHEN the planner evaluates the row
- THEN it reports the row class and proposed digest change without writing files
- AND malformed or outside-root rows fail closed with an explicit diagnostic.

### Requirement: Evidence manifest refresh mode

r[mc_compatibility.evidence_manifest_refresh.refresh_mode] The helper MUST provide separate check-only and explicit refresh modes.

#### Scenario: Check mode does not write

r[mc_compatibility.evidence_manifest_refresh.refresh_mode.check_only]
- GIVEN stale manifest rows exist
- WHEN check-only mode runs
- THEN it exits unsuccessfully with the stale row diagnostics
- AND manifest files remain unchanged.

#### Scenario: Refresh mode reaches a deterministic fixpoint

r[mc_compatibility.evidence_manifest_refresh.refresh_mode.fixpoint]
- GIVEN stale digest rows can cascade across manifests
- WHEN explicit refresh mode runs
- THEN it updates only digest fields for existing in-repository files
- AND it repeats planning until a deterministic fixpoint is reached or reports non-convergence.

### Requirement: Evidence manifest workflow integration

r[mc_compatibility.evidence_manifest_refresh.integration] The helper SHOULD be exposed through repo-local app/check surfaces and workflow documentation.

#### Scenario: Cairn drains can refresh evidence predictably

r[mc_compatibility.evidence_manifest_refresh.integration.workflow]
- GIVEN a Cairn drain updates evidence logs, accepted specs, archive tasks, or nested manifests
- WHEN the operator follows the documented workflow
- THEN the helper can refresh manifests before the evidence-manifest and task-evidence checks run
- AND the check surface can detect stale manifests in CI.

### Requirement: Evidence manifest refresh tests

r[mc_compatibility.evidence_manifest_refresh.tests] The change MUST include positive and negative tests for manifest refresh behavior.

#### Scenario: Positive and negative fixtures cover refresh safety

r[mc_compatibility.evidence_manifest_refresh.tests.fixtures]
- GIVEN fixture manifests cover unchanged rows, stale rows, missing files, malformed rows, outside-root paths, and cascading manifest references
- WHEN the helper tests run
- THEN valid fixture refreshes produce expected output
- AND invalid fixtures fail with explicit diagnostics rather than silently rewriting unsafe rows.

### Requirement: Evidence manifest refresh validation

r[mc_compatibility.evidence_manifest_refresh.validation] The change MUST record focused helper tests, existing evidence checkers, Cairn gates, and Cairn validation before archive.

#### Scenario: Refresh automation is reviewable

r[mc_compatibility.evidence_manifest_refresh.validation.logs]
- GIVEN the helper is implemented
- WHEN the change is archived
- THEN reviewable logs show helper tests, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, and Cairn validation passing.

### Requirement: Runner scenario module boundaries

r[mc_compatibility.runner_scenario_modules.boundaries] The runner MUST define explicit module boundaries for scenario core logic before further expanding scenario behavior.

#### Scenario: Scenario core has a small public API

r[mc_compatibility.runner_scenario_modules.boundaries.api]
- GIVEN scenario behavior is maintained in modules
- WHEN other runner code needs a scenario name, alias lookup, milestone list, forbidden pattern, or behavior hook
- THEN it uses the scenario-core API rather than open-coded matches
- AND module exports remain limited to the data and functions needed by runner orchestration.

### Requirement: Pure scenario core extraction

r[mc_compatibility.runner_scenario_modules.scenario_core] Scenario identity, static specs, behavior lookup, and spec validation SHOULD live in pure scenario modules.

#### Scenario: Scenario validation is testable without orchestration

r[mc_compatibility.runner_scenario_modules.scenario_core.pure]
- GIVEN invalid or valid scenario specs are constructed in memory
- WHEN scenario validation tests run
- THEN validation results are produced without starting servers, clients, reading files, writing receipts, or depending on process environment.

### Requirement: Imperative runner shell remains explicit

r[mc_compatibility.runner_scenario_modules.imperative_shell] CLI parsing, backend/client orchestration, environment mutation, log collection, and receipt writing MUST remain in imperative shell code.

#### Scenario: Side effects do not enter scenario validation

r[mc_compatibility.runner_scenario_modules.imperative_shell.side_effects]
- GIVEN scenario validation or behavior lookup is executed
- WHEN tests inspect the scenario-core path
- THEN it performs no filesystem, process, clock, network, or environment side effects
- AND side-effectful runner operations stay in named orchestration code.

### Requirement: Runner surface parity

r[mc_compatibility.runner_scenario_modules.surface_parity] The module split MUST preserve existing compatibility and evidence surfaces unless a separate change explicitly expands them.

#### Scenario: Public runner output remains stable

r[mc_compatibility.runner_scenario_modules.surface_parity.outputs]
- GIVEN the scenario module split is complete
- WHEN existing dry-run, manifest, receipt, and evidence-evaluation paths run
- THEN scenario names, accepted aliases, required milestones, forbidden patterns, receipt fields, non-claim flags, and checker-visible manifest rows match the pre-split behavior.

### Requirement: Runner scenario module tests

r[mc_compatibility.runner_scenario_modules.tests] The change MUST include positive parity tests and negative invalid-definition tests for the extracted modules.

#### Scenario: Module tests prove both success and fail-closed behavior

r[mc_compatibility.runner_scenario_modules.tests.coverage]
- GIVEN the extracted scenario modules expose validation and lookup functions
- WHEN module tests run
- THEN every valid scenario passes parity checks
- AND invalid fixtures for duplicate canonical names, missing aliases, missing milestones, and unsupported behavior defaults fail with explicit diagnostics.

### Requirement: Runner scenario module validation

r[mc_compatibility.runner_scenario_modules.validation] The change MUST record runner tests, manifest checks, dry-run checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Structural split is reviewable

r[mc_compatibility.runner_scenario_modules.validation.logs]
- GIVEN the runner module split is complete
- WHEN the change is archived
- THEN reviewable logs show runner tests, scenario manifest checks, dry-run checks, Cairn proposal/design/tasks gates, and Cairn validation passing.

### Requirement: Runner responsibility inventory

r[mc_compatibility.runner_shell_decoupling.inventory] The compatibility runner MUST inventory `main.rs` responsibilities, side effects, compatibility surfaces, and candidate module owners before large extraction work begins.

#### Scenario: Responsibility clusters are reviewable

r[mc_compatibility.runner_shell_decoupling.inventory.reviewable]
- GIVEN runner shell decoupling is selected
- WHEN reviewers inspect the change design or inventory
- THEN CLI parsing, config loading, planning, backend lifecycle, client driving, MCP control, evidence evaluation, receipt rendering, typed-event graphing, JSON/wire helpers, tests, and side effects are each classified
- AND compatibility surfaces that must not drift are named.

### Requirement: Runner module boundary contract

r[mc_compatibility.runner_shell_decoupling.module_boundaries] The runner SHOULD define crate-private modules around stable responsibility boundaries before moving behavior.

#### Scenario: Module APIs are narrow

r[mc_compatibility.runner_shell_decoupling.module_boundaries.narrow]
- GIVEN a runner responsibility is moved out of `main.rs`
- WHEN reviewers inspect the new module API
- THEN the API accepts explicit inputs and returns explicit results or diagnostics
- AND it does not expose unrelated helper state or require callers to know raw CLI parsing details unless it is the CLI module.

### Requirement: Pure runner cores

r[mc_compatibility.runner_shell_decoupling.pure_cores] Planning, scenario/evidence evaluation, typed-event graphing, and receipt shaping MUST be pure deterministic cores over in-memory inputs.

#### Scenario: Pure core has no shell effects

r[mc_compatibility.runner_shell_decoupling.pure_cores.no_effects]
- GIVEN a moved runner core evaluates a plan, scenario, typed-event graph, receipt, or failure bundle
- WHEN the core executes in a unit test
- THEN it returns deterministic data or diagnostics
- AND it does not read files, inspect environment, spawn processes, open sockets, use clocks, write stdout/stderr, or mutate repository state.

### Requirement: Explicit shell modules

r[mc_compatibility.runner_shell_decoupling.shell_modules] Backend lifecycle, client driving, MCP process control, filesystem, socket, and command execution code MUST remain in explicit shell modules or the top-level shell.

#### Scenario: Shell work is plan-driven

r[mc_compatibility.runner_shell_decoupling.shell_modules.plan_driven]
- GIVEN a backend, client, MCP, or artifact shell performs side effects
- WHEN it is invoked by the runner
- THEN it receives a validated plan/config input and returns typed evidence or diagnostics
- AND raw parsing, evidence-policy decisions, and unrelated scenario semantics are not reimplemented inside the shell.

### Requirement: Runner compatibility preservation

r[mc_compatibility.runner_shell_decoupling.compatibility] The decoupling MUST preserve CLI behavior, scenario aliases, generated manifests, receipt schemas, dry-run output, milestone matching, and non-claim boundaries unless a separate Cairn explicitly changes them.

#### Scenario: Existing surfaces remain stable

r[mc_compatibility.runner_shell_decoupling.compatibility.stable]
- GIVEN the runner split is implemented
- WHEN existing dry-run, receipt, scenario manifest, and compare-receipt checks run
- THEN user-visible names, aliases, fields, non-claims, and diagnostics remain compatible with the pre-split contract
- AND any intentional drift is rejected unless backed by another accepted change.

### Requirement: Runner split tests

r[mc_compatibility.runner_shell_decoupling.tests] The runner split MUST include positive parity tests and negative fail-closed tests for moved cores and migrated shells.

#### Scenario: Positive parity is covered

r[mc_compatibility.runner_shell_decoupling.tests.positive]
- GIVEN valid existing runner configurations, scenarios, receipts, typed-event logs, and failure-bundle inputs
- WHEN moved cores and migrated shells are tested
- THEN outputs match the pre-split behavior or documented compatibility contract.

#### Scenario: Invalid inputs fail closed

r[mc_compatibility.runner_shell_decoupling.tests.negative]
- GIVEN malformed plans, invalid artifact paths, unsupported scenario/receipt combinations, malformed typed events, missing milestones, forbidden milestones, or backend/client shell failures
- WHEN the moved modules evaluate them
- THEN deterministic diagnostics are returned
- AND no false successful evidence or overbroad compatibility claim is emitted.

### Requirement: Runner decoupling validation

r[mc_compatibility.runner_shell_decoupling.validation] Runner decoupling work MUST record focused runner tests, scenario manifest checks, receipt validation, selected dry-runs, Cairn gates, and task-evidence checks before archive.

#### Scenario: Runner closeout is reviewable

r[mc_compatibility.runner_shell_decoupling.validation.log]
- GIVEN the runner split is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative runner tests, compatibility-preserving dry-runs, receipt/schema checks, scenario manifest freshness, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

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

### Requirement: Targeted packet live KV contract

r[mc_compatibility.targeted_packet_live_kv_schema.contract] Targeted packet live promotion evidence MUST use a shared KV schema before rows move beyond fixture-bounded status.

#### Scenario: Common live promotion keys are explicit

r[mc_compatibility.targeted_packet_live_kv_schema.contract.common_keys]
- GIVEN a targeted packet row is proposed for live promotion
- WHEN reviewers inspect the evidence KV
- THEN it includes row id, live promotion status, evidence mode, packet row, scenario, backend/client path, receipt path, receipt BLAKE3 digest, digest currentness, revision metadata when available, and explicit non-claims
- AND blocker or selection notes are not accepted as live promotion evidence.

### Requirement: Pure KV schema core

r[mc_compatibility.targeted_packet_live_kv_schema.core] Live evidence schema validation MUST be pure deterministic logic over parsed key/value records.

#### Scenario: Schema validation has no side effects

r[mc_compatibility.targeted_packet_live_kv_schema.core.pure]
- GIVEN parsed key/value evidence and a row contract
- WHEN schema validation runs
- THEN it returns success or diagnostics without reading files, writing files, spawning commands, inspecting environment, using clocks, or performing network access.

### Requirement: Row extension hooks

r[mc_compatibility.targeted_packet_live_kv_schema.extensions] The schema MUST support row-specific validation extensions without weakening common live-promotion requirements.

#### Scenario: Extensions keep row metrics explicit

r[mc_compatibility.targeted_packet_live_kv_schema.extensions.row_metrics]
- GIVEN creative inventory, resource-pack status, sign editor, or future targeted packet rows need row-specific metrics
- WHEN their live evidence is validated
- THEN common keys are checked first
- AND extension diagnostics name missing or mismatched row-specific metrics such as slot/item/count, local resource-pack offer/status, sign position/payload, or backend correlation.

### Requirement: KV schema tests

r[mc_compatibility.targeted_packet_live_kv_schema.tests] The change MUST include positive and negative tests for common and row-specific live evidence validation.

#### Scenario: Invalid live evidence fails closed

r[mc_compatibility.targeted_packet_live_kv_schema.tests.negative]
- GIVEN evidence is missing required keys, names the wrong packet row, reports a stale receipt digest, lacks required revision metadata, has malformed row-specific fields, or claims broad protocol/gameplay/public-server coverage
- WHEN the checker validates the evidence
- THEN it fails with explicit diagnostics and no promotion is accepted.

### Requirement: KV schema documentation

r[mc_compatibility.targeted_packet_live_kv_schema.docs] The repository SHOULD document the live evidence KV schema and future live-rail workflow.

#### Scenario: Future live rails can follow the schema

r[mc_compatibility.targeted_packet_live_kv_schema.docs.workflow]
- GIVEN a future targeted packet live rail is implemented
- WHEN maintainers inspect the workflow docs
- THEN they can identify required common keys, row-extension fields, non-claim requirements, checker command shape, and evidence-manifest expectations.

### Requirement: KV schema validation

r[mc_compatibility.targeted_packet_live_kv_schema.validation] The change MUST record targeted packet checks, evidence-manifest/task-evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.targeted_packet_live_kv_schema.validation.logs]
- GIVEN the KV schema work is complete
- WHEN the change is archived
- THEN reviewable logs show checker positive/negative tests, targeted packet checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.

### Requirement: Scenario live capability contract

r[mc_compatibility.scenario_live_probe_capabilities.contract] The runner SHOULD define an explicit scenario live-probe capability registry before future targeted packet live promotions rely on scenario selection.

#### Scenario: Capability entries name scope and non-claims

r[mc_compatibility.scenario_live_probe_capabilities.contract.scope]
- GIVEN a scenario can produce or cannot produce a targeted packet live signal
- WHEN maintainers inspect the capability registry
- THEN each entry names scenario id, packet row ids, capability kind, backend/client path, evidence mode, required signals, required non-claims, and optional blocker reason
- AND registry entries do not claim live promotion without separate receipt and checker evidence.

### Requirement: Pure capability registry core

r[mc_compatibility.scenario_live_probe_capabilities.core] Capability lookup and validation MUST be pure deterministic logic over in-memory registry data.

#### Scenario: Registry validation has no side effects

r[mc_compatibility.scenario_live_probe_capabilities.core.pure]
- GIVEN static capability definitions and known scenario/packet row inputs
- WHEN registry validation runs
- THEN it returns success or diagnostics without reading files, writing files, spawning commands, inspecting environment, using clocks, or performing network access.

### Requirement: Seeded capability entries

r[mc_compatibility.scenario_live_probe_capabilities.seed] The registry SHOULD seed entries for currently known targeted packet live candidates and explicit blockers.

#### Scenario: Blocked live paths are visible

r[mc_compatibility.scenario_live_probe_capabilities.seed.blockers]
- GIVEN a targeted packet row lacks a deterministic owned-local live path
- WHEN the registry is queried
- THEN it can report an explicit blocker entry instead of implying that fixture evidence is live promotion evidence.

### Requirement: Capability checker integration

r[mc_compatibility.scenario_live_probe_capabilities.checker] Scenario manifest or focused checker coverage MUST fail closed when capability registry entries drift from known scenarios, packet rows, evidence modes, or non-claim requirements.

#### Scenario: Invalid capability rows fail closed

r[mc_compatibility.scenario_live_probe_capabilities.checker.rejects]
- GIVEN a capability entry names an unknown scenario, unknown packet row, unsupported evidence mode, empty required signals, duplicate scenario/row pair, or missing non-claims
- WHEN registry validation runs through tests or checker coverage
- THEN it fails with explicit diagnostics.

### Requirement: Capability registry tests

r[mc_compatibility.scenario_live_probe_capabilities.tests] The change MUST include positive and negative tests for capability registry lookup and validation.

#### Scenario: Valid and invalid capability fixtures are covered

r[mc_compatibility.scenario_live_probe_capabilities.tests.coverage]
- GIVEN valid capability entries and invalid fixtures for duplicates, unknown rows, unknown scenarios, unsupported modes, empty signals, and missing non-claims
- WHEN registry tests run
- THEN valid entries pass and invalid entries fail with useful diagnostics.

### Requirement: Capability registry documentation

r[mc_compatibility.scenario_live_probe_capabilities.docs] The repository SHOULD document how future live packet rails use the registry for selection and blocker reporting.

#### Scenario: Future live rails have a selection workflow

r[mc_compatibility.scenario_live_probe_capabilities.docs.workflow]
- GIVEN a future targeted packet live rail is proposed
- WHEN maintainers inspect the workflow docs
- THEN they can identify how to add or query capability entries, record blockers, and avoid live-promotion overclaims.

### Requirement: Capability registry validation

r[mc_compatibility.scenario_live_probe_capabilities.validation] The change MUST record scenario tests/manifest checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.scenario_live_probe_capabilities.validation.logs]
- GIVEN capability registry work is complete
- WHEN the change is archived
- THEN reviewable logs show registry tests, scenario manifest or focused checker coverage, relevant runner dry-runs, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.

### Requirement: Creative inventory live contract

r[mc_compatibility.creative_inventory_live_rail.contract] The `creative-inventory-action` live rail MUST define a bounded owned-local contract before live promotion is attempted.

#### Scenario: Contract names one creative mutation

r[mc_compatibility.creative_inventory_live_rail.contract.scope]
- GIVEN the creative inventory row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, creative-mode precondition, semantic slot, wire slot, item id, item count, packet row `CreativeInventoryActionC2SPacket`, backend/client path, expected server correlation, and non-claims
- AND all creative inventory semantics, all slots, all items, all game-mode transitions, pick-block behavior, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Creative inventory baseline

r[mc_compatibility.creative_inventory_live_rail.baseline] The change MUST run existing targeted packet, matrix, bundle, and packet-inventory checks before changing the runner.

#### Scenario: Baseline preserves fixture status

r[mc_compatibility.creative_inventory_live_rail.baseline.recorded]
- GIVEN `creative-inventory-action` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record the existing fixture status and current non-claims before live evidence is introduced.

### Requirement: Creative inventory live rail

r[mc_compatibility.creative_inventory_live_rail.rail] The harness MUST expose an isolated owned-local creative inventory rail or deterministic fixture path for the configured mutation.

#### Scenario: Existing inventory rows remain separate

r[mc_compatibility.creative_inventory_live_rail.rail.isolated]
- GIVEN survival/player-inventory rows and targeted packet fixture rows already exist
- WHEN the creative rail is added
- THEN existing survival/player-inventory scenario semantics remain unchanged
- AND the creative row records its own game-mode, packet, slot-state, and backend-correlation evidence.

### Requirement: Creative inventory live evidence

r[mc_compatibility.creative_inventory_live_rail.evidence] Creative inventory live evidence MUST be reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence is row-specific

r[mc_compatibility.creative_inventory_live_rail.evidence.reviewable]
- GIVEN the configured creative mutation is observed or blocked by a missing live driver
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `creative-inventory-action`, the packet row, scenario, backend/client path, revision metadata when available, slot/item/count metrics, server correlation or blocker, and explicit non-claims.

### Requirement: Creative inventory live checker

r[mc_compatibility.creative_inventory_live_rail.checker] The targeted packet live-evidence checker MUST pass before `creative-inventory-action` moves beyond fixture-bounded status.

#### Scenario: Weak creative evidence fails closed

r[mc_compatibility.creative_inventory_live_rail.checker.rejects]
- GIVEN creative evidence is missing, names the wrong packet row, omits the game-mode precondition, omits server correlation, reports a stale receipt digest, or claims broad creative inventory semantics
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no matrix or bundle row is promoted.

### Requirement: Creative inventory narrow promotion

r[mc_compatibility.creative_inventory_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `creative-inventory-action` after row-specific live evidence passes.

#### Scenario: Other targeted rows stay fixture-bounded

r[mc_compatibility.creative_inventory_live_rail.promotion.narrow]
- GIVEN creative live evidence passes and other targeted rows lack live evidence
- WHEN docs are updated
- THEN only `creative-inventory-action` moves beyond fixture-bounded status
- AND every unproven targeted row retains its prior evidence classification and non-claim notes.

### Requirement: Creative inventory validation

r[mc_compatibility.creative_inventory_live_rail.validation] The change MUST record runner checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.creative_inventory_live_rail.validation.logs]
- GIVEN the creative inventory live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.

### Requirement: Resource-pack local contract

r[mc_compatibility.resource_pack_status_local_rail.contract] The `resource-pack-status` rail MUST define a bounded owned-local offer/status contract before live promotion is attempted.

#### Scenario: Contract names one local exchange

r[mc_compatibility.resource_pack_status_local_rail.contract.scope]
- GIVEN the resource-pack status row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, local fixture identity or hash, offer metadata, expected status response, packet rows, no-external-fetch guarantee, redaction policy, backend/client path, and non-claims
- AND pack download/application, trust/security validation, all status variants, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Resource-pack baseline

r[mc_compatibility.resource_pack_status_local_rail.baseline] The change MUST run existing targeted packet, matrix, bundle, and packet-inventory checks before adding resource-pack rail behavior.

#### Scenario: Fixture status is preserved before live work

r[mc_compatibility.resource_pack_status_local_rail.baseline.recorded]
- GIVEN `resource-pack-status` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs show the existing evidence classification and non-claims.

### Requirement: Resource-pack local rail

r[mc_compatibility.resource_pack_status_local_rail.rail] The harness MUST expose an isolated owned-local resource-pack offer/status rail or deterministic fixture path.

#### Scenario: Rail avoids external fetches

r[mc_compatibility.resource_pack_status_local_rail.rail.local_only]
- GIVEN the resource-pack status rail runs
- WHEN the offer/status exchange is exercised
- THEN any asset fixture is owned-local and bounded
- AND evidence records that no external resource-pack fetch is required.

### Requirement: Resource-pack evidence

r[mc_compatibility.resource_pack_status_local_rail.evidence] Resource-pack status evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes local safety fields

r[mc_compatibility.resource_pack_status_local_rail.evidence.reviewable]
- GIVEN a configured resource-pack status exchange is observed or blocked by a missing live driver
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `resource-pack-status`, packet rows, local fixture identity, expected status response, no-external-fetch metric, redaction policy, backend/client path, revision metadata when available, blocker or server correlation, and explicit non-claims.

### Requirement: Resource-pack checker

r[mc_compatibility.resource_pack_status_local_rail.checker] The targeted packet live-evidence checker MUST pass before `resource-pack-status` moves beyond fixture-bounded status.

#### Scenario: Weak resource-pack evidence fails closed

r[mc_compatibility.resource_pack_status_local_rail.checker.rejects]
- GIVEN evidence lacks local scope, names the wrong packet row, omits status response, omits no-external-fetch proof, has a stale receipt digest, or claims asset trust/application/public-server safety
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Resource-pack narrow promotion

r[mc_compatibility.resource_pack_status_local_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `resource-pack-status` after row-specific live evidence passes.

#### Scenario: Broader resource-pack behavior remains non-claim

r[mc_compatibility.resource_pack_status_local_rail.promotion.nonclaims]
- GIVEN resource-pack status live evidence passes
- WHEN docs are updated
- THEN only the configured status row moves beyond fixture-bounded status
- AND asset loading, trust/security, all statuses, public-server safety, full protocol coverage, and production readiness remain explicit non-claims.

### Requirement: Resource-pack validation

r[mc_compatibility.resource_pack_status_local_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.resource_pack_status_local_rail.validation.logs]
- GIVEN resource-pack status rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, local rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.

### Requirement: Sign editor live contract

r[mc_compatibility.sign_editor_live_rail.contract] The `sign-editor-open-update` rail MUST define a bounded live sign-edit contract before promotion is attempted.

#### Scenario: Contract names one sign edit

r[mc_compatibility.sign_editor_live_rail.contract.scope]
- GIVEN the sign editor row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, sign position, initial sign state, submitted four-line payload, packet rows `SignEditorOpenS2CPacket` and `UpdateSignC2SPacket`, backend/client path, expected accepted-update correlation, and non-claims
- AND all sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Sign editor baseline

r[mc_compatibility.sign_editor_live_rail.baseline] The change MUST run existing targeted packet, matrix, bundle, and packet-inventory checks before modifying sign editor rail behavior.

#### Scenario: Existing sign evidence is not reused as live sign-edit proof

r[mc_compatibility.sign_editor_live_rail.baseline.recorded]
- GIVEN sign block-entity persistence evidence exists separately
- WHEN sign editor live work begins
- THEN baseline logs record that `sign-editor-open-update` remains fixture-bounded until dedicated live open/update evidence passes.

### Requirement: Sign editor rail

r[mc_compatibility.sign_editor_live_rail.rail] The harness MUST expose an isolated sign editor open/update rail or deterministic fixture path.

#### Scenario: Sign editor and sign persistence remain separate

r[mc_compatibility.sign_editor_live_rail.rail.isolated]
- GIVEN existing sign block-entity persistence and packet-family rows are maintained separately
- WHEN the sign editor rail is added
- THEN existing sign persistence claims remain unchanged
- AND the sign editor row records separate open/update packet evidence and backend accepted-update correlation.

### Requirement: Sign editor evidence

r[mc_compatibility.sign_editor_live_rail.evidence] Sign editor live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes open and update correlation

r[mc_compatibility.sign_editor_live_rail.evidence.reviewable]
- GIVEN the configured sign edit is observed
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `sign-editor-open-update`, both packet rows, sign position, submitted payload, client open/update milestones, backend accepted-update correlation, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Sign editor checker

r[mc_compatibility.sign_editor_live_rail.checker] The targeted packet live-evidence checker MUST pass before `sign-editor-open-update` moves beyond fixture-bounded status.

#### Scenario: Weak sign editor evidence fails closed

r[mc_compatibility.sign_editor_live_rail.checker.rejects]
- GIVEN evidence lacks open or update correlation, names the wrong packet row, reports the wrong sign position or payload, has a stale receipt digest, or claims broad sign editing/block-entity coverage
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Sign editor narrow promotion

r[mc_compatibility.sign_editor_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `sign-editor-open-update` after row-specific live evidence passes.

#### Scenario: Broader sign behavior remains non-claim

r[mc_compatibility.sign_editor_live_rail.promotion.nonclaims]
- GIVEN sign editor live evidence passes
- WHEN docs are updated
- THEN only the configured sign editor row moves beyond fixture-bounded status
- AND sign persistence breadth, arbitrary sign text, all sign variants, arbitrary NBT, all block entities, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Sign editor validation

r[mc_compatibility.sign_editor_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.sign_editor_live_rail.validation.logs]
- GIVEN sign editor live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, sign editor rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.

### Requirement: Post-drain hygiene contract

r[mc_compatibility.post_drain_validation_hygiene.contract] The hygiene pass MUST define the checked validation, evidence, manifest, policy, and non-claim scope before mutating any review metadata.

#### Scenario: Hygiene scope is explicit

r[mc_compatibility.post_drain_validation_hygiene.contract.scope]
- GIVEN the active Cairn queue has just been drained
- WHEN the hygiene pass starts
- THEN the pass names the validation commands, evidence checks, manifest checks, drain-state checks, and policy/schema checks it will run
- AND it states that gameplay coverage, protocol coverage, public-server safety, production readiness, and semantic-equivalence claims are unchanged.

### Requirement: Post-drain baseline

r[mc_compatibility.post_drain_validation_hygiene.baseline] The hygiene pass MUST run a non-mutating baseline before refreshing manifests or repairing metadata.

#### Scenario: Baseline separates diagnosis from repair

r[mc_compatibility.post_drain_validation_hygiene.baseline.recorded]
- GIVEN validation or evidence drift may exist
- WHEN baseline checks run
- THEN diagnostics are recorded before any manifest, drain-state, policy, or evidence metadata file is changed
- AND each diagnostic is classified as metadata drift, evidence freshness drift, task citation drift, policy/schema drift, implementation defect, or blocker.

### Requirement: Deterministic hygiene remediation

r[mc_compatibility.post_drain_validation_hygiene.remediation] The hygiene pass MAY repair only deterministic review-metadata drift and MUST NOT change compatibility behavior.

#### Scenario: Metadata-only repairs stay narrow

r[mc_compatibility.post_drain_validation_hygiene.remediation.narrow]
- GIVEN a baseline diagnostic identifies stale BLAKE3 rows, stale drain-state text, or missing review metadata for already-tracked evidence
- WHEN the hygiene pass repairs it
- THEN only the deterministic metadata fields or docs are updated
- AND no runner scenario, checker semantics, acceptance matrix claim, packet inventory claim, or current-bundle compatibility claim is broadened.

### Requirement: Post-drain hygiene evidence

r[mc_compatibility.post_drain_validation_hygiene.evidence] Hygiene results MUST be reviewable under `docs/evidence/` before closeout.

#### Scenario: Evidence records positive and negative outcomes

r[mc_compatibility.post_drain_validation_hygiene.evidence.reviewable]
- GIVEN the hygiene pass completes checks or encounters a blocker
- WHEN reviewers inspect `docs/evidence/`
- THEN run logs record successful checks with explicit `exit_status=0` lines
- AND fail-closed or blocked checks record the diagnostic, owner, and next action without converting blockers into compatibility claims.

### Requirement: Post-drain hygiene validation

r[mc_compatibility.post_drain_validation_hygiene.validation] The hygiene pass MUST rerun relevant validation after remediation and before archive.

#### Scenario: Closeout validation is complete

r[mc_compatibility.post_drain_validation_hygiene.validation.logs]
- GIVEN deterministic remediation is complete or no remediation was needed
- WHEN the change is archived
- THEN reviewable logs show Cairn validation/gates, evidence-manifest checks, task-evidence checks, and any matrix/current-bundle checks passing with explicit successful exit status.

### Requirement: Chat command live contract

r[mc_compatibility.chat_command_live_rail.contract] The `chat-command-containment` live rail MUST define a bounded owned-local contract before live promotion is attempted.

#### Scenario: Contract names one harmless payload

r[mc_compatibility.chat_command_live_rail.contract.scope]
- GIVEN the chat/command containment row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, one harmless payload identity, packet row or rows, owned-local target scope, expected server receipt or rejection metric, redaction policy, backend/client path, and non-claims
- AND chat signing/security, command permissions, moderation, all commands, malicious-client resilience, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Chat command baseline

r[mc_compatibility.chat_command_live_rail.baseline] The change MUST run existing targeted packet, matrix, current-bundle, and packet-inventory checks before modifying chat/command rail behavior.

#### Scenario: Fixture status is recorded first

r[mc_compatibility.chat_command_live_rail.baseline.recorded]
- GIVEN `chat-command-containment` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record its existing evidence classification and non-claims before live evidence is introduced.

### Requirement: Chat command live rail

r[mc_compatibility.chat_command_live_rail.rail] The harness MUST expose an isolated owned-local chat/command rail or deterministic missing-driver blocker for the configured payload.

#### Scenario: Rail is isolated from public-server safety

r[mc_compatibility.chat_command_live_rail.rail.isolated]
- GIVEN existing public-server and network-safety rows have separate authorization contracts
- WHEN the chat/command rail is added
- THEN it uses only owned-local fixture targets
- AND existing CTF, survival, combat, inventory, network, public-server, and production-readiness claims remain unchanged.

### Requirement: Chat command live evidence

r[mc_compatibility.chat_command_live_rail.evidence] Chat/command live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes containment and redaction fields

r[mc_compatibility.chat_command_live_rail.evidence.reviewable]
- GIVEN the configured chat/command payload is observed or blocked by a missing driver
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `chat-command-containment`, packet rows, payload identity, owned-local scope, server containment metric or blocker, redaction policy, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Chat command live checker

r[mc_compatibility.chat_command_live_rail.checker] The targeted packet live-evidence checker MUST pass before `chat-command-containment` moves beyond fixture-bounded status.

#### Scenario: Weak chat evidence fails closed

r[mc_compatibility.chat_command_live_rail.checker.rejects]
- GIVEN chat evidence is missing, lacks owned-local scope, names the wrong payload or packet row, omits server correlation, reports a stale receipt digest, lacks redaction policy, or claims public-server/security/command breadth
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Chat command narrow promotion

r[mc_compatibility.chat_command_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `chat-command-containment` after row-specific live evidence passes.

#### Scenario: Broader chat behavior remains non-claim

r[mc_compatibility.chat_command_live_rail.promotion.nonclaims]
- GIVEN chat/command live evidence passes
- WHEN docs are updated
- THEN only the configured owned-local containment row moves beyond fixture-bounded status
- AND chat signing/security, command permissions, moderation, all commands, public-server safety, full protocol coverage, and production readiness remain explicit non-claims.

### Requirement: Chat command validation

r[mc_compatibility.chat_command_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.chat_command_live_rail.validation.logs]
- GIVEN chat/command live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.

### Requirement: Entity status-effect live contract

r[mc_compatibility.entity_status_effect_live_rail.contract] The `entity-status-effect-packets` live rail MUST define a bounded owned-local status-effect contract before live promotion is attempted.

#### Scenario: Contract names one effect transition

r[mc_compatibility.entity_status_effect_live_rail.contract.scope]
- GIVEN the status-effect packet row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor or target entity, effect id or name, amplifier, duration, packet row or rows, backend/client path, expected server correlation, and non-claims
- AND all effects, stacking, particles/UI, gameplay modifiers, combat balancing, survival parity, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Entity status-effect baseline

r[mc_compatibility.entity_status_effect_live_rail.baseline] The change MUST run existing targeted packet, matrix, current-bundle, and packet-inventory checks before modifying status-effect rail behavior.

#### Scenario: Fixture status is recorded first

r[mc_compatibility.entity_status_effect_live_rail.baseline.recorded]
- GIVEN `entity-status-effect-packets` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record its existing evidence classification and non-claims before live evidence is introduced.

### Requirement: Entity status-effect live rail

r[mc_compatibility.entity_status_effect_live_rail.rail] The harness MUST expose an isolated status-effect rail or deterministic missing-signal blocker for the configured effect transition.

#### Scenario: Effect packet row stays separate from mechanics claims

r[mc_compatibility.entity_status_effect_live_rail.rail.isolated]
- GIVEN existing combat and survival rows have separate scoped claims
- WHEN the status-effect rail is added
- THEN existing combat, survival, CTF, inventory, network, and negative-live semantics remain unchanged
- AND the status-effect row records only packet observation and server-correlation metrics.

### Requirement: Entity status-effect live evidence

r[mc_compatibility.entity_status_effect_live_rail.evidence] Status-effect live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes effect metrics

r[mc_compatibility.entity_status_effect_live_rail.evidence.reviewable]
- GIVEN the configured effect transition is observed or blocked by a missing signal
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `entity-status-effect-packets`, packet rows, entity identity, effect id or name, amplifier, duration, client apply and optional remove observations, server correlation or blocker, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Entity status-effect live checker

r[mc_compatibility.entity_status_effect_live_rail.checker] The targeted packet live-evidence checker MUST pass before `entity-status-effect-packets` moves beyond fixture-bounded status.

#### Scenario: Weak status-effect evidence fails closed

r[mc_compatibility.entity_status_effect_live_rail.checker.rejects]
- GIVEN status-effect evidence is missing, uses stale revisions or receipt digest, names the wrong entity, effect, amplifier, duration, or packet row, omits required apply or remove correlation, or claims broad effect/modifier semantics
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Entity status-effect narrow promotion

r[mc_compatibility.entity_status_effect_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `entity-status-effect-packets` after row-specific live evidence passes.

#### Scenario: Broader effect behavior remains non-claim

r[mc_compatibility.entity_status_effect_live_rail.promotion.nonclaims]
- GIVEN status-effect live evidence passes
- WHEN docs are updated
- THEN only the configured status-effect packet row moves beyond fixture-bounded status
- AND all effects, stacking, particles/UI, modifiers, combat balancing, survival parity, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Entity status-effect validation

r[mc_compatibility.entity_status_effect_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.entity_status_effect_live_rail.validation.logs]
- GIVEN status-effect live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.

### Requirement: Recipe-book settings live contract

r[mc_compatibility.recipe_book_settings_live_rail.contract] The `recipe-book-client-settings` live rail MUST define a bounded owned-local settings transition contract before live promotion is attempted.

#### Scenario: Contract names one settings transition

r[mc_compatibility.recipe_book_settings_live_rail.contract.scope]
- GIVEN the recipe-book settings row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, packet row `RecipeBookDataC2SPacket`, configured recipe-book state fields, client action metric, backend/client path, expected Valence server correlation, and non-claims
- AND recipe-book UI behavior, recipe discovery, all recipe categories, all recipes, crafting breadth, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Recipe-book settings baseline

r[mc_compatibility.recipe_book_settings_live_rail.baseline] The change MUST run existing targeted packet, matrix, current-bundle, and packet-inventory checks before modifying recipe-book rail behavior.

#### Scenario: Fixture status is recorded first

r[mc_compatibility.recipe_book_settings_live_rail.baseline.recorded]
- GIVEN `recipe-book-client-settings` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record its existing evidence classification and non-claims before live evidence is introduced.

### Requirement: Recipe-book settings live rail

r[mc_compatibility.recipe_book_settings_live_rail.rail] The harness MUST expose an isolated recipe-book settings rail or deterministic missing-driver blocker for the configured transition.

#### Scenario: Settings row stays separate from crafting parity

r[mc_compatibility.recipe_book_settings_live_rail.rail.isolated]
- GIVEN existing crafting evidence covers one crafting-table recipe path
- WHEN the recipe-book settings rail is added
- THEN existing crafting claims and survival scenario semantics remain unchanged
- AND the new row records only settings packet evidence and server correlation.

### Requirement: Recipe-book settings live evidence

r[mc_compatibility.recipe_book_settings_live_rail.evidence] Recipe-book settings live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes settings fields

r[mc_compatibility.recipe_book_settings_live_rail.evidence.reviewable]
- GIVEN the configured recipe-book settings transition is observed or blocked by a missing driver
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `recipe-book-client-settings`, packet row, configured settings fields, client action metric, server correlation or blocker, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Recipe-book settings live checker

r[mc_compatibility.recipe_book_settings_live_rail.checker] The targeted packet live-evidence checker MUST pass before `recipe-book-client-settings` moves beyond fixture-bounded status.

#### Scenario: Weak recipe-book evidence fails closed

r[mc_compatibility.recipe_book_settings_live_rail.checker.rejects]
- GIVEN recipe-book evidence is missing, names the wrong packet row or settings fields, omits client action or server correlation, reports a stale receipt digest, or claims broad recipe-book/crafting coverage
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Recipe-book settings narrow promotion

r[mc_compatibility.recipe_book_settings_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `recipe-book-client-settings` after row-specific live evidence passes.

#### Scenario: Broader recipe behavior remains non-claim

r[mc_compatibility.recipe_book_settings_live_rail.promotion.nonclaims]
- GIVEN recipe-book settings live evidence passes
- WHEN docs are updated
- THEN only the configured settings row moves beyond fixture-bounded status
- AND recipe-book UI, discovery, all recipes, crafting breadth, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Recipe-book settings validation

r[mc_compatibility.recipe_book_settings_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.recipe_book_settings_live_rail.validation.logs]
- GIVEN recipe-book settings live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.

### Requirement: Evidence refresh smoke contract

r[mc_compatibility.evidence_refresh_flake_smoke.contract] The evidence refresh pass MUST define a bounded smoke set, command scope, runtime limits, evidence paths, and non-claims before running checks.

#### Scenario: Smoke scope is explicit

r[mc_compatibility.evidence_refresh_flake_smoke.contract.scope]
- GIVEN evidence refresh work starts
- WHEN reviewers inspect the smoke contract
- THEN it names selected Cairn checks, targeted packet checks, scenario manifest checks, representative flake dry-runs or smokes, expected evidence outputs, runtime limits, and non-claims
- AND live gameplay parity, public-server safety, WAN behavior, production readiness, and new packet/gameplay coverage remain explicit non-claims.

### Requirement: Evidence refresh baseline

r[mc_compatibility.evidence_refresh_flake_smoke.baseline] The evidence refresh pass MUST run selected non-mutating baseline checks before manifest refresh.

#### Scenario: Baseline runs before evidence mutation

r[mc_compatibility.evidence_refresh_flake_smoke.baseline.recorded]
- GIVEN selected checks and dry-runs can reveal stale metadata
- WHEN baseline checks run
- THEN logs record command shape, selected scope, success or fail-closed diagnostics, and non-claim status before BLAKE3 manifests are refreshed.

### Requirement: Evidence refresh logs

r[mc_compatibility.evidence_refresh_flake_smoke.logs] Review-critical smoke output MUST be recorded under `docs/evidence/` with explicit successful exit statuses or blocker notes.

#### Scenario: Run logs are citeable

r[mc_compatibility.evidence_refresh_flake_smoke.logs.reviewable]
- GIVEN a smoke or check output is cited by tasks or closeout notes
- WHEN reviewers inspect the evidence
- THEN the cited `.run.log` contains an `exit_status=0` line for successful checks
- AND any failed or skipped check is represented as a blocker note with owner and next action rather than a passing claim.

### Requirement: Evidence refresh manifests

r[mc_compatibility.evidence_refresh_flake_smoke.manifests] BLAKE3 manifests MUST be refreshed only for changed tracked evidence files and rerun to a deterministic fixpoint.

#### Scenario: Manifest refresh is deterministic

r[mc_compatibility.evidence_refresh_flake_smoke.manifests.fixpoint]
- GIVEN smoke logs or evidence sidecars changed
- WHEN manifest refresh runs
- THEN only digest fields for existing in-repository evidence files are updated
- AND evidence-manifest checks pass after refresh or report a fail-closed blocker.

### Requirement: Evidence refresh validation

r[mc_compatibility.evidence_refresh_flake_smoke.validation] The evidence refresh pass MUST record task-evidence checks, Cairn gates, sync/archive checks, and post-archive validation before closeout.

#### Scenario: Closeout validation is complete

r[mc_compatibility.evidence_refresh_flake_smoke.validation.logs]
- GIVEN smoke logs and manifests are refreshed or blockers are recorded
- WHEN the change is archived
- THEN reviewable logs show selected smoke checks, manifest refresh/checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing without promoting new compatibility rows.

### Requirement: Runner architecture hardening contract

r[mc_compatibility.runner_architecture_hardening.contract] The architecture hardening pass MUST select one bounded runner or checker seam and name public output invariants before refactoring.

#### Scenario: Hardening scope is explicit

r[mc_compatibility.runner_architecture_hardening.contract.scope]
- GIVEN the runner/checker architecture is prepared for a hardening pass
- WHEN reviewers inspect the contract
- THEN it names the selected seam, public scenario or checker outputs, receipt fields, diagnostics, non-claims, and behavior that must remain unchanged
- AND it states that no new gameplay, protocol, public-server, production-readiness, or semantic-equivalence coverage is added.

### Requirement: Runner architecture baseline

r[mc_compatibility.runner_architecture_hardening.baseline] The hardening pass MUST run focused baseline tests and dry-runs before refactoring the selected seam.

#### Scenario: Baseline captures current output

r[mc_compatibility.runner_architecture_hardening.baseline.recorded]
- GIVEN the selected seam has existing behavior
- WHEN baseline checks run
- THEN logs capture current pass/fail behavior, output fields, diagnostics, and non-claim state before implementation changes are introduced.

### Requirement: Runner architecture pure core

r[mc_compatibility.runner_architecture_hardening.core] The selected seam MUST be split into pure deterministic core logic and a thin imperative shell.

#### Scenario: Core is side-effect free

r[mc_compatibility.runner_architecture_hardening.core.pure]
- GIVEN in-memory inputs for the selected seam
- WHEN the extracted core logic runs
- THEN it returns deterministic decisions, diagnostics, or normalized records without reading files, writing files, spawning commands, inspecting environment, using clocks, performing network access, or mutating external state
- AND non-obvious numeric values are named constants.

#### Scenario: Shell owns side effects

r[mc_compatibility.runner_architecture_hardening.core.shell]
- GIVEN the selected seam needs filesystem, process, environment, network, receipt-writing, or stdout/stderr behavior
- WHEN the migrated path executes
- THEN those effects remain in named shell code
- AND receipt schemas, scenario names, milestone IDs, backend names, checker row ids, and non-claim flags match the baseline unless a separate Cairn changes them.

### Requirement: Runner architecture hardening tests

r[mc_compatibility.runner_architecture_hardening.tests] The hardening pass MUST include positive parity tests and negative fail-closed tests for the selected seam.

#### Scenario: Positive and negative fixtures are covered

r[mc_compatibility.runner_architecture_hardening.tests.coverage]
- GIVEN valid baseline-equivalent fixtures and invalid malformed, unknown-name, missing-evidence, stale-revision, and overclaim fixtures
- WHEN focused tests run
- THEN valid fixtures preserve baseline output
- AND invalid fixtures fail closed with explicit diagnostics instead of changing coverage claims.

### Requirement: Runner architecture evidence

r[mc_compatibility.runner_architecture_hardening.evidence] Architecture-hardening evidence MUST be reviewable under `docs/evidence/` before closeout.

#### Scenario: Evidence records no compatibility expansion

r[mc_compatibility.runner_architecture_hardening.evidence.reviewable]
- GIVEN the selected seam is migrated
- WHEN artifacts are written
- THEN logs identify the selected seam, baseline checks, parity checks, negative fixtures, and unchanged public-output contract
- AND no new compatibility row is promoted by the hardening evidence alone.

### Requirement: Runner architecture validation

r[mc_compatibility.runner_architecture_hardening.validation] The change MUST record focused tests, relevant runner/checker checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout validation is complete

r[mc_compatibility.runner_architecture_hardening.validation.logs]
- GIVEN architecture hardening is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, positive parity tests, negative fail-closed tests, runner/checker checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.

### Requirement: Stevenarella resource-pack driver contract

r[mc_compatibility.stevenarella_resource_pack_status_driver.contract] The Stevenarella resource-pack status driver MUST define a bounded owned-local offer/status contract before implementation.

#### Scenario: Driver scope is explicit

r[mc_compatibility.stevenarella_resource_pack_status_driver.contract.scope]
- GIVEN the resource-pack status row is blocked by a missing client driver
- WHEN reviewers inspect the driver contract
- THEN it names one offer identity, owned-local scope rule, expected status response, no-external-fetch guarantee, redaction policy, protocol output path, backend/client integration path, and non-claims
- AND asset download/application, trust/security validation, all status variants, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Stevenarella resource-pack baseline

r[mc_compatibility.stevenarella_resource_pack_status_driver.baseline] The change MUST run focused baseline checks before modifying Stevenarella resource-pack behavior.

#### Scenario: Existing blocker is recorded first

r[mc_compatibility.stevenarella_resource_pack_status_driver.baseline.recorded]
- GIVEN `resource-pack-status` currently has a local rail blocker
- WHEN implementation begins
- THEN baseline logs record the existing blocker, relevant Stevenarella focused tests, runner dry-runs, targeted-packet checks, and current non-claims before driver changes are introduced.

### Requirement: Stevenarella resource-pack driver

r[mc_compatibility.stevenarella_resource_pack_status_driver.driver] Stevenarella MUST handle the configured owned-local resource-pack offer through pure decision logic and a thin protocol-response shell.

#### Scenario: Driver emits protocol response without external fetch

r[mc_compatibility.stevenarella_resource_pack_status_driver.driver.no_fetch]
- GIVEN Stevenarella receives the configured owned-local resource-pack offer
- WHEN the driver evaluates it
- THEN pure decision logic selects the configured status response from explicit inputs
- AND the imperative shell sends the status through the protocol path without fetching external assets, writing unbounded files, or using host OS input synthesis.

### Requirement: Stevenarella resource-pack driver tests

r[mc_compatibility.stevenarella_resource_pack_status_driver.tests] The driver MUST include positive and negative tests before any live row depends on it.

#### Scenario: Valid and invalid offers are covered

r[mc_compatibility.stevenarella_resource_pack_status_driver.tests.coverage]
- GIVEN valid owned-local offer fixtures and invalid malformed, external-scope, unsupported-status, missing-state, and overlarge/redaction fixtures
- WHEN focused tests run
- THEN the valid local offer emits the configured status response
- AND invalid fixtures fail closed before protocol response, external fetch, or unbounded artifact write.

### Requirement: Stevenarella resource-pack runner integration

r[mc_compatibility.stevenarella_resource_pack_status_driver.integration] Runner or control-plane integration MUST expose the driver only through an isolated resource-pack status path.

#### Scenario: Integration does not broaden resource-pack claims

r[mc_compatibility.stevenarella_resource_pack_status_driver.integration.isolated]
- GIVEN the driver is available
- WHEN the runner or MCP-controlled path uses it
- THEN evidence is scoped to the configured owned-local offer/status exchange
- AND asset loading, trust/security, all statuses, public-server behavior, production readiness, and other targeted packet rows remain unchanged.

### Requirement: Stevenarella resource-pack evidence

r[mc_compatibility.stevenarella_resource_pack_status_driver.evidence] Driver evidence MUST be durable and reviewable under `docs/evidence/` before live promotion is attempted.

#### Scenario: Evidence includes no-external-fetch proof

r[mc_compatibility.stevenarella_resource_pack_status_driver.evidence.reviewable]
- GIVEN the driver produces resource-pack status evidence
- WHEN artifacts are written
- THEN KV, receipt, and log artifacts name the offer identity, expected status, no-external-fetch metric, redaction policy, backend/client path, child revisions, server correlation if available, and explicit non-claims.

### Requirement: Stevenarella resource-pack validation

r[mc_compatibility.stevenarella_resource_pack_status_driver.validation] The change MUST record driver tests, runner checks, targeted-packet checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.stevenarella_resource_pack_status_driver.validation.logs]
- GIVEN resource-pack status driver work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, positive and negative driver tests, runner checks, targeted-packet checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.

### Requirement: Stevenarella sign-editor driver contract

r[mc_compatibility.stevenarella_sign_editor_driver.contract] The Stevenarella sign-editor driver MUST define a bounded open/update contract before implementation.

#### Scenario: Driver scope is explicit

r[mc_compatibility.stevenarella_sign_editor_driver.contract.scope]
- GIVEN the sign-editor open/update row is blocked by a missing client driver
- WHEN reviewers inspect the driver contract
- THEN it names one actor, sign position, initial sign state, submitted four-line payload, line and length bounds, expected open/update milestones, protocol output path, backend/client integration path, and non-claims
- AND sign editing UI behavior, all sign variants, arbitrary text formats, arbitrary NBT semantics, all block entities, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Stevenarella sign-editor baseline

r[mc_compatibility.stevenarella_sign_editor_driver.baseline] The change MUST run focused baseline checks before modifying Stevenarella sign-editor behavior.

#### Scenario: Existing blocker is recorded first

r[mc_compatibility.stevenarella_sign_editor_driver.baseline.recorded]
- GIVEN `sign-editor-open-update` currently lacks dedicated live open/update proof
- WHEN implementation begins
- THEN baseline logs record the existing blocker, relevant Stevenarella focused tests, runner dry-runs, targeted-packet checks, and current non-claims before driver changes are introduced.

### Requirement: Stevenarella sign-editor driver

r[mc_compatibility.stevenarella_sign_editor_driver.driver] Stevenarella MUST handle the configured sign-editor open/update flow through pure validation logic and a thin main-thread/protocol shell.

#### Scenario: Driver submits bounded update through protocol path

r[mc_compatibility.stevenarella_sign_editor_driver.driver.protocol]
- GIVEN Stevenarella observes the configured sign-editor open state
- WHEN the driver receives the configured four-line update request
- THEN pure validation checks the position, line count, line lengths, payload content, and connected state from explicit inputs
- AND the imperative shell submits the update through the protocol path without host OS input synthesis or direct mutation from a worker thread.

### Requirement: Stevenarella sign-editor driver tests

r[mc_compatibility.stevenarella_sign_editor_driver.tests] The driver MUST include positive and negative tests before any live row depends on it.

#### Scenario: Valid and invalid updates are covered

r[mc_compatibility.stevenarella_sign_editor_driver.tests.coverage]
- GIVEN valid configured open/update fixtures and invalid missing-open, wrong-position, malformed-payload, line-count, line-length, disconnected-state, and overclaim fixtures
- WHEN focused tests run
- THEN the valid update emits the configured protocol action
- AND invalid fixtures fail closed before protocol output or state mutation.

### Requirement: Stevenarella sign-editor runner integration

r[mc_compatibility.stevenarella_sign_editor_driver.integration] Runner or control-plane integration MUST expose the driver only through an isolated sign-editor path.

#### Scenario: Integration does not reuse sign-persistence proof

r[mc_compatibility.stevenarella_sign_editor_driver.integration.isolated]
- GIVEN sign block-entity persistence evidence exists separately
- WHEN the runner or MCP-controlled path uses the sign-editor driver
- THEN evidence is scoped to the configured open/update packet exchange
- AND sign persistence, arbitrary NBT, all block entities, public-server behavior, production readiness, and other targeted packet rows remain unchanged.

### Requirement: Stevenarella sign-editor evidence

r[mc_compatibility.stevenarella_sign_editor_driver.evidence] Driver evidence MUST be durable and reviewable under `docs/evidence/` before live promotion is attempted.

#### Scenario: Evidence includes open and update correlation

r[mc_compatibility.stevenarella_sign_editor_driver.evidence.reviewable]
- GIVEN the driver produces sign-editor evidence
- WHEN artifacts are written
- THEN KV, receipt, and log artifacts name the sign position, submitted payload, client open milestone, client update milestone, backend accepted-update correlation if available, backend/client path, child revisions, and explicit non-claims.

### Requirement: Stevenarella sign-editor validation

r[mc_compatibility.stevenarella_sign_editor_driver.validation] The change MUST record driver tests, runner checks, targeted-packet checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.stevenarella_sign_editor_driver.validation.logs]
- GIVEN sign-editor driver work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, positive and negative driver tests, runner checks, targeted-packet checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.

### Requirement: Server-correlation rail contract

r[mc_compatibility.server_correlation_rail.contract] The compatibility evidence set MUST define a reusable owned-local server-correlation receipt contract before live promotion uses resource-pack or sign-editor driver output.

#### Scenario: Receipt scope is explicit

r[mc_compatibility.server_correlation_rail.contract.scope]
- GIVEN bounded Stevenarella drivers can emit resource-pack status and sign-editor update milestones
- WHEN reviewers inspect the server-correlation contract
- THEN it names row, scenario, actor, owned-local scope, packet rows, backend path, client path, child revisions, client milestones, server events, correlation status, redaction policy, and non-claims
- AND public-server safety, production readiness, full protocol 763 compatibility, broad Minecraft compatibility, arbitrary sign semantics, and resource-pack asset trust remain explicit non-claims.

### Requirement: Server-correlation rail baseline

r[mc_compatibility.server_correlation_rail.baseline] The change MUST run current nonpromotion and manifest checks before adding the checker.

#### Scenario: Existing blocked state is preserved

r[mc_compatibility.server_correlation_rail.baseline.recorded]
- GIVEN resource-pack status and sign-editor rows are currently fixture-bounded blockers
- WHEN implementation begins
- THEN baseline logs record targeted-packet nonpromotion, scenario-manifest validation, and Cairn gates before checker changes are introduced.

### Requirement: Server-correlation receipt checker

r[mc_compatibility.server_correlation_rail.checker] The repository MUST provide a deterministic checker for owned-local server-correlation receipts.

#### Scenario: Promotion receipts fail closed

r[mc_compatibility.server_correlation_rail.checker.fail_closed]
- GIVEN a server-correlation receipt is provided for validation
- WHEN required fields, row-specific milestones, server events, revisions, non-claims, or `correlation.status=observed` are missing or malformed
- THEN the checker rejects the receipt with a deterministic diagnostic
- AND the checker performs validation in a pure core over in-memory receipt values with a thin CLI file-reading shell.

### Requirement: Server-correlation fixtures

r[mc_compatibility.server_correlation_rail.fixtures] The checker MUST include positive and negative fixtures for the initial driver-backed rows.

#### Scenario: Supported rows have happy and sad path coverage

r[mc_compatibility.server_correlation_rail.fixtures.coverage]
- GIVEN resource-pack status and sign-editor open/update receipts
- WHEN self-tests run
- THEN valid owned-local observed-correlation fixtures pass
- AND missing server events, blocked correlation status, wrong sign position, wrong resource-pack status, malformed packet rows, and overclaim fixtures fail closed.

### Requirement: Server-correlation rail integration

r[mc_compatibility.server_correlation_rail.integration] The checker MUST be wired into the flake check graph without changing targeted-packet row promotion state.

#### Scenario: Integration is non-promoting

r[mc_compatibility.server_correlation_rail.integration.nonpromoting]
- GIVEN the maintained rail exists
- WHEN flake checks and targeted-packet checks run
- THEN the server-correlation checker validates fixture receipts
- AND resource-pack status and sign-editor targeted-packet rows remain fixture-bounded until real live receipts are supplied.

### Requirement: Server-correlation rail evidence

r[mc_compatibility.server_correlation_rail.evidence] The rail MUST emit reviewable evidence under `docs/evidence/`.

#### Scenario: Evidence is reviewable

r[mc_compatibility.server_correlation_rail.evidence.reviewable]
- GIVEN the checker and fixtures pass
- WHEN evidence is recorded
- THEN KV, JSON receipt, run-log, and BLAKE3 manifest artifacts name the supported rows, checker command, fixture identities, nonpromotion status, and non-claims.

### Requirement: Server-correlation rail validation

r[mc_compatibility.server_correlation_rail.validation] The change MUST record checker tests, integration checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.server_correlation_rail.validation.logs]
- GIVEN server-correlation rail work is complete
- WHEN the change is archived
- THEN reviewable logs show checker self-tests, fixture checks, flake integration, targeted-packet nonpromotion checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.

### Requirement: Stevenarella block registry boundaries

r[mc_compatibility.stevenarella_blocks.registry_boundaries] Stevenarella block registry code SHOULD separate generated or declarative block data from hand-authored runtime APIs, helper logic, id-map logic, and public compatibility exports.

#### Scenario: Block data and logic are distinguishable

r[mc_compatibility.stevenarella_blocks.registry_boundaries.ownership]
- GIVEN a block registry change is reviewed
- WHEN maintainers inspect the block crate module tree
- THEN generated block facts and hand-authored runtime logic are owned by distinct modules
- AND reviewers can identify whether the change alters data, logic, or public exports.

### Requirement: Stevenarella block registry parity

r[mc_compatibility.stevenarella_blocks.registry_parity] Block registry separation MUST preserve public block names, numeric IDs, exports, material and collision semantics, `VanillaIDMap` lookup behavior, missing-block fallback behavior, modded-block fallback behavior, and evidence non-claims.

#### Scenario: Existing block lookups remain stable

r[mc_compatibility.stevenarella_blocks.registry_parity.stable]
- GIVEN a supported pre-refactor block lookup, material lookup, collision lookup, or public export use
- WHEN the separated block registry processes the same input
- THEN the returned block, metadata, fallback, and public symbol behavior remain equivalent
- AND no new block support or world compatibility claim is promoted.

### Requirement: Stevenarella block generated freshness

r[mc_compatibility.stevenarella_blocks.generated_freshness] If block data is generated or snapshot-owned, the change SHOULD include a deterministic freshness check that rejects stale checked-in generated data.

#### Scenario: Generated block data is fresh

r[mc_compatibility.stevenarella_blocks.generated_freshness.check]
- GIVEN the generated block data source or generator changes
- WHEN the block registry freshness check runs
- THEN checked-in generated block data is verified against the source
- AND stale generated output is rejected before archive.

### Requirement: Stevenarella block registry positive tests

r[mc_compatibility.stevenarella_blocks.positive_tests] The change MUST include positive tests for representative block id lookups, flat and hierarchical mappings, material access, collision access, and public re-exports.

#### Scenario: Supported block registry paths pass

r[mc_compatibility.stevenarella_blocks.positive_tests.coverage]
- GIVEN representative supported block registry inputs
- WHEN separated block registry modules process them
- THEN tests prove the expected blocks, metadata, collisions, and exported symbols are available.

### Requirement: Stevenarella block registry negative tests

r[mc_compatibility.stevenarella_blocks.negative_tests] The change MUST include negative tests for missing ids, unsupported modded ids, stale generated data, invalid data indices, and unknown fallback paths.

#### Scenario: Invalid block registry paths fail closed

r[mc_compatibility.stevenarella_blocks.negative_tests.fail_closed]
- GIVEN invalid or unsupported block registry inputs
- WHEN separated block registry modules process them
- THEN tests prove the inputs return the current missing-block or diagnostic behavior without panicking or corrupting lookup state.

### Requirement: Stevenarella block registry validation

r[mc_compatibility.stevenarella_blocks.validation] The change MUST record focused block tests, generated freshness checks if added, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_blocks.validation.logs]
- GIVEN block registry separation is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative block-registry tests plus freshness checks when applicable and Cairn gates passing.

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

### Requirement: Maintained scenario dry-run coverage

r[mc_compatibility.harness_dry_run_coverage.contract] Maintained mc-compat scenarios SHOULD have deterministic dry-run receipt-shape coverage unless a reviewed waiver records why the row cannot yet be represented by a dry-run fixture.

#### Scenario: Maintained row has executable shape coverage

r[mc_compatibility.harness_dry_run_coverage.contract.covered]
- GIVEN a scenario manifest row is marked maintained
- WHEN the dry-run coverage gate evaluates the row
- THEN the row has a dry-run wrapper and check that produce a bounded receipt shape
- OR the row has a waiver with an owner, reason, non-claim boundary, and next action.

### Requirement: Dry-run coverage checker

r[mc_compatibility.harness_dry_run_coverage.checker] The repository MUST include a deterministic checker core that validates dry-run coverage and waiver metadata from in-memory manifest data before any file-system shell reports diagnostics.

#### Scenario: Missing dry-run metadata fails closed

r[mc_compatibility.harness_dry_run_coverage.checker.negative]
- GIVEN a maintained scenario lacks a dry-run check
- WHEN no complete waiver metadata is present
- THEN the checker fails with a diagnostic naming the scenario and missing dry-run or waiver field.

### Requirement: Eligible wrapper conversion

r[mc_compatibility.harness_dry_run_coverage.wrappers] Eligible maintained exclusions SHOULD be converted into deterministic dry-run wrappers that preserve existing scenario names, milestone IDs, receipt schemas, and non-claims.

#### Scenario: Converted wrapper does not broaden evidence

r[mc_compatibility.harness_dry_run_coverage.wrappers.nonclaim]
- GIVEN an excluded row is converted to dry-run shape coverage
- WHEN the new wrapper emits its receipt
- THEN the receipt records deterministic fixture scope
- AND it does not claim live gameplay parity, full protocol compatibility, semantic equivalence, public-server safety, or production readiness.

### Requirement: Documentation separates evidence classes

r[mc_compatibility.harness_dry_run_coverage.docs] README and evidence-bundle wording MUST distinguish dry-run receipt-shape coverage from live, paired-reference, and promoted row evidence.

#### Scenario: Reviewer can identify evidence type

r[mc_compatibility.harness_dry_run_coverage.docs.review]
- GIVEN a reviewer inspects a maintained row
- WHEN they read the README, current bundle, or manifest output
- THEN they can identify whether the row is covered by a dry-run fixture, live receipt, paired-reference comparator, or waiver.

### Requirement: Dry-run coverage validation

r[mc_compatibility.harness_dry_run_coverage.validation] The change MUST record focused runner tests, scenario-manifest checks, affected dry-run checks, maintained dry-run aggregate output, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves coverage policy

r[mc_compatibility.harness_dry_run_coverage.validation.log]
- GIVEN dry-run coverage gaps are closed or waiver-backed
- WHEN the change is archived
- THEN reviewable logs show positive and negative coverage fixtures, successful wrapper dry-runs, aggregate maintained dry-run checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Typed-event readiness contract

r[mc_compatibility.typed_event_readiness.contract] Maintained scenarios SHOULD migrate from substring fallback to typed-event-ready status when client milestones, server milestones, forbidden patterns, and receipt timeline evidence have typed-event equivalents or explicit derivation rules.

#### Scenario: Row-level readiness is complete

r[mc_compatibility.typed_event_readiness.contract.complete]
- GIVEN a scenario row is marked `typed-event-ready`
- WHEN its milestone and forbidden-pattern surfaces are inspected
- THEN each required client milestone, server milestone, and forbidden pattern is backed by a typed event or named derivation rule
- AND the row records no new compatibility claim solely because typed events exist.

### Requirement: Typed-event readiness checker

r[mc_compatibility.typed_event_readiness.checker] The scenario-manifest checker MUST evaluate typed-event readiness and fallback waivers from pure in-memory scenario and fixture data.

#### Scenario: Incomplete readiness fails closed

r[mc_compatibility.typed_event_readiness.checker.negative]
- GIVEN a scenario row is marked `typed-event-ready`
- WHEN a required client typed event, server typed event, forbidden-pattern mapping, or derivation rule is missing
- THEN the checker fails with a diagnostic naming the scenario and missing typed-event surface.

### Requirement: Controlled migration

r[mc_compatibility.typed_event_readiness.migration] Eligible scenario rows MAY move to `typed-event-ready` only after parity fixtures prove existing milestone IDs, forbidden IDs, receipt scenario names, and non-claim wording remain stable.

#### Scenario: Migrated row preserves evidence semantics

r[mc_compatibility.typed_event_readiness.migration.parity]
- GIVEN a row moves from substring fallback to typed-event-ready
- WHEN dry-run or receipt-shape validation runs
- THEN required/observed/missing milestone calculations match the pre-migration contract
- AND receipt non-claims remain unchanged.

### Requirement: Typed-event-first tests

r[mc_compatibility.typed_event_readiness.tests] The runner MUST include positive and negative tests proving typed-event-ready rows prefer typed-event evidence and fail closed before substring fallback can hide missing structured events.

#### Scenario: Missing typed events are not masked

r[mc_compatibility.typed_event_readiness.tests.fail_closed]
- GIVEN a typed-event-ready fixture omits a required typed event but includes the legacy substring
- WHEN the typed-event oracle evaluates the fixture
- THEN the fixture fails with a structured missing-event diagnostic.

### Requirement: Typed-event readiness documentation

r[mc_compatibility.typed_event_readiness.docs] README and evidence docs MUST explain typed-event-ready status, substring fallback waivers, and reviewer expectations for migrated rows.

#### Scenario: Reviewer can distinguish migration states

r[mc_compatibility.typed_event_readiness.docs.review]
- GIVEN a reviewer inspects scenario manifest output
- WHEN a row is typed-event-ready or waiver-backed fallback
- THEN the row's validation basis and next action are visible without reading live logs.

### Requirement: Typed-event readiness validation

r[mc_compatibility.typed_event_readiness.validation] The change MUST record focused runner tests, typed-event oracle fixtures, scenario-manifest checks, affected dry-run checks, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves typed-event migration safety

r[mc_compatibility.typed_event_readiness.validation.log]
- GIVEN rows are migrated or waiver-backed
- WHEN the change is archived
- THEN reviewable logs show typed-event positive and negative fixtures, manifest migration-state checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Structured receipt-test contract

r[mc_compatibility.receipt_schema_tests.contract] Receipt tests MUST validate evidence-critical JSON structure and values with typed or structured assertions when receipt JSON is the public contract.

#### Scenario: Evidence-critical fields are structured

r[mc_compatibility.receipt_schema_tests.contract.critical]
- GIVEN a receipt records scenario evidence
- WHEN tests validate non-claims, child revisions, typed-event artifacts, backend identity, artifact paths, or overclaim fields
- THEN tests use structured field assertions rather than substring presence alone.

### Requirement: Receipt parser helpers

r[mc_compatibility.receipt_schema_tests.parser] The runner or checker test support SHOULD provide pure receipt summary parsing helpers that operate on in-memory receipt text.

#### Scenario: Positive fixture parses representative receipts

r[mc_compatibility.receipt_schema_tests.parser.positive]
- GIVEN representative dry-run, typed-event, MCP, and paired-reference receipt fixtures
- WHEN receipt summary parsing runs
- THEN it extracts scenario name, backend, non-claims, child revision status, typed-event artifact status, and evidence mode without file-system access.

### Requirement: Negative receipt fixtures

r[mc_compatibility.receipt_schema_tests.negative] Receipt schema tests MUST include negative fixtures for missing nonclaims, stale or dirty child revisions, missing typed events, wrong backend, malformed artifact paths, duplicate or wrong-typed fields, and broad overclaim keys.

#### Scenario: Malformed receipt fails closed

r[mc_compatibility.receipt_schema_tests.negative.fail_closed]
- GIVEN a receipt fixture omits an evidence-critical field or records an invalid value
- WHEN structured receipt validation runs
- THEN it fails with a diagnostic naming the missing or malformed field
- AND no compatibility evidence is accepted from that fixture.

### Requirement: Substring assertion migration

r[mc_compatibility.receipt_schema_tests.migration] Existing substring-only assertions for receipt JSON SHOULD migrate to structured assertions unless the tested contract is explicitly free-form CLI text.

#### Scenario: JSON tests no longer depend on incidental text

r[mc_compatibility.receipt_schema_tests.migration.structured]
- GIVEN a runner unit test validates receipt JSON
- WHEN the test asserts receipt content
- THEN it validates parsed structure and values
- AND uses substring checks only for intentionally free-form output fields or legacy compatibility text.

### Requirement: Receipt schema test documentation

r[mc_compatibility.receipt_schema_tests.docs] Documentation or checker output SHOULD explain which receipt fields are evidence-critical and why structured tests are required.

#### Scenario: Reviewer sees validation boundary

r[mc_compatibility.receipt_schema_tests.docs.review]
- GIVEN a reviewer inspects receipt test evidence
- WHEN they read the change evidence or checker notes
- THEN they can identify which fields are structurally validated and which text checks remain intentionally free-form.

### Requirement: Receipt schema validation

r[mc_compatibility.receipt_schema_tests.validation] The change MUST record runner tests, receipt checker fixtures, affected dry-run checks, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves schema hardening

r[mc_compatibility.receipt_schema_tests.validation.log]
- GIVEN structured receipt tests are introduced
- WHEN the change is archived
- THEN reviewable logs show positive and negative receipt fixtures, migrated runner assertions, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Generated surface contract

r[mc_compatibility.generated_harness_surfaces.contract] Scenario-derived harness surfaces SHOULD be generated from the typed scenario manifest when the generated output is stable, bounded, and reviewable.

#### Scenario: Runtime remains Rust-owned

r[mc_compatibility.generated_harness_surfaces.contract.runtime]
- GIVEN generated harness surfaces exist
- WHEN `tools/mc-compat-runner` starts
- THEN it consumes checked-in Rust or static artifacts
- AND it does not evaluate Nickel at runtime.

### Requirement: Pure generator core

r[mc_compatibility.generated_harness_surfaces.generator] The generator MUST separate pure manifest parsing/rendering from the imperative shell that reads and writes repository files.

#### Scenario: Invalid manifest fixture fails closed

r[mc_compatibility.generated_harness_surfaces.generator.negative]
- GIVEN a manifest fixture has a missing required field, duplicate generated name, unsupported migration state, or unsafe output path
- WHEN the generator core evaluates the fixture
- THEN it returns deterministic diagnostics and emits no partial generated artifact.

### Requirement: Generated Rust scenario tables

r[mc_compatibility.generated_harness_surfaces.rust_tables] The checked-in Rust scenario tables SHOULD be generated from the manifest while preserving scenario names, aliases, milestone IDs, forbidden-pattern IDs, behavior metadata, and receipt semantics.

#### Scenario: Generated Rust preserves scenario parity

r[mc_compatibility.generated_harness_surfaces.rust_tables.parity]
- GIVEN generated Rust tables replace or refresh manual tables
- WHEN runner parity tests enumerate all scenarios
- THEN parsed names, aliases, client milestones, server milestones, forbidden patterns, dry-run metadata, and migration states match the manifest.

### Requirement: Generated documentation blocks

r[mc_compatibility.generated_harness_surfaces.docs_blocks] Documentation or index output MAY be generated only inside clearly delimited machine-owned blocks while human-authored evidence interpretation remains outside generated sections.

#### Scenario: Generated block is bounded

r[mc_compatibility.generated_harness_surfaces.docs_blocks.review]
- GIVEN a README or evidence index contains generated scenario commands
- WHEN a reviewer inspects the file
- THEN generated content is bounded by ownership markers
- AND prose outside the markers is not overwritten by the generator.

### Requirement: Generated output freshness

r[mc_compatibility.generated_harness_surfaces.freshness] The repository MUST include a check that regenerates manifest-derived outputs and fails when checked-in generated artifacts are stale.

#### Scenario: Stale generated artifact fails Nix check

r[mc_compatibility.generated_harness_surfaces.freshness.drift]
- GIVEN the scenario manifest changes without refreshing generated outputs
- WHEN the generated-output freshness check runs
- THEN it reports the stale artifact path and fails before evidence can be promoted.

### Requirement: Generated surface validation

r[mc_compatibility.generated_harness_surfaces.validation] The change MUST record generator fixtures, runner tests, scenario-manifest checks, generated-output freshness checks, maintained dry-run aggregate output, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves generation safety

r[mc_compatibility.generated_harness_surfaces.validation.log]
- GIVEN generated surfaces are introduced
- WHEN the change is archived
- THEN reviewable logs show positive and negative generator fixtures, stale-output rejection, runner parity tests, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Stevenarella login outcome model

r[mc_compatibility.stevenarella_login.login_outcome_model] Stevenarella login handling SHOULD normalize supported login packet variants into explicit pure login events, decisions, and outcomes before constructing the live `Server` session.

#### Scenario: Login success variants normalize

r[mc_compatibility.stevenarella_login.login_outcome_model.normalize_success]
- GIVEN a supported login success packet variant is observed during login
- WHEN the login core processes the packet summary
- THEN it returns a normalized login outcome with username, UUID, and optional property facts
- AND downstream shell code does not need to duplicate packet-variant-specific success handling.

### Requirement: Stevenarella login shell boundary

r[mc_compatibility.stevenarella_login.login_shell_boundary] Stevenarella login extraction MUST keep network I/O, packet writes, encryption side effects, Mojang session joins, random secret generation, reader spawning, and live `Server` construction outside the pure login core.

#### Scenario: Side effects remain in shell

r[mc_compatibility.stevenarella_login.login_shell_boundary.effects]
- GIVEN the login core indicates compression, encryption, disconnect, or login success
- WHEN the Stevenarella connection shell applies that decision
- THEN only the shell mutates the connection, performs cryptographic/session side effects, spawns readers, or constructs the live server object
- AND the core remains testable with in-memory inputs.

### Requirement: Stevenarella login parity

r[mc_compatibility.stevenarella_login.login_parity] Stevenarella login-core extraction MUST preserve online and offline login behavior, compression propagation, encryption behavior, disconnect handling, wrong-packet diagnostics, milestone output, and protocol-version behavior.

#### Scenario: Existing login semantics remain stable

r[mc_compatibility.stevenarella_login.login_parity.stable]
- GIVEN a supported pre-refactor login path
- WHEN the extracted login core and shell process the same packet sequence
- THEN the resulting connection state, login outcome, milestone output, and error behavior remain equivalent
- AND the refactor does not claim new protocol support.

### Requirement: Stevenarella login positive tests

r[mc_compatibility.stevenarella_login.login_positive_tests] The change MUST include positive tests for offline login success variants, encrypted login decisions, compression-before-success handling, and normalized UUID/property outcomes.

#### Scenario: Supported login paths pass

r[mc_compatibility.stevenarella_login.login_positive_tests.coverage]
- GIVEN representative supported login packet sequences
- WHEN the login core processes them
- THEN tests prove the expected decisions and normalized outcomes are produced.

### Requirement: Stevenarella login negative tests

r[mc_compatibility.stevenarella_login.login_negative_tests] The change MUST include negative tests for disconnects, wrong packets, malformed login outcomes, unsupported FML network versions, and incomplete encryption negotiation.

#### Scenario: Invalid login paths fail closed

r[mc_compatibility.stevenarella_login.login_negative_tests.fail_closed]
- GIVEN invalid or unsupported login inputs
- WHEN the login core or shell processes them
- THEN tests prove the login path returns the existing diagnostic or containment behavior without constructing a corrupt live server session.

### Requirement: Stevenarella login validation

r[mc_compatibility.stevenarella_login.login_validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_login.login_validation.logs]
- GIVEN login-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative login tests plus affected dry-runs and Cairn gates passing.

### Requirement: Failure-bundle contract

r[mc_compatibility.failure_evidence_bundles.contract] Failed or blocked mc-compat runs SHOULD produce a bounded diagnostic bundle that records scenario, backend, command summary, first failure, artifact paths, artifact BLAKE3 digests, and explicit non-claims.

#### Scenario: Failure bundle is diagnostic only

r[mc_compatibility.failure_evidence_bundles.contract.nonclaim]
- GIVEN a compatibility rail fails or is blocked
- WHEN a failure bundle is written
- THEN the bundle records a failed or blocked outcome
- AND it does not claim scenario success, gameplay parity, full protocol compatibility, public-server safety, production readiness, or semantic equivalence.

### Requirement: Failure-bundle validator

r[mc_compatibility.failure_evidence_bundles.validator] The repository MUST include a pure validator for failure-bundle shape, digest format, artifact path policy, outcome status, and non-claim presence.

#### Scenario: Invalid failure bundle fails closed

r[mc_compatibility.failure_evidence_bundles.validator.negative]
- GIVEN a bundle has missing artifacts, path escapes, malformed BLAKE3 digests, missing nonclaims, or a success-labeled outcome
- WHEN validation runs
- THEN it fails with diagnostics naming each invalid field
- AND the bundle cannot be cited as review evidence.

### Requirement: Runner failure bundle emission

r[mc_compatibility.failure_evidence_bundles.runner] Runner failure paths SHOULD collect available receipt, client log, server log, typed-event log, stderr, and command-summary metadata into a failure bundle while preserving the original failing exit status.

#### Scenario: Original failure remains visible

r[mc_compatibility.failure_evidence_bundles.runner.exit]
- GIVEN a dry-run or live rail fails
- WHEN the runner writes a failure bundle
- THEN the command still exits as failed
- AND the bundle names the first failure without rewriting it as a passing receipt.

### Requirement: Failure-bundle documentation

r[mc_compatibility.failure_evidence_bundles.docs] Documentation MUST explain when failure bundles should be copied into `docs/evidence/`, how their BLAKE3 identities are recorded, and why they remain non-claiming diagnostic artifacts.

#### Scenario: Reviewer can reproduce artifact identity

r[mc_compatibility.failure_evidence_bundles.docs.review]
- GIVEN a failure bundle is cited in Cairn tasks or evidence notes
- WHEN a reviewer inspects the cited files
- THEN the bundle and each critical artifact path resolve under reviewable evidence storage
- AND BLAKE3 digests identify the cited bytes.

### Requirement: Failure-bundle validation evidence

r[mc_compatibility.failure_evidence_bundles.validation] The change MUST record failure-bundle positive and negative fixtures, runner failure-path tests, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves diagnostic bundle safety

r[mc_compatibility.failure_evidence_bundles.validation.log]
- GIVEN failure bundle support is implemented
- WHEN the change is archived
- THEN reviewable logs show validator fixtures, fail-only outcome rejection, path and digest rejection, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Harness planning-core contract

r[mc_compatibility.harness_planning_core.contract] Non-trivial mc-compat runner orchestration logic SHOULD be represented by pure planning cores before shell code performs side effects.

#### Scenario: Plan core has no side effects

r[mc_compatibility.harness_planning_core.contract.pure]
- GIVEN runner orchestration logic derives server, client, receipt, artifact, or cleanup intent
- WHEN that logic is moved into the planning core
- THEN it performs no filesystem reads, filesystem writes, process execution, Docker calls, environment mutation, sleeps, clocks, or network probes
- AND it returns deterministic plans or diagnostics from explicit inputs.

### Requirement: Explicit plan structs

r[mc_compatibility.harness_planning_core.plan_structs] The runner SHOULD define focused plan structs for server startup, client sessions, receipt output, artifact collection, and cleanup actions.

#### Scenario: Plan records orchestration intent

r[mc_compatibility.harness_planning_core.plan_structs.intent]
- GIVEN a validated runner config and scenario metadata
- WHEN plan generation runs
- THEN the resulting plans name backend, ports, client sessions, scenario expectations, receipt destinations, artifact paths, cleanup actions, and non-claim context without launching external services.

### Requirement: Thin imperative shell

r[mc_compatibility.harness_planning_core.shell] Side-effecting runner code MUST remain in thin shell functions that consume plans and report outcomes without duplicating planning policy.

#### Scenario: Shell executes rather than decides

r[mc_compatibility.harness_planning_core.shell.boundary]
- GIVEN the shell starts servers, clients, Docker containers, cleanup, or artifact collection
- WHEN it executes a plan
- THEN plan policy has already been computed by the core
- AND shell code owns only I/O, process management, environment mutation, and error plumbing.

### Requirement: Positive planning fixtures

r[mc_compatibility.harness_planning_core.positive_tests] The change MUST include positive tests for representative dry-run, live, matrix, reconnect, multi-client, Paper, Valence, cleanup, and failure-bundle planning paths.

#### Scenario: Supported plans are deterministic

r[mc_compatibility.harness_planning_core.positive_tests.coverage]
- GIVEN representative supported configurations
- WHEN plan generation runs repeatedly
- THEN it returns the same plan data for the same inputs
- AND preserves existing CLI defaults, scenario names, receipt paths, and non-claim boundaries.

### Requirement: Negative planning fixtures

r[mc_compatibility.harness_planning_core.negative_tests] The change MUST include negative tests for invalid backend/config combinations, unsafe public-server inputs, missing receipt destinations, matrix flag conflicts, path hazards, and cleanup hazards.

#### Scenario: Unsafe plan fails before side effects

r[mc_compatibility.harness_planning_core.negative_tests.fail_closed]
- GIVEN a configuration would target an unsafe public server, escape an artifact path, conflict matrix flags, or remove an unsafe cleanup path
- WHEN plan generation runs
- THEN it returns diagnostics before any shell side effect is attempted.

### Requirement: Harness planning-core validation

r[mc_compatibility.harness_planning_core.validation] The change MUST record baseline runner tests before refactor, post-refactor runner tests, plan-core fixtures, scenario-manifest checks, affected dry-run checks, evidence manifest checks, and Cairn gates before archive.

#### Scenario: Validation proves architecture parity

r[mc_compatibility.harness_planning_core.validation.log]
- GIVEN planning core extraction is complete
- WHEN the change is archived
- THEN reviewable logs show baseline and post-refactor runner tests, positive and negative plan fixtures, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Aggregate survival claim boundary contract

r[mc_compatibility.survival_aggregate_parity_claim_boundary.contract] The repo MUST define an aggregate survival claim boundary before any documentation can claim full survival compatibility or broad vanilla survival parity.

#### Scenario: Boundary names prerequisites

r[mc_compatibility.survival_aggregate_parity_claim_boundary.contract.scope]
- GIVEN row-scoped survival evidence exists
- WHEN the aggregate claim boundary is reviewed
- THEN it names required survival row families, required paired evidence artifacts, manifest freshness requirements, and allowed claim vocabulary
- AND it states that row-scoped coverage alone is not full survival compatibility or broad vanilla parity.

### Requirement: Aggregate survival boundary checker

r[mc_compatibility.survival_aggregate_parity_claim_boundary.checker] A deterministic checker MUST reject aggregate survival claims unless every prerequisite row and evidence artifact passes.

#### Scenario: Broad overclaim fails closed

r[mc_compatibility.survival_aggregate_parity_claim_boundary.checker.rejects]
- GIVEN docs claim full survival compatibility or broad vanilla parity while any prerequisite row, comparator output, evidence manifest, child revision, or aggregate evidence bundle is missing or stale
- WHEN the checker evaluates the docs
- THEN it fails and names the missing prerequisite or overclaiming text.

### Requirement: Aggregate gate wiring

r[mc_compatibility.survival_aggregate_parity_claim_boundary.gate] The aggregate boundary checker MUST be available as a focused verification gate without changing row-scoped survival evidence semantics.

#### Scenario: Existing row gates remain unchanged

r[mc_compatibility.survival_aggregate_parity_claim_boundary.gate.isolated]
- GIVEN existing survival row gates pass
- WHEN the aggregate gate is added
- THEN row-scoped reference-parity labels remain unchanged
- AND aggregate full-survival wording is checked by the new boundary gate.

### Requirement: Instrumentation inventory

r[mc_compatibility.compat_instrumentation_boundary.inventory] The harness MUST inventory compatibility-specific client probes, capture hooks, MCP surfaces, server fixture milestones, and scenario toggles before moving or gating instrumentation code.

#### Scenario: Evidence hook is identified

r[mc_compatibility.compat_instrumentation_boundary.inventory.hook]
- GIVEN a probe, event, milestone, or scenario toggle contributes to compatibility evidence
- WHEN the instrumentation inventory is reviewed
- THEN the hook is listed with owner, component path, scenario usage, event vocabulary, and migration status.

### Requirement: Instrumentation boundary contract

r[mc_compatibility.compat_instrumentation_boundary.contract] Compatibility instrumentation SHOULD be isolated behind explicit modules, Cargo features, environment toggles, or harness-only entrypoints rather than being implicit in core client/server logic.

#### Scenario: Instrumentation is opt-in

r[mc_compatibility.compat_instrumentation_boundary.contract.opt_in]
- GIVEN a compat probe or harness-only action exists
- WHEN core component code is reviewed
- THEN the probe is reachable through an explicit instrumentation boundary
- AND default product behavior is not silently coupled to scenario-specific harness actions.

### Requirement: Instrumentation migration

r[mc_compatibility.compat_instrumentation_boundary.migration] Moving instrumentation MUST preserve required typed-event and milestone vocabulary unless the evidence checkers and fixtures migrate in the same change.

#### Scenario: Event vocabulary remains stable

r[mc_compatibility.compat_instrumentation_boundary.migration.events]
- GIVEN an instrumentation family is moved behind a boundary
- WHEN typed-event fixtures and scenario dry-runs execute
- THEN required event names, milestone IDs, correlation IDs, and non-claim fields remain equivalent
- OR any vocabulary change is accompanied by checker and fixture updates.

### Requirement: Instrumentation tests

r[mc_compatibility.compat_instrumentation_boundary.tests] Instrumentation boundary changes MUST include positive tests for enabled instrumentation and negative tests for disabled or core-only paths.

#### Scenario: Disabled instrumentation does not fire

r[mc_compatibility.compat_instrumentation_boundary.tests.disabled]
- GIVEN compat instrumentation is disabled or the component runs outside the harness profile
- WHEN the tested core path executes
- THEN harness-only events or scenario actions are not emitted
- AND core behavior remains valid for the tested path.

### Requirement: Instrumentation documentation

r[mc_compatibility.compat_instrumentation_boundary.docs] Component and harness docs SHOULD describe instrumentation boundaries, feature flags, event vocabulary ownership, and evidence implications.

#### Scenario: Instrumentation owner is discoverable

r[mc_compatibility.compat_instrumentation_boundary.docs.owner]
- GIVEN a developer needs to change a compat probe
- WHEN they read subtree-local docs or harness docs
- THEN they can find the owning module/feature, scenarios affected, tests to run, and evidence checker implications.

### Requirement: Instrumentation validation

r[mc_compatibility.compat_instrumentation_boundary.validation] Instrumentation boundary work MUST record affected component tests, typed-event fixtures, selected dry-runs/live checks if required, and Cairn gates before archive.

#### Scenario: Boundary closeout is reviewable

r[mc_compatibility.compat_instrumentation_boundary.validation.log]
- GIVEN compat instrumentation has been isolated or gated
- WHEN the change is archived
- THEN reviewable logs show enabled-path tests, disabled-path tests, typed-event fixture checks, selected scenario checks if required, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Aggregate boundary docs

r[mc_compatibility.survival_aggregate_parity_claim_boundary.docs] Survival matrix, acceptance matrix, and current bundle docs MUST point broad survival claims at the aggregate boundary gate.

#### Scenario: Current non-claims remain explicit

r[mc_compatibility.survival_aggregate_parity_claim_boundary.docs.nonclaims]
- GIVEN the aggregate boundary docs are updated
- WHEN reviewers inspect current evidence
- THEN full survival compatibility and broad vanilla parity remain explicit non-claims until the aggregate gate passes with required evidence.

### Requirement: Aggregate boundary evidence

r[mc_compatibility.survival_aggregate_parity_claim_boundary.evidence] Gate output and manifest evidence for the aggregate boundary MUST be copied under `docs/evidence/` before archive.

#### Scenario: Evidence is reviewable

r[mc_compatibility.survival_aggregate_parity_claim_boundary.evidence.reviewable]
- GIVEN the aggregate boundary checker is added
- WHEN the change is ready for review
- THEN checker output, manifest evidence, and task evidence logs are present under `docs/evidence/`.

### Requirement: Aggregate boundary validation

r[mc_compatibility.survival_aggregate_parity_claim_boundary.validation] The change MUST record checker, focused flake check, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_aggregate_parity_claim_boundary.validation.log]
- GIVEN the change is completed
- WHEN it is archived
- THEN repo-local evidence logs record checker self-tests, focused flake checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Crafting breadth contract

r[mc_compatibility.survival_crafting_recipe_breadth_parity.contract] The `survival-crafting-recipe-breadth-parity` row MUST define a bounded recipe matrix before evidence is promoted.

#### Scenario: Contract fixes finite recipe scope

r[mc_compatibility.survival_crafting_recipe_breadth_parity.contract.scope]
- GIVEN crafting breadth work starts
- WHEN the evidence contract is reviewed
- THEN it names one shaped recipe, one shapeless recipe, one invalid or insufficient-input rejection, one configured collection mode, and normalized recipe/input/result/inventory metrics
- AND all-recipes, recipe-book UI, arbitrary collection modes, full survival compatibility, and broad vanilla parity remain explicit non-claims.

### Requirement: Crafting breadth checker

r[mc_compatibility.survival_crafting_recipe_breadth_parity.checker] A deterministic checker MUST compare paired Paper/reference and Valence crafting-breadth metrics before promotion.

#### Scenario: Weak crafting evidence fails closed

r[mc_compatibility.survival_crafting_recipe_breadth_parity.checker.rejects]
- GIVEN evidence is missing the Paper record, contains only Valence evidence, omits recipe ids, omits slot/result metrics, mismatches item counts, reports stale child revisions, or claims all recipes
- WHEN the checker evaluates the row
- THEN it fails with diagnostics naming the missing or invalid metric.

### Requirement: Isolated crafting breadth rail

r[mc_compatibility.survival_crafting_recipe_breadth_parity.rail] The harness MUST expose an isolated crafting-breadth rail without changing the existing `survival-crafting-table` row.

#### Scenario: Existing crafting row remains unchanged

r[mc_compatibility.survival_crafting_recipe_breadth_parity.rail.isolated]
- GIVEN the existing stick crafting row is already promoted
- WHEN the crafting-breadth rail is added
- THEN the existing row keeps its milestones, receipts, and non-claims
- AND the new row records its own recipe matrix metrics.

### Requirement: Reviewable crafting breadth receipts

r[mc_compatibility.survival_crafting_recipe_breadth_parity.receipts] Paired crafting-breadth receipts and logs MUST be copied under `docs/evidence/` with child revision metadata and BLAKE3 manifests.

#### Scenario: Receipts are local and paired

r[mc_compatibility.survival_crafting_recipe_breadth_parity.receipts.reviewable]
- GIVEN the row is ready for review
- WHEN reviewers inspect the repository
- THEN Paper/reference and Valence receipts, client logs, server logs, comparator output, and manifests are available under `docs/evidence/`.

### Requirement: Narrow crafting breadth promotion

r[mc_compatibility.survival_crafting_recipe_breadth_parity.promotion] Matrix and bundle docs MUST promote only the bounded crafting-breadth row after paired evidence passes.

#### Scenario: Broader crafting remains a non-claim

r[mc_compatibility.survival_crafting_recipe_breadth_parity.promotion.nonclaims]
- GIVEN paired crafting-breadth evidence passes
- WHEN docs are updated
- THEN only the configured crafting-breadth row is marked covered
- AND all-recipes, recipe-book UI, arbitrary collection modes, full survival compatibility, broad vanilla parity, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Crafting breadth validation evidence

r[mc_compatibility.survival_crafting_recipe_breadth_parity.validation] The change MUST record checker, comparator, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.survival_crafting_recipe_breadth_parity.validation.log]
- GIVEN the row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker self-tests, paired comparator, scenario checks, evidence manifests, task-evidence gate, Cairn gates, and Cairn validation.

### Requirement: Role-based core component boundaries

r[mc_compatibility.core_component_layout.boundaries] Core Minecraft source trees SHOULD be organized by product role rather than upstream provenance when the project owns ongoing changes to that source tree.

#### Scenario: Core components are named by role

r[mc_compatibility.core_component_layout.boundaries.roles]
- GIVEN Stevenarella and Valence are parent-owned core source trees
- WHEN the project layout is inspected
- THEN the client implementation is discoverable under a client role boundary
- AND the server implementation is discoverable under a server role boundary
- AND historical upstream ancestry is documented without labeling those trees as passive vendors.

### Requirement: Central source layout resolver

r[mc_compatibility.core_component_layout.resolver] The compatibility harness MUST resolve core component source roots through a single typed layout resolver instead of scattering path probes across runner, wrapper, and documentation code.

#### Scenario: Resolver accepts valid layouts and rejects unsafe layouts

r[mc_compatibility.core_component_layout.resolver.fixtures]
- GIVEN the repository is in either the approved transition layout or the final role-based layout
- WHEN the resolver locates client, server, and compatibility roots
- THEN it returns canonical component paths for downstream callers
- AND missing roots, ambiguous duplicate roots, or nested Git directories under core components fail with deterministic diagnostics.

### Requirement: Core source moves preserve evidence semantics

r[mc_compatibility.core_component_layout.core_moves] Moving Valence or Stevenarella into role-based component roots MUST preserve parent-owned source-tree semantics and path-scoped revision evidence.

#### Scenario: Revision evidence stays path-scoped after moves

r[mc_compatibility.core_component_layout.core_moves.revision_scope]
- GIVEN a core component has moved to a role-based path
- WHEN the runner records client or server revision evidence
- THEN the recorded revision and dirty-state checks are scoped to the resolved component path
- AND they do not rely on nested Git repositories inside the component tree.

### Requirement: Compatibility harness boundary

r[mc_compatibility.core_component_layout.compat_boundary] Compatibility runner source, scenario manifests, generated harness surfaces, and Paper/reference fixtures SHOULD live under a dedicated compatibility boundary when moving them does not weaken generated-surface freshness or evidence checks.

#### Scenario: Harness paths remain generated and checkable

r[mc_compatibility.core_component_layout.compat_boundary.generated]
- GIVEN compatibility harness files are moved under the compatibility boundary
- WHEN generated-surface and scenario-manifest checks run
- THEN generated paths, wrapper names, dry-run metadata, and scenario indexes remain current
- AND the move does not change scenario behavior or pass/fail semantics.

### Requirement: Core component documentation

r[mc_compatibility.core_component_layout.docs] Documentation MUST describe the current role-based ownership model for clients, servers, and compatibility harnesses, including upstream ancestry where relevant.

#### Scenario: Documentation avoids vendor/fork terminology for core components

r[mc_compatibility.core_component_layout.docs.terminology]
- GIVEN a reviewer reads README, AGENTS, or architecture notes after the migration
- WHEN those docs refer to Stevenarella or Valence
- THEN they describe the trees as core client and server components
- AND upstream ancestry is presented as provenance metadata rather than the active ownership boundary.

### Requirement: Layout migration validation

r[mc_compatibility.core_component_layout.validation] The layout migration MUST NOT be marked complete until tests and reviewable evidence prove the resolver, moved paths, docs, generated surfaces, and Cairn lifecycle are consistent.

#### Scenario: Validation evidence covers positive and negative cases

r[mc_compatibility.core_component_layout.validation.evidence]
- GIVEN tasks claim the role-based layout migration is complete
- WHEN reviewers inspect promoted evidence
- THEN logs show positive and negative resolver fixtures, no nested Git directories under core components, runner dry-run path discovery, scenario-manifest checks, generated-surface freshness checks, Cairn gates, and Cairn validation
- AND the evidence explicitly states that compatibility semantics and live parity claims are unchanged.

### Requirement: Inventory drag typed-event readiness

r[mc_compatibility.inventory_drag_typed_event_migration.readiness] The `inventory-drag-transactions` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence quick-craft server milestones, forbidden surfaces, and ordered drag phases.

#### Scenario: Inventory drag row is typed-event-ready

r[mc_compatibility.inventory_drag_typed_event_migration.readiness.complete]
- GIVEN `inventory-drag-transactions` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Inventory drag typed-event gate

r[mc_compatibility.inventory_drag_typed_event_migration.gate] The runner MUST include `inventory-drag-transactions` in the typed-event pass/fail gate so missing or invalid structured drag events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed drag evidence fails closed

r[mc_compatibility.inventory_drag_typed_event_migration.gate.missing]
- GIVEN a drag receipt fixture contains legacy substring-compatible milestones but omits a required typed drag event
- WHEN typed-event validation evaluates `inventory-drag-transactions`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed drag phases fail closed

r[mc_compatibility.inventory_drag_typed_event_migration.gate.order]
- GIVEN a drag receipt fixture contains all required typed drag events but puts the quick-craft end before a required target phase
- WHEN typed-event validation evaluates `inventory-drag-transactions`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Inventory drag migration evidence

r[mc_compatibility.inventory_drag_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.inventory_drag_typed_event_migration.validation.log]
- GIVEN the inventory drag row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the inventory-drag dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival break/place typed-event readiness

r[mc_compatibility.survival_break_place_typed_event_migration.readiness] The `survival-break-place-pickup` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence survival server milestones, forbidden surfaces, and ordered break/pickup/place phases.

#### Scenario: Survival break/place row is typed-event-ready

r[mc_compatibility.survival_break_place_typed_event_migration.readiness.complete]
- GIVEN `survival-break-place-pickup` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival break/place typed-event gate

r[mc_compatibility.survival_break_place_typed_event_migration.gate] The runner MUST include `survival-break-place-pickup` in the typed-event pass/fail gate so missing or invalid structured survival events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed survival evidence fails closed

r[mc_compatibility.survival_break_place_typed_event_migration.gate.missing]
- GIVEN a survival break/place fixture contains legacy substring-compatible milestones but omits a required typed survival event
- WHEN typed-event validation evaluates `survival-break-place-pickup`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed survival phases fail closed

r[mc_compatibility.survival_break_place_typed_event_migration.gate.order]
- GIVEN a survival break/place fixture contains all required typed survival events but puts the server place phase before the required pickup phase
- WHEN typed-event validation evaluates `survival-break-place-pickup`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival break/place migration evidence

r[mc_compatibility.survival_break_place_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_break_place_typed_event_migration.validation.log]
- GIVEN the survival break/place row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the survival break/place dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival crafting-table typed-event readiness

r[mc_compatibility.survival_crafting_table_typed_event_migration.readiness] The `survival-crafting-table` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence crafting-table server milestones, forbidden surfaces, and ordered crafting phases.

#### Scenario: Survival crafting-table row is typed-event-ready

r[mc_compatibility.survival_crafting_table_typed_event_migration.readiness.complete]
- GIVEN `survival-crafting-table` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival crafting-table typed-event gate

r[mc_compatibility.survival_crafting_table_typed_event_migration.gate] The runner MUST include `survival-crafting-table` in the typed-event pass/fail gate so missing or invalid structured crafting events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed crafting evidence fails closed

r[mc_compatibility.survival_crafting_table_typed_event_migration.gate.missing]
- GIVEN a survival crafting-table fixture contains legacy substring-compatible milestones but omits a required typed crafting event
- WHEN typed-event validation evaluates `survival-crafting-table`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed crafting phases fail closed

r[mc_compatibility.survival_crafting_table_typed_event_migration.gate.order]
- GIVEN a survival crafting-table fixture contains all required typed crafting events but puts the server collect phase before the required result phase
- WHEN typed-event validation evaluates `survival-crafting-table`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival crafting-table migration evidence

r[mc_compatibility.survival_crafting_table_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_crafting_table_typed_event_migration.validation.log]
- GIVEN the survival crafting-table row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the survival crafting-table dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Scenario manifest fallback baseline

r[mc_compatibility.scenario_manifest_fallback_budget_gate.baseline] The system MUST record a deterministic baseline for scenario rows that still use substring fallback, including row identity and waiver metadata that names owner, reason, non-claim, and next action.

#### Scenario: Existing fallback rows are accounted for

r[mc_compatibility.scenario_manifest_fallback_budget_gate.baseline.accounted]
- GIVEN the scenario manifest contains existing substring-fallback rows
- WHEN the fallback budget gate evaluates the manifest
- THEN each existing fallback row is either present in the checked baseline with waiver metadata or reported as unapproved fallback debt.

### Requirement: Scenario manifest fallback gate

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate] The system MUST fail closed when the scenario manifest adds unapproved substring fallback rows, removes required waiver metadata, or regresses a typed-event-ready row back to substring fallback.

#### Scenario: New fallback row fails closed

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate.new_fallback]
- GIVEN a scenario manifest adds a row with `migration_state` set to `substring-fallback`
- WHEN the row is not in the approved fallback baseline with complete waiver metadata
- THEN the fallback budget gate fails with a diagnostic naming the row.

#### Scenario: Typed-event regression fails closed

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate.regression]
- GIVEN a scenario row is recorded as typed-event-ready in the baseline
- WHEN the manifest changes that row back to substring fallback
- THEN the fallback budget gate fails with a typed-event regression diagnostic.

#### Scenario: Fallback removal is reported as progress

r[mc_compatibility.scenario_manifest_fallback_budget_gate.gate.removal]
- GIVEN a row is removed from substring fallback by a typed-event migration
- WHEN the fallback budget gate evaluates the manifest
- THEN the gate passes and reports the row as fallback debt removed.

### Requirement: Scenario manifest fallback gate integration

r[mc_compatibility.scenario_manifest_fallback_budget_gate.integration] The fallback budget gate MUST run as part of the focused mc-compat validation surface without changing scenario behavior, wrapper selection, or evidence claims.

#### Scenario: Focused validation includes fallback accounting

r[mc_compatibility.scenario_manifest_fallback_budget_gate.integration.focused]
- GIVEN focused mc-compat validation runs
- WHEN scenario manifest checks execute
- THEN fallback budget accounting runs against the checked-in manifest surfaces and reports approved, removed, new, and regressed rows.

### Requirement: Scenario manifest fallback gate documentation and evidence

r[mc_compatibility.scenario_manifest_fallback_budget_gate.docs] The change MUST document the fallback budget report shape and preserve explicit non-claims for unmigrated rows.

#### Scenario: Documentation keeps fallback rows non-claiming

r[mc_compatibility.scenario_manifest_fallback_budget_gate.docs.non_claiming]
- GIVEN fallback budget documentation is updated
- WHEN reviewers read the fallback report description
- THEN it states that fallback accounting does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, or production readiness.

### Requirement: Scenario manifest fallback gate validation

r[mc_compatibility.scenario_manifest_fallback_budget_gate.validation] The change MUST record reviewable evidence for positive and negative gate fixtures, generated-surface freshness, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.scenario_manifest_fallback_budget_gate.validation.log]
- GIVEN the fallback budget gate is implemented
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative gate fixtures, generated-surface freshness, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival chest persistence typed-event readiness

r[mc_compatibility.survival_chest_persistence_typed_event_migration.readiness] The `survival-chest-persistence` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence chest server milestones, forbidden surfaces, and ordered two-session persistence phases.

#### Scenario: Survival chest persistence row is typed-event-ready

r[mc_compatibility.survival_chest_persistence_typed_event_migration.readiness.complete]
- GIVEN `survival-chest-persistence` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival chest persistence typed-event gate

r[mc_compatibility.survival_chest_persistence_typed_event_migration.gate] The runner MUST include `survival-chest-persistence` in the typed-event pass/fail gate so missing or invalid structured chest events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed chest evidence fails closed

r[mc_compatibility.survival_chest_persistence_typed_event_migration.gate.missing]
- GIVEN a survival chest fixture contains legacy substring-compatible milestones but omits a required persisted-state typed event
- WHEN typed-event validation evaluates `survival-chest-persistence`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered two-session chest phases fail closed

r[mc_compatibility.survival_chest_persistence_typed_event_migration.gate.order]
- GIVEN a survival chest fixture contains all required typed chest events but puts second-session reopen before the required first-session close phase
- WHEN typed-event validation evaluates `survival-chest-persistence`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival chest persistence migration evidence

r[mc_compatibility.survival_chest_persistence_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_chest_persistence_typed_event_migration.validation.log]
- GIVEN the survival chest persistence row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the survival chest dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival furnace persistence typed-event readiness

r[mc_compatibility.survival_furnace_persistence_typed_event_migration.readiness] The `survival-furnace-persistence` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence furnace server milestones, forbidden surfaces, and ordered persistence phases.

#### Scenario: Survival furnace persistence row is typed-event-ready

r[mc_compatibility.survival_furnace_persistence_typed_event_migration.readiness.complete]
- GIVEN `survival-furnace-persistence` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival furnace persistence typed-event gate

r[mc_compatibility.survival_furnace_persistence_typed_event_migration.gate] The runner MUST include `survival-furnace-persistence` in the typed-event pass/fail gate so missing or invalid structured furnace events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed furnace evidence fails closed

r[mc_compatibility.survival_furnace_persistence_typed_event_migration.gate.missing]
- GIVEN a survival furnace fixture contains legacy substring-compatible milestones but omits a required typed output or persisted-state event
- WHEN typed-event validation evaluates `survival-furnace-persistence`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed furnace phases fail closed

r[mc_compatibility.survival_furnace_persistence_typed_event_migration.gate.order]
- GIVEN a survival furnace fixture contains all required typed furnace events but puts reconnect before output collection
- WHEN typed-event validation evaluates `survival-furnace-persistence`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival furnace persistence migration evidence

r[mc_compatibility.survival_furnace_persistence_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_furnace_persistence_typed_event_migration.validation.log]
- GIVEN the survival furnace persistence row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the survival furnace persistence dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival furnace smelting breadth typed-event readiness

r[mc_compatibility.survival_furnace_smelting_breadth_typed_event_migration.readiness] The `survival-furnace-smelting-breadth` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence smelting server milestones, forbidden surfaces, and ordered valid/invalid smelting phases.

#### Scenario: Survival furnace smelting breadth row is typed-event-ready

r[mc_compatibility.survival_furnace_smelting_breadth_typed_event_migration.readiness.complete]
- GIVEN `survival-furnace-smelting-breadth` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival furnace smelting breadth typed-event gate

r[mc_compatibility.survival_furnace_smelting_breadth_typed_event_migration.gate] The runner MUST include `survival-furnace-smelting-breadth` in the typed-event pass/fail gate so missing or invalid structured smelting events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed smelting evidence fails closed

r[mc_compatibility.survival_furnace_smelting_breadth_typed_event_migration.gate.missing]
- GIVEN a survival smelting-breadth fixture contains legacy substring-compatible milestones but omits a required invalid-fuel rejection typed event
- WHEN typed-event validation evaluates `survival-furnace-smelting-breadth`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed smelting phases fail closed

r[mc_compatibility.survival_furnace_smelting_breadth_typed_event_migration.gate.order]
- GIVEN a survival smelting-breadth fixture contains all required typed furnace events but puts output collection before output availability
- WHEN typed-event validation evaluates `survival-furnace-smelting-breadth`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival furnace smelting breadth migration evidence

r[mc_compatibility.survival_furnace_smelting_breadth_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_furnace_smelting_breadth_typed_event_migration.validation.log]
- GIVEN the survival furnace smelting breadth row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the smelting-breadth dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: MCP-controlled smoke typed-event readiness

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.readiness] The `mcp-controlled-smoke` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's MCP initialize, tools/list, status, command outcome, stdout cleanliness, look/input, frame capture, and frame artifact identity evidence.

#### Scenario: MCP-controlled smoke row is typed-event-ready

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.readiness.complete]
- GIVEN `mcp-controlled-smoke` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, frame artifact scope, current-bundle row, and non-claim scope.

### Requirement: MCP-controlled smoke typed-event gate

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.gate] The runner MUST include `mcp-controlled-smoke` in the typed-event pass/fail gate so missing or invalid structured MCP control and frame-artifact evidence fails before substring fallback can satisfy the row.

#### Scenario: Missing MCP frame artifact identity fails closed

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.gate.missing]
- GIVEN an MCP-controlled fixture contains legacy substring-compatible control milestones but omits reviewable frame artifact identity
- WHEN typed-event validation evaluates `mcp-controlled-smoke`
- THEN the fixture fails with a structured diagnostic naming the missing artifact identity event.

#### Scenario: Misordered MCP control phases fail closed

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.gate.order]
- GIVEN an MCP-controlled fixture contains all required typed control events but puts frame capture before the required status, look, and input phases
- WHEN typed-event validation evaluates `mcp-controlled-smoke`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: MCP-controlled smoke migration evidence

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.validation] The migration MUST record reviewable evidence for MCP fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.mcp_controlled_smoke_typed_event_migration.validation.log]
- GIVEN the MCP-controlled smoke row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the MCP dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival crafting recipe breadth typed-event readiness

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.readiness] The `survival-crafting-recipe-breadth` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence recipe-breadth server milestones, forbidden surfaces, and ordered valid/invalid crafting phases.

#### Scenario: Survival crafting recipe breadth row is typed-event-ready

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.readiness.complete]
- GIVEN `survival-crafting-recipe-breadth` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival crafting recipe breadth typed-event gate

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.gate] The runner MUST include `survival-crafting-recipe-breadth` in the typed-event pass/fail gate so missing or invalid structured recipe events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed recipe evidence fails closed

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.gate.missing]
- GIVEN a survival recipe-breadth fixture contains legacy substring-compatible milestones but omits a required invalid-rejection typed event
- WHEN typed-event validation evaluates `survival-crafting-recipe-breadth`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed recipe phases fail closed

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.gate.order]
- GIVEN a survival recipe-breadth fixture contains all required typed recipe events but reorders the shapeless result and grid-clear phases
- WHEN typed-event validation evaluates `survival-crafting-recipe-breadth`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival crafting recipe breadth migration evidence

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_crafting_recipe_breadth_typed_event_migration.validation.log]
- GIVEN the survival crafting recipe breadth row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the recipe-breadth dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival hunger health-cycle typed-event readiness

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.readiness] The `survival-hunger-health-cycle` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence health-cycle server milestones, forbidden surfaces, and ordered consume/recovery phases.

#### Scenario: Survival hunger health-cycle row is typed-event-ready

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.readiness.complete]
- GIVEN `survival-hunger-health-cycle` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN the row includes typed-event-ready receipt expectations and no longer uses substring fallback for pass/fail
- AND the manifest still records the existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival hunger health-cycle typed-event gate

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.gate] The runner MUST include `survival-hunger-health-cycle` in the typed-event pass/fail gate so missing or invalid structured health-cycle events fail before substring fallback can satisfy the row.

#### Scenario: Missing typed health-cycle evidence fails closed

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.gate.missing]
- GIVEN a survival hunger health-cycle fixture contains legacy substring-compatible milestones but omits a required final state typed event
- WHEN typed-event validation evaluates `survival-hunger-health-cycle`
- THEN the fixture fails with a structured diagnostic naming the missing event.

#### Scenario: Misordered typed consume phases fail closed

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.gate.order]
- GIVEN a survival hunger health-cycle fixture contains all required typed consume events but puts inventory decrement before recovery observation
- WHEN typed-event validation evaluates `survival-hunger-health-cycle`
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival hunger health-cycle migration evidence

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run receipt shape, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_hunger_health_cycle_typed_event_migration.validation.log]
- GIVEN the survival hunger health-cycle row is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, the hunger health-cycle dry-run wrapper check, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Remaining survival breadth typed-event readiness

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.readiness] The remaining survival breadth scenarios SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover each row's required client milestones, Valence server milestones, forbidden surfaces, and row-specific ordered phases.

#### Scenario: Remaining survival breadth rows are typed-event-ready

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.readiness.complete]
- GIVEN the remaining survival breadth rows are marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN `survival-mob-ai-loot-breadth`, `survival-redstone-circuit-breadth`, `survival-biome-dimension-travel`, `survival-world-multichunk-durability`, `survival-container-block-entity-breadth`, and `survival-sign-editing-live` include typed-event-ready receipt expectations
- AND the manifest still records each existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Remaining survival breadth typed-event gates

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.gate] The runner MUST include the remaining survival breadth rows in typed-event pass/fail gates so missing or invalid structured row evidence fails before substring fallback can satisfy a row.

#### Scenario: Missing survival breadth typed evidence fails closed

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.gate.missing]
- GIVEN a remaining survival breadth fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Misordered survival breadth phases fail closed

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.gate.order]
- GIVEN a remaining survival breadth fixture contains all required typed events but puts a row postcondition before the required action phase
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Remaining survival breadth migration evidence

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.validation] The migration MUST record reviewable evidence for runner fixtures, scenario-manifest checks, generated-surface freshness, dry-run coverage, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_remaining_breadth_typed_event_migration.validation.log]
- GIVEN the remaining survival breadth rows are migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, historical survival dry-run coverage, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: CTF typed-event migration inventory

r[mc_compatibility.ctf_rule_typed_event_migration_wave.readiness] The system MUST define a row-family inventory for maintained CTF rows before marking any CTF rule row `typed-event-ready`.

#### Scenario: CTF row families are inventoried

r[mc_compatibility.ctf_rule_typed_event_migration_wave.readiness.inventory]
- GIVEN maintained CTF rows are selected for typed-event migration
- WHEN reviewers inspect the migration inventory
- THEN each selected row is assigned to a family with required client milestones, Valence server milestones, forbidden surfaces, actor or state correlation, and non-claim scope.

### Requirement: CTF typed-event family gates

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate] The runner MUST include migrated CTF rows in typed-event pass/fail gates so missing, misordered, or incorrectly correlated structured CTF events fail before substring fallback can satisfy a row.

#### Scenario: Missing CTF typed evidence fails closed

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate.missing]
- GIVEN a CTF fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that CTF row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Incorrect CTF correlation fails closed

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate.correlation]
- GIVEN a CTF fixture contains typed events with the wrong actor, victim, flag, team, or state correlation for the selected row
- WHEN typed-event validation evaluates that CTF row
- THEN the fixture fails with a correlation diagnostic instead of passing through substring fallback.

#### Scenario: Misordered CTF phases fail closed

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate.order]
- GIVEN a CTF fixture contains all required typed events but puts a postcondition before the required action or server transition
- WHEN typed-event validation evaluates that CTF row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: CTF typed-event documentation

r[mc_compatibility.ctf_rule_typed_event_migration_wave.docs] The migration MUST update generated and human documentation to name migrated CTF rows as typed-event-ready while preserving full CTF correctness and broad compatibility non-claims.

#### Scenario: Documentation stays non-overclaiming

r[mc_compatibility.ctf_rule_typed_event_migration_wave.docs.non_claiming]
- GIVEN CTF rows are migrated to typed-event-ready
- WHEN README and evidence docs are inspected
- THEN they state that typed-event migration changes observability and pass/fail only and does not claim full CTF correctness, all races, all invalid actions, adversarial security, public-server safety, production readiness, or vanilla/reference parity.

### Requirement: CTF typed-event migration validation

r[mc_compatibility.ctf_rule_typed_event_migration_wave.validation] The migration MUST record reviewable evidence for row-family fixtures, scenario-manifest checks, generated-surface freshness, dry-run wrappers, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.ctf_rule_typed_event_migration_wave.validation.log]
- GIVEN CTF row families are migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, CTF dry-run wrapper checks, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.

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

### Requirement: Runner JSON evidence inventory

r[mc_compatibility.runner_json_evidence_boundary.inventory] Runner JSON/evidence work MUST inventory manual JSON helpers, receipt schemas, evidence artifact types, consumers, legacy markers, and compatibility-sensitive fields before migration.

#### Scenario: Evidence schemas are known

r[mc_compatibility.runner_json_evidence_boundary.inventory.schemas]
- GIVEN runner JSON/evidence logic is selected for extraction
- WHEN reviewers inspect the inventory
- THEN smoke/scenario receipts, failure bundles, typed-event artifacts, MCP control evidence, frame artifacts, latency/jitter receipts, public-server safety receipts, projectile causality evidence, and compare-receipt inputs are classified
- AND legacy markers and non-claim fields are identified as compatibility-sensitive.

### Requirement: JSON evidence module contract

r[mc_compatibility.runner_json_evidence_boundary.contract] Runner evidence code SHOULD define dedicated module boundaries with typed data structs, parse/render contracts, diagnostics, and dependency policy.

#### Scenario: Schema logic is local

r[mc_compatibility.runner_json_evidence_boundary.contract.local]
- GIVEN a receipt or evidence artifact is parsed or rendered
- WHEN reviewers inspect the implementation
- THEN field names, required/optional status, type expectations, redaction policy, and non-claim defaults are owned by evidence modules
- AND top-level orchestration code does not assemble schema JSON through unrelated string helpers.

### Requirement: Evidence rendering migration

r[mc_compatibility.runner_json_evidence_boundary.migration] In-memory receipt, failure-bundle, typed-event, MCP/frame, latency, public-server safety, and scenario evidence parsing/rendering SHOULD move out of the top-level runner shell.

#### Scenario: Evidence core is in-memory

r[mc_compatibility.runner_json_evidence_boundary.migration.in_memory]
- GIVEN evidence parse/render logic is migrated
- WHEN module tests execute it
- THEN it accepts in-memory typed inputs or JSON text and returns typed evidence, rendered JSON, or diagnostics
- AND it does not read files, create directories, compute file hashes from paths, spawn processes, inspect environment, or print stdout/stderr.

### Requirement: Evidence shell separation

r[mc_compatibility.runner_json_evidence_boundary.shell] Filesystem writes, directory creation, BLAKE3 file hashing, stdout/stderr, and exit-code handling MUST remain in shell code outside evidence parse/render cores.

#### Scenario: Shell owns artifacts

r[mc_compatibility.runner_json_evidence_boundary.shell.artifacts]
- GIVEN the runner writes a receipt, failure bundle, typed-event log, or promoted artifact
- WHEN shell code performs the write
- THEN evidence modules provide validated bytes or typed data and shell code owns paths, file IO, hashing, and user-facing diagnostics
- AND evidence modules do not mutate repository state.

### Requirement: JSON evidence compatibility preservation

r[mc_compatibility.runner_json_evidence_boundary.compatibility] JSON/evidence extraction MUST preserve receipt schema compatibility, legacy markers, non-claim fields, overclaim rejection, and existing validation behavior unless another Cairn changes them.

#### Scenario: Existing receipts remain valid

r[mc_compatibility.runner_json_evidence_boundary.compatibility.valid]
- GIVEN existing valid runner receipts and failure bundles are parsed after extraction
- WHEN receipt validation and compare-receipt checks run
- THEN they accept the same valid inputs and reject the same invalid or overbroad inputs
- AND no new full-compatibility, public-server safety, production-readiness, or vanilla-parity claim appears by default.

### Requirement: JSON evidence tests

r[mc_compatibility.runner_json_evidence_boundary.tests] JSON/evidence extraction MUST include positive schema/render tests and negative malformed-input tests.

#### Scenario: Valid evidence fixtures pass

r[mc_compatibility.runner_json_evidence_boundary.tests.positive]
- GIVEN valid receipts, failure bundles, typed-event artifacts, MCP/frame evidence, and scenario evidence inputs
- WHEN migrated parse/render tests run
- THEN rendered schemas and parsed summaries match the compatibility contract.

#### Scenario: Invalid evidence fixtures fail closed

r[mc_compatibility.runner_json_evidence_boundary.tests.negative]
- GIVEN malformed JSON, bad escaping, missing fields, wrong types, invalid artifact paths, stale revisions, duplicate keys, forbidden overclaims, or unsupported schema values
- WHEN migrated parse/render tests run
- THEN deterministic diagnostics identify the field or artifact
- AND no invalid evidence is accepted as successful compatibility proof.

### Requirement: JSON evidence boundary validation

r[mc_compatibility.runner_json_evidence_boundary.validation] JSON/evidence extraction MUST record runner tests, receipt/failure-bundle checks, selected dry-runs, evidence checks, Cairn gates, and task-evidence checks before archive.

#### Scenario: JSON evidence closeout is reviewable

r[mc_compatibility.runner_json_evidence_boundary.validation.log]
- GIVEN JSON/evidence extraction is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative schema tests, receipt/failure-bundle validation, selected dry-runs, evidence manifest or task-evidence checks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Valence fixture inventory

r[mc_compatibility.valence_fixture_core.inventory] Valence compatibility fixture work MUST inventory CTF and survival fixture responsibilities, rule decisions, Bevy shell code, milestone strings, env toggles, global state, and non-goals before extraction.

#### Scenario: Fixture responsibilities are reviewable

r[mc_compatibility.valence_fixture_core.inventory.reviewable]
- GIVEN `ctf.rs` or `survival_compat.rs` is selected for fixture-core extraction
- WHEN reviewers inspect the inventory
- THEN rule decisions, Bevy systems, resources, env toggles, milestone emitters, global state, and evidence boundaries are classified
- AND non-goals such as production gameplay, full CTF correctness, full survival correctness, and vanilla parity are explicit.

### Requirement: Valence fixture boundaries

r[mc_compatibility.valence_fixture_core.boundaries] Compatibility fixtures SHOULD separate deterministic fixture cores from Bevy ECS adapter systems.

#### Scenario: Adapter owns ECS access

r[mc_compatibility.valence_fixture_core.boundaries.adapter]
- GIVEN a fixture rule is extracted
- WHEN reviewers inspect its boundary
- THEN the pure core consumes explicit snapshots/events and returns decisions, state transitions, or milestone text
- AND Bevy queries, commands, resources, timers, logging, file markers, and world mutation remain in adapter systems.

### Requirement: CTF fixture core

r[mc_compatibility.valence_fixture_core.ctf_core] CTF compatibility fixture behavior SHOULD move deterministic flag, score, race, spawn-reset, inventory, combat, and milestone decisions into testable core functions.

#### Scenario: CTF transition is deterministic

r[mc_compatibility.valence_fixture_core.ctf_core.transition]
- GIVEN explicit CTF state and an observed event such as pickup, return, capture, race attempt, inventory click, or combat hit
- WHEN the CTF fixture core evaluates it
- THEN it returns the documented accepted, rejected, milestone, or no-op transition deterministically
- AND it does not depend on Bevy iteration order or global mutable state.

### Requirement: Survival fixture core

r[mc_compatibility.valence_fixture_core.survival_core] Survival compatibility fixture behavior SHOULD move deterministic block, container, crafting, furnace, hunger, mob, redstone, persistence, block-entity, biome/dimension, and milestone decisions into testable core functions.

#### Scenario: Survival fixture decision is deterministic

r[mc_compatibility.valence_fixture_core.survival_core.decision]
- GIVEN explicit survival fixture state and an observed interaction or packet-derived event
- WHEN the survival fixture core evaluates it
- THEN it returns the documented mutation, rejection, milestone, marker write request, or no-op decision
- AND filesystem marker writes and Bevy world mutations stay in the adapter shell.

### Requirement: Valence fixture state ownership

r[mc_compatibility.valence_fixture_core.state_ownership] Global mutable fixture state SHOULD be replaced by explicit Bevy resources, fixture state structs, or documented temporary compatibility shims.

#### Scenario: Policy state is explicit

r[mc_compatibility.valence_fixture_core.state_ownership.policy]
- GIVEN fixture policy or reload state is needed by a Valence compatibility example
- WHEN the state is read or updated
- THEN ownership is represented by a resource, fixture state value, or explicit input/output boundary
- AND any remaining global state is documented with its safety assumptions and retirement path.

### Requirement: Valence fixture compatibility preservation

r[mc_compatibility.valence_fixture_core.compatibility] Fixture-core extraction MUST preserve example commands, env var contracts, milestone text, scenario behavior, and evidence non-claim boundaries unless another Cairn changes them.

#### Scenario: Fixture evidence remains comparable

r[mc_compatibility.valence_fixture_core.compatibility.stable]
- GIVEN selected mc-compat scenarios run against the extracted fixtures
- WHEN receipts and logs are compared to the pre-extraction contract
- THEN required milestones, forbidden milestones, env toggles, and non-claim fields remain compatible
- AND no default Valence gameplay, production readiness, or vanilla parity claim is added.

### Requirement: Valence fixture tests

r[mc_compatibility.valence_fixture_core.tests] Fixture-core extraction MUST include positive transition tests and negative fail-closed tests for migrated CTF and survival decisions.

#### Scenario: Valid fixture transitions pass

r[mc_compatibility.valence_fixture_core.tests.positive]
- GIVEN valid CTF and survival fixture states and events
- WHEN core tests run
- THEN accepted transitions, expected milestones, and adapter requests match the existing fixture contract.

#### Scenario: Invalid fixture transitions fail closed

r[mc_compatibility.valence_fixture_core.tests.negative]
- GIVEN duplicate wins, duplicate pickups, wrong-team returns, invalid inventory events, missing persistence markers, malformed block-entity state, unsupported policy output, or out-of-order fixture events
- WHEN core tests run
- THEN deterministic rejection/no-op diagnostics are produced
- AND no false success milestone is emitted.

### Requirement: Valence fixture-core validation

r[mc_compatibility.valence_fixture_core.validation] Fixture-core extraction MUST record focused Valence tests/example checks, selected mc-compat rails, Cairn gates, and task-evidence checks before archive.

#### Scenario: Fixture-core closeout is reviewable

r[mc_compatibility.valence_fixture_core.validation.log]
- GIVEN fixture-core extraction is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative fixture-core tests, focused Valence/example checks, selected mc-compat dry-runs or live rails as scoped, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Stevenarella unsafe boundary audit

r[mc_compatibility.stevenarella_runtime_boundaries.audit] Stevenarella runtime-boundary work MUST audit unsafe code, global mutable state, synchronization assumptions, caller invariants, and failure modes before refactoring GL, resources, or ECS internals.

#### Scenario: Unsafe assumptions are explicit

r[mc_compatibility.stevenarella_runtime_boundaries.audit.explicit]
- GIVEN a GL, resource, ECS, render, model, or chunk-builder boundary is selected for hardening
- WHEN reviewers inspect the audit
- THEN each unsafe block, unsafe impl, global mutable state item, and relevant unwrap/panic assumption is classified with ownership, invariant, failure mode, and planned mitigation
- AND unsupported full memory-safety proof claims remain non-claims.

### Requirement: Explicit GL context boundary

r[mc_compatibility.stevenarella_runtime_boundaries.gl_context] Stevenarella rendering SHOULD replace or quarantine the global mutable GL context behind an explicit initialization and context-access boundary.

#### Scenario: GL use is initialization-checked

r[mc_compatibility.stevenarella_runtime_boundaries.gl_context.initialized]
- GIVEN rendering code needs GL access
- WHEN it requests the context through the hardened boundary
- THEN the boundary proves initialization or returns a deterministic diagnostic/panic policy documented for startup-only failure
- AND raw global mutable pointer access is isolated to the smallest possible module or removed.

### Requirement: Resource manager sharing boundary

r[mc_compatibility.stevenarella_runtime_boundaries.resources] Resource manager sharing MUST separate pack IO, reload/version state, progress reporting, and worker-thread access behind safe ownership or synchronization contracts.

#### Scenario: Resource access is synchronized by design

r[mc_compatibility.stevenarella_runtime_boundaries.resources.synchronized]
- GIVEN render, model, UI, or chunk-builder code reads resources while reload/progress work may occur
- WHEN the resource boundary is used
- THEN immutable reads, mutable pack updates, progress updates, and worker communication are represented by explicit synchronized types or ownership handoffs
- AND unsafe `Sync` assumptions are removed or documented as temporary shims with tests and retirement tasks.

### Requirement: ECS unsafe storage boundary

r[mc_compatibility.stevenarella_runtime_boundaries.ecs] ECS raw storage and lifetime-sensitive access MUST be encapsulated behind safe APIs that enforce entity generation, component membership, aliasing, and drop invariants.

#### Scenario: Invalid ECS access fails closed

r[mc_compatibility.stevenarella_runtime_boundaries.ecs.invalid]
- GIVEN an entity key is stale, a component is absent, storage has been removed, or an invalid borrow pattern is attempted through safe APIs
- WHEN ECS accessors are called
- THEN they return `None`, a deterministic diagnostic, or a compile-time rejection according to the API contract
- AND safe callers cannot observe unchecked transmute or raw pointer access.

### Requirement: Stevenarella runtime compatibility preservation

r[mc_compatibility.stevenarella_runtime_boundaries.compatibility] Runtime-boundary hardening MUST preserve observable render/resource/probe behavior and existing mc-compat evidence boundaries unless another Cairn changes them.

#### Scenario: Client rails remain comparable

r[mc_compatibility.stevenarella_runtime_boundaries.compatibility.stable]
- GIVEN a hardened runtime boundary is used by existing render, resource, ECS, or probe paths
- WHEN focused Stevenarella checks or selected mc-compat rails run
- THEN existing user-visible behavior, capture output shape, probe milestones, and receipt fields remain compatible
- AND the change does not claim full client safety, renderer portability, or full protocol compatibility.

### Requirement: Stevenarella runtime-boundary tests

r[mc_compatibility.stevenarella_runtime_boundaries.tests] Runtime-boundary hardening MUST include positive invariant tests and negative fail-closed tests for each hardened boundary.

#### Scenario: Valid runtime invariants pass

r[mc_compatibility.stevenarella_runtime_boundaries.tests.positive]
- GIVEN GL initialization, resource reads, progress updates, entity generation, component membership, and component drop cases satisfy the documented invariants
- WHEN focused tests run
- THEN the hardened boundaries allow existing behavior and preserve data correctly.

#### Scenario: Invalid runtime invariants fail closed

r[mc_compatibility.stevenarella_runtime_boundaries.tests.negative]
- GIVEN GL access is uninitialized, a resource is missing, progress state is invalid, an entity generation is stale, a component is absent, removal/drop ordering is invalid, or a borrow pattern is unsupported
- WHEN boundary tests run
- THEN deterministic diagnostics, `None`, or documented startup failure behavior occurs
- AND no undefined behavior is exposed through safe APIs.

### Requirement: Stevenarella runtime-boundary validation

r[mc_compatibility.stevenarella_runtime_boundaries.validation] Runtime-boundary hardening MUST record focused Stevenarella tests/checks, selected render/capture or mc-compat checks, Cairn gates, and task-evidence checks before archive.

#### Scenario: Runtime-boundary closeout is reviewable

r[mc_compatibility.stevenarella_runtime_boundaries.validation.log]
- GIVEN runtime-boundary hardening is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative boundary tests, focused Stevenarella checks through the mc devshell, selected render/capture or mc-compat evidence as scoped, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Stevenarella server responsibility inventory

r[mc_compatibility.stevenarella_server_probe_split.inventory] Stevenarella server probe work MUST inventory server module responsibilities, probe families, side effects, and shared helpers before extraction.

#### Scenario: Probe ownership is classified

r[mc_compatibility.stevenarella_server_probe_split.inventory.classified]
- GIVEN `clients/stevenarella/src/server/mod.rs` is selected for modularization
- WHEN reviewers inspect the inventory
- THEN general protocol handling, connection state, world/entity updates, inventory/window handling, compatibility probe decisions, environment parsing, and milestone logging are classified
- AND probe families that can move independently are named.

### Requirement: Stevenarella server module boundaries

r[mc_compatibility.stevenarella_server_probe_split.boundaries] The Stevenarella server split SHOULD define module boundaries for general server state, probe cores, probe shells, inventory/window helpers, block-entity/sign helpers, and environment/config inputs.

#### Scenario: Probe modules do not own packet IO

r[mc_compatibility.stevenarella_server_probe_split.boundaries.packet_io]
- GIVEN a probe family is extracted
- WHEN reviewers inspect the module boundary
- THEN pure probe logic is separate from packet decoding/encoding and connection mutation
- AND shell handlers remain responsible for interacting with the protocol connection and world state.

### Requirement: Pure Stevenarella probe cores

r[mc_compatibility.stevenarella_server_probe_split.pure_probes] Compatibility probe decisions MUST be pure deterministic state machines over explicit inputs wherever practical.

#### Scenario: Probe core returns actions

r[mc_compatibility.stevenarella_server_probe_split.pure_probes.actions]
- GIVEN a probe core receives explicit tick, session, inventory, world, entity, dimension, or packet-observation inputs
- WHEN it evaluates the next probe step
- THEN it returns an action, expected milestone, state update, or no-op
- AND it does not read environment variables, write logs, mutate global state, write packets, or access renderer/resources.

### Requirement: Stevenarella probe shell wiring

r[mc_compatibility.stevenarella_server_probe_split.shell_wiring] Packet-handler shells MUST call probe cores and translate returned actions into existing packet writes, state mutations, and milestone logs.

#### Scenario: Shell remains thin

r[mc_compatibility.stevenarella_server_probe_split.shell_wiring.thin]
- GIVEN a packet handler drives a compatibility probe
- WHEN probe shell wiring runs
- THEN raw packet/world inputs are converted into core inputs, core outputs are applied, and diagnostics are logged as before
- AND scenario-specific decision tables are not duplicated in the shell.

### Requirement: Stevenarella probe compatibility preservation

r[mc_compatibility.stevenarella_server_probe_split.compatibility] The server probe split MUST preserve env var names, milestone text, fixture constants, packet action order, and evidence non-claim boundaries unless another Cairn changes them.

#### Scenario: Existing rail behavior stays stable

r[mc_compatibility.stevenarella_server_probe_split.compatibility.stable]
- GIVEN an existing mc-compat scenario drives Stevenarella after the split
- WHEN selected dry-run or focused rail checks run
- THEN the same probe env vars are honored, the same client actions are sent, and the same milestone text appears
- AND no full-client, full-survival, full-CTF, or public-server claim is added.

### Requirement: Stevenarella probe tests

r[mc_compatibility.stevenarella_server_probe_split.tests] The probe split MUST include positive action tests and negative fail-closed tests for each migrated probe family.

#### Scenario: Valid probe fixtures pass

r[mc_compatibility.stevenarella_server_probe_split.tests.positive]
- GIVEN valid probe state, inventory/window state, packet observations, session values, and fixture positions
- WHEN migrated probe cores are tested
- THEN the expected actions and milestones match pre-split behavior.

#### Scenario: Invalid probe fixtures fail closed

r[mc_compatibility.stevenarella_server_probe_split.tests.negative]
- GIVEN malformed env/config input, missing fixture state, out-of-order packets, invalid window IDs, stale sign/block-entity data, missing dimension bounds, or server rejection evidence
- WHEN migrated probe cores and shells are tested
- THEN diagnostics or no-op/rejected outcomes are produced deterministically
- AND no false success milestone is emitted.

### Requirement: Stevenarella probe split validation

r[mc_compatibility.stevenarella_server_probe_split.validation] Stevenarella probe split work MUST record focused Stevenarella tests, selected mc-compat checks, Cairn gates, and task-evidence checks before archive.

#### Scenario: Probe split closeout is reviewable

r[mc_compatibility.stevenarella_server_probe_split.validation.log]
- GIVEN the Stevenarella server probe split is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative probe tests, selected component checks through the mc devshell, selected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.

### Requirement: Stevenarella server module boundaries

r[mc_compatibility.stevenarella_modularity.server_boundaries] Stevenarella server session code SHOULD be split into cohesive modules for session lifecycle, packet dispatch, world and dimension state, chunks, entities, inventory/window behavior, plugin messages, and compat-probe shells.

#### Scenario: Packet-family ownership is explicit

r[mc_compatibility.stevenarella_modularity.server_boundaries.ownership]
- GIVEN a server packet or session responsibility is reviewed
- WHEN maintainers inspect the Stevenarella server module tree
- THEN the responsibility is owned by a focused module rather than by unrelated code in the root server module
- AND the root server module exposes only the shell API needed to coordinate those modules.

### Requirement: Stevenarella handler functional cores

r[mc_compatibility.stevenarella_modularity.handler_functional_core] Non-trivial Stevenarella packet-handler decisions SHOULD live in pure deterministic cores that take explicit inputs and return decisions or state updates for the shell to apply.

#### Scenario: Handler decision is testable without a live session

r[mc_compatibility.stevenarella_modularity.handler_functional_core.testable]
- GIVEN handler logic decides how to update world, dimension, entity, inventory, plugin-message, or compat-probe state
- WHEN that logic is extracted
- THEN the decision core can be tested with in-memory inputs
- AND connection I/O, packet writes, ECS/world mutation, and logging remain in the imperative shell.

### Requirement: Stevenarella server modularization parity

r[mc_compatibility.stevenarella_modularity.server_parity] Stevenarella server modularization MUST preserve the public `Server` API, protocol behavior, compat milestone vocabulary, typed-event hooks, MCP/control boundaries, and evidence non-claims.

#### Scenario: Existing client behavior remains stable

r[mc_compatibility.stevenarella_modularity.server_parity.stable]
- GIVEN a supported pre-refactor Stevenarella server session or mc-compat probe input
- WHEN the modularized server session processes the same input
- THEN packet handling, milestone output, typed-event hooks, and non-claim boundaries remain equivalent
- AND the refactor does not promote new compatibility evidence.

### Requirement: Stevenarella server positive tests

r[mc_compatibility.stevenarella_modularity.server_positive_tests] The change MUST include positive tests for representative packet-family decisions, dispatch routing, dimension/world updates, inventory/window behavior, plugin-message routing, and compat-probe scheduling.

#### Scenario: Supported handler paths pass

r[mc_compatibility.stevenarella_modularity.server_positive_tests.coverage]
- GIVEN representative supported packet and probe inputs
- WHEN extracted server cores or routing helpers process them
- THEN tests prove they produce the expected decisions, updates, or shell actions.

### Requirement: Stevenarella server negative tests

r[mc_compatibility.stevenarella_modularity.server_negative_tests] The change MUST include negative tests for unsupported packets, malformed state summaries, invalid inventory/window actions, unknown plugin channels, missing dimension data, and disabled compat probes.

#### Scenario: Invalid handler paths fail closed

r[mc_compatibility.stevenarella_modularity.server_negative_tests.fail_closed]
- GIVEN invalid packet, state, inventory, plugin-message, dimension, or probe inputs
- WHEN extracted server cores or routing helpers process them
- THEN tests prove the inputs are rejected, ignored, or contained according to the existing behavior without panicking or corrupting state.

### Requirement: Stevenarella server modularization validation

r[mc_compatibility.stevenarella_modularity.server_validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_modularity.server_validation.logs]
- GIVEN Stevenarella server modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative module tests plus affected dry-runs and Cairn gates passing.

### Requirement: Stevenarella UI widget boundaries

r[mc_compatibility.stevenarella_ui.widget_boundaries] Stevenarella UI code SHOULD expose cohesive boundaries for layout regions, containers, image and batch elements, text and formatted text, buttons, text boxes, and input or focus helpers.

#### Scenario: UI responsibility has one owner

r[mc_compatibility.stevenarella_ui.widget_boundaries.ownership]
- GIVEN a UI widget responsibility is reviewed
- WHEN maintainers inspect UI modules
- THEN the responsibility is owned by a focused module
- AND unrelated layout, rendering, text, button, textbox, and input concerns are not reintroduced into one module.

### Requirement: Stevenarella UI widget core

r[mc_compatibility.stevenarella_ui.widget_core] UI layout, attachment, focus, text-formatting, button-state, and textbox-edit decisions SHOULD be pure over explicit inputs.

#### Scenario: Widget decision is testable without renderer

r[mc_compatibility.stevenarella_ui.widget_core.testable]
- GIVEN widget state, layout bounds, text, or input summaries
- WHEN the widget core processes them
- THEN the decision can be tested without renderer, clipboard, resource, or window side effects.

### Requirement: Stevenarella UI parity

r[mc_compatibility.stevenarella_ui.parity] UI modularization MUST preserve public UI builders, widget behavior, text formatting, input semantics, renderer boundaries, and evidence non-claims.

#### Scenario: UI behavior remains stable

r[mc_compatibility.stevenarella_ui.parity.stable]
- GIVEN a supported pre-refactor UI input or builder use
- WHEN the modularized UI code processes the same input
- THEN layout, text, widget state, and input behavior remain equivalent.

### Requirement: Stevenarella UI positive tests

r[mc_compatibility.stevenarella_ui.positive_tests] The change MUST include positive tests for layout regions, attachment calculations, formatted text, button state, textbox editing, focus changes, and container traversal.

#### Scenario: Supported UI paths pass

r[mc_compatibility.stevenarella_ui.positive_tests.coverage]
- GIVEN representative supported UI inputs
- WHEN extracted UI cores process them
- THEN tests prove the expected layout, text, focus, or widget decisions are produced.

### Requirement: Stevenarella UI negative tests

r[mc_compatibility.stevenarella_ui.negative_tests] The change MUST include negative tests for invalid layout bounds, empty text, malformed formatting, disabled widgets, invalid cursor positions, focus loss, and unsupported clipboard paths.

#### Scenario: Invalid UI paths fail closed

r[mc_compatibility.stevenarella_ui.negative_tests.fail_closed]
- GIVEN invalid UI inputs
- WHEN extracted UI cores process them
- THEN tests prove the inputs are rejected, ignored, clamped, or contained according to current behavior.

### Requirement: Stevenarella UI validation

r[mc_compatibility.stevenarella_ui.validation] The change MUST record focused Stevenarella UI tests, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_ui.validation.logs]
- GIVEN UI modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative UI tests plus Cairn gates passing.

### Requirement: Survival hunger/food typed-event readiness

r[mc_compatibility.survival_hunger_food_typed_event_migration.readiness] The `survival-hunger-food` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence server milestones, forbidden surfaces, and ordered phases.

#### Scenario: Survival hunger/food row is typed-event-ready

r[mc_compatibility.survival_hunger_food_typed_event_migration.readiness.complete]
- GIVEN `survival-hunger-food` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN `survival-hunger-food` includes the `typed-event-ready` receipt expectation
- AND the manifest still records its existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival hunger/food typed-event gate

r[mc_compatibility.survival_hunger_food_typed_event_migration.gate] The runner MUST include `survival-hunger-food` in typed-event pass/fail gates so missing or invalid structured row evidence fails before substring fallback can satisfy the row.

#### Scenario: Missing hunger/food typed evidence fails closed

r[mc_compatibility.survival_hunger_food_typed_event_migration.gate.missing]
- GIVEN a `survival-hunger-food` fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Misordered hunger/food phases fail closed

r[mc_compatibility.survival_hunger_food_typed_event_migration.gate.order]
- GIVEN a `survival-hunger-food` fixture contains all required typed events but puts a post-consumption or inventory event before its prerequisite phase
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival hunger/food migration evidence

r[mc_compatibility.survival_hunger_food_typed_event_migration.validation] The migration MUST record reviewable evidence for runner typed-event fixtures, scenario-manifest checks, generated-surface freshness, fallback accounting, Cairn gates, task evidence, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_hunger_food_typed_event_migration.validation.log]
- GIVEN `survival-hunger-food` is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, task-evidence validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival mob-drop typed-event readiness

r[mc_compatibility.survival_mob_drop_typed_event_migration.readiness] The `survival-mob-drop` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence server milestones, forbidden surfaces, and ordered phases.

#### Scenario: Survival mob-drop row is typed-event-ready

r[mc_compatibility.survival_mob_drop_typed_event_migration.readiness.complete]
- GIVEN `survival-mob-drop` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN `survival-mob-drop` includes the `typed-event-ready` receipt expectation
- AND the manifest still records its existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival mob-drop typed-event gate

r[mc_compatibility.survival_mob_drop_typed_event_migration.gate] The runner MUST include `survival-mob-drop` in typed-event pass/fail gates so missing or invalid structured row evidence fails before substring fallback can satisfy the row.

#### Scenario: Missing mob-drop typed evidence fails closed

r[mc_compatibility.survival_mob_drop_typed_event_migration.gate.missing]
- GIVEN a `survival-mob-drop` fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Misordered mob-drop phases fail closed

r[mc_compatibility.survival_mob_drop_typed_event_migration.gate.order]
- GIVEN a `survival-mob-drop` fixture contains all required typed events but puts a drop, pickup, or inventory event before its prerequisite phase
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival mob-drop migration evidence

r[mc_compatibility.survival_mob_drop_typed_event_migration.validation] The migration MUST record reviewable evidence for runner typed-event fixtures, scenario-manifest checks, generated-surface freshness, fallback accounting, Cairn gates, task evidence, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_mob_drop_typed_event_migration.validation.log]
- GIVEN `survival-mob-drop` is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, task-evidence validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Survival redstone-toggle typed-event readiness

r[mc_compatibility.survival_redstone_toggle_typed_event_migration.readiness] The `survival-redstone-toggle` scenario SHOULD move from waiver-backed substring fallback to `typed-event-ready` only when typed events cover the row's required client milestones, Valence server milestones, forbidden surfaces, and ordered phases.

#### Scenario: Survival redstone-toggle row is typed-event-ready

r[mc_compatibility.survival_redstone_toggle_typed_event_migration.readiness.complete]
- GIVEN `survival-redstone-toggle` is marked `typed-event-ready`
- WHEN the scenario manifest and generated runner surfaces are inspected
- THEN `survival-redstone-toggle` includes the `typed-event-ready` receipt expectation
- AND the manifest still records its existing wrapper, dry-run check, current-bundle row, and non-claim scope.

### Requirement: Survival redstone-toggle typed-event gate

r[mc_compatibility.survival_redstone_toggle_typed_event_migration.gate] The runner MUST include `survival-redstone-toggle` in typed-event pass/fail gates so missing or invalid structured row evidence fails before substring fallback can satisfy the row.

#### Scenario: Missing redstone-toggle typed evidence fails closed

r[mc_compatibility.survival_redstone_toggle_typed_event_migration.gate.missing]
- GIVEN a `survival-redstone-toggle` fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Misordered redstone-toggle phases fail closed

r[mc_compatibility.survival_redstone_toggle_typed_event_migration.gate.order]
- GIVEN a `survival-redstone-toggle` fixture contains all required typed events but puts a return or powered-off event before its prerequisite phase
- WHEN typed-event validation evaluates that row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: Survival redstone-toggle migration evidence

r[mc_compatibility.survival_redstone_toggle_typed_event_migration.validation] The migration MUST record reviewable evidence for runner typed-event fixtures, scenario-manifest checks, generated-surface freshness, fallback accounting, Cairn gates, task evidence, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.survival_redstone_toggle_typed_event_migration.validation.log]
- GIVEN `survival-redstone-toggle` is migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, task-evidence validation, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Paired-reference dry-run inventory

r[mc_compatibility.paired_reference_dry_run_shapes.inventory] The change MUST inventory the current paired-reference scenario manifest entries, live comparator receipt fields, and existing dry-run exclusion rationale before adding shape coverage.

#### Scenario: Dry-run gap is reviewable

r[mc_compatibility.paired_reference_dry_run_shapes.inventory.reviewable]
- GIVEN paired-reference combat scenarios lack deterministic dry-run shape checks
- WHEN reviewers inspect the inventory
- THEN it names the affected scenarios, required live comparator fields, current exclusion rationale, and non-claim boundary.

### Requirement: Deterministic dry-run shape contract

r[mc_compatibility.paired_reference_dry_run_shapes.contract] The selected paired-reference scenarios MUST have deterministic dry-run receipt shapes that record scenario identity, reference backend label, Valence backend label, metric names, tolerance fields, comparison-status placeholder, and dry-run non-claims.

#### Scenario: Shape uses placeholders instead of live evidence

r[mc_compatibility.paired_reference_dry_run_shapes.contract.placeholders]
- GIVEN a selected paired-reference scenario runs in dry-run mode
- WHEN the receipt shape is written
- THEN it uses deterministic placeholder values for backend evidence and source revisions
- AND it does not assert live metric equality or exact vanilla parity.

### Requirement: Pure shape validation

r[mc_compatibility.paired_reference_dry_run_shapes.validation_core] Dry-run shape validation MUST be a pure deterministic core over normalized receipt inputs and MUST include positive and negative fixtures for the paired-reference scenarios.

#### Scenario: Valid paired-reference shapes pass

r[mc_compatibility.paired_reference_dry_run_shapes.validation_core.positive]
- GIVEN a dry-run receipt contains the required reference, Valence, metric, tolerance, scenario, and non-claim fields
- WHEN the shape validator evaluates it
- THEN validation passes with stable diagnostics.

#### Scenario: Weak paired-reference shape fails

r[mc_compatibility.paired_reference_dry_run_shapes.validation_core.negative]
- GIVEN a dry-run receipt lacks reference fields, Valence fields, tolerance fields, allowed backend labels, or dry-run non-claim text
- WHEN the shape validator evaluates it
- THEN validation fails and names the missing or invalid field.

### Requirement: Scenario manifest integration

r[mc_compatibility.paired_reference_dry_run_shapes.integration] Scenario manifest dry-run metadata and generated surfaces MUST expose the new dry-run checks without changing live comparator promotion rules.

#### Scenario: Generated index distinguishes shape from parity

r[mc_compatibility.paired_reference_dry_run_shapes.integration.generated]
- GIVEN generated scenario surfaces are refreshed
- WHEN reviewers inspect the paired-reference rows
- THEN the rows name the deterministic dry-run shape checks
- AND they continue to state that live paired comparator evidence is required for parity promotion.

### Requirement: Dry-run documentation and non-claims

r[mc_compatibility.paired_reference_dry_run_shapes.docs] Evidence docs MUST state that paired-reference dry-run shape coverage is not live Paper/Valence evidence and does not promote vanilla parity.

#### Scenario: Dry-run non-claims are explicit

r[mc_compatibility.paired_reference_dry_run_shapes.docs.nonclaims]
- GIVEN dry-run shape evidence is cited
- WHEN reviewers read the evidence docs
- THEN exact vanilla parity, full combat parity, public-server safety, production readiness, and live comparator success remain explicit non-claims.

### Requirement: Paired-reference dry-run closeout

r[mc_compatibility.paired_reference_dry_run_shapes.closeout] The change MUST record reviewable logs for dry-run shape fixtures, scenario manifest checks, generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.paired_reference_dry_run_shapes.closeout.log]
- GIVEN paired-reference dry-run shapes are implemented
- WHEN reviewers inspect task evidence
- THEN logs show positive and negative shape fixtures, scenario manifest checks, generated-surface freshness, evidence manifest validation, task-evidence validation, Cairn gates, and Cairn validation.

### Requirement: Restart persistence fallback inventory

r[mc_compatibility.restart_persistence_typed_events.inventory] The change MUST inventory selected restart persistence rows, current substring fallback behavior, existing evidence, session count, and non-claim boundaries before migration.

#### Scenario: Selected rows are scoped

r[mc_compatibility.restart_persistence_typed_events.inventory.scoped]
- GIVEN restart persistence typed-event work begins
- WHEN reviewers inspect the inventory
- THEN it names the selected scenarios, current fallback evidence, required sessions, existing receipts, and explicit non-claims.

### Requirement: Restart persistence typed contract

r[mc_compatibility.restart_persistence_typed_events.contract] Selected restart persistence rows MUST define typed milestones for pre-boundary mutation, restart or crash boundary, reconnect when required, post-boundary client observation, and server restored state.

#### Scenario: Boundary sequence is explicit

r[mc_compatibility.restart_persistence_typed_events.contract.sequence]
- GIVEN a selected restart persistence scenario runs
- WHEN its receipt is evaluated
- THEN the typed milestones identify the pre-boundary mutation, boundary event, reconnect requirement, post-boundary observation, and restored server state
- AND the receipt keeps arbitrary durability and full survival compatibility as non-claims.

### Requirement: Pure restart persistence validator

r[mc_compatibility.restart_persistence_typed_events.validator] Restart persistence validation MUST be a pure deterministic core over normalized receipt events and MUST fail closed for missing, unordered, duplicate, mismatched, or stale milestones.

#### Scenario: Complete restart persistence receipt passes

r[mc_compatibility.restart_persistence_typed_events.validator.positive]
- GIVEN a selected scenario receipt contains complete ordered client and server milestones with matching restored state
- WHEN the validator evaluates the receipt
- THEN validation passes with stable diagnostics.

#### Scenario: Weak restart persistence receipt fails

r[mc_compatibility.restart_persistence_typed_events.validator.negative]
- GIVEN a selected scenario receipt is missing the boundary milestone, reconnect milestone, post-boundary observation, or matching restored server state
- WHEN the validator evaluates the receipt
- THEN validation fails and names the missing or mismatched milestone.

### Requirement: Restart persistence typed wiring

r[mc_compatibility.restart_persistence_typed_events.wiring] Runner, client, and server fixture shells MUST emit the selected typed milestones without changing maintained wrapper names, scenario names, or bounded live claims.

#### Scenario: Substring fallback is no longer required

r[mc_compatibility.restart_persistence_typed_events.wiring.no_substring]
- GIVEN typed milestone emission is wired for a selected row
- WHEN the scenario validation runs
- THEN promotion checks depend on typed receipt fields rather than substring log matching
- AND raw logs remain review evidence only.

### Requirement: Restart persistence manifest migration

r[mc_compatibility.restart_persistence_typed_events.manifest] Scenario manifest migration states, fallback budget baseline, and generated surfaces MUST update only after typed validation for the selected rows passes.

#### Scenario: Manifest reflects typed readiness

r[mc_compatibility.restart_persistence_typed_events.manifest.ready]
- GIVEN typed validation passes for a selected row
- WHEN generated scenario surfaces are refreshed
- THEN the row is marked typed-event-ready and removed from approved fallback debt
- AND non-claim fields remain visible.

### Requirement: Restart persistence closeout

r[mc_compatibility.restart_persistence_typed_events.closeout] The change MUST record focused receipt checks, scenario manifest checks, generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.restart_persistence_typed_events.closeout.log]
- GIVEN selected restart persistence rows have migrated
- WHEN reviewers inspect task evidence
- THEN logs show positive and negative typed validation fixtures, focused scenario checks, scenario manifest checks, generated-surface freshness, evidence manifest validation, task-evidence validation, Cairn gates, and Cairn validation.

### Requirement: Fallback ratchet inventory

r[mc_compatibility.scenario_fallback_budget_ratchet.inventory] The system MUST inventory current scenario manifest migration states against the checked fallback-budget baseline before changing approved fallback rows.

#### Scenario: Stale fallback approvals are visible

r[mc_compatibility.scenario_fallback_budget_ratchet.inventory.visible]
- GIVEN the scenario manifest and fallback baseline are evaluated together
- WHEN a baseline fallback entry no longer corresponds to a current substring-fallback row
- THEN the inventory names the row as migrated or stale fallback approval
- AND it keeps current unapproved fallback rows separate from migrated rows.

### Requirement: Ratcheted fallback baseline

r[mc_compatibility.scenario_fallback_budget_ratchet.baseline] The fallback-budget baseline MUST approve only current substring-fallback rows with complete owner, reason, non-claim, and next-action metadata, while preserving regression protection for rows that have migrated to typed-event-ready.

#### Scenario: Migrated rows leave the approved fallback list

r[mc_compatibility.scenario_fallback_budget_ratchet.baseline.migrated]
- GIVEN a row is typed-event-ready in the current manifest
- WHEN the fallback baseline is ratcheted
- THEN the row is not listed as an approved fallback row
- AND future movement back to substring fallback remains a fail-closed regression unless explicitly re-waived.

### Requirement: Ratchet gate behavior

r[mc_compatibility.scenario_fallback_budget_ratchet.gate] The scenario manifest checker MUST fail closed for unapproved current fallback rows, incomplete waiver metadata, and typed-event-ready regression after the ratchet.

#### Scenario: Removed fallback row cannot silently return

r[mc_compatibility.scenario_fallback_budget_ratchet.gate.removed_returns]
- GIVEN a migrated row was removed from the approved fallback list
- WHEN the manifest changes that row back to substring fallback without a new complete waiver
- THEN the checker fails with a diagnostic naming the row.

#### Scenario: Current fallback waiver remains required

r[mc_compatibility.scenario_fallback_budget_ratchet.gate.waiver]
- GIVEN a current fallback row remains approved
- WHEN the baseline lacks owner, reason, non-claim, or next-action metadata for that row
- THEN the checker fails with a missing-waiver diagnostic.

### Requirement: Ratchet generated surfaces and docs

r[mc_compatibility.scenario_fallback_budget_ratchet.surfaces] Generated scenario surfaces and fallback-budget documentation MUST be refreshed so reviewers can see the current fallback set and the non-claim boundary.

#### Scenario: Documentation describes accounting only

r[mc_compatibility.scenario_fallback_budget_ratchet.surfaces.docs]
- GIVEN fallback-budget docs or generated indexes are reviewed
- WHEN the ratcheted fallback set is displayed
- THEN the docs state that fallback accounting does not prove typed-event coverage, live compatibility, semantic equivalence, public-server safety, or production readiness.

### Requirement: Ratchet validation evidence

r[mc_compatibility.scenario_fallback_budget_ratchet.validation] The change MUST record reviewable evidence for checker fixtures, generated-surface freshness, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.scenario_fallback_budget_ratchet.validation.log]
- GIVEN the fallback budget has been ratcheted
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative checker fixtures, generated-surface freshness, evidence manifest validation, Cairn proposal/design/tasks gates, task-evidence validation, and Cairn validation.

### Requirement: Env patch core

r[mc_compatibility.runner_modularity.env_patch_core] Runner environment derivation SHOULD return deterministic `EnvPatch` data before any process `Command` is mutated.

#### Scenario: Env derivation is pure

r[mc_compatibility.runner_modularity.env_patch_core.pure]
- GIVEN scenario, backend, client index, session, and runtime config inputs
- WHEN environment derivation runs
- THEN it returns env patch data without mutating a process command
- AND the patch records enough source context for diagnostics.

### Requirement: Env patch shell application

r[mc_compatibility.runner_modularity.env_patch_shell] The runner shell MUST apply validated env patches to `Command` instances without duplicating scenario env policy.

#### Scenario: Shell applies computed patch

r[mc_compatibility.runner_modularity.env_patch_shell.apply]
- GIVEN an env patch has been computed and validated
- WHEN the client or backend shell prepares a process command
- THEN it applies the patch entries to the command
- AND it does not recompute scenario env policy in the shell.

### Requirement: Env patch positive tests

r[mc_compatibility.runner_modularity.env_patch_positive_tests] The change MUST include positive tests for env patch composition and representative env output across inventory, survival, combat, projectile, CTF, reconnect, and MCP scenarios.

#### Scenario: Supported env patches compose

r[mc_compatibility.runner_modularity.env_patch_positive_tests.coverage]
- GIVEN supported scenario env fragments
- WHEN patches are composed for representative clients and backends
- THEN the resulting env map contains the expected keys and values.

### Requirement: Env patch negative tests

r[mc_compatibility.runner_modularity.env_patch_negative_tests] The change MUST include negative tests for conflicting keys, malformed keys, missing required session values, and backend-incompatible env fragments.

#### Scenario: Invalid env composition fails closed

r[mc_compatibility.runner_modularity.env_patch_negative_tests.fail_closed]
- GIVEN env fragments conflict or omit required inputs
- WHEN patch composition runs
- THEN it returns an actionable diagnostic before process launch.

### Requirement: Env patch validation

r[mc_compatibility.runner_modularity.env_patch_validation] The change MUST record runner tests, env-patch checks, dry-run smoke checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.env_patch_validation.logs]
- GIVEN env patch extraction is complete
- WHEN the change is closed
- THEN reviewable logs show env parity, positive and negative fixtures, dry-run smoke checks, and Cairn validation passing.

### Requirement: Explicit production imports

r[mc_compatibility.runner_modularity.explicit_imports] Production mc-compat runner modules MUST use explicit imports from owning modules instead of broad root wildcard imports.

#### Scenario: Production dependency is visible

r[mc_compatibility.runner_modularity.explicit_imports.visible]
- GIVEN a production runner module depends on a type, function, or constant
- WHEN the module imports that dependency
- THEN the import names the owning module explicitly
- AND production code does not rely on `use super::*` to reach root-owned symbols.

### Requirement: Shared type ownership

r[mc_compatibility.runner_modularity.shared_type_ownership] Shared runner data types SHOULD live in modules that own their responsibility rather than remaining in the root entrypoint solely to satisfy broad imports.

#### Scenario: Type home matches responsibility

r[mc_compatibility.runner_modularity.shared_type_ownership.home]
- GIVEN a shared runner type is used by multiple modules
- WHEN its owner is selected
- THEN the type is defined with the module that owns its responsibility
- AND consumers import it through that owner module.

### Requirement: Import-boundary positive tests

r[mc_compatibility.runner_modularity.import_boundary_positive_tests] The change MUST include positive coverage proving explicit production imports and scoped test imports are accepted.

#### Scenario: Allowed imports pass

r[mc_compatibility.runner_modularity.import_boundary_positive_tests.accepts]
- GIVEN production modules use explicit imports and test modules use scoped local imports
- WHEN the import-boundary check or focused tests run
- THEN the allowed import forms pass.

### Requirement: Import-boundary negative tests

r[mc_compatibility.runner_modularity.import_boundary_negative_tests] The change MUST include negative coverage proving production `use super::*` regressions are rejected.

#### Scenario: Wildcard production import fails

r[mc_compatibility.runner_modularity.import_boundary_negative_tests.rejects]
- GIVEN a production runner module reintroduces a root wildcard import
- WHEN the import-boundary check or focused test fixture runs
- THEN the regression is rejected with an actionable diagnostic.

### Requirement: Import-boundary validation

r[mc_compatibility.runner_modularity.import_boundary_validation] The change MUST record runner tests, import-boundary checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.import_boundary_validation.logs]
- GIVEN import cleanup is complete
- WHEN the change is closed
- THEN reviewable logs show runner behavior preserved and production wildcard imports mechanically guarded.

### Requirement: Module test colocation

r[mc_compatibility.runner_modularity.module_test_colocation] Runner unit tests SHOULD live beside the module that owns the behavior under test instead of accumulating in the root entrypoint.

#### Scenario: Unit test documents owner

r[mc_compatibility.runner_modularity.module_test_colocation.owner]
- GIVEN a unit test validates config, planning, wire, layout, receipt, evidence, scenario, or client-driver behavior
- WHEN tests are organized after the move
- THEN the test is located with the module that owns that behavior
- AND the root entrypoint no longer owns unrelated unit test families.

### Requirement: Shared test support

r[mc_compatibility.runner_modularity.test_support] Shared runner test fixtures SHOULD live in deterministic test-support helpers with explicit inputs and no hidden global environment mutation.

#### Scenario: Shared fixture is deterministic

r[mc_compatibility.runner_modularity.test_support.deterministic]
- GIVEN multiple module tests need the same fixture
- WHEN the fixture helper is invoked
- THEN it derives its output from explicit inputs
- AND it does not rely on hidden process environment state.

### Requirement: Integration test boundary

r[mc_compatibility.runner_modularity.integration_test_boundary] Cross-module runner tests MAY remain at crate-root or integration-test scope when they validate behavior that intentionally spans multiple owner modules.

#### Scenario: Integration test names boundary

r[mc_compatibility.runner_modularity.integration_test_boundary.cross_module]
- GIVEN a test validates behavior across config, planning, execution, receipts, and evidence
- WHEN the test remains outside a single owner module
- THEN the test setup names the cross-module boundary it covers
- AND unit-level assertions remain in owner modules where practical.

### Requirement: Module-test positive coverage

r[mc_compatibility.runner_modularity.module_test_positive_coverage] The move MUST preserve or add positive tests for every moved module family.

#### Scenario: Positive coverage remains visible

r[mc_compatibility.runner_modularity.module_test_positive_coverage.visible]
- GIVEN a module test family moves from the root entrypoint
- WHEN tests run after the move
- THEN the module still has happy-path coverage for its primary contract.

### Requirement: Module-test negative coverage

r[mc_compatibility.runner_modularity.module_test_negative_coverage] The move MUST preserve or add negative tests for every moved module family, including invalid config, malformed receipts, missing evidence, bad wire data, unsafe paths, and scenario validation failures.

#### Scenario: Negative coverage remains visible

r[mc_compatibility.runner_modularity.module_test_negative_coverage.visible]
- GIVEN a module test family moves from the root entrypoint
- WHEN tests run after the move
- THEN the module still has fail-closed coverage for malformed or unsafe inputs.

### Requirement: Module-test validation

r[mc_compatibility.runner_modularity.module_test_validation] The change MUST record runner tests, integration smoke tests, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.module_test_validation.logs]
- GIVEN module-test colocation is complete
- WHEN the change is closed
- THEN reviewable logs show moved positive and negative coverage plus Cairn validation passing.

### Requirement: Runner entrypoint boundary

r[mc_compatibility.runner_modularity.entrypoint_boundary] The mc-compat runner MUST keep its entrypoint thin by limiting `main.rs` to module wiring, process exit translation, and delegation into focused runner modules.

#### Scenario: Entrypoint delegates behavior

r[mc_compatibility.runner_modularity.entrypoint_boundary.delegates]
- GIVEN the runner starts from `main.rs`
- WHEN configuration, mode dispatch, backend lifecycle, scenario behavior, planning, receipt writing, or failure-bundle behavior is needed
- THEN `main.rs` delegates that behavior to an owning module
- AND `main.rs` does not own non-trivial policy logic for those responsibilities.

### Requirement: Runner module ownership

r[mc_compatibility.runner_modularity.entrypoint_modules] Extracted runner modules SHOULD have cohesive ownership boundaries and narrow public APIs for config, app dispatch, backend runtime, scenario behavior, planning, receipts, and failure bundles.

#### Scenario: Module owns one responsibility family

r[mc_compatibility.runner_modularity.entrypoint_modules.ownership]
- GIVEN a runner responsibility is extracted from `main.rs`
- WHEN the new module API is reviewed
- THEN the module exposes only the types and functions needed by neighboring modules
- AND side-effecting shell code remains separate from pure planning or evidence logic.

### Requirement: Entrypoint extraction preserves public behavior

r[mc_compatibility.runner_modularity.entrypoint_parity] Entrypoint modularization MUST preserve existing CLI flags, environment variables, receipt schemas, scenario names, scenario semantics, dry-run/live behavior, and evidence non-claims.

#### Scenario: Public surfaces remain stable

r[mc_compatibility.runner_modularity.entrypoint_parity.stable]
- GIVEN a supported pre-refactor runner invocation
- WHEN the modularized runner receives the same inputs
- THEN it produces the same mode selection, scenario selection, receipt shape, and non-claim boundaries
- AND it does not promote new compatibility evidence.

### Requirement: Entrypoint positive tests

r[mc_compatibility.runner_modularity.entrypoint_positive_tests] The change MUST include positive tests for representative dry-run, run, build-client, status, cleanup, matrix, receipt, and failure-bundle paths.

#### Scenario: Supported paths still pass

r[mc_compatibility.runner_modularity.entrypoint_positive_tests.coverage]
- GIVEN representative supported runner inputs
- WHEN the modularized entrypoint delegates into owner modules
- THEN tests prove the supported paths still produce the expected plans, receipts, and outcomes.

### Requirement: Entrypoint negative tests

r[mc_compatibility.runner_modularity.entrypoint_negative_tests] The change MUST include negative tests for unknown arguments, missing option values, unsafe cleanup/path plans, receipt/failure-bundle follow-up failures, and invalid mode combinations.

#### Scenario: Invalid paths fail closed

r[mc_compatibility.runner_modularity.entrypoint_negative_tests.fail_closed]
- GIVEN an invalid runner input or unsafe plan
- WHEN the modularized entrypoint delegates into owner modules
- THEN tests prove the runner returns the expected diagnostic before unintended side effects occur.

### Requirement: Entrypoint modularization validation

r[mc_compatibility.runner_modularity.entrypoint_validation] The change MUST record focused runner tests, dry-run smoke checks, scenario manifest checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.entrypoint_validation.logs]
- GIVEN entrypoint modularization is complete
- WHEN the change is closed
- THEN reviewable evidence logs show positive and negative tests plus Cairn proposal, design, tasks, and validation gates passing.

### Requirement: Client-driver pure core

r[mc_compatibility.runner_modularity.client_driver_core] Non-trivial client-driver decisions SHOULD be implemented as pure deterministic cores over explicit config, run records, logs, and scenario metadata.

#### Scenario: Client-driver logic is testable without live processes

r[mc_compatibility.runner_modularity.client_driver_core.pure]
- GIVEN client-driver logic derives run plans, combines logs, evaluates scenario evidence, or classifies outcomes
- WHEN that logic is invoked by tests
- THEN it can run without Xvfb, process spawning, filesystem reads, server restarts, sleeps, or stdout/stderr.

### Requirement: Client run planning

r[mc_compatibility.runner_modularity.client_run_planning] The runner SHOULD derive client usernames, session counts, timeouts, log strategies, restart needs, and dry-run evidence modes through pure client run planning.

#### Scenario: Run plan is deterministic

r[mc_compatibility.runner_modularity.client_run_planning.deterministic]
- GIVEN the same config and scenario metadata
- WHEN client run planning runs repeatedly
- THEN it returns the same client run plan each time.

### Requirement: Client evidence classification

r[mc_compatibility.runner_modularity.client_evidence_classification] Client evidence classification MUST preserve existing classification strings, evidence fields, scenario evaluation, server-correlation behavior, projectile checks, and non-claim boundaries.

#### Scenario: Classification preserves evidence contract

r[mc_compatibility.runner_modularity.client_evidence_classification.parity]
- GIVEN client run records and logs equivalent to pre-refactor behavior
- WHEN the pure classification core evaluates them
- THEN it returns the same classification, evidence fields, pass/fail result, and diagnostics as the pre-refactor runner.

### Requirement: Client-driver shell boundary

r[mc_compatibility.runner_modularity.client_driver_shell] Client-driver shell code MUST own process execution, timeout handling, filesystem log access, restart transitions, stdout/stderr, and error plumbing without duplicating pure evidence policy.

#### Scenario: Shell produces run records

r[mc_compatibility.runner_modularity.client_driver_shell.records]
- GIVEN a live client scenario is executed
- WHEN shell code finishes a client process or restart transition
- THEN it produces run records for the pure core
- AND shell code does not recompute evidence classification policy.

### Requirement: Client-driver positive tests

r[mc_compatibility.runner_modularity.client_driver_positive_tests] The change MUST include positive tests for dry-run evidence, successful single-client, reconnect, multi-client, projectile, and timeout-success classifications.

#### Scenario: Supported classifications pass

r[mc_compatibility.runner_modularity.client_driver_positive_tests.coverage]
- GIVEN representative successful run records and logs
- WHEN the pure client-driver core evaluates them
- THEN it produces the expected passing classifications and evidence fields.

### Requirement: Client-driver negative tests

r[mc_compatibility.runner_modularity.client_driver_negative_tests] The change MUST include negative tests for missing milestones, forbidden markers, bad exit codes, missing server correlation, projectile order failures, and restart-state failures.

#### Scenario: Bad evidence fails closed

r[mc_compatibility.runner_modularity.client_driver_negative_tests.fail_closed]
- GIVEN malformed or incomplete run records and logs
- WHEN the pure client-driver core evaluates them
- THEN it rejects the evidence with actionable diagnostics before receipts claim success.

### Requirement: Client-driver validation

r[mc_compatibility.runner_modularity.client_driver_validation] The change MUST record focused client-driver tests, runner tests, dry-run smoke checks, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.client_driver_validation.logs]
- GIVEN client-driver core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show core parity, positive and negative fixtures, dry-run smoke checks, and Cairn validation passing.

### Requirement: Scenario behavior metadata

r[mc_compatibility.runner_modularity.scenario_metadata] Scenario behavior facts SHOULD be represented as explicit scenario metadata or generated scenario surfaces when the facts are deterministic and declarative.

#### Scenario: Scenario row carries behavior facts

r[mc_compatibility.runner_modularity.scenario_metadata.row]
- GIVEN a scenario has deterministic run strategy, env intent, evidence selector, typed-event edge, or non-claim facts
- WHEN that scenario is represented in the catalog
- THEN those facts are available from scenario metadata or generated scenario surfaces
- AND consumers do not need unrelated large match statements to recover those facts.

### Requirement: Scenario extension path

r[mc_compatibility.runner_modularity.scenario_extension_path] Adding a supported scenario SHOULD require updating a bounded metadata surface plus any explicitly named specialized handler, rather than editing multiple unrelated consumer match statements.

#### Scenario: New scenario has one auditable path

r[mc_compatibility.runner_modularity.scenario_extension_path.auditable]
- GIVEN a new scenario is added
- WHEN reviewers inspect its behavior definition
- THEN names, aliases, milestones, run strategy, env intents, typed-event graph behavior, evidence selectors, and non-claims are discoverable from the scenario metadata path
- AND any custom code hook is explicitly named.

### Requirement: Scenario metadata positive tests

r[mc_compatibility.runner_modularity.scenario_metadata_positive_tests] The change MUST include positive validation for representative single-client, reconnect, multi-client, projectile, inventory, survival, CTF, and MCP scenario metadata.

#### Scenario: Representative metadata validates

r[mc_compatibility.runner_modularity.scenario_metadata_positive_tests.coverage]
- GIVEN representative scenario rows from each behavior family
- WHEN scenario metadata validation runs
- THEN each row produces the expected run strategy, env intents, typed-event edges, evidence selectors, and non-claims.

### Requirement: Scenario metadata negative tests

r[mc_compatibility.runner_modularity.scenario_metadata_negative_tests] The change MUST include negative validation for missing required facts, unknown env intents, invalid graph edges, duplicate aliases, and unsupported handler references.

#### Scenario: Incomplete metadata fails closed

r[mc_compatibility.runner_modularity.scenario_metadata_negative_tests.fail_closed]
- GIVEN a malformed scenario metadata row
- WHEN scenario metadata validation runs
- THEN it rejects the row before runtime execution
- AND the diagnostic names the malformed field.

### Requirement: Scenario metadata validation

r[mc_compatibility.runner_modularity.scenario_metadata_validation] The change MUST record scenario-spec validation, generated-surface freshness checks, runner tests, Cairn gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.scenario_metadata_validation.logs]
- GIVEN data-driven scenario behavior migration is complete
- WHEN the change is closed
- THEN reviewable logs show metadata parity, positive and negative fixtures, generated freshness checks, and Cairn validation passing.

### Requirement: Stevenarella protocol version manifest

r[mc_compatibility.stevenarella_protocol_versions.version_manifest] Stevenarella protocol version support SHOULD be represented by typed metadata that records canonical names, aliases, numeric protocol ids, translation module owners, and explicit reuse or fallback relationships.

#### Scenario: Protocol row is auditable

r[mc_compatibility.stevenarella_protocol_versions.version_manifest.row]
- GIVEN a supported protocol version or alias is reviewed
- WHEN maintainers inspect the protocol-version metadata
- THEN the canonical name, aliases, numeric id, translation module, and fallback relationship are visible in one bounded source of truth
- AND reviewers do not need unrelated match arms to recover those facts.

### Requirement: Stevenarella generated protocol dispatch

r[mc_compatibility.stevenarella_protocol_versions.generated_dispatch] The protocol-version dispatch functions SHOULD be generated from or validated against the typed protocol-version metadata while preserving existing public APIs.

#### Scenario: Dispatch matches metadata

r[mc_compatibility.stevenarella_protocol_versions.generated_dispatch.fresh]
- GIVEN protocol-version metadata changes
- WHEN the generated or validated dispatch surface is checked
- THEN stale Rust dispatch tables, missing modules, unknown fallback targets, and protocol-number mismatches are rejected before runtime.

### Requirement: Stevenarella protocol version parity

r[mc_compatibility.stevenarella_protocol_versions.version_parity] Data-driving protocol versions MUST preserve current protocol-name parsing, numeric protocol input behavior, translation dispatch behavior, unsupported-input behavior, packet-boundary behavior, and evidence non-claims.

#### Scenario: Existing protocol behavior remains stable

r[mc_compatibility.stevenarella_protocol_versions.version_parity.stable]
- GIVEN a supported pre-refactor protocol name, numeric version, or packet translation dispatch input
- WHEN the data-driven protocol-version surface processes the same input
- THEN the selected protocol id, translation module, output id, and unsupported-input behavior remain equivalent
- AND no new packet-support or full-protocol compatibility claim is promoted.

### Requirement: Stevenarella protocol version positive tests

r[mc_compatibility.stevenarella_protocol_versions.positive_tests] The change MUST include positive tests for supported names, numeric protocol inputs, alias rows, translation module dispatch, and explicit fallback or reuse relationships.

#### Scenario: Supported protocol metadata passes

r[mc_compatibility.stevenarella_protocol_versions.positive_tests.coverage]
- GIVEN representative supported protocol metadata rows
- WHEN the metadata and dispatch validators run
- THEN tests prove names, aliases, numeric ids, modules, and fallback relationships resolve as expected.

### Requirement: Stevenarella protocol version negative tests

r[mc_compatibility.stevenarella_protocol_versions.negative_tests] The change MUST include negative tests for duplicate aliases, missing translation modules, unknown fallback targets, protocol-number mismatches, unsupported names, and stale generated surfaces.

#### Scenario: Invalid protocol metadata fails closed

r[mc_compatibility.stevenarella_protocol_versions.negative_tests.fail_closed]
- GIVEN malformed protocol-version metadata or stale generated output
- WHEN protocol metadata validation runs
- THEN tests prove the input is rejected with a diagnostic naming the malformed field or stale surface.

### Requirement: Stevenarella protocol version validation

r[mc_compatibility.stevenarella_protocol_versions.validation] The change MUST record focused Stevenarella protocol tests, generated-surface freshness checks, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_protocol_versions.validation.logs]
- GIVEN protocol-version data-driving is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative protocol metadata tests plus freshness checks and Cairn gates passing.

### Requirement: Stevenarella model core

r[mc_compatibility.stevenarella_model.model_core] Stevenarella model code SHOULD expose pure cores for resource reference parsing, model path normalization, blockstate variant selection, multipart rule evaluation, model inheritance decisions, biome and light calculations, and vertex planning.

#### Scenario: Model decision is explicit

r[mc_compatibility.stevenarella_model.model_core.explicit]
- GIVEN resource identifiers, blockstate facts, model summaries, or lighting inputs
- WHEN model logic needs a path, selected variant, rule outcome, inherited model, or vertex plan
- THEN the decision is produced by a pure core over explicit inputs.

### Requirement: Stevenarella model shell boundary

r[mc_compatibility.stevenarella_model.model_shell_boundary] Model-core extraction MUST keep resource reads, JSON decoding, texture lookup, random source selection, renderer allocation, and logging outside pure model cores.

#### Scenario: Model side effects remain in shell

r[mc_compatibility.stevenarella_model.model_shell_boundary.effects]
- GIVEN the model core returns a reference, rule, geometry, or lighting plan
- WHEN the model shell applies that plan
- THEN only the shell performs resource access, decoding, texture lookup, random selection, renderer allocation, or logging.

### Requirement: Stevenarella model parity

r[mc_compatibility.stevenarella_model.parity] Model-core extraction MUST preserve current model/resource behavior, blockstate selection semantics, lighting and biome behavior, public model types, and evidence non-claims.

#### Scenario: Model behavior remains stable

r[mc_compatibility.stevenarella_model.parity.stable]
- GIVEN a supported pre-refactor model or blockstate input
- WHEN the extracted model core and shell process the same input
- THEN resource selection, model selection, geometry planning, light/biome behavior, and public type behavior remain equivalent.

### Requirement: Stevenarella model positive tests

r[mc_compatibility.stevenarella_model.positive_tests] The change MUST include positive tests for resource references, absolute and relative model paths, blockstate resources, parent resources, multipart matches, light and biome calculations, and vertex plans.

#### Scenario: Supported model paths pass

r[mc_compatibility.stevenarella_model.positive_tests.coverage]
- GIVEN representative supported model inputs
- WHEN extracted model cores process them
- THEN tests prove the expected references, paths, selections, calculations, or vertex plans are produced.

### Requirement: Stevenarella model negative tests

r[mc_compatibility.stevenarella_model.negative_tests] The change MUST include negative tests for malformed references, unsafe paths, missing parents, invalid multipart rules, unknown builtins, invalid face data, and unsupported tint or light inputs.

#### Scenario: Invalid model paths fail closed

r[mc_compatibility.stevenarella_model.negative_tests.fail_closed]
- GIVEN invalid model inputs
- WHEN extracted model cores process them
- THEN tests prove the inputs are rejected, defaulted, or contained according to current behavior.

### Requirement: Stevenarella model validation

r[mc_compatibility.stevenarella_model.validation] The change MUST record focused Stevenarella model/render tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_model.validation.logs]
- GIVEN model-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative model tests plus affected dry-runs and Cairn gates passing.

### Requirement: Stevenarella dimension and chunk core

r[mc_compatibility.stevenarella_world.dimension_chunk_core] Stevenarella SHOULD expose pure deterministic cores for dimension bounds selection, chunk-section layout, biome and light interpretation, block update decisions, and storage update planning.

#### Scenario: Dimension and chunk decisions are explicit

r[mc_compatibility.stevenarella_world.dimension_chunk_core.explicit]
- GIVEN join-game dimension facts, dimension-codec facts, protocol-version facts, or chunk payload summaries
- WHEN world/chunk logic needs bounds or layout decisions
- THEN the decisions are produced by pure core functions
- AND storage mutation or rendering side effects are not required to inspect the decision.

### Requirement: Stevenarella world shell boundary

r[mc_compatibility.stevenarella_world.world_shell_boundary] Stevenarella world-core extraction MUST keep byte reading, NBT traversal, packet variant handling, world storage mutation, render invalidation, and logging outside the pure world core.

#### Scenario: World side effects remain in shell

r[mc_compatibility.stevenarella_world.world_shell_boundary.effects]
- GIVEN the world core returns a dimension, chunk, biome, light, or block update plan
- WHEN the Stevenarella world shell applies that plan
- THEN only the shell reads raw packet bytes, traverses NBT, mutates world storage, invalidates rendering, or logs diagnostics
- AND the core remains testable with in-memory summaries.

### Requirement: Stevenarella world parity

r[mc_compatibility.stevenarella_world.world_parity] World-core extraction MUST preserve existing world behavior, dimension fallback behavior, protocol-version handling, chunk parsing semantics, and evidence non-claims.

#### Scenario: Existing world behavior remains stable

r[mc_compatibility.stevenarella_world.world_parity.stable]
- GIVEN a supported pre-refactor world or chunk input
- WHEN the extracted world core and shell process the same input
- THEN the selected bounds, storage updates, parsing outcomes, and non-claim boundaries remain equivalent
- AND no full protocol 763 or broad compatibility claim is promoted.

### Requirement: Stevenarella world positive tests

r[mc_compatibility.stevenarella_world.world_positive_tests] The change MUST include positive tests for selected dimension-codec bounds, min-y and height application, section-count derivation, biome and light payload acceptance, and block update plans.

#### Scenario: Supported world inputs pass

r[mc_compatibility.stevenarella_world.world_positive_tests.coverage]
- GIVEN representative supported world, dimension, and chunk inputs
- WHEN extracted world cores process them
- THEN tests prove the expected bounds, layouts, and update plans are produced.

### Requirement: Stevenarella world negative tests

r[mc_compatibility.stevenarella_world.world_negative_tests] The change MUST include negative tests for missing dimension type, invalid min-y or height, truncated chunk data, inconsistent section counts, malformed biome or light data, and unsupported dimension inputs.

#### Scenario: Invalid world inputs fail closed

r[mc_compatibility.stevenarella_world.world_negative_tests.fail_closed]
- GIVEN malformed or unsupported world, dimension, or chunk inputs
- WHEN extracted world cores process them
- THEN tests prove the inputs are rejected, defaulted, or contained according to current behavior without corrupting storage plans.

### Requirement: Stevenarella world validation

r[mc_compatibility.stevenarella_world.world_validation] The change MUST record focused Stevenarella world/protocol tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_world.world_validation.logs]
- GIVEN world-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative world-core tests plus affected dry-runs and Cairn gates passing.

### Requirement: Valence entity core

r[mc_compatibility.valence_entity.entity_core] Valence entity code SHOULD expose pure cores for attribute math, status-effect application and expiry, tracked-data updates, hitbox calculations, flag changes, and query predicates where practical.

#### Scenario: Entity decision is explicit

r[mc_compatibility.valence_entity.entity_core.explicit]
- GIVEN entity, attribute, status, tracked-data, hitbox, flag, or query summaries
- WHEN entity logic needs a deterministic decision
- THEN the decision is produced by a pure core over explicit inputs.

### Requirement: Valence entity shell boundary

r[mc_compatibility.valence_entity.entity_shell_boundary] Entity-core extraction MUST keep Bevy component mutation, event emission, packet composition, schedule wiring, and logging outside pure entity cores.

#### Scenario: Entity side effects remain in shell

r[mc_compatibility.valence_entity.entity_shell_boundary.effects]
- GIVEN the entity core returns a state or packet-facing decision
- WHEN the Valence entity shell applies that decision
- THEN only the shell mutates ECS state, emits events, composes packets, wires schedules, or logs diagnostics.

### Requirement: Valence entity parity

r[mc_compatibility.valence_entity.parity] Entity-core extraction MUST preserve public entity APIs, attribute/status semantics, tracked-data encoding behavior, hitbox behavior, flags, and evidence non-claims.

#### Scenario: Entity behavior remains stable

r[mc_compatibility.valence_entity.parity.stable]
- GIVEN a supported pre-refactor entity input
- WHEN extracted entity cores and shells process the same input
- THEN the returned state, packet-facing data, public API behavior, and non-claim boundary remain equivalent.

### Requirement: Valence entity positive tests

r[mc_compatibility.valence_entity.positive_tests] The change MUST include positive tests for attribute modifiers, status-effect insertion and expiry, tracked-data updates, hitbox selection, flags, and entity query predicates.

#### Scenario: Supported entity paths pass

r[mc_compatibility.valence_entity.positive_tests.coverage]
- GIVEN representative supported entity inputs
- WHEN extracted entity cores process them
- THEN tests prove the expected entity decisions or state updates are produced.

### Requirement: Valence entity negative tests

r[mc_compatibility.valence_entity.negative_tests] The change MUST include negative tests for duplicate modifiers, invalid status durations, malformed tracked data, invalid hitboxes, unknown flags, and empty query inputs.

#### Scenario: Invalid entity paths fail closed

r[mc_compatibility.valence_entity.negative_tests.fail_closed]
- GIVEN invalid entity inputs
- WHEN extracted entity cores process them
- THEN tests prove the inputs are rejected, ignored, clamped, or contained according to current behavior.

### Requirement: Valence entity validation

r[mc_compatibility.valence_entity.validation] The change MUST record focused Valence entity tests, affected workspace checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_entity.validation.logs]
- GIVEN entity-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative entity tests plus affected checks and Cairn gates passing.

### Requirement: Valence inventory click transaction core

r[mc_compatibility.valence_inventory.click_transaction_core] Valence inventory click handling SHOULD expose a pure transaction core that receives explicit inventory, cursor, open-window, click-mode, slot, and slot-change summaries and returns explicit transaction decisions.

#### Scenario: Inventory click decision is testable without Bevy

r[mc_compatibility.valence_inventory.click_transaction_core.testable]
- GIVEN an inventory click packet summary and explicit inventory state summaries
- WHEN the transaction core processes the input
- THEN it returns a decision such as apply transaction, drop cursor, resync invalid, emit click event, ignore, or reject
- AND Bevy queries or live client objects are not required to inspect the decision.

### Requirement: Valence inventory click shell boundary

r[mc_compatibility.valence_inventory.click_shell_boundary] Inventory click extraction MUST keep Bevy queries, packet resync sends, event writers, inventory mutation, cursor mutation, and drop-item event emission outside the pure transaction core.

#### Scenario: Inventory side effects remain in shell

r[mc_compatibility.valence_inventory.click_shell_boundary.effects]
- GIVEN the inventory click core returns a transaction decision
- WHEN the Valence inventory shell applies that decision
- THEN only the shell mutates ECS state, sends resync packets, writes events, or emits dropped items
- AND the core remains testable with in-memory inputs.

### Requirement: Valence inventory click parity

r[mc_compatibility.valence_inventory.click_parity] Inventory click-core extraction MUST preserve existing packet validation, invalid resync behavior, cursor and drop semantics, key/drop mode behavior, regular click flow behavior, emitted event shapes, and evidence non-claims.

#### Scenario: Existing inventory click behavior remains stable

r[mc_compatibility.valence_inventory.click_parity.stable]
- GIVEN a supported pre-refactor inventory click input
- WHEN the extracted transaction core and shell process the same input
- THEN the inventory mutation, cursor state, resync behavior, emitted events, and non-claim boundaries remain equivalent
- AND no new inventory semantic compatibility claim is promoted.

### Requirement: Valence inventory click positive tests

r[mc_compatibility.valence_inventory.click_positive_tests] The change MUST include positive tests for valid regular clicks, outside-window cursor drops, drop-key paths, open-inventory clicks, cursor updates, and emitted click-event plans.

#### Scenario: Supported inventory click paths pass

r[mc_compatibility.valence_inventory.click_positive_tests.coverage]
- GIVEN representative supported inventory click inputs
- WHEN the transaction core processes them
- THEN tests prove the expected transaction decisions, cursor outcomes, and event plans are produced.

### Requirement: Valence inventory click negative tests

r[mc_compatibility.valence_inventory.click_negative_tests] The change MUST include negative tests for invalid packets, unsafe slot indices, malformed slot changes, missing clients, missing open inventories, invalid cursor states, and resync plans.

#### Scenario: Invalid inventory click paths fail closed

r[mc_compatibility.valence_inventory.click_negative_tests.fail_closed]
- GIVEN malformed or unsupported inventory click inputs
- WHEN the transaction core or shell processes them
- THEN tests prove the inputs are rejected, ignored, or resynced according to current behavior without corrupting inventory state.

### Requirement: Valence inventory click validation

r[mc_compatibility.valence_inventory.click_validation] The change MUST record focused Valence inventory tests, affected mc-compat inventory dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_inventory.click_validation.logs]
- GIVEN inventory click-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative inventory click tests plus affected dry-runs and Cairn gates passing.

### Requirement: Runner config patch model

r[mc_compatibility.runner_modularity.config_patch_model] The mc-compat runner SHOULD represent partial configuration updates as explicit pure patch data before applying them to a resolved configuration.

#### Scenario: Config source produces a patch

r[mc_compatibility.runner_modularity.config_patch_model.patch]
- GIVEN a supported config source such as defaults, Nickel-exported JSON, restricted Steel config, environment variables, or CLI arguments
- WHEN the runner parses that source
- THEN the parsed result is available as a deterministic config patch or a source-specific diagnostic
- AND parsing the source does not require mutating the final resolved configuration.

### Requirement: Runner config source precedence

r[mc_compatibility.runner_modularity.config_source_precedence] The runner MUST preserve existing config precedence while making the ordered source list explicit and reviewable.

#### Scenario: Later sources override earlier sources

r[mc_compatibility.runner_modularity.config_source_precedence.ordered]
- GIVEN multiple supported config sources set the same configurable value
- WHEN patches are composed in the documented runner source order
- THEN the resolved configuration matches the pre-refactor precedence behavior
- AND the source order can be inspected from the config resolution core.

### Requirement: Runner config validation

r[mc_compatibility.runner_modularity.config_validation] The runner MUST validate cross-field safety and mode constraints after config patches are resolved and before side-effecting execution begins.

#### Scenario: Unsafe resolved config fails before execution

r[mc_compatibility.runner_modularity.config_validation.fail_closed]
- GIVEN config patches resolve to an unsafe path, invalid timeout, missing required value, invalid backend, invalid scenario, or unsupported mode combination
- WHEN config validation runs
- THEN the runner rejects the resolved configuration with an actionable diagnostic
- AND no server, client, receipt, cleanup, or artifact side effect is started.

### Requirement: Runner config positive tests

r[mc_compatibility.runner_modularity.config_positive_tests] The change MUST include positive tests for representative config defaults, file/env/CLI precedence, Steel and Nickel config inputs, mode selection, receipt paths, backend selection, and scenario selection.

#### Scenario: Supported config paths resolve

r[mc_compatibility.runner_modularity.config_positive_tests.coverage]
- GIVEN representative supported config inputs
- WHEN the config patch core resolves them
- THEN tests prove the resulting configuration matches the current runner behavior for those inputs.

### Requirement: Runner config negative tests

r[mc_compatibility.runner_modularity.config_negative_tests] The change MUST include negative tests for unknown flags, missing option values, invalid backend or scenario values, invalid timeouts, unsafe output paths, and conflicting mode or source combinations.

#### Scenario: Invalid config inputs fail closed

r[mc_compatibility.runner_modularity.config_negative_tests.fail_closed]
- GIVEN malformed or unsafe config inputs
- WHEN the config patch core parses, composes, or validates them
- THEN tests prove the inputs are rejected with the expected diagnostic before side effects.

### Requirement: Runner config patch validation evidence

r[mc_compatibility.runner_modularity.config_validation_evidence] The change MUST record focused config tests, dry-run smoke checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.runner_modularity.config_validation_evidence.logs]
- GIVEN the config patch refactor is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative config fixtures plus Cairn gates and validation passing.

### Requirement: Stevenarella ECS boundaries

r[mc_compatibility.stevenarella_ecs.ecs_boundaries] Stevenarella ECS code SHOULD expose cohesive boundaries for entity IDs, component storage, query access, system registration, system execution, and diagnostics.

#### Scenario: ECS responsibility has one owner

r[mc_compatibility.stevenarella_ecs.ecs_boundaries.ownership]
- GIVEN an ECS responsibility is reviewed
- WHEN maintainers inspect ECS modules
- THEN the responsibility is owned by a focused module
- AND unrelated allocation, storage, query, system, and diagnostic concerns are not reintroduced into one module.

### Requirement: Stevenarella ECS core

r[mc_compatibility.stevenarella_ecs.ecs_core] ECS allocation, lookup, query-shape, system-ordering, and diagnostic decisions SHOULD be deterministic and testable through focused cores where practical.

#### Scenario: ECS invariant is testable directly

r[mc_compatibility.stevenarella_ecs.ecs_core.testable]
- GIVEN ECS state summaries and operation inputs
- WHEN an ECS core processes them
- THEN the result can be tested without running renderer, server, UI, or network systems.

### Requirement: Stevenarella ECS parity

r[mc_compatibility.stevenarella_ecs.parity] ECS modularization MUST preserve public ECS APIs, execution order, component behavior, borrow/error behavior, and evidence non-claims.

#### Scenario: ECS behavior remains stable

r[mc_compatibility.stevenarella_ecs.parity.stable]
- GIVEN a supported pre-refactor ECS operation
- WHEN the modularized ECS processes the same input
- THEN entity, component, query, system, and diagnostic behavior remain equivalent.

### Requirement: Stevenarella ECS positive tests

r[mc_compatibility.stevenarella_ecs.positive_tests] The change MUST include positive tests for entity allocation, component insert/remove/get, query matching, system ordering, and diagnostic reporting.

#### Scenario: Supported ECS paths pass

r[mc_compatibility.stevenarella_ecs.positive_tests.coverage]
- GIVEN representative supported ECS inputs
- WHEN extracted ECS modules process them
- THEN tests prove the expected storage, query, or system behavior is produced.

### Requirement: Stevenarella ECS negative tests

r[mc_compatibility.stevenarella_ecs.negative_tests] The change MUST include negative tests for missing components, duplicate entities, invalid removals, conflicting borrows, empty systems, and invalid query shapes.

#### Scenario: Invalid ECS paths fail closed

r[mc_compatibility.stevenarella_ecs.negative_tests.fail_closed]
- GIVEN invalid ECS inputs
- WHEN extracted ECS modules process them
- THEN tests prove the inputs are rejected, ignored, or diagnosed according to current behavior.

### Requirement: Stevenarella ECS validation

r[mc_compatibility.stevenarella_ecs.validation] The change MUST record focused ECS tests, affected client checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_ecs.validation.logs]
- GIVEN ECS modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative ECS tests plus affected checks and Cairn gates passing.

### Requirement: Valence network session boundaries

r[mc_compatibility.valence_network.session_boundaries] Valence network code SHOULD expose cohesive boundaries for connect and listen orchestration, status and legacy ping handling, login/session negotiation, packet IO framing, profile/cache adapters, and pure session decisions.

#### Scenario: Network responsibility has one owner

r[mc_compatibility.valence_network.session_boundaries.ownership]
- GIVEN a network session responsibility is reviewed
- WHEN maintainers inspect Valence network modules
- THEN the responsibility is owned by a focused module
- AND unrelated socket, packet, status, login, and profile concerns are not reintroduced into one module.

### Requirement: Valence network session core

r[mc_compatibility.valence_network.session_core] Network state transitions, validation, compression choices, status response composition, disconnect classification, and legacy ping classification SHOULD be pure over explicit inputs.

#### Scenario: Network decision is testable without sockets

r[mc_compatibility.valence_network.session_core.testable]
- GIVEN session state summaries and packet or status facts
- WHEN the network core processes them
- THEN the decision can be tested without sockets, async tasks, channels, profile cache IO, or clocks.

### Requirement: Valence network parity

r[mc_compatibility.valence_network.parity] Network modularization MUST preserve public APIs, packet/session behavior, status and legacy ping behavior, profile/cache behavior, async side-effect boundaries, and evidence non-claims.

#### Scenario: Network behavior remains stable

r[mc_compatibility.valence_network.parity.stable]
- GIVEN a supported pre-refactor network input
- WHEN the modularized network code processes the same input
- THEN the session state, packets, status output, profile behavior, and non-claim boundary remain equivalent.

### Requirement: Valence network positive tests

r[mc_compatibility.valence_network.positive_tests] The change MUST include positive tests for status responses, legacy ping classification, login/session transitions, compression decisions, packet framing decisions, and profile adapter outcomes.

#### Scenario: Supported network paths pass

r[mc_compatibility.valence_network.positive_tests.coverage]
- GIVEN representative supported network inputs
- WHEN extracted network cores process them
- THEN tests prove the expected session, status, packet, or profile decisions are produced.

### Requirement: Valence network negative tests

r[mc_compatibility.valence_network.negative_tests] The change MUST include negative tests for malformed handshakes, invalid state transitions, unsupported compression, bad packet frames, missing profiles, closed channels, and timeout classifications.

#### Scenario: Invalid network paths fail closed

r[mc_compatibility.valence_network.negative_tests.fail_closed]
- GIVEN invalid network inputs
- WHEN extracted network cores process them
- THEN tests prove the inputs are rejected, disconnected, or contained according to current behavior.

### Requirement: Valence network validation

r[mc_compatibility.valence_network.validation] The change MUST record focused Valence network tests, affected smoke or dry-run checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_network.validation.logs]
- GIVEN network modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative network tests plus affected checks and Cairn gates passing.

### Requirement: Stevenarella capture module boundaries

r[mc_compatibility.stevenarella_capture.module_boundaries] Stevenarella capture code SHOULD expose cohesive module boundaries for request validation and planning, queueing, framebuffer readback normalization, artifact persistence, recording cadence and state, metadata validation, and service orchestration.

#### Scenario: Capture responsibility has one owner

r[mc_compatibility.stevenarella_capture.module_boundaries.ownership]
- GIVEN a capture responsibility is reviewed
- WHEN maintainers inspect the capture module tree
- THEN the responsibility is owned by a focused module
- AND unrelated validation, queue, readback, persistence, and recording concerns are not reintroduced into one root module.

### Requirement: Stevenarella capture functional core

r[mc_compatibility.stevenarella_capture.functional_core] Capture dimension, path, digest, metadata, and recording cadence decisions SHOULD be pure over explicit inputs, with side effects isolated in shells or adapters.

#### Scenario: Capture decision is testable without renderer or filesystem

r[mc_compatibility.stevenarella_capture.functional_core.testable]
- GIVEN capture logic computes validation, artifact paths, buffer sizes, row normalization, metadata checks, or recording due-ness
- WHEN the logic is extracted
- THEN it can be tested with in-memory inputs
- AND framebuffer reads, filesystem writes, PNG encoding, clocks, and channel operations remain outside the pure core.

### Requirement: Stevenarella capture parity

r[mc_compatibility.stevenarella_capture.parity] Capture service splitting MUST preserve capture request shapes, artifact path semantics, BLAKE3 metadata, redaction state, recording bounds, MCP-facing behavior, and evidence non-claims.

#### Scenario: Capture evidence surface remains stable

r[mc_compatibility.stevenarella_capture.parity.stable]
- GIVEN a supported pre-refactor capture request or MCP capture call
- WHEN the split capture service processes the same input
- THEN the output mode, artifact path, metadata, digest, redaction state, and non-claim boundaries remain equivalent
- AND no new rendering or capture correctness claim is promoted.

### Requirement: Stevenarella capture positive tests

r[mc_compatibility.stevenarella_capture.positive_tests] The change MUST include positive tests for request validation, default paths, metadata validation, queue send and receive behavior, readback normalization, artifact plans, and recording cadence.

#### Scenario: Supported capture paths pass

r[mc_compatibility.stevenarella_capture.positive_tests.coverage]
- GIVEN representative supported capture inputs
- WHEN extracted capture modules process them
- THEN tests prove the expected plans, metadata, queue results, normalized frames, and recording decisions are produced.

### Requirement: Stevenarella capture negative tests

r[mc_compatibility.stevenarella_capture.negative_tests] The change MUST include negative tests for invalid dimensions, unsafe artifact paths, oversized artifacts, invalid metadata, closed queues, pending-limit exhaustion, and recording bound violations.

#### Scenario: Invalid capture paths fail closed

r[mc_compatibility.stevenarella_capture.negative_tests.fail_closed]
- GIVEN invalid or unsafe capture inputs
- WHEN extracted capture modules process them
- THEN tests prove the inputs are rejected or contained with the expected diagnostic before unsafe artifacts or corrupt metadata are produced.

### Requirement: Stevenarella capture validation

r[mc_compatibility.stevenarella_capture.validation] The change MUST record focused Stevenarella capture tests, affected MCP dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_capture.validation.logs]
- GIVEN capture service splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative capture tests plus affected MCP dry-runs and Cairn gates passing.

### Requirement: Stevenarella player boundaries

r[mc_compatibility.stevenarella_player.player_boundaries] Stevenarella player entity code SHOULD expose cohesive boundaries for player construction, model state, rendering, movement, collision, and ECS system wiring.

#### Scenario: Player responsibility has one owner

r[mc_compatibility.stevenarella_player.player_boundaries.ownership]
- GIVEN a player entity responsibility is reviewed
- WHEN maintainers inspect player modules
- THEN the responsibility is owned by a focused module
- AND unrelated rendering, movement, collision, and ECS concerns are not reintroduced into one module.

### Requirement: Stevenarella player core

r[mc_compatibility.stevenarella_player.player_core] Player movement, collision, model-part visibility, and local/remote state decisions SHOULD be pure over explicit inputs.

#### Scenario: Player decision is testable without renderer

r[mc_compatibility.stevenarella_player.player_core.testable]
- GIVEN player state, movement, collision, or model summaries
- WHEN the player core processes them
- THEN the decision can be tested without renderer, resource manager, network, or live ECS side effects.

### Requirement: Stevenarella player parity

r[mc_compatibility.stevenarella_player.parity] Player entity splitting MUST preserve existing player behavior, model visibility, movement/collision semantics, public APIs, and evidence non-claims.

#### Scenario: Player behavior remains stable

r[mc_compatibility.stevenarella_player.parity.stable]
- GIVEN a supported pre-refactor player input
- WHEN the split player modules process the same input
- THEN player state, visibility, movement, collision, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Stevenarella player positive tests

r[mc_compatibility.stevenarella_player.positive_tests] The change MUST include positive tests for local/remote creation facts, model visibility, movement updates, collision decisions, and renderer-shell plans.

#### Scenario: Supported player paths pass

r[mc_compatibility.stevenarella_player.positive_tests.coverage]
- GIVEN representative supported player inputs
- WHEN extracted player cores process them
- THEN tests prove the expected state, movement, collision, or render plans are produced.

### Requirement: Stevenarella player negative tests

r[mc_compatibility.stevenarella_player.negative_tests] The change MUST include negative tests for invalid movement input, collision edge cases, missing model resources, disabled model parts, missing ECS components, and empty entity sets.

#### Scenario: Invalid player paths fail closed

r[mc_compatibility.stevenarella_player.negative_tests.fail_closed]
- GIVEN invalid player inputs
- WHEN extracted player cores or shells process them
- THEN tests prove the inputs are rejected, ignored, clamped, or contained according to current behavior.

### Requirement: Stevenarella player validation

r[mc_compatibility.stevenarella_player.validation] The change MUST record focused Stevenarella entity/player tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_player.validation.logs]
- GIVEN player entity splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative player tests plus affected checks and Cairn gates passing.

### Requirement: Stevenarella renderer boundaries

r[mc_compatibility.stevenarella_render.renderer_boundaries] Stevenarella renderer code SHOULD expose cohesive boundaries for camera/view state, chunk buffers, texture management, skin or remote texture cache, pending uploads, frame orchestration, and capture/readback integration.

#### Scenario: Renderer responsibility has one owner

r[mc_compatibility.stevenarella_render.renderer_boundaries.ownership]
- GIVEN a renderer responsibility is reviewed
- WHEN maintainers inspect renderer modules
- THEN the responsibility is owned by a focused module
- AND unrelated GL, cache, chunk, texture, frame, and capture concerns are not reintroduced into one module.

### Requirement: Stevenarella render core

r[mc_compatibility.stevenarella_render.render_core] Renderer URL, cache-path, upload-plan, chunk visibility/order, and frame/capture planning decisions SHOULD be pure over explicit inputs.

#### Scenario: Render plan is testable without OpenGL

r[mc_compatibility.stevenarella_render.render_core.testable]
- GIVEN renderer state summaries and resource identifiers
- WHEN the render core computes a cache, upload, chunk, or frame plan
- THEN the plan can be tested without OpenGL, resource-manager locks, filesystem, or network side effects.

### Requirement: Stevenarella renderer parity

r[mc_compatibility.stevenarella_render.parity] Renderer splitting MUST preserve visible rendering behavior, capture interactions, texture cache semantics, GL side-effect boundaries, and evidence non-claims.

#### Scenario: Renderer behavior remains stable

r[mc_compatibility.stevenarella_render.parity.stable]
- GIVEN a supported pre-refactor render or capture input
- WHEN the split renderer processes the same input
- THEN the selected render plan, cache behavior, capture interaction, and non-claim boundary remain equivalent.

### Requirement: Stevenarella renderer positive tests

r[mc_compatibility.stevenarella_render.positive_tests] The change MUST include positive tests for texture URL normalization, skin cache paths, upload plans, chunk render plans, camera/view facts, and capture frame plans.

#### Scenario: Supported renderer paths pass

r[mc_compatibility.stevenarella_render.positive_tests.coverage]
- GIVEN representative supported renderer inputs
- WHEN extracted renderer cores process them
- THEN tests prove the expected cache, upload, chunk, frame, or capture plans are produced.

### Requirement: Stevenarella renderer negative tests

r[mc_compatibility.stevenarella_render.negative_tests] The change MUST include negative tests for invalid texture URLs, unsafe cache paths, missing resources, invalid frame dimensions, empty chunk buffers, and unavailable capture contexts.

#### Scenario: Invalid renderer paths fail closed

r[mc_compatibility.stevenarella_render.negative_tests.fail_closed]
- GIVEN invalid renderer inputs
- WHEN extracted renderer cores process them
- THEN tests prove the inputs are rejected, ignored, or contained before unsafe side effects occur.

### Requirement: Stevenarella renderer validation

r[mc_compatibility.stevenarella_render.validation] The change MUST record focused Stevenarella render/capture tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_render.validation.logs]
- GIVEN renderer splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative renderer tests plus affected dry-runs and Cairn gates passing.

### Requirement: Stevenarella resource boundaries

r[mc_compatibility.stevenarella_resources.resource_boundaries] Stevenarella resource code SHOULD expose cohesive boundaries for resource identifiers and paths, pack discovery, lookup/indexing, cache policy, archive access, IO shells, and shared manager state.

#### Scenario: Resource responsibility has one owner

r[mc_compatibility.stevenarella_resources.resource_boundaries.ownership]
- GIVEN a resource manager responsibility is reviewed
- WHEN maintainers inspect resource modules
- THEN the responsibility is owned by a focused module
- AND unrelated path, pack, lookup, cache, archive, IO, and shared-state concerns are not reintroduced into one module.

### Requirement: Stevenarella resource core

r[mc_compatibility.stevenarella_resources.resource_core] Resource identifier parsing, lookup precedence, cache key derivation, path containment, and pack selection SHOULD be pure over explicit inputs.

#### Scenario: Resource decision is testable without IO

r[mc_compatibility.stevenarella_resources.resource_core.testable]
- GIVEN resource identifiers, pack metadata, cache facts, or path summaries
- WHEN the resource core processes them
- THEN the result can be tested without filesystem, archive, download, lock, or logging side effects.

### Requirement: Stevenarella resource parity

r[mc_compatibility.stevenarella_resources.parity] Resource manager splitting MUST preserve public resource APIs, lookup precedence, cache behavior, path safety, IO boundaries, and evidence non-claims.

#### Scenario: Resource behavior remains stable

r[mc_compatibility.stevenarella_resources.parity.stable]
- GIVEN a supported pre-refactor resource lookup or cache input
- WHEN the split resource modules process the same input
- THEN lookup result, cache behavior, path safety, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Stevenarella resource positive tests

r[mc_compatibility.stevenarella_resources.positive_tests] The change MUST include positive tests for resource identifiers, pack selection, lookup precedence, cache keys, archive entries, and contained paths.

#### Scenario: Supported resource paths pass

r[mc_compatibility.stevenarella_resources.positive_tests.coverage]
- GIVEN representative supported resource inputs
- WHEN extracted resource cores process them
- THEN tests prove the expected identifier, lookup, cache, archive, or path decisions are produced.

### Requirement: Stevenarella resource negative tests

r[mc_compatibility.stevenarella_resources.negative_tests] The change MUST include negative tests for unsafe paths, missing resources, duplicate pack entries, malformed archives, invalid identifiers, and failed IO adapters.

#### Scenario: Invalid resource paths fail closed

r[mc_compatibility.stevenarella_resources.negative_tests.fail_closed]
- GIVEN invalid or unsafe resource inputs
- WHEN extracted resource cores or shells process them
- THEN tests prove the inputs are rejected, missing, or contained according to current behavior.

### Requirement: Stevenarella resource validation

r[mc_compatibility.stevenarella_resources.validation] The change MUST record focused resource tests, affected model/render checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_resources.validation.logs]
- GIVEN resource manager splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative resource tests plus affected checks and Cairn gates passing.

### Requirement: Valence Anvil snapshot boundaries

r[mc_compatibility.valence_anvil.snapshot_boundaries] Valence Anvil snapshot code SHOULD expose cohesive boundaries for snapshot model types, region and chunk lookup planning, parsing and validation, cache policy, directory/filesystem shell, and Bevy integration adapters.

#### Scenario: Snapshot responsibility has one owner

r[mc_compatibility.valence_anvil.snapshot_boundaries.ownership]
- GIVEN an Anvil snapshot responsibility is reviewed
- WHEN maintainers inspect Anvil modules
- THEN the responsibility is owned by a focused module
- AND unrelated model, parse, cache, filesystem, and Bevy concerns are not reintroduced into one module.

### Requirement: Valence Anvil snapshot core

r[mc_compatibility.valence_anvil.snapshot_core] Region coordinate calculation, chunk selection, missing or corrupt classification, parse validation, and snapshot update planning SHOULD be pure over explicit inputs.

#### Scenario: Snapshot decision is testable without filesystem

r[mc_compatibility.valence_anvil.snapshot_core.testable]
- GIVEN snapshot, region, chunk, parse, or cache summaries
- WHEN the snapshot core processes them
- THEN the decision can be tested without filesystem, compression, directory traversal, Bevy resources, or logging.

### Requirement: Valence Anvil parity

r[mc_compatibility.valence_anvil.parity] Snapshot-core splitting MUST preserve public APIs, Anvil format behavior, missing/corrupt region behavior, cache behavior, Bevy integration behavior, and evidence non-claims.

#### Scenario: Anvil snapshot behavior remains stable

r[mc_compatibility.valence_anvil.parity.stable]
- GIVEN a supported pre-refactor Anvil snapshot input
- WHEN the split snapshot core and shell process the same input
- THEN lookup behavior, parse behavior, cache behavior, public APIs, and non-claim boundaries remain equivalent.

### Requirement: Valence Anvil positive tests

r[mc_compatibility.valence_anvil.positive_tests] The change MUST include positive tests for region coordinate mapping, chunk lookup, valid parse summaries, missing chunk behavior, cache plans, and snapshot update plans.

#### Scenario: Supported Anvil paths pass

r[mc_compatibility.valence_anvil.positive_tests.coverage]
- GIVEN representative supported Anvil snapshot inputs
- WHEN extracted snapshot cores process them
- THEN tests prove the expected lookup, classification, parse, cache, or update decisions are produced.

### Requirement: Valence Anvil negative tests

r[mc_compatibility.valence_anvil.negative_tests] The change MUST include negative tests for invalid coordinates, missing regions, corrupt chunks, malformed parse summaries, stale cache entries, and unavailable directories.

#### Scenario: Invalid Anvil paths fail closed

r[mc_compatibility.valence_anvil.negative_tests.fail_closed]
- GIVEN invalid or unavailable Anvil snapshot inputs
- WHEN extracted snapshot cores or shells process them
- THEN tests prove the inputs are rejected, classified, or contained according to current behavior.

### Requirement: Valence Anvil validation

r[mc_compatibility.valence_anvil.validation] The change MUST record focused Valence Anvil tests, affected workspace checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_anvil.validation.logs]
- GIVEN Anvil snapshot splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative Anvil tests plus affected checks and Cairn gates passing.

### Requirement: Checker crate migration inventory

r[mc_compatibility.checker_crate_migration.inventory] Checker crate migration work MUST inventory loose Rust checkers, legacy Python gates, already migrated checker crate rows, flake wiring, evidence inputs, owner, next action, and non-claim impact before migrating behavior.

#### Scenario: Checker migration scope is reviewable

r[mc_compatibility.checker_crate_migration.inventory.reviewable]
- GIVEN a checker migration wave is selected
- WHEN reviewers inspect the inventory
- THEN selected checkers, untouched debt rows, current command surfaces, flake checks, evidence inputs, owners, and next actions are named
- AND baseline checker validation is recorded before migration.

### Requirement: Checker core and shell boundary

r[mc_compatibility.checker_crate_migration.core_shell] Migrated checker logic MUST place deterministic validation cores in `tools/checkers/src/checkers/*` or shared library modules, while CLI shells own arguments, filesystem reads, stdout/stderr, and exit-code handling.

#### Scenario: Checker core is testable without filesystem access

r[mc_compatibility.checker_crate_migration.core_shell.testable]
- GIVEN explicit evidence text, modeled file metadata, or parsed fixture input
- WHEN a migrated checker core validates the input
- THEN tests can verify diagnostics without reading files, inspecting environment, spawning processes, or writing output
- AND the CLI shell remains responsible for side effects.

### Requirement: Checker wrapper parity

r[mc_compatibility.checker_crate_migration.wrapper_parity] Checker migration MUST preserve legacy `tools/check_*.rs` command surfaces, flake check names, evidence formats, diagnostics, self-test text, exit-code behavior, and non-claim boundaries unless a separate Cairn changes them.

#### Scenario: Existing checker consumers remain compatible

r[mc_compatibility.checker_crate_migration.wrapper_parity.stable]
- GIVEN an existing checker command, evidence file, or flake check invocation
- WHEN the checker is migrated behind a wrapper
- THEN the invocation, pass/fail result, diagnostics, self-test behavior, and copied evidence outputs remain compatible
- AND no new compatibility or parity claim is introduced by the migration.

### Requirement: Checker migration docs

r[mc_compatibility.checker_crate_migration.docs] The checker crate documentation SHOULD list migrated checker rows and untouched debt rows with owner, reason, non-claim impact, and next action.

#### Scenario: Checker debt is visible

r[mc_compatibility.checker_crate_migration.docs.visible]
- GIVEN a checker remains outside the crate after a migration wave
- WHEN reviewers inspect checker documentation
- THEN the row records why it remains standalone, who owns it, what non-claim impact applies, and what should happen before future behavior changes.

### Requirement: Checker crate migration tests

r[mc_compatibility.checker_crate_migration.tests] Each migrated checker MUST include positive tests for valid evidence and negative tests for malformed input, missing fields, stale baselines, unsafe paths, overclaims, and wrapper drift where applicable.

#### Scenario: Valid checker evidence passes

r[mc_compatibility.checker_crate_migration.tests.positive]
- GIVEN valid representative evidence for a migrated checker
- WHEN the checker core, CLI, and compatibility wrapper process it
- THEN tests prove the expected pass result and report output are produced.

#### Scenario: Invalid checker evidence fails clearly

r[mc_compatibility.checker_crate_migration.tests.negative]
- GIVEN malformed evidence, missing required fields, stale baselines, unsafe paths, overclaim markers, or wrapper drift
- WHEN migrated checker validation runs
- THEN tests prove the checker reports a specific diagnostic and exits unsuccessfully without promoting invalid evidence.

### Requirement: Checker crate migration validation

r[mc_compatibility.checker_crate_migration.validation] Checker migration work MUST record baseline and post-change checker self-tests/current-tree checks, crate tests, wrapper parity checks, affected flake checks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Checker migration closeout is reviewable

r[mc_compatibility.checker_crate_migration.validation.logs]
- GIVEN a checker migration wave is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change checker validation, positive and negative regression coverage, wrapper parity, affected flake checks, Cairn gates, and Cairn validation passing.

### Requirement: Runner shell modularization inventory

r[mc_compatibility.runner_shell_modularization.inventory] Runner shell modularization work MUST inventory current `compat/runner/src/lib.rs` responsibilities, public CLI and dry-run surfaces, receipt schemas, failure-bundle behavior, and baseline validation before extraction.

#### Scenario: Runner shell ownership is reviewable

r[mc_compatibility.runner_shell_modularization.inventory.reviewable]
- GIVEN runner shell modularization is selected
- WHEN reviewers inspect the inventory
- THEN CLI parsing, scenario routing, configuration, orchestration, env patching, receipt writing, failure-bundle writing, backend lifecycle, and public wrapper dependencies are named
- AND baseline validation commands are recorded before core changes.

### Requirement: Runner shell module boundaries

r[mc_compatibility.runner_shell_modularization.module_boundaries] The runner SHOULD keep `lib.rs` as a thin public façade and expose focused modules for CLI parsing, scenario route compatibility, orchestration, environment patch planning, receipt artifact writing, and failure-bundle artifact writing.

#### Scenario: Runner responsibilities have focused owners

r[mc_compatibility.runner_shell_modularization.module_boundaries.focused]
- GIVEN a runner responsibility is reviewed
- WHEN maintainers inspect the runner module tree
- THEN the responsibility is owned by the focused module for its domain
- AND unrelated CLI, environment, receipt, orchestration, and artifact-writing concerns are not reintroduced into one catch-all shell file.

### Requirement: Runner core and shell boundary

r[mc_compatibility.runner_shell_modularization.core_shell] Deterministic runner decisions SHOULD be pure over explicit inputs, while filesystem reads/writes, process execution, Docker lifecycle, sockets, clocks, environment reads, stdout/stderr, and exit-code handling remain in thin shells.

#### Scenario: Runner decisions are testable without side effects

r[mc_compatibility.runner_shell_modularization.core_shell.testable]
- GIVEN explicit CLI arguments, config patches, scenario metadata, and receipt inputs
- WHEN the extracted runner core computes parser, plan, env, receipt, or failure-bundle decisions
- THEN tests can verify the result without touching files, processes, sockets, Docker, clocks, or ambient environment
- AND shells own the side effects.

### Requirement: Runner shell parity

r[mc_compatibility.runner_shell_modularization.parity] Runner shell modularization MUST preserve CLI flags and aliases, flake app behavior, exit-code behavior, receipt schemas, dry-run text, failure-bundle shape, and non-claim boundaries.

#### Scenario: Existing runner command shape remains stable

r[mc_compatibility.runner_shell_modularization.parity.stable]
- GIVEN a supported pre-refactor runner command or wrapper dry-run
- WHEN the modularized runner processes the same input
- THEN the public command shape, dry-run output, receipt schema, failure-bundle fields, and non-claim text remain equivalent
- AND no new live compatibility or semantic parity claim is introduced.

### Requirement: Runner shell modularization tests

r[mc_compatibility.runner_shell_modularization.tests] The change MUST include positive tests for supported parser, planner, environment, receipt, and artifact paths plus negative tests for unknown flags, missing values, unsafe paths, invalid config, stale outputs, and failed preflights.

#### Scenario: Supported runner paths pass

r[mc_compatibility.runner_shell_modularization.tests.positive]
- GIVEN representative supported runner inputs
- WHEN extracted modules process them
- THEN tests prove expected config, plan, env patch, receipt, and artifact decisions are produced.

#### Scenario: Invalid runner paths fail clearly

r[mc_compatibility.runner_shell_modularization.tests.negative]
- GIVEN invalid flags, missing values, unsafe output paths, malformed config, stale generated output, or failed preflight inputs
- WHEN extracted modules process them
- THEN tests prove diagnostics are specific and the runner fails closed without writing misleading evidence.

### Requirement: Runner shell modularization validation

r[mc_compatibility.runner_shell_modularization.validation] The change MUST record runner tests, generated-surface checks when touched, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Runner shell closeout is reviewable

r[mc_compatibility.runner_shell_modularization.validation.logs]
- GIVEN runner shell modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change runner tests, affected dry-runs, positive and negative regression coverage, Cairn gates, and Cairn validation passing.

### Requirement: Stevenarella server modularization inventory

r[mc_compatibility.stevenarella_server_modularization.inventory] Stevenarella server modularization work MUST inventory the current `server/mod.rs` responsibilities, packet handler domains, compat probe state groups, and baseline tests before extraction.

#### Scenario: Server runtime ownership is reviewable

r[mc_compatibility.stevenarella_server_modularization.inventory.reviewable]
- GIVEN Stevenarella server runtime modularization is selected
- WHEN reviewers inspect the inventory
- THEN login/session, world/chunk, entity, inventory/window, block-entity/sign, chat/plugin-message, dispatch, and compat-probe responsibilities are named
- AND baseline validation commands are recorded before core changes.

### Requirement: Stevenarella server module boundaries

r[mc_compatibility.stevenarella_server_modularization.module_boundaries] Stevenarella server packet handling SHOULD expose cohesive modules for login/session, chunks/world, entities, inventory/windows, block entities/signs, chat/plugin messages, and dispatch helpers.

#### Scenario: Packet handler domains have focused owners

r[mc_compatibility.stevenarella_server_modularization.module_boundaries.focused]
- GIVEN a packet handler is reviewed
- WHEN maintainers inspect the server module tree
- THEN the handler belongs to the focused module for its packet/state domain
- AND unrelated packet domains are not reintroduced into one catch-all runtime file.

### Requirement: Stevenarella compat probe state modules

r[mc_compatibility.stevenarella_server_modularization.probe_state] Stevenarella mc-compat probe state SHOULD be grouped into cohesive state modules with pure transition helpers for CTF, inventory, survival, combat/projectile, and sign/dimension behavior.

#### Scenario: Probe decisions are testable without client I/O

r[mc_compatibility.stevenarella_server_modularization.probe_state.testable]
- GIVEN explicit probe input state and an observed packet or tick
- WHEN a probe transition helper evaluates the next action
- THEN tests can verify the decision without network sockets, renderer state, ECS mutation, packet writes, or filesystem access
- AND the `Server` shell remains responsible for side effects.

### Requirement: Stevenarella server parity

r[mc_compatibility.stevenarella_server_modularization.parity] Stevenarella server modularization MUST preserve packet dispatch behavior, compat milestone/event vocabulary, environment variable contracts, receipt non-claims, and default non-instrumented client behavior.

#### Scenario: Existing compatibility rails observe stable output

r[mc_compatibility.stevenarella_server_modularization.parity.stable]
- GIVEN a supported pre-refactor packet, scenario probe, or default client path
- WHEN the modularized server runtime processes the same input
- THEN packet state updates, probe milestones, typed event IDs, and default instrumentation absence remain equivalent
- AND no new gameplay, protocol, or public-server claim is introduced.

### Requirement: Stevenarella server modularization tests

r[mc_compatibility.stevenarella_server_modularization.tests] The change MUST include positive tests for representative extracted handlers/probe transitions and negative tests for malformed packet/probe inputs, invalid state transitions, missing windows/entities, and disabled probes.

#### Scenario: Valid extracted paths pass

r[mc_compatibility.stevenarella_server_modularization.tests.positive]
- GIVEN valid representative server packet or probe inputs
- WHEN extracted modules process them
- THEN tests prove the expected state transition, packet action, or milestone decision is produced.

#### Scenario: Invalid extracted paths fail closed

r[mc_compatibility.stevenarella_server_modularization.tests.negative]
- GIVEN malformed packet data, missing entity/window state, disabled probes, or invalid probe transition inputs
- WHEN extracted modules process them
- THEN tests prove the inputs are ignored, rejected, or diagnosed without panic or stale state promotion.

### Requirement: Stevenarella server modularization validation

r[mc_compatibility.stevenarella_server_modularization.validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Server modularization closeout is reviewable

r[mc_compatibility.stevenarella_server_modularization.validation.logs]
- GIVEN Stevenarella server modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change tests, affected dry-runs, positive and negative regression coverage, Cairn gates, and Cairn validation passing.

### Requirement: Stevenarella hotspot module inventory

r[mc_compatibility.stevenarella_hotspot_modules.inventory] Stevenarella hotspot module reduction work MUST inventory selected `world`, `model`, `ui`, `ecs`, and `control` responsibilities, public APIs, internal consumers, and baseline tests before extraction.

#### Scenario: Hotspot ownership is reviewable

r[mc_compatibility.stevenarella_hotspot_modules.inventory.reviewable]
- GIVEN a Stevenarella hotspot migration wave is selected
- WHEN reviewers inspect the inventory
- THEN the selected modules, public items, responsibility groups, internal consumers, and baseline tests are named
- AND the first migration wave is scoped before core changes.

### Requirement: Stevenarella façade modules

r[mc_compatibility.stevenarella_hotspot_modules.facades] Large Stevenarella `mod.rs` or root modules SHOULD become thin façades over focused child modules while preserving public names where practical or documenting intentional local call-site updates.

#### Scenario: Hotspot modules expose focused children

r[mc_compatibility.stevenarella_hotspot_modules.facades.focused]
- GIVEN a selected hotspot module is reviewed
- WHEN maintainers inspect the module tree
- THEN the root module primarily declares or re-exports focused child modules
- AND unrelated data, rendering, ECS, parser, UI, and shell concerns are not reintroduced into one catch-all module.

### Requirement: Stevenarella hotspot core and shell boundary

r[mc_compatibility.stevenarella_hotspot_modules.core_shell] Deterministic parsing, normalization, layout, ECS planning, and state-transition logic SHOULD be pure over explicit inputs, while renderer, GL, filesystem, network, input, and global state effects remain in shells.

#### Scenario: Extracted hotspot logic is testable without the client runtime

r[mc_compatibility.stevenarella_hotspot_modules.core_shell.testable]
- GIVEN explicit hotspot module inputs such as model data, UI state, world state, ECS query facts, or control actions
- WHEN an extracted pure helper computes a decision
- THEN tests can verify the result without renderer state, GL context, filesystem access, network access, global console state, or event-loop startup
- AND shells own those side effects.

### Requirement: Stevenarella hotspot parity

r[mc_compatibility.stevenarella_hotspot_modules.parity] Hotspot module reduction MUST preserve default client behavior, compat instrumentation boundaries, module API compatibility where practical, protocol behavior, rendering behavior for touched paths, and non-claim boundaries.

#### Scenario: Existing client paths remain stable

r[mc_compatibility.stevenarella_hotspot_modules.parity.stable]
- GIVEN a supported pre-refactor client code path in a touched hotspot module
- WHEN the modularized code processes the same input
- THEN public API behavior, client-visible state, rendering-visible output for touched paths, and instrumentation boundaries remain compatible
- AND no new protocol, rendering correctness, compatibility, or production-readiness claim is introduced.

### Requirement: Stevenarella hotspot module tests

r[mc_compatibility.stevenarella_hotspot_modules.tests] The change MUST include positive tests for extracted pure logic and negative tests for invalid inputs, missing resources, empty collections, malformed state, unsupported layouts, and API drift where applicable.

#### Scenario: Valid hotspot inputs pass

r[mc_compatibility.stevenarella_hotspot_modules.tests.positive]
- GIVEN valid representative inputs for extracted hotspot helpers
- WHEN the helpers process them
- THEN tests prove expected parsing, layout, ECS planning, state transition, or control decisions are produced.

#### Scenario: Invalid hotspot inputs fail safely

r[mc_compatibility.stevenarella_hotspot_modules.tests.negative]
- GIVEN invalid inputs, missing resources, empty collections, malformed state, unsupported layouts, or API drift fixtures
- WHEN extracted hotspot helpers or façade tests process them
- THEN tests prove the code rejects, defaults, or diagnoses the input without panic or hidden state corruption.

### Requirement: Stevenarella hotspot module validation

r[mc_compatibility.stevenarella_hotspot_modules.validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs if instrumentation behavior changes, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Hotspot modularization closeout is reviewable

r[mc_compatibility.stevenarella_hotspot_modules.validation.logs]
- GIVEN a Stevenarella hotspot migration wave is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change focused tests, affected dry-runs when applicable, positive and negative regression coverage, Cairn gates, and Cairn validation passing.

### Requirement: Scenario core family inventory

r[mc_compatibility.scenario_core_family_modules.inventory] Scenario core modularization work MUST inventory scenario families, manifest/generated ownership, live capability contracts, duplicated surfaces, and baseline validation before extraction.

#### Scenario: Scenario family ownership is reviewable

r[mc_compatibility.scenario_core_family_modules.inventory.reviewable]
- GIVEN scenario core modularization is selected
- WHEN reviewers inspect the inventory
- THEN CTF, inventory, survival, combat/projectile/equipment, negative, MCP, and targeted-packet live capability responsibilities are named
- AND generated and hand-authored scenario surfaces are identified with baseline checks.

### Requirement: Scenario family module boundaries

r[mc_compatibility.scenario_core_family_modules.family_boundaries] Scenario behavior SHOULD be split into focused family modules for CTF, inventory, survival, combat/projectile/equipment, negative rails, MCP, and targeted-packet live capability contracts behind a stable central façade.

#### Scenario: Scenario behavior has a focused owner

r[mc_compatibility.scenario_core_family_modules.family_boundaries.focused]
- GIVEN a scenario behavior or contract is reviewed
- WHEN maintainers inspect the scenario module tree
- THEN the behavior belongs to its focused family module
- AND unrelated scenario families are not reintroduced into one catch-all scenario core file.

### Requirement: Scenario manifest parity

r[mc_compatibility.scenario_core_family_modules.manifest_parity] Scenario family extraction MUST preserve parity with `compat/config/scenario-manifest.ncl` and generated surfaces for scenario names, aliases, milestones, forbidden patterns, run strategies, receipt expectations, wrapper metadata, and live capability rows.

#### Scenario: Generated and hand-authored scenario data agree

r[mc_compatibility.scenario_core_family_modules.manifest_parity.fresh]
- GIVEN scenario metadata is split across family modules and generated surfaces
- WHEN generated-surface and scenario validation checks run
- THEN every maintained scenario has matching names, aliases, milestones, forbidden patterns, receipt expectations, and wrapper metadata
- AND stale or missing generated rows fail clearly.

### Requirement: Scenario live capability contracts

r[mc_compatibility.scenario_core_family_modules.live_capabilities] Targeted packet and live capability contracts MUST remain fail-closed with explicit required signals, required non-claims, blocker reasons when blocked, backend/client paths, and validation helpers.

#### Scenario: Live capability rows do not overclaim

r[mc_compatibility.scenario_core_family_modules.live_capabilities.non_overclaiming]
- GIVEN a targeted packet or live capability row is reviewed
- WHEN static validation evaluates the row
- THEN required signals, non-claims, backend/client paths, and blocker reasons are present where required
- AND broad compatibility, semantic equivalence, public-server safety, and production-readiness claims remain absent.

### Requirement: Scenario family module tests

r[mc_compatibility.scenario_core_family_modules.tests] The change MUST include positive tests for representative family lookup and behavior plus negative tests for duplicate aliases, missing manifest rows, invalid live capability rows, unsupported env intents, and stale generated surfaces.

#### Scenario: Valid family metadata passes

r[mc_compatibility.scenario_core_family_modules.tests.positive]
- GIVEN valid representative scenario family metadata
- WHEN lookup, behavior, manifest parity, and live capability checks run
- THEN tests prove the expected scenario API and validation output are produced.

#### Scenario: Invalid family metadata fails clearly

r[mc_compatibility.scenario_core_family_modules.tests.negative]
- GIVEN duplicate aliases, missing rows, unsupported env intents, invalid live capability contracts, or stale generated surfaces
- WHEN scenario validation runs
- THEN tests prove diagnostics name the invalid scenario surface and prevent stale metadata promotion.

### Requirement: Scenario family module validation

r[mc_compatibility.scenario_core_family_modules.validation] The change MUST record scenario tests, generated-surface checks, representative maintained dry-runs, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Scenario family closeout is reviewable

r[mc_compatibility.scenario_core_family_modules.validation.logs]
- GIVEN scenario family modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change scenario tests, generated-surface checks, representative dry-runs, positive and negative regression coverage, Cairn gates, and Cairn validation passing.
