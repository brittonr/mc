# Model-owned entity relationships with hierarchy inventory

## Scope

This evidence note records the inventory, classification, wiring decision, compatibility boundary, and validation plan for `model-owned-entity-relationships-with-hierarchy`. It supports `r[valence_bevy_ecs.entity_hierarchy.inventory]`, `r[valence_bevy_ecs.entity_hierarchy.classification]`, `r[valence_bevy_ecs.entity_hierarchy.wiring]`, `r[valence_bevy_ecs.entity_hierarchy.compatibility]`, `r[valence_bevy_ecs.entity_hierarchy.tests]`, and `r[valence_bevy_ecs.entity_hierarchy.validation]`.

Hyperion code and concepts were not used for this Valence change; no adopt, port, reference, or reject classification is needed beyond this note.

## Relationship inventory

| Relationship | Owner | Child lifecycle | Traversal need | Current representation | Cleanup behavior | Schedule impact | Evidence impact |
| --- | --- | --- | --- | --- | --- | --- | --- |
| GUI client view to backing GUI inventory | Client entity | The client owns the view relationship; the backing inventory can outlive a viewer and can be shared by multiple clients. | Systems need direct client-to-view lookup, not tree traversal. | `GuiViewer` component on the client plus `OpenInventory` pointing at the inventory entity. | `GuiPlugin` emits `GuiCloseEvent` and removes `GuiViewer` when the view closes, is replaced, or the client is despawned. | No new plugin, schedule label, system set, ordering edge, or default plugin membership change. | Focused `valence_inventory` GUI tests and Cairn/evidence gates are sufficient. |
| Client open inventory relationship | Client entity | The client owns the currently open inventory window; the inventory entity may be a player inventory, container, GUI menu, or shared resource. | Window sync needs current `ClientInventoryState` and packet context, not descendant traversal. | `OpenInventory` component with an inventory entity reference. | Existing inventory cleanup removes stale opens and sends close/update packets before flush. | No schedule change in this pass. | Covered by existing inventory tests plus focused GUI relationship tests. |
| Player inventory state | Client entity | Player inventory state is part of the client entity lifecycle. | Queried directly with the client entity. | `Inventory`, `ClientInventoryState`, `CursorItem`, and `HeldItem` components. | Removed by normal component/entity lifecycle. | No schedule change in this pass. | Existing inventory tests remain the coverage boundary. |
| Layer membership, entity IDs, and protocol IDs | Valence server/world state | Membership and IDs are indexes or protocol identity, not owned child entities. | Lookup/index semantics are required. | Resources, layer data, and protocol identifiers. | Existing layer/entity cleanup remains explicit. | Not changed. | Non-claim; no broad layer/entity-ID migration is made. |
| Advancement tree data | `valence_advancement` | Advancement nodes already form a tree. | Tree traversal is natural. | Existing Bevy hierarchy usage in the advancement crate. | Existing advancement behavior remains out of scope. | Not changed. | Reference-only example of hierarchy-suitable data. |

## Classification

| Relationship | Classification | Rationale |
| --- | --- | --- |
| GUI client view to backing GUI inventory | Explicit relationship component | The relationship is client-owned, but the inventory can be shared by many viewers, so it is not a Bevy parent/child tree. The component keeps ownership visible on the client entity and lets cleanup validate the current `OpenInventory`. |
| Client open inventory relationship | Explicit relationship component | The protocol window relationship is tied to `ClientInventoryState`, packet emission, and stale target validation. Hierarchy would obscure protocol semantics. |
| Player inventory state | Entity-owned component data | State belongs directly to the client entity and already follows component lifecycle. |
| Layer membership, entity IDs, and protocol IDs | Resource/index or external identity | These relationships are indexes or externally visible identifiers. They are intentionally not hidden behind hierarchy. |
| Advancement tree data | Hierarchy-suitable, already implemented | This remains the existing natural tree case and is not modified by this change. |

## Wiring decision

This change keeps GUI ownership as an explicit client component and tightens duplicate-viewer behavior. Opening a different GUI for a client now plans and emits one `GuiCloseEvent` for the previous GUI relationship before inserting the new `GuiViewer` and `OpenInventory` pair. Opening the same GUI again remains an idempotent open relationship.

The pure core is `plan_open_transition`: it receives the requested inventory and the current GUI relationship and returns `Open`, `Replace`, or `Rejected`. The ECS shell reads events and components, emits close events for `Replace`, and inserts the explicit relationship components.

## Compatibility and non-claims

The change does not add Bevy hierarchy dependencies, recursive despawn behavior, default plugin membership, schedule labels, ordering edges, layer/entity-ID semantics, live compatibility evidence, vanilla parity, public-server safety, or production-readiness claims. GUI helper behavior remains optional; keeping `GuiPlugin` disabled still leaves lower-level inventory APIs unchanged.

Selected mc-compat live rails are not applicable because no live runner fixture, protocol rail, or compatibility milestone changed.

## Focused tests and negative fixtures

The focused GUI test plan covers:

- positive GUI open and action lifecycle;
- positive replacement planning for a client that opens a different GUI;
- negative plugin-disabled open and cleanup planning;
- negative stale window rejection;
- negative orphan viewer cleanup when `OpenInventory` is absent;
- negative invalid relationship rejection when a click references the wrong open inventory;
- negative invalid slot rejection;
- negative duplicate viewer cleanup that emits one close event for the previous GUI and keeps the new relationship;
- negative despawn cleanup that emits one disconnect close and does not duplicate it.

## Validation evidence plan

Task closeout should cite promoted logs under `docs/evidence/` for the pre-implementation Cairn gates, focused baseline GUI tests, focused post-change GUI tests, formatting, Cairn gates, Cairn validation, task-evidence validation, sync/archive receipts, accepted spec verification, post-archive validation, and BLAKE3 manifest freshness.
