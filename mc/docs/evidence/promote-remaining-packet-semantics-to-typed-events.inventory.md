# Promote remaining packet semantics to typed events: inventory and contract

## Scope

This evidence records the selected Valence packet-derived semantics promoted in `promote-remaining-packet-semantics-to-typed-events`.

Selected semantics:

- inventory hotbar selection from `UpdateSelectedSlotC2s`;
- inventory click-slot input from `ClickSlotC2s`;
- creative inventory action from `CreativeInventoryActionC2s`;
- handled-screen close from `CloseHandledScreenC2s`;
- command execution from `CommandExecutionC2s`;
- survival chest close fixture consumption of the typed close event.

Non-claims: this does not remove raw `PacketEvent`, promote every serverbound packet, change low-level protocol access, prove vanilla semantic equivalence, prove broad Minecraft compatibility, prove public-server safety, prove production readiness, or claim full survival correctness.

## Inventory

| Selected semantic | Previous raw reader | Packet type | Schedule phase | Mutation or consumer target | Previous malformed/stale behavior |
| --- | --- | --- | --- | --- | --- |
| Hotbar selection | `valence_inventory::hotbar::handle_update_selected_slot` | `UpdateSelectedSlotC2s` | `EventLoopPreUpdate` | `HeldItem`, `UpdateSelectedSlotEvent` | `PacketEvent::decode` returned `None` for wrong IDs, decode errors, and partial decodes; missing `HeldItem` or out-of-range slots skipped mutation. |
| Slot clicks | `valence_inventory::click::handle_packets` | `ClickSlotC2s` | `EventLoopPreUpdate` | inventory/cursor state, `ClickSlotEvent`, `DropItemStackEvent`, resync packets | `PacketEvent::decode` returned `None` for wrong IDs, decode errors, and partial decodes; invalid semantic clicks used existing validation/resync paths. |
| Creative inventory action | `valence_inventory::mode::handle_creative_inventory_action` | `CreativeInventoryActionC2s` | `EventLoopPreUpdate` | player inventory, state ID, slot update packets, creative/drop events | `PacketEvent::decode` returned `None` for wrong IDs, decode errors, and partial decodes; non-creative clients and invalid slots skipped mutation. |
| Handled-screen close | `valence_inventory::handle_close_handled_screen` and `examples/survival_compat::handle_survival_chest_close` | `CloseHandledScreenC2s` | `EventLoopPreUpdate` | `OpenInventory`, `SurvivalOpenContainer`, survival close milestone | `PacketEvent::decode` returned `None` for wrong IDs, decode errors, and partial decodes; stale clients or non-open survival containers skipped cleanup/milestones. |
| Command execution | `valence_command::manager::read_incoming_packets` | `CommandExecutionC2s` | `EventLoopPreUpdate` | `CommandExecutionEvent`, command parser systems | `PacketEvent::decode` returned `None` for wrong IDs, decode errors, and partial decodes; stale raw packet client references could be ignored only by downstream consumers. |
| Inventory player actions | `valence_inventory::control::handle_player_actions` | `PlayerActionC2s` | `EventLoopPreUpdate` | held-item drop/swap behavior | Left as a direct raw reader in this change so standalone `InventoryPlugin` behavior does not require `ActionPlugin`; the existing core `PlayerActionEvent` remains documented separately. |

## Ownership contract

| Typed event | Adapter owner | Source-client check | Timing metadata | Decoded fields | Domain consumer | Raw-packet compatibility |
| --- | --- | --- | --- | --- | --- | --- |
| `UpdateSelectedSlotPacketEvent` | `packet_semantics::emit_update_selected_slot_packet_events` | Requires `ClientInventoryState` on the source entity. | Carries raw packet arrival `Instant`. | Source client and hotbar slot. | `hotbar::handle_update_selected_slot` validates range, mutates `HeldItem`, and emits `UpdateSelectedSlotEvent`. | Raw `PacketEvent` remains readable; raw observers can run in `EventLoopSet::RawPacketObservers`. |
| `ClickSlotPacketEvent` | `packet_semantics::emit_click_slot_packet_events` | Requires `ClientInventoryState` on the source entity. | Carries raw packet arrival `Instant`. | Source client plus owned `ClickSlotC2s<'static>` body. | `click::handle_packets` performs validation, resync, mutation, and public click/drop event emission. | Raw `PacketEvent` remains readable; malformed packet bodies emit no typed event. |
| `CreativeInventoryActionPacketEvent` | `packet_semantics::emit_creative_inventory_action_packet_events` | Requires `ClientInventoryState` on the source entity. | Carries raw packet arrival `Instant`. | Source client, slot, and clicked item. | `mode::handle_creative_inventory_action` checks creative mode and owns inventory/drop mutations. | Raw `PacketEvent` remains readable; unsupported packet semantics stay raw. |
| `CloseHandledScreenEvent` | `packet_semantics::emit_close_handled_screen_events` | Requires `ClientInventoryState` on the source entity. | Carries raw packet arrival `Instant`. | Source client and window ID. | Inventory removes `OpenInventory`; survival fixture removes `SurvivalOpenContainer` and logs the existing close milestone. | Raw `PacketEvent` remains readable and low-level users may still observe the packet. |
| `CommandExecutionPacketEvent` | `manager::emit_command_execution_packet_events` | Requires `CommandScopes` on the source entity. | Carries raw packet arrival `Instant`. | Source client and decoded command string. | `manager::emit_command_execution_events` preserves existing `CommandExecutionEvent`; parser systems consume the public event. | Raw `PacketEvent` remains readable; malformed packet bodies emit no typed event. |

## Validation evidence

- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-pre-gates.run.log` records pre-implementation Cairn proposal, design, tasks, and validation gates.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-baseline.run.log` records the passing pre-edit Valence action, inventory, and command baselines.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-fmt-devshell.run.log` records successful formatting with the mc devshell cargo wrapper.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-focused-final.run.log` records positive typed adapter tests, negative wrong-ID/malformed/partial/stale tests, duplicate-event and raw-access regression tests, and schedule contract tests.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-affected-crates.run.log` records full affected `valence_inventory` and `valence_command` library tests after the migration.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-schedule-dryrun.run.log` records the Valence schedule hygiene flake check and Valence wrapper dry-run.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-task-evidence-first-five.run.log` records task-evidence validation for the first five checked tasks.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-cairn-gates-final.run.log` records final Cairn proposal, design, tasks, and validation gates before the validation task was checked.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-task-evidence-final.run.log` records task-evidence validation after all tasks were checked.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-sync-dry-run.run.log` records the unblocked accepted-spec sync plan.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-sync-execute.run.log` records sync execution; its mutation manifest showed no accepted-spec content change, so the delta was merged manually.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-post-sync-validate.run.log` records Cairn validation after the manual accepted-spec merge and verifies the accepted spec contains `packet_semantic_events` IDs.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-archive-dry-run.run.log` records the unblocked archive plan.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-archive-execute.run.log` records archive execution to `cairn/archive/2026-06-26-promote-remaining-packet-semantics-to-typed-events/`.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-post-archive-validate.run.log` records post-archive Cairn validation and confirms zero remaining active changes.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-post-archive-task-evidence.run.log` records the archived task-evidence check.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-final-closeout.run.log` records final Cairn validation, an empty active change list, and task-evidence after final manifest refresh.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-evidence-manifest-refresh.run.log` records refreshing stale BLAKE3 manifests after accepted spec and Valence README changes.
- `docs/evidence/run-logs/2026-06-26/promote-remaining-packet-semantics-evidence-manifest-check-post-refresh.run.log` records the evidence manifest freshness check after this change's manifest was refreshed.

Fixture behavior note: the survival chest close fixture changed its input source from raw close packet decoding to `CloseHandledScreenEvent`; no live fixture rail was promoted because the change preserves the same close milestone path and does not alter scenario timing, required milestones, forbidden milestones, or non-claim fields.
