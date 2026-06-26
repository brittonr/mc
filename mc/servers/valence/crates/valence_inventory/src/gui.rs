//! Optional helpers for inventory-backed GUI menus.
//!
//! The helpers in this module build on [`Inventory`], [`OpenInventory`], and
//! [`ClickSlotEvent`]. They do not replace Valence inventory packet handling or
//! claim vanilla container parity.

use std::fmt;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_server::{text::IntoText, Despawned};

use crate::{
    ClickMode, ClickSlotEvent, ClientInventoryState, Inventory, InventoryKind, OpenInventory,
};

/// A named outcome emitted when a GUI slot is clicked.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GuiAction {
    id: String,
}

impl GuiAction {
    /// Creates a GUI action identifier.
    pub fn new<T>(id: T) -> Result<Self, GuiModelError>
    where
        T: Into<String>,
    {
        let id = id.into();
        if id.is_empty() {
            return Err(GuiModelError::EmptyActionId);
        }
        Ok(Self { id })
    }

    /// Returns the stable action identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Configuration for a single GUI slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuiSlot {
    action: Option<GuiAction>,
    readonly: bool,
}

impl GuiSlot {
    /// Creates a passive readonly slot with no emitted action.
    #[must_use]
    pub fn passive() -> Self {
        Self {
            action: None,
            readonly: true,
        }
    }

    /// Creates a readonly action slot.
    pub fn action<T>(action_id: T) -> Result<Self, GuiModelError>
    where
        T: Into<String>,
    {
        Ok(Self {
            action: Some(GuiAction::new(action_id)?),
            readonly: true,
        })
    }

    /// Creates an action slot that allows normal Valence inventory mutation.
    pub fn mutable_action<T>(action_id: T) -> Result<Self, GuiModelError>
    where
        T: Into<String>,
    {
        Ok(Self {
            action: Some(GuiAction::new(action_id)?),
            readonly: false,
        })
    }

    /// Overrides whether the slot should reject inventory mutation.
    #[must_use]
    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Returns whether the slot rejects inventory mutation.
    #[must_use]
    pub fn is_readonly(&self) -> bool {
        self.readonly
    }

    /// Returns the configured action, if any.
    #[must_use]
    pub fn action_id(&self) -> Option<&str> {
        self.action.as_ref().map(GuiAction::id)
    }
}

impl Default for GuiSlot {
    fn default() -> Self {
        Self::passive()
    }
}

/// Pure model for a GUI menu's slots and click policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuiMenuModel {
    slots: Vec<GuiSlot>,
}

impl GuiMenuModel {
    /// Creates a menu model with the requested slot count.
    pub fn new(slot_count: u16) -> Result<Self, GuiModelError> {
        if slot_count == 0 {
            return Err(GuiModelError::EmptyMenu);
        }
        Ok(Self {
            slots: vec![GuiSlot::default(); usize::from(slot_count)],
        })
    }

    /// Returns the number of modeled GUI slots.
    #[must_use]
    pub fn slot_count(&self) -> u16 {
        let Ok(slot_count) = u16::try_from(self.slots.len()) else {
            unreachable!("GUI slot count was constructed from u16")
        };
        slot_count
    }

    /// Returns the slot configuration at `slot`.
    #[must_use]
    pub fn slot(&self, slot: u16) -> Option<&GuiSlot> {
        self.slots.get(usize::from(slot))
    }

    /// Replaces a slot configuration.
    pub fn set_slot(&mut self, slot: u16, config: GuiSlot) -> Result<(), GuiModelError> {
        let slot_count = self.slot_count();
        let Some(target) = self.slots.get_mut(usize::from(slot)) else {
            return Err(GuiModelError::SlotOutOfBounds { slot, slot_count });
        };
        *target = config;
        Ok(())
    }

    /// Iterates over modeled slots.
    pub fn slots(&self) -> impl ExactSizeIterator<Item = &GuiSlot> + Clone + '_ {
        self.slots.iter()
    }
}

/// Errors returned while constructing GUI models.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuiModelError {
    /// A GUI model must contain at least one slot.
    EmptyMenu,
    /// Action identifiers must not be empty.
    EmptyActionId,
    /// The requested slot is outside the modeled slot range.
    SlotOutOfBounds {
        /// Requested slot.
        slot: u16,
        /// Number of slots in the menu model.
        slot_count: u16,
    },
}

impl fmt::Display for GuiModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMenu => f.write_str("GUI menus must contain at least one slot"),
            Self::EmptyActionId => f.write_str("GUI action identifiers must not be empty"),
            Self::SlotOutOfBounds { slot, slot_count } => {
                write!(f, "GUI slot {slot} is outside slot count {slot_count}")
            }
        }
    }
}

impl std::error::Error for GuiModelError {}

/// Tracks the inventory and client window currently associated with a GUI view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiViewerState {
    /// Inventory entity backing the GUI.
    pub inventory: Entity,
    /// Current client window identifier from [`ClientInventoryState`].
    pub window_id: u8,
}

/// Input to the pure GUI open planner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiOpenInput {
    /// Whether the optional GUI plugin is active.
    pub plugin_enabled: bool,
    /// Inventory entity to open.
    pub inventory: Entity,
}

/// Result of evaluating a GUI open request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuiOpenDecision {
    /// The shell should insert [`OpenInventory`] for this inventory.
    Open {
        /// Inventory entity to open.
        inventory: Entity,
    },
    /// The open request was rejected.
    Rejected(GuiRejectReason),
}

/// Purely evaluates a GUI open request.
#[must_use]
pub fn plan_open(input: GuiOpenInput) -> GuiOpenDecision {
    if !input.plugin_enabled {
        return GuiOpenDecision::Rejected(GuiRejectReason::PluginDisabled);
    }
    GuiOpenDecision::Open {
        inventory: input.inventory,
    }
}

/// Input to the pure GUI click planner.
#[derive(Clone, Copy, Debug)]
pub struct GuiClickInput<'a> {
    /// Whether the optional GUI plugin is active.
    pub plugin_enabled: bool,
    /// Menu model that owns slot policy.
    pub menu: &'a GuiMenuModel,
    /// Current GUI viewer state, if any.
    pub viewer: Option<GuiViewerState>,
    /// Inventory entity referenced by the current open inventory component.
    pub inventory: Entity,
    /// Window identifier from the client packet.
    pub window_id: u8,
    /// Slot identifier from the client packet.
    pub slot_id: i16,
    /// Click mode from the client packet.
    pub mode: ClickMode,
}

/// Whether a GUI click should mutate inventory state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuiInventoryMutation {
    /// The GUI helper plans no inventory mutation for this click.
    None,
    /// Normal Valence inventory handling may apply the click mutation.
    AllowValenceInventory,
}

/// Result of evaluating a GUI click.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuiClickOutcome {
    /// A configured GUI action should be emitted.
    Action {
        /// Clicked GUI slot.
        slot: u16,
        /// Configured action.
        action: GuiAction,
    },
    /// The slot was valid but has no action.
    NoAction {
        /// Clicked GUI slot.
        slot: u16,
    },
    /// The click was rejected before any action was emitted.
    Rejected(GuiRejectReason),
}

/// Pure GUI click plan consumed by thin ECS shells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuiClickPlan {
    /// Action, no-op, or rejection outcome.
    pub outcome: GuiClickOutcome,
    /// Inventory mutation policy for this click.
    pub inventory_mutation: GuiInventoryMutation,
}

/// Reasons a GUI transition can be rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuiRejectReason {
    /// The optional GUI plugin is not active.
    PluginDisabled,
    /// The client is not viewing a GUI.
    NoOpenGui,
    /// The client is viewing a different inventory entity.
    WrongInventory {
        /// Expected GUI inventory entity.
        expected: Entity,
        /// Actual open inventory entity.
        actual: Entity,
    },
    /// The click references an old or future window identifier.
    StaleWindow {
        /// Current window identifier.
        expected: u8,
        /// Packet window identifier.
        actual: u8,
    },
    /// The packet slot identifier cannot address a GUI slot.
    InvalidSlot {
        /// Packet slot identifier.
        slot_id: i16,
    },
    /// The slot is outside the modeled GUI range.
    SlotOutOfBounds {
        /// Requested slot.
        slot: u16,
        /// Number of slots in the model.
        slot_count: u16,
    },
}

/// Purely evaluates a GUI click against the model and viewer state.
#[must_use]
pub fn plan_click(input: GuiClickInput<'_>) -> GuiClickPlan {
    if !input.plugin_enabled {
        return rejected_click(GuiRejectReason::PluginDisabled);
    }

    let Some(viewer) = input.viewer else {
        return rejected_click(GuiRejectReason::NoOpenGui);
    };

    if viewer.inventory != input.inventory {
        return rejected_click(GuiRejectReason::WrongInventory {
            expected: viewer.inventory,
            actual: input.inventory,
        });
    }

    if viewer.window_id != input.window_id {
        return rejected_click(GuiRejectReason::StaleWindow {
            expected: viewer.window_id,
            actual: input.window_id,
        });
    }

    let Ok(slot) = u16::try_from(input.slot_id) else {
        return rejected_click(GuiRejectReason::InvalidSlot {
            slot_id: input.slot_id,
        });
    };

    let Some(config) = input.menu.slot(slot) else {
        return rejected_click(GuiRejectReason::SlotOutOfBounds {
            slot,
            slot_count: input.menu.slot_count(),
        });
    };

    let inventory_mutation = if config.is_readonly() {
        GuiInventoryMutation::None
    } else {
        GuiInventoryMutation::AllowValenceInventory
    };

    let Some(action) = config.action.clone() else {
        return GuiClickPlan {
            outcome: GuiClickOutcome::NoAction { slot },
            inventory_mutation,
        };
    };

    GuiClickPlan {
        outcome: GuiClickOutcome::Action { slot, action },
        inventory_mutation,
    }
}

fn rejected_click(reason: GuiRejectReason) -> GuiClickPlan {
    GuiClickPlan {
        outcome: GuiClickOutcome::Rejected(reason),
        inventory_mutation: GuiInventoryMutation::None,
    }
}

/// Why a GUI viewer was closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuiCloseReason {
    /// The client closed the screen or stopped viewing the GUI inventory.
    ClientClosed,
    /// The client entity disconnected or despawned.
    Disconnected,
}

/// Input to close or disconnect cleanup planners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiLifecycleInput {
    /// Current GUI viewer state, if any.
    pub viewer: Option<GuiViewerState>,
}

/// Input to GUI viewer cleanup planning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiCleanupInput {
    /// Whether the optional GUI plugin is active.
    pub plugin_enabled: bool,
    /// Current GUI viewer state, if any.
    pub viewer: Option<GuiViewerState>,
    /// Inventory entity currently referenced by [`OpenInventory`], if any.
    pub open_inventory: Option<Entity>,
    /// Whether Valence has marked the client for explicit despawn finalization.
    pub despawned: bool,
}

/// Result of evaluating GUI lifecycle cleanup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuiLifecycleDecision {
    /// The shell should remove GUI viewer state and emit a close event.
    Close {
        /// Inventory entity that was being viewed.
        inventory: Entity,
        /// Reason for the cleanup.
        reason: GuiCloseReason,
    },
    /// There was no GUI viewer to clean up.
    Noop,
}

/// Plans cleanup after a normal GUI close.
#[must_use]
pub fn plan_close(input: GuiLifecycleInput) -> GuiLifecycleDecision {
    plan_lifecycle(input, GuiCloseReason::ClientClosed)
}

/// Plans cleanup after a disconnect or despawn.
#[must_use]
pub fn plan_disconnect(input: GuiLifecycleInput) -> GuiLifecycleDecision {
    plan_lifecycle(input, GuiCloseReason::Disconnected)
}

/// Plans GUI viewer cleanup from current component state.
#[must_use]
pub fn plan_viewer_cleanup(input: GuiCleanupInput) -> GuiLifecycleDecision {
    if !input.plugin_enabled {
        return GuiLifecycleDecision::Noop;
    }

    let Some(viewer) = input.viewer else {
        return GuiLifecycleDecision::Noop;
    };

    let lifecycle_input = GuiLifecycleInput {
        viewer: Some(viewer),
    };
    if input.despawned {
        return plan_disconnect(lifecycle_input);
    }

    if input.open_inventory == Some(viewer.inventory) {
        return GuiLifecycleDecision::Noop;
    }

    plan_close(lifecycle_input)
}

fn plan_lifecycle(input: GuiLifecycleInput, reason: GuiCloseReason) -> GuiLifecycleDecision {
    let Some(viewer) = input.viewer else {
        return GuiLifecycleDecision::Noop;
    };
    GuiLifecycleDecision::Close {
        inventory: viewer.inventory,
        reason,
    }
}

/// Component marking an inventory as a GUI menu.
#[derive(Component, Clone, Debug)]
pub struct GuiMenu {
    model: GuiMenuModel,
}

impl GuiMenu {
    /// Creates a GUI menu component from a pure model.
    #[must_use]
    pub fn new(model: GuiMenuModel) -> Self {
        Self { model }
    }

    /// Creates a readonly inventory and matching GUI menu component.
    pub fn readonly_inventory<'a, T>(
        kind: InventoryKind,
        title: T,
        model: GuiMenuModel,
    ) -> (Inventory, Self)
    where
        T: IntoText<'a>,
    {
        let mut inventory = Inventory::with_title(kind, title);
        inventory.readonly = true;
        (inventory, Self::new(model))
    }

    /// Returns the pure menu model.
    #[must_use]
    pub fn model(&self) -> &GuiMenuModel {
        &self.model
    }

    /// Returns the mutable pure menu model.
    #[must_use]
    pub fn model_mut(&mut self) -> &mut GuiMenuModel {
        &mut self.model
    }
}

/// Component tracking that a client is viewing a GUI menu.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiViewer {
    inventory: Entity,
}

impl GuiViewer {
    /// Creates GUI viewer state for `inventory`.
    #[must_use]
    pub fn new(inventory: Entity) -> Self {
        Self { inventory }
    }

    /// Returns the GUI inventory entity being viewed.
    #[must_use]
    pub fn inventory(&self) -> Entity {
        self.inventory
    }

    /// Converts the component into a pure viewer state.
    #[must_use]
    pub fn state(&self, window_id: u8) -> GuiViewerState {
        GuiViewerState {
            inventory: self.inventory,
            window_id,
        }
    }
}

/// Request to open a GUI inventory for a client.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiOpenEvent {
    /// Client entity that should view the GUI.
    pub client: Entity,
    /// Inventory entity with a [`GuiMenu`] component.
    pub inventory: Entity,
}

/// Emitted when a GUI click maps to an explicit action.
#[derive(Event, Clone, Debug, PartialEq, Eq)]
pub struct GuiClickEvent {
    /// Client entity that clicked.
    pub client: Entity,
    /// GUI inventory entity.
    pub inventory: Entity,
    /// Clicked GUI slot.
    pub slot: u16,
    /// Packet click mode.
    pub mode: ClickMode,
    /// Configured action.
    pub action: GuiAction,
    /// Inventory mutation policy for the click.
    pub inventory_mutation: GuiInventoryMutation,
}

/// Emitted when a GUI click is rejected by the helper model.
#[derive(Event, Clone, Debug, PartialEq, Eq)]
pub struct GuiRejectedClickEvent {
    /// Client entity that clicked.
    pub client: Entity,
    /// Packet slot identifier.
    pub slot_id: i16,
    /// Packet click mode.
    pub mode: ClickMode,
    /// Rejection reason.
    pub reason: GuiRejectReason,
}

/// Emitted when GUI viewer state is cleaned up.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuiCloseEvent {
    /// Client entity that stopped viewing the GUI.
    pub client: Entity,
    /// GUI inventory entity that had been viewed.
    pub inventory: Entity,
    /// Reason for the cleanup.
    pub reason: GuiCloseReason,
}

/// Optional plugin that connects GUI model transitions to Valence inventory events.
#[derive(Default)]
pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<ClickSlotEvent>>()
            .add_event::<GuiOpenEvent>()
            .add_event::<GuiClickEvent>()
            .add_event::<GuiRejectedClickEvent>()
            .add_event::<GuiCloseEvent>()
            .add_systems(
                Update,
                (open_gui_windows, route_gui_clicks, cleanup_gui_viewers),
            );
    }
}

fn open_gui_windows(
    mut commands: Commands,
    mut events: EventReader<GuiOpenEvent>,
    menus: Query<&GuiMenu>,
) {
    for event in events.read() {
        if menus.get(event.inventory).is_err() {
            continue;
        }

        let decision = plan_open(GuiOpenInput {
            plugin_enabled: true,
            inventory: event.inventory,
        });
        let GuiOpenDecision::Open { inventory } = decision else {
            continue;
        };

        let Some(mut client) = commands.get_entity(event.client) else {
            continue;
        };
        client.insert((OpenInventory::new(inventory), GuiViewer::new(inventory)));
    }
}

fn route_gui_clicks(
    mut clicks: EventReader<ClickSlotEvent>,
    clients: Query<(
        &ClientInventoryState,
        Option<&OpenInventory>,
        Option<&GuiViewer>,
    )>,
    menus: Query<&GuiMenu>,
    mut actions: EventWriter<GuiClickEvent>,
    mut rejections: EventWriter<GuiRejectedClickEvent>,
) {
    for click in clicks.read() {
        let Ok((inventory_state, open_inventory, gui_viewer)) = clients.get(click.client) else {
            continue;
        };
        let Some(gui_viewer) = gui_viewer else {
            continue;
        };
        let Some(open_inventory) = open_inventory else {
            rejections.send(GuiRejectedClickEvent {
                client: click.client,
                slot_id: click.slot_id,
                mode: click.mode,
                reason: GuiRejectReason::NoOpenGui,
            });
            continue;
        };
        let Ok(menu) = menus.get(gui_viewer.inventory()) else {
            continue;
        };

        let plan = plan_click(GuiClickInput {
            plugin_enabled: true,
            menu: menu.model(),
            viewer: Some(gui_viewer.state(inventory_state.window_id())),
            inventory: open_inventory.entity,
            window_id: click.window_id,
            slot_id: click.slot_id,
            mode: click.mode,
        });

        match plan.outcome {
            GuiClickOutcome::Action { slot, action } => {
                actions.send(GuiClickEvent {
                    client: click.client,
                    inventory: gui_viewer.inventory(),
                    slot,
                    mode: click.mode,
                    action,
                    inventory_mutation: plan.inventory_mutation,
                });
            }
            GuiClickOutcome::NoAction { .. } => {}
            GuiClickOutcome::Rejected(reason) => {
                rejections.send(GuiRejectedClickEvent {
                    client: click.client,
                    slot_id: click.slot_id,
                    mode: click.mode,
                    reason,
                });
            }
        }
    }
}

fn cleanup_gui_viewers(
    mut commands: Commands,
    clients: Query<(
        Entity,
        &ClientInventoryState,
        &GuiViewer,
        Option<&OpenInventory>,
        Has<Despawned>,
    )>,
    mut close_events: EventWriter<GuiCloseEvent>,
) {
    for (client_entity, inventory_state, gui_viewer, open_inventory, despawned) in &clients {
        let decision = plan_viewer_cleanup(GuiCleanupInput {
            plugin_enabled: true,
            viewer: Some(gui_viewer.state(inventory_state.window_id())),
            open_inventory: open_inventory.map(|open_inventory| open_inventory.entity),
            despawned,
        });
        let GuiLifecycleDecision::Close { inventory, reason } = decision else {
            continue;
        };
        close_events.send(GuiCloseEvent {
            client: client_entity,
            inventory,
            reason,
        });

        let Some(mut client) = commands.get_entity(client_entity) else {
            continue;
        };
        client.remove::<GuiViewer>();
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::event::Events;
    use valence_server::protocol::packets::play::click_slot_c2s::SlotChange;

    use super::*;
    use crate::ItemStack;

    const ACTION_ID: &str = "open_settings";
    const ACTION_SLOT: u16 = 4;
    const ACTION_SLOT_ID: i16 = 4;
    const CURRENT_WINDOW_ID: u8 = 9;
    const MENU_SLOT_COUNT: u16 = 9;
    const OUT_OF_BOUNDS_SLOT: u16 = 10;
    const OUT_OF_BOUNDS_SLOT_ID: i16 = 10;
    const STALE_WINDOW_ID: u8 = 8;
    const TEST_STATE_ID: i32 = 12;
    const NEGATIVE_SLOT_ID: i16 = -1;
    const PRIMARY_BUTTON: i8 = 0;

    fn test_entities() -> (Entity, Entity) {
        let mut world = World::new();
        let inventory = world.spawn_empty().id();
        let client = world.spawn_empty().id();
        (inventory, client)
    }

    fn action_menu() -> GuiMenuModel {
        let mut menu = GuiMenuModel::new(MENU_SLOT_COUNT).unwrap();
        menu.set_slot(ACTION_SLOT, GuiSlot::action(ACTION_ID).unwrap())
            .unwrap();
        menu
    }

    fn viewer_state(inventory: Entity) -> GuiViewerState {
        GuiViewerState {
            inventory,
            window_id: CURRENT_WINDOW_ID,
        }
    }

    fn click_input<'a>(inventory: Entity, menu: &'a GuiMenuModel) -> GuiClickInput<'a> {
        GuiClickInput {
            plugin_enabled: true,
            menu,
            viewer: Some(viewer_state(inventory)),
            inventory,
            window_id: CURRENT_WINDOW_ID,
            slot_id: ACTION_SLOT_ID,
            mode: ClickMode::Click,
        }
    }

    #[test]
    fn gui_open_enabled_returns_open_plan() {
        let (inventory, _) = test_entities();

        assert_eq!(
            plan_open(GuiOpenInput {
                plugin_enabled: true,
                inventory,
            }),
            GuiOpenDecision::Open { inventory }
        );
    }

    #[test]
    fn gui_open_disabled_is_rejected() {
        let (inventory, _) = test_entities();

        assert_eq!(
            plan_open(GuiOpenInput {
                plugin_enabled: false,
                inventory,
            }),
            GuiOpenDecision::Rejected(GuiRejectReason::PluginDisabled)
        );
    }

    #[test]
    fn gui_readonly_action_click_emits_action_without_mutation() {
        let (inventory, _) = test_entities();
        let menu = action_menu();

        let plan = plan_click(click_input(inventory, &menu));

        assert_eq!(plan.inventory_mutation, GuiInventoryMutation::None);
        assert_eq!(
            plan.outcome,
            GuiClickOutcome::Action {
                slot: ACTION_SLOT,
                action: GuiAction::new(ACTION_ID).unwrap(),
            }
        );
    }

    #[test]
    fn gui_mutable_action_click_allows_valence_inventory_mutation() {
        let (inventory, _) = test_entities();
        let mut menu = GuiMenuModel::new(MENU_SLOT_COUNT).unwrap();
        menu.set_slot(ACTION_SLOT, GuiSlot::mutable_action(ACTION_ID).unwrap())
            .unwrap();

        let plan = plan_click(click_input(inventory, &menu));

        assert_eq!(
            plan.inventory_mutation,
            GuiInventoryMutation::AllowValenceInventory
        );
        assert!(matches!(plan.outcome, GuiClickOutcome::Action { .. }));
    }

    #[test]
    fn gui_click_rejects_stale_window_id() {
        let (inventory, _) = test_entities();
        let menu = action_menu();
        let mut input = click_input(inventory, &menu);
        input.window_id = STALE_WINDOW_ID;

        let plan = plan_click(input);

        assert_eq!(plan.inventory_mutation, GuiInventoryMutation::None);
        assert_eq!(
            plan.outcome,
            GuiClickOutcome::Rejected(GuiRejectReason::StaleWindow {
                expected: CURRENT_WINDOW_ID,
                actual: STALE_WINDOW_ID,
            })
        );
    }

    #[test]
    fn gui_click_rejects_invalid_slot_ids() {
        let (inventory, _) = test_entities();
        let menu = action_menu();
        let mut input = click_input(inventory, &menu);
        input.slot_id = NEGATIVE_SLOT_ID;

        let negative_plan = plan_click(input);

        assert_eq!(
            negative_plan.outcome,
            GuiClickOutcome::Rejected(GuiRejectReason::InvalidSlot {
                slot_id: NEGATIVE_SLOT_ID,
            })
        );

        input.slot_id = OUT_OF_BOUNDS_SLOT_ID;
        let out_of_bounds_plan = plan_click(input);

        assert_eq!(
            out_of_bounds_plan.outcome,
            GuiClickOutcome::Rejected(GuiRejectReason::SlotOutOfBounds {
                slot: OUT_OF_BOUNDS_SLOT,
                slot_count: MENU_SLOT_COUNT,
            })
        );
    }

    #[test]
    fn gui_close_event_plans_cleanup() {
        let (inventory, _) = test_entities();

        assert_eq!(
            plan_close(GuiLifecycleInput {
                viewer: Some(viewer_state(inventory)),
            }),
            GuiLifecycleDecision::Close {
                inventory,
                reason: GuiCloseReason::ClientClosed,
            }
        );
    }

    #[test]
    fn gui_disconnect_plans_cleanup_and_late_clicks_are_rejected() {
        let (inventory, _) = test_entities();
        let menu = action_menu();

        assert_eq!(
            plan_disconnect(GuiLifecycleInput {
                viewer: Some(viewer_state(inventory)),
            }),
            GuiLifecycleDecision::Close {
                inventory,
                reason: GuiCloseReason::Disconnected,
            }
        );

        let input = GuiClickInput {
            plugin_enabled: true,
            menu: &menu,
            viewer: None,
            inventory,
            window_id: CURRENT_WINDOW_ID,
            slot_id: ACTION_SLOT_ID,
            mode: ClickMode::Click,
        };

        assert_eq!(
            plan_click(input).outcome,
            GuiClickOutcome::Rejected(GuiRejectReason::NoOpenGui)
        );
    }

    #[test]
    fn gui_viewer_cleanup_keeps_current_viewer() {
        let (inventory, _) = test_entities();

        let decision = plan_viewer_cleanup(GuiCleanupInput {
            plugin_enabled: true,
            viewer: Some(viewer_state(inventory)),
            open_inventory: Some(inventory),
            despawned: false,
        });

        assert_eq!(decision, GuiLifecycleDecision::Noop);
    }

    #[test]
    fn gui_viewer_cleanup_rejects_missing_owner_and_disabled_plugin() {
        let (inventory, _) = test_entities();

        assert_eq!(
            plan_viewer_cleanup(GuiCleanupInput {
                plugin_enabled: true,
                viewer: None,
                open_inventory: Some(inventory),
                despawned: false,
            }),
            GuiLifecycleDecision::Noop
        );
        assert_eq!(
            plan_viewer_cleanup(GuiCleanupInput {
                plugin_enabled: false,
                viewer: Some(viewer_state(inventory)),
                open_inventory: None,
                despawned: true,
            }),
            GuiLifecycleDecision::Noop
        );
    }

    #[test]
    fn gui_viewer_cleanup_closes_stale_and_despawned_viewers() {
        let (inventory, client) = test_entities();

        assert_eq!(
            plan_viewer_cleanup(GuiCleanupInput {
                plugin_enabled: true,
                viewer: Some(viewer_state(inventory)),
                open_inventory: Some(client),
                despawned: false,
            }),
            GuiLifecycleDecision::Close {
                inventory,
                reason: GuiCloseReason::ClientClosed,
            }
        );
        assert_eq!(
            plan_viewer_cleanup(GuiCleanupInput {
                plugin_enabled: true,
                viewer: Some(viewer_state(inventory)),
                open_inventory: Some(inventory),
                despawned: true,
            }),
            GuiLifecycleDecision::Close {
                inventory,
                reason: GuiCloseReason::Disconnected,
            }
        );
    }

    #[test]
    fn gui_model_rejects_malformed_inputs() {
        assert_eq!(GuiMenuModel::new(0).unwrap_err(), GuiModelError::EmptyMenu);
        assert_eq!(
            GuiAction::new("").unwrap_err(),
            GuiModelError::EmptyActionId
        );
    }

    #[test]
    fn gui_plugin_routes_open_and_click_events() {
        let mut app = App::new();
        app.add_plugins(GuiPlugin);

        let menu = action_menu();
        let (inventory_component, gui_menu) =
            GuiMenu::readonly_inventory(InventoryKind::Generic9x1, "Menu", menu);
        let inventory = app.world_mut().spawn((inventory_component, gui_menu)).id();
        let client = app
            .world_mut()
            .spawn((ClientInventoryState {
                window_id: CURRENT_WINDOW_ID,
                state_id: std::num::Wrapping(TEST_STATE_ID),
                slots_changed: 0,
                client_updated_cursor_item: None,
            },))
            .id();

        app.world_mut()
            .resource_mut::<Events<GuiOpenEvent>>()
            .send(GuiOpenEvent { client, inventory });
        app.update();

        let has_open_inventory = app.world().get::<OpenInventory>(client).is_some();
        let has_gui_viewer = app.world().get::<GuiViewer>(client).is_some();
        assert!(has_open_inventory);
        assert!(has_gui_viewer);

        app.world_mut()
            .resource_mut::<Events<ClickSlotEvent>>()
            .send(ClickSlotEvent {
                client,
                window_id: CURRENT_WINDOW_ID,
                state_id: TEST_STATE_ID,
                slot_id: ACTION_SLOT_ID,
                button: PRIMARY_BUTTON,
                mode: ClickMode::Click,
                slot_changes: Vec::<SlotChange>::new(),
                carried_item: ItemStack::EMPTY,
            });
        app.update();

        let events = app.world().resource::<Events<GuiClickEvent>>();
        let mut reader = events.get_reader();
        let actions: Vec<_> = reader.read(events).collect();
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].client, client);
        assert_eq!(actions[0].inventory, inventory);
        assert_eq!(actions[0].slot, ACTION_SLOT);
        assert_eq!(actions[0].mode, ClickMode::Click);
        assert_eq!(actions[0].action.id(), ACTION_ID);
        assert_eq!(actions[0].inventory_mutation, GuiInventoryMutation::None);
    }

    #[test]
    fn gui_plugin_cleans_up_closed_viewers() {
        let mut app = App::new();
        app.add_plugins(GuiPlugin);

        let inventory = app.world_mut().spawn_empty().id();
        let client = app
            .world_mut()
            .spawn((
                ClientInventoryState {
                    window_id: CURRENT_WINDOW_ID,
                    state_id: std::num::Wrapping(TEST_STATE_ID),
                    slots_changed: 0,
                    client_updated_cursor_item: None,
                },
                GuiViewer::new(inventory),
            ))
            .id();

        app.update();

        assert!(app.world().get::<GuiViewer>(client).is_none());
        let events = app.world().resource::<Events<GuiCloseEvent>>();
        let mut reader = events.get_reader();
        let closes: Vec<_> = reader.read(events).collect();
        assert_eq!(closes.len(), 1);
        assert_eq!(closes[0].client, client);
        assert_eq!(closes[0].inventory, inventory);
        assert_eq!(closes[0].reason, GuiCloseReason::ClientClosed);
    }

    #[test]
    fn gui_plugin_cleans_up_despawned_viewers_once() {
        let mut app = App::new();
        app.add_plugins(GuiPlugin);

        let inventory = app.world_mut().spawn_empty().id();
        let old_client = app
            .world_mut()
            .spawn((
                ClientInventoryState {
                    window_id: CURRENT_WINDOW_ID,
                    state_id: std::num::Wrapping(TEST_STATE_ID),
                    slots_changed: 0,
                    client_updated_cursor_item: None,
                },
                OpenInventory::new(inventory),
                GuiViewer::new(inventory),
                Despawned,
            ))
            .id();
        let reconnect_client = app
            .world_mut()
            .spawn((ClientInventoryState {
                window_id: CURRENT_WINDOW_ID,
                state_id: std::num::Wrapping(TEST_STATE_ID),
                slots_changed: 0,
                client_updated_cursor_item: None,
            },))
            .id();
        let mut reader = app.world().resource::<Events<GuiCloseEvent>>().get_reader();

        app.update();

        assert!(app.world().get::<GuiViewer>(old_client).is_none());
        assert!(app.world().get::<GuiViewer>(reconnect_client).is_none());
        let events = app.world().resource::<Events<GuiCloseEvent>>();
        let closes: Vec<_> = reader.read(events).collect();
        assert_eq!(closes.len(), 1);
        assert_eq!(closes[0].client, old_client);
        assert_eq!(closes[0].inventory, inventory);
        assert_eq!(closes[0].reason, GuiCloseReason::Disconnected);

        app.update();

        let events = app.world().resource::<Events<GuiCloseEvent>>();
        let duplicate_closes: Vec<_> = reader.read(events).collect();
        assert!(duplicate_closes.is_empty());
    }
}
