#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    Red,
    Blue,
}

impl Team {
    pub fn label(self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Blue => "Blue",
        }
    }

    pub fn opponent(self) -> Self {
        match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Red,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlagPresence {
    AtBase,
    Held,
}

impl FlagPresence {
    pub fn label(self) -> &'static str {
        match self {
            Self::AtBase => "at_base",
            Self::Held => "held",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScoreSnapshot {
    pub red: u32,
    pub blue: u32,
}

impl ScoreSnapshot {
    pub fn for_team(self, team: Team) -> u32 {
        match team {
            Team::Red => self.red,
            Team::Blue => self.blue,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlagSnapshot {
    pub red: FlagPresence,
    pub blue: FlagPresence,
}

impl FlagSnapshot {
    pub fn for_team(self, team: Team) -> FlagPresence {
        match team {
            Team::Red => self.red,
            Team::Blue => self.blue,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlagPickupDecision {
    Accept,
    RejectOwnFlag,
    RejectAlreadyHeld,
}

pub fn evaluate_flag_pickup(
    player_team: Team,
    flag_team: Team,
    flag_presence: FlagPresence,
) -> FlagPickupDecision {
    if player_team == flag_team {
        return FlagPickupDecision::RejectOwnFlag;
    }
    if flag_presence == FlagPresence::Held {
        return FlagPickupDecision::RejectAlreadyHeld;
    }
    FlagPickupDecision::Accept
}

pub fn race_duplicate_pickup_blocked(accepted_username_present: bool) -> bool {
    accepted_username_present
}

pub fn race_accepted_transition_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
    accepted_transition: &str,
    race_window_ticks: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_race_accepted_transition username={} player_team={} \
         flag_team={} transition={} race_window_ticks={}",
        username,
        player_team.label(),
        flag_team.label(),
        accepted_transition,
        race_window_ticks
    )
}

pub fn race_rejected_transition_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
    rejected_transition: &str,
    race_window_ticks: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_race_rejected_transition username={} player_team={} \
         flag_team={} transition={} reason=flag_already_held race_window_ticks={}",
        username,
        player_team.label(),
        flag_team.label(),
        rejected_transition,
        race_window_ticks
    )
}

pub struct RaceFinalContract<'a> {
    pub expected_capture_team: Team,
    pub expected_carried_flag: Team,
    pub expected_score: ScoreSnapshot,
    pub expected_flag_state: FlagPresence,
    pub flag_state_label: &'a str,
    pub race_window_ticks: u32,
    pub accepted_transition: &'a str,
    pub rejected_transition: &'a str,
}

pub fn race_final_state_milestone(
    accepted_username: &str,
    rejected_username: &str,
    capture_username: &str,
    capture_team: Team,
    carried_flag: Team,
    score: ScoreSnapshot,
    flags: FlagSnapshot,
    contract: RaceFinalContract<'_>,
) -> Option<String> {
    if capture_team != contract.expected_capture_team {
        return None;
    }
    if carried_flag != contract.expected_carried_flag {
        return None;
    }
    if score != contract.expected_score {
        return None;
    }
    if flags.for_team(carried_flag) != contract.expected_flag_state {
        return None;
    }
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_race_final_state capture_username={} \
         accepted_username={} rejected_username={} capture_team={} carried_flag={} \
         final_blue_flag_state={} red_score={} blue_score={} race_window_ticks={} \
         accepted_transition={} rejected_transition={}",
        capture_username,
        accepted_username,
        rejected_username,
        capture_team.label(),
        carried_flag.label(),
        contract.flag_state_label,
        score.red,
        score.blue,
        contract.race_window_ticks,
        contract.accepted_transition,
        contract.rejected_transition
    ))
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SpawnResetState {
    pub red_count: u32,
    pub blue_count: u32,
    pub red_username: Option<String>,
    pub blue_username: Option<String>,
}

impl SpawnResetState {
    pub fn record_assignment(&mut self, username: &str, team: Team) {
        match team {
            Team::Red => {
                self.red_count += 1;
                self.red_username = Some(username.to_owned());
            }
            Team::Blue => {
                self.blue_count += 1;
                self.blue_username = Some(username.to_owned());
            }
        }
    }
}

pub struct SpawnResetContract {
    pub expected_red_count: u32,
    pub expected_blue_count: u32,
    pub expected_blue_username: &'static str,
    pub reset_score: ScoreSnapshot,
    pub slot36_resource: &'static str,
    pub red_slot37_resource: &'static str,
    pub blue_slot37_resource: &'static str,
    pub reset_slot37_resource: &'static str,
    pub reset_state: &'static str,
}

pub fn spawn_team_assignment_milestone(
    username: &str,
    team: Team,
    red_count: u32,
    blue_count: u32,
    slot36_resource: &str,
    slot37_resource: &str,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_spawn_team_assignment username={} team={} red_count={} \
         blue_count={} spawn={} slot36={} slot37={}",
        username,
        team.label(),
        red_count,
        blue_count,
        team.label().to_ascii_lowercase(),
        slot36_resource,
        slot37_resource
    )
}

pub fn spawn_team_balance_milestone(
    state: &SpawnResetState,
    contract: &SpawnResetContract,
) -> Option<String> {
    if state.red_count != contract.expected_red_count {
        return None;
    }
    if state.blue_count != contract.expected_blue_count {
        return None;
    }
    let red_username = state.red_username.as_deref()?;
    let blue_username = state.blue_username.as_deref()?;
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_spawn_team_balance red_count={} blue_count={} \
         selected_teams={}:Red,{}:Blue",
        state.red_count, state.blue_count, red_username, blue_username
    ))
}

pub fn spawn_resource_reset_state_milestone(
    state: &SpawnResetState,
    username: &str,
    team: Team,
    reset_flag: Team,
    score: ScoreSnapshot,
    contract: &SpawnResetContract,
) -> Option<String> {
    if state.red_count != contract.expected_red_count {
        return None;
    }
    if state.blue_count != contract.expected_blue_count {
        return None;
    }
    if state.blue_username.as_deref() != Some(contract.expected_blue_username) {
        return None;
    }
    if score != contract.reset_score {
        return None;
    }
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_spawn_resource_reset_state username={} team={} \
         reset_flag={} red_score={} blue_score={} slot36={} slot37={} reset_state={}",
        username,
        team.label(),
        reset_flag.label(),
        score.red,
        score.blue,
        contract.slot36_resource,
        contract.reset_slot37_resource,
        contract.reset_state
    ))
}

pub fn defer_spawn_assignment(username: &str, team: Team, expected_blue_username: &str) -> bool {
    username == expected_blue_username && team == Team::Red
}

pub fn invalid_flag_pickup_rejection_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
    pre_owner: &str,
    post_owner: &str,
    score: ScoreSnapshot,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE invalid_flag_pickup_rejected username={} player_team={} \
         flag_team={} pre_owner={} post_owner={} red_score={} blue_score={} \
         outcome=no_owner_transfer_no_score",
        username,
        player_team.label(),
        flag_team.label(),
        pre_owner,
        post_owner,
        score.red,
        score.blue
    )
}

pub fn invalid_return_drop_rejection_milestone(
    milestone: &str,
    username: &str,
    actor_team: Team,
    flag_team: Team,
    pre_state: &str,
    post_state: &str,
    score: ScoreSnapshot,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE {} username={} actor_team={} flag_team={} pre_state={} \
         post_state={} red_score={} blue_score={} outcome=no_flag_state_mutation_no_score",
        milestone,
        username,
        actor_team.label(),
        flag_team.label(),
        pre_state,
        post_state,
        score.red,
        score.blue
    )
}

pub fn score_limit_pre_state_milestone(score: ScoreSnapshot, score_limit: u32) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_pre_state score_limit={} red_score={} blue_score={}",
        score_limit, score.red, score.blue
    )
}

pub fn score_limit_final_capture_milestone(
    username: &str,
    capture_team: Team,
    carried_flag: Team,
    before: ScoreSnapshot,
    after: ScoreSnapshot,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_final_capture username={} capture_team={} \
         carried_flag={} red_score_before={} blue_score_before={} red_score_after={} \
         blue_score_after={}",
        username,
        capture_team.label(),
        carried_flag.label(),
        before.red,
        before.blue,
        after.red,
        after.blue
    )
}

pub fn score_limit_win_condition_milestone(
    username: &str,
    winning_team: Team,
    score: ScoreSnapshot,
    win_emissions: u32,
    score_limit: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_win_condition username={} winning_team={} \
         red_score={} blue_score={} score_limit={} end_state=winner_declared win_emissions={} \
         duplicate_win=false post_win_score_delta=0",
        username,
        winning_team.label(),
        score.red,
        score.blue,
        score_limit,
        win_emissions
    )
}

pub fn score_limit_duplicate_win_milestone(username: &str, winning_team: Team) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_duplicate_win username={} winning_team={} \
         outcome=forbidden_duplicate_win",
        username,
        winning_team.label()
    )
}

pub fn score_limit_post_win_score_mutation_milestone(username: &str, winning_team: Team) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_post_win_score_mutation username={} winning_team={} \
         outcome=forbidden_score_after_win",
        username,
        winning_team.label()
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryProbeItem {
    ExpectedStackItem,
    Other,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryClickMode {
    Click,
    Drag,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryItemStack {
    pub item: InventoryProbeItem,
    pub count: i8,
}

impl InventoryItemStack {
    pub fn empty(empty_count: i8) -> Self {
        Self {
            item: InventoryProbeItem::Empty,
            count: empty_count,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventorySlotChange {
    pub slot: i16,
    pub stack: InventoryItemStack,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryClickSnapshot {
    pub actor_matches: bool,
    pub window_id: u8,
    pub slot_id: i16,
    pub button: i8,
    pub mode: InventoryClickMode,
    pub carried_item: InventoryItemStack,
    pub slot_changes: Vec<InventorySlotChange>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryStackContract {
    pub window_id: u8,
    pub source_slot: i16,
    pub destination_slot: i16,
    pub full_count: i8,
    pub half_count: i8,
    pub empty_count: i8,
    pub left_button: i8,
    pub right_button: i8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InventoryStackState {
    pub split_pickup_seen: bool,
    pub split_place_seen: bool,
    pub merge_pickup_seen: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryStackAction {
    SplitPickup,
    SplitPlace,
    MergePickup,
    MergePlace,
}

pub fn classify_inventory_stack_split_merge_event(
    event: &InventoryClickSnapshot,
    state: InventoryStackState,
    contract: InventoryStackContract,
) -> Option<InventoryStackAction> {
    if !event.actor_matches || event.window_id != contract.window_id {
        return None;
    }
    if event.mode != InventoryClickMode::Click {
        return None;
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.source_slot,
        InventoryProbeItem::ExpectedStackItem,
        contract.half_count,
    ) && event.slot_id == contract.source_slot
        && event.button == contract.right_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.half_count
        && !state.split_pickup_seen
    {
        return Some(InventoryStackAction::SplitPickup);
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.destination_slot,
        InventoryProbeItem::ExpectedStackItem,
        contract.half_count,
    ) && event.slot_id == contract.destination_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::Empty
        && event.carried_item.count == contract.empty_count
        && state.split_pickup_seen
        && !state.split_place_seen
    {
        return Some(InventoryStackAction::SplitPlace);
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.destination_slot,
        InventoryProbeItem::Empty,
        contract.empty_count,
    ) && event.slot_id == contract.destination_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.half_count
        && state.split_place_seen
        && !state.merge_pickup_seen
    {
        return Some(InventoryStackAction::MergePickup);
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.source_slot,
        InventoryProbeItem::ExpectedStackItem,
        contract.full_count,
    ) && event.slot_id == contract.source_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::Empty
        && event.carried_item.count == contract.empty_count
        && state.merge_pickup_seen
    {
        return Some(InventoryStackAction::MergePlace);
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryDragContract {
    pub window_id: u8,
    pub source_slot: i16,
    pub target_slot_a: i16,
    pub target_slot_b: i16,
    pub outside_slot: i16,
    pub full_count: i8,
    pub half_count: i8,
    pub empty_count: i8,
    pub left_button: i8,
    pub drag_start_button: i8,
    pub drag_add_slot_button: i8,
    pub drag_end_button: i8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InventoryDragState {
    pub pickup_seen: bool,
    pub drag_start_seen: bool,
    pub target_a_seen: bool,
    pub target_b_seen: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryDragAction {
    PickupSource,
    DragStart,
    AddTargetA,
    AddTargetB,
    DragEnd,
}

pub fn classify_inventory_drag_transactions_event(
    event: &InventoryClickSnapshot,
    state: InventoryDragState,
    contract: InventoryDragContract,
) -> Option<InventoryDragAction> {
    if !event.actor_matches || event.window_id != contract.window_id {
        return None;
    }
    if event.mode == InventoryClickMode::Click
        && event.slot_id == contract.source_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && inventory_stack_slot_change_matches(
            event,
            contract.source_slot,
            InventoryProbeItem::Empty,
            contract.empty_count,
        )
        && !state.pickup_seen
    {
        return Some(InventoryDragAction::PickupSource);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.outside_slot
        && event.button == contract.drag_start_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && event.slot_changes.is_empty()
        && state.pickup_seen
        && !state.drag_start_seen
    {
        return Some(InventoryDragAction::DragStart);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.target_slot_a
        && event.button == contract.drag_add_slot_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && event.slot_changes.is_empty()
        && state.drag_start_seen
        && !state.target_a_seen
    {
        return Some(InventoryDragAction::AddTargetA);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.target_slot_b
        && event.button == contract.drag_add_slot_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && event.slot_changes.is_empty()
        && state.target_a_seen
        && !state.target_b_seen
    {
        return Some(InventoryDragAction::AddTargetB);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.outside_slot
        && event.button == contract.drag_end_button
        && event.carried_item.item == InventoryProbeItem::Empty
        && event.carried_item.count == contract.empty_count
        && inventory_stack_slot_change_matches(
            event,
            contract.target_slot_a,
            InventoryProbeItem::ExpectedStackItem,
            contract.half_count,
        )
        && inventory_stack_slot_change_matches(
            event,
            contract.target_slot_b,
            InventoryProbeItem::ExpectedStackItem,
            contract.half_count,
        )
        && state.target_b_seen
    {
        return Some(InventoryDragAction::DragEnd);
    }
    None
}

fn inventory_stack_slot_change_matches(
    event: &InventoryClickSnapshot,
    slot: i16,
    item: InventoryProbeItem,
    count: i8,
) -> bool {
    event.slot_changes.iter().any(|change| {
        change.slot == slot && change.stack.item == item && change.stack.count == count
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorState {
    DiamondChestplate,
    Other,
    Empty,
}

pub fn reference_hit_for(
    enabled: bool,
    attacker: &str,
    victim: &str,
    expected_attacker: &str,
    expected_victim: &str,
) -> bool {
    enabled && attacker == expected_attacker && victim == expected_victim
}

pub fn combat_armor_mitigation_for(
    reference_probe_enabled: bool,
    armor_probe_enabled: bool,
    chest_item: ArmorState,
    base_damage: f32,
    compatibility_mitigation: f32,
    diamond_armor_points: f32,
    diamond_toughness: f32,
) -> f32 {
    if reference_probe_enabled && chest_item == ArmorState::DiamondChestplate {
        return vanilla_armor_mitigation_for(base_damage, diamond_armor_points, diamond_toughness);
    }
    if armor_probe_enabled && chest_item == ArmorState::DiamondChestplate {
        return compatibility_mitigation;
    }
    0.0
}

pub fn vanilla_armor_mitigation_for(base_damage: f32, armor_points: f32, toughness: f32) -> f32 {
    let toughness_reduction = armor_points
        - base_damage
            / (toughness / VANILLA_ARMOR_TOUGHNESS_QUARTER_DIVISOR + VANILLA_ARMOR_TOUGHNESS_BASE);
    let minimum_reduction = armor_points / VANILLA_ARMOR_MIN_REDUCTION_DIVISOR;
    let reduction_points = toughness_reduction
        .max(minimum_reduction)
        .min(VANILLA_ARMOR_MAX_REDUCTION_POINTS);
    base_damage * reduction_points / VANILLA_COMBAT_ARMOR_REDUCTION_DENOMINATOR
}

pub fn knockback_metric(knockback_velocity: [f32; KNOCKBACK_VECTOR_COMPONENT_COUNT]) -> f64 {
    f64::from(knockback_velocity[KNOCKBACK_X_COMPONENT])
        .hypot(f64::from(knockback_velocity[KNOCKBACK_Z_COMPONENT]))
        / VANILLA_COMBAT_REFERENCE_KNOCKBACK_SCALE
}

const KNOCKBACK_VECTOR_COMPONENT_COUNT: usize = 3;
const KNOCKBACK_X_COMPONENT: usize = 0;
const KNOCKBACK_Z_COMPONENT: usize = 2;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_SCALE: f64 = 20.0;
const VANILLA_COMBAT_ARMOR_REDUCTION_DENOMINATOR: f32 = 25.0;
const VANILLA_ARMOR_TOUGHNESS_QUARTER_DIVISOR: f32 = 4.0;
const VANILLA_ARMOR_TOUGHNESS_BASE: f32 = 2.0;
const VANILLA_ARMOR_MIN_REDUCTION_DIVISOR: f32 = 5.0;
const VANILLA_ARMOR_MAX_REDUCTION_POINTS: f32 = 20.0;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_USER_A: &str = "compatbota";
    const TEST_USER_B: &str = "compatbotb";
    const TEST_OTHER_USER: &str = "compatbotc";
    const TEST_STACK_WINDOW: u8 = 0;
    const TEST_STACK_SOURCE_SLOT: i16 = 37;
    const TEST_STACK_DESTINATION_SLOT: i16 = 38;
    const TEST_DRAG_TARGET_A: i16 = 38;
    const TEST_DRAG_TARGET_B: i16 = 39;
    const TEST_DRAG_OUTSIDE_SLOT: i16 = -999;
    const TEST_STACK_FULL_COUNT: i8 = 64;
    const TEST_STACK_HALF_COUNT: i8 = 32;
    const TEST_STACK_EMPTY_COUNT: i8 = 0;
    const TEST_LEFT_BUTTON: i8 = 0;
    const TEST_RIGHT_BUTTON: i8 = 1;
    const TEST_DRAG_START_BUTTON: i8 = 0;
    const TEST_DRAG_ADD_SLOT_BUTTON: i8 = 1;
    const TEST_DRAG_END_BUTTON: i8 = 2;
    const TEST_SCORE_LIMIT: u32 = 2;
    const TEST_RACE_WINDOW_TICKS: u32 = 40;
    const TEST_COMPAT_MITIGATION: f32 = 2.0;
    const TEST_DIAMOND_ARMOR_POINTS: f32 = 8.0;
    const TEST_DIAMOND_TOUGHNESS: f32 = 2.0;
    const TEST_BASE_DAMAGE: f32 = 6.0;
    const TEST_REFERENCE_MITIGATION: f32 = 1.344;
    const TEST_FLOAT_TOLERANCE: f32 = 0.0001;
    const TEST_KNOCKBACK_X: f32 = 8.0;
    const TEST_KNOCKBACK_Y: f32 = 6.432;
    const TEST_KNOCKBACK_Z: f32 = 0.0;
    const TEST_NORMALIZED_KNOCKBACK: f64 = 0.40;

    fn stack_contract() -> InventoryStackContract {
        InventoryStackContract {
            window_id: TEST_STACK_WINDOW,
            source_slot: TEST_STACK_SOURCE_SLOT,
            destination_slot: TEST_STACK_DESTINATION_SLOT,
            full_count: TEST_STACK_FULL_COUNT,
            half_count: TEST_STACK_HALF_COUNT,
            empty_count: TEST_STACK_EMPTY_COUNT,
            left_button: TEST_LEFT_BUTTON,
            right_button: TEST_RIGHT_BUTTON,
        }
    }

    fn drag_contract() -> InventoryDragContract {
        InventoryDragContract {
            window_id: TEST_STACK_WINDOW,
            source_slot: TEST_STACK_SOURCE_SLOT,
            target_slot_a: TEST_DRAG_TARGET_A,
            target_slot_b: TEST_DRAG_TARGET_B,
            outside_slot: TEST_DRAG_OUTSIDE_SLOT,
            full_count: TEST_STACK_FULL_COUNT,
            half_count: TEST_STACK_HALF_COUNT,
            empty_count: TEST_STACK_EMPTY_COUNT,
            left_button: TEST_LEFT_BUTTON,
            drag_start_button: TEST_DRAG_START_BUTTON,
            drag_add_slot_button: TEST_DRAG_ADD_SLOT_BUTTON,
            drag_end_button: TEST_DRAG_END_BUTTON,
        }
    }

    fn item(count: i8) -> InventoryItemStack {
        InventoryItemStack {
            item: InventoryProbeItem::ExpectedStackItem,
            count,
        }
    }

    fn click(
        slot_id: i16,
        button: i8,
        carried_item: InventoryItemStack,
        slot_changes: Vec<InventorySlotChange>,
    ) -> InventoryClickSnapshot {
        InventoryClickSnapshot {
            actor_matches: true,
            window_id: TEST_STACK_WINDOW,
            slot_id,
            button,
            mode: InventoryClickMode::Click,
            carried_item,
            slot_changes,
        }
    }

    fn drag(
        slot_id: i16,
        button: i8,
        carried_item: InventoryItemStack,
        slot_changes: Vec<InventorySlotChange>,
    ) -> InventoryClickSnapshot {
        InventoryClickSnapshot {
            actor_matches: true,
            window_id: TEST_STACK_WINDOW,
            slot_id,
            button,
            mode: InventoryClickMode::Drag,
            carried_item,
            slot_changes,
        }
    }

    fn slot(slot: i16, stack: InventoryItemStack) -> InventorySlotChange {
        InventorySlotChange { slot, stack }
    }

    #[test]
    fn flag_pickup_accepts_enemy_base_flag_and_rejects_wrong_team_or_duplicate() {
        assert_eq!(
            evaluate_flag_pickup(Team::Red, Team::Blue, FlagPresence::AtBase),
            FlagPickupDecision::Accept
        );
        assert_eq!(
            evaluate_flag_pickup(Team::Red, Team::Red, FlagPresence::AtBase),
            FlagPickupDecision::RejectOwnFlag
        );
        assert_eq!(
            evaluate_flag_pickup(Team::Red, Team::Blue, FlagPresence::Held),
            FlagPickupDecision::RejectAlreadyHeld
        );
    }

    #[test]
    fn race_and_score_milestones_preserve_receipt_strings_and_fail_closed() {
        let accepted = race_accepted_transition_milestone(
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            "pickup",
            TEST_RACE_WINDOW_TICKS,
        );
        let rejected = race_rejected_transition_milestone(
            TEST_USER_B,
            Team::Red,
            Team::Blue,
            "duplicate_pickup",
            TEST_RACE_WINDOW_TICKS,
        );
        let final_contract = RaceFinalContract {
            expected_capture_team: Team::Red,
            expected_carried_flag: Team::Blue,
            expected_score: ScoreSnapshot { red: 1, blue: 0 },
            expected_flag_state: FlagPresence::AtBase,
            flag_state_label: "at_base",
            race_window_ticks: TEST_RACE_WINDOW_TICKS,
            accepted_transition: "pickup",
            rejected_transition: "duplicate_pickup",
        };
        let final_state = race_final_state_milestone(
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            ScoreSnapshot { red: 1, blue: 0 },
            FlagSnapshot {
                red: FlagPresence::AtBase,
                blue: FlagPresence::AtBase,
            },
            final_contract,
        )
        .expect("valid final state should emit milestone");
        let rejected_final_contract = RaceFinalContract {
            expected_capture_team: Team::Red,
            expected_carried_flag: Team::Blue,
            expected_score: ScoreSnapshot { red: 1, blue: 0 },
            expected_flag_state: FlagPresence::AtBase,
            flag_state_label: "at_base",
            race_window_ticks: TEST_RACE_WINDOW_TICKS,
            accepted_transition: "pickup",
            rejected_transition: "duplicate_pickup",
        };
        let rejected_final_state = race_final_state_milestone(
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            ScoreSnapshot { red: 2, blue: 0 },
            FlagSnapshot {
                red: FlagPresence::AtBase,
                blue: FlagPresence::AtBase,
            },
            rejected_final_contract,
        );
        let win = score_limit_win_condition_milestone(
            TEST_USER_A,
            Team::Red,
            ScoreSnapshot { red: 2, blue: 0 },
            1,
            TEST_SCORE_LIMIT,
        );

        assert!(
            accepted.contains("ctf_race_accepted_transition"),
            "{accepted}"
        );
        assert!(accepted.contains("username=compatbota"), "{accepted}");
        assert!(rejected.contains("reason=flag_already_held"), "{rejected}");
        assert!(
            final_state.contains("ctf_race_final_state"),
            "{final_state}"
        );
        assert!(rejected_final_state.is_none());
        assert!(win.contains("score_limit=2"), "{win}");
        assert!(score_limit_duplicate_win_milestone(TEST_USER_A, Team::Red)
            .contains("outcome=forbidden_duplicate_win"));
    }

    #[test]
    fn spawn_reset_core_records_balanced_assignments_and_rejects_malformed_state() {
        let contract = SpawnResetContract {
            expected_red_count: 1,
            expected_blue_count: 1,
            expected_blue_username: TEST_USER_B,
            reset_score: ScoreSnapshot { red: 1, blue: 0 },
            slot36_resource: "WoodenSword:1",
            red_slot37_resource: "RedWool:64",
            blue_slot37_resource: "BlueWool:64",
            reset_slot37_resource: "TeamWool:64",
            reset_state: "scoreboard_flags_and_resources_coherent",
        };
        let mut state = SpawnResetState::default();
        state.record_assignment(TEST_USER_A, Team::Red);
        state.record_assignment(TEST_USER_B, Team::Blue);

        let balance = spawn_team_balance_milestone(&state, &contract)
            .expect("balanced state should emit milestone");
        let reset = spawn_resource_reset_state_milestone(
            &state,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            ScoreSnapshot { red: 1, blue: 0 },
            &contract,
        )
        .expect("valid reset should emit milestone");
        let malformed = spawn_resource_reset_state_milestone(
            &state,
            TEST_USER_A,
            Team::Red,
            Team::Blue,
            ScoreSnapshot { red: 2, blue: 0 },
            &contract,
        );

        assert!(balance.contains("red_count=1 blue_count=1"), "{balance}");
        assert!(
            reset.contains("reset_state=scoreboard_flags_and_resources_coherent"),
            "{reset}"
        );
        assert!(malformed.is_none());
        assert!(defer_spawn_assignment(TEST_USER_B, Team::Red, TEST_USER_B));
        assert!(!defer_spawn_assignment(
            TEST_OTHER_USER,
            Team::Red,
            TEST_USER_B
        ));
    }

    #[test]
    fn inventory_stack_core_accepts_ordered_sequence_and_rejects_invalid_inventory() {
        let contract = stack_contract();
        let split_pickup = click(
            TEST_STACK_SOURCE_SLOT,
            TEST_RIGHT_BUTTON,
            item(TEST_STACK_HALF_COUNT),
            vec![slot(TEST_STACK_SOURCE_SLOT, item(TEST_STACK_HALF_COUNT))],
        );
        let split_place = click(
            TEST_STACK_DESTINATION_SLOT,
            TEST_LEFT_BUTTON,
            InventoryItemStack::empty(TEST_STACK_EMPTY_COUNT),
            vec![slot(
                TEST_STACK_DESTINATION_SLOT,
                item(TEST_STACK_HALF_COUNT),
            )],
        );
        let wrong_actor = InventoryClickSnapshot {
            actor_matches: false,
            ..split_pickup.clone()
        };
        let wrong_count = click(
            TEST_STACK_SOURCE_SLOT,
            TEST_RIGHT_BUTTON,
            item(TEST_STACK_FULL_COUNT),
            vec![slot(TEST_STACK_SOURCE_SLOT, item(TEST_STACK_FULL_COUNT))],
        );

        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &split_pickup,
                InventoryStackState::default(),
                contract,
            ),
            Some(InventoryStackAction::SplitPickup)
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &split_place,
                InventoryStackState {
                    split_pickup_seen: true,
                    split_place_seen: false,
                    merge_pickup_seen: false,
                },
                contract,
            ),
            Some(InventoryStackAction::SplitPlace)
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &wrong_actor,
                InventoryStackState::default(),
                contract
            ),
            None
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                &wrong_count,
                InventoryStackState::default(),
                contract
            ),
            None
        );
    }

    #[test]
    fn inventory_drag_core_accepts_ordered_sequence_and_rejects_out_of_order_or_bad_distribution() {
        let contract = drag_contract();
        let pickup = click(
            TEST_STACK_SOURCE_SLOT,
            TEST_LEFT_BUTTON,
            item(TEST_STACK_FULL_COUNT),
            vec![slot(
                TEST_STACK_SOURCE_SLOT,
                InventoryItemStack::empty(TEST_STACK_EMPTY_COUNT),
            )],
        );
        let start = drag(
            TEST_DRAG_OUTSIDE_SLOT,
            TEST_DRAG_START_BUTTON,
            item(TEST_STACK_FULL_COUNT),
            Vec::new(),
        );
        let end_bad_distribution = drag(
            TEST_DRAG_OUTSIDE_SLOT,
            TEST_DRAG_END_BUTTON,
            InventoryItemStack::empty(TEST_STACK_EMPTY_COUNT),
            vec![slot(TEST_DRAG_TARGET_A, item(TEST_STACK_FULL_COUNT))],
        );

        assert_eq!(
            classify_inventory_drag_transactions_event(
                &pickup,
                InventoryDragState::default(),
                contract
            ),
            Some(InventoryDragAction::PickupSource)
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                &start,
                InventoryDragState {
                    pickup_seen: true,
                    drag_start_seen: false,
                    target_a_seen: false,
                    target_b_seen: false,
                },
                contract,
            ),
            Some(InventoryDragAction::DragStart)
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                &start,
                InventoryDragState::default(),
                contract
            ),
            None
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                &end_bad_distribution,
                InventoryDragState {
                    pickup_seen: true,
                    drag_start_seen: true,
                    target_a_seen: true,
                    target_b_seen: true,
                },
                contract,
            ),
            None
        );
    }

    #[test]
    fn combat_core_handles_reference_hits_and_armor_mitigation() {
        assert!(reference_hit_for(
            true,
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            TEST_USER_B,
        ));
        assert!(!reference_hit_for(
            false,
            TEST_USER_A,
            TEST_USER_B,
            TEST_USER_A,
            TEST_USER_B,
        ));
        assert!(!reference_hit_for(
            true,
            TEST_USER_B,
            TEST_USER_A,
            TEST_USER_A,
            TEST_USER_B,
        ));
        let mitigation = combat_armor_mitigation_for(
            true,
            false,
            ArmorState::DiamondChestplate,
            TEST_BASE_DAMAGE,
            TEST_COMPAT_MITIGATION,
            TEST_DIAMOND_ARMOR_POINTS,
            TEST_DIAMOND_TOUGHNESS,
        );
        assert!((mitigation - TEST_REFERENCE_MITIGATION).abs() < TEST_FLOAT_TOLERANCE);
        assert_eq!(
            combat_armor_mitigation_for(
                false,
                true,
                ArmorState::DiamondChestplate,
                TEST_BASE_DAMAGE,
                TEST_COMPAT_MITIGATION,
                TEST_DIAMOND_ARMOR_POINTS,
                TEST_DIAMOND_TOUGHNESS,
            ),
            TEST_COMPAT_MITIGATION
        );
        assert_eq!(
            combat_armor_mitigation_for(
                true,
                false,
                ArmorState::Other,
                TEST_BASE_DAMAGE,
                TEST_COMPAT_MITIGATION,
                TEST_DIAMOND_ARMOR_POINTS,
                TEST_DIAMOND_TOUGHNESS,
            ),
            0.0
        );
        assert_eq!(
            knockback_metric([TEST_KNOCKBACK_X, TEST_KNOCKBACK_Y, TEST_KNOCKBACK_Z]),
            TEST_NORMALIZED_KNOCKBACK
        );
    }
}
