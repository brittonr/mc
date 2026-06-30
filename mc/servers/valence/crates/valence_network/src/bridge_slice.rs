//! Optional Valence/Hyperion bridge-slice planning helpers.
//!
//! This module is a Valence-owned prototype boundary for bridge experiments.
//! It contains pure planning cores for join/chunk delivery, movement mapping,
//! and chat/broadcast routing over explicit summaries. The fixture harness is
//! default-disabled and returns planned shell intents instead of touching
//! sockets, ECS state, clocks, logging, or packet writers.

use std::collections::BTreeSet;
use std::fmt;

const DEFAULT_MAX_INITIAL_CHUNK_RADIUS: u16 = 8;
const DEFAULT_MAX_CHAT_MESSAGE_CHARS: usize = 256;
const MIN_PITCH_DEGREES: f32 = -90.0;
const MAX_PITCH_DEGREES: f32 = 90.0;

/// Result type returned by bridge-slice planning cores.
pub type BridgePlanResult<T> = Result<T, BridgeDiagnostic>;

/// Configuration for the optional bridge-slice fixture harness.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BridgeSliceConfig {
    /// Whether bridge fixture planning is enabled.
    pub enabled: bool,
    /// Largest initial chunk view radius accepted by the join planner.
    pub max_initial_chunk_radius: u16,
    /// Largest chat or broadcast message accepted by the route planner.
    pub max_chat_message_chars: usize,
}

impl BridgeSliceConfig {
    /// Returns the default-disabled bridge-slice configuration.
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            max_initial_chunk_radius: DEFAULT_MAX_INITIAL_CHUNK_RADIUS,
            max_chat_message_chars: DEFAULT_MAX_CHAT_MESSAGE_CHARS,
        }
    }

    /// Returns the default bridge-slice configuration with planning enabled.
    pub const fn enabled() -> Self {
        Self {
            enabled: true,
            max_initial_chunk_radius: DEFAULT_MAX_INITIAL_CHUNK_RADIUS,
            max_chat_message_chars: DEFAULT_MAX_CHAT_MESSAGE_CHARS,
        }
    }
}

impl Default for BridgeSliceConfig {
    fn default() -> Self {
        Self::disabled()
    }
}

/// Stable bridge-local player identifier.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BridgePlayerId(u64);

impl BridgePlayerId {
    /// Creates a player identifier from an owner-provided value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw identifier value.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Stable bridge-local session identifier.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BridgeSessionId(u64);

impl BridgeSessionId {
    /// Creates a session identifier from an owner-provided value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw identifier value.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Stable bridge-local entity identifier.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BridgeEntityId(i32);

impl BridgeEntityId {
    /// Creates an entity identifier from an owner-provided value.
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the raw identifier value.
    pub const fn value(self) -> i32 {
        self.0
    }
}

/// Dimension key carried by bridge summaries.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BridgeDimensionKey(String);

impl BridgeDimensionKey {
    /// Creates a dimension key.
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self(value.into())
    }

    /// Returns the raw dimension key.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Chunk position carried by bridge summaries.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BridgeChunkPosition {
    /// Chunk X coordinate.
    pub x: i32,
    /// Chunk Z coordinate.
    pub z: i32,
}

impl BridgeChunkPosition {
    /// Creates a chunk position.
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

/// Dimension bounds required before join and movement planning.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeDimensionBounds {
    /// Dimension key.
    pub key: BridgeDimensionKey,
    /// Lowest buildable Y coordinate.
    pub min_y: i32,
    /// Height of the dimension in blocks.
    pub height: u16,
}

impl BridgeDimensionBounds {
    /// Creates dimension bounds.
    pub const fn new(key: BridgeDimensionKey, min_y: i32, height: u16) -> Self {
        Self { key, min_y, height }
    }

    fn contains_y(&self, y: f64) -> bool {
        let max_y = self.min_y.saturating_add(i32::from(self.height));
        f64::from(self.min_y) <= y && y < f64::from(max_y)
    }

    fn is_valid(&self) -> bool {
        self.height != 0
    }
}

/// Bridge-local session facts gathered by owner-specific shells.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeSessionFact {
    /// Player represented by this session.
    pub player_id: BridgePlayerId,
    /// Session represented by this fact.
    pub session_id: BridgeSessionId,
    /// Entity controlled by this session.
    pub entity_id: BridgeEntityId,
    /// Dimension in which the session is active.
    pub dimension: BridgeDimensionKey,
    /// Whether the owner shell considers the client open.
    pub client_open: bool,
    /// Whether the owner shell considers the mapping fresh.
    pub fresh: bool,
}

impl BridgeSessionFact {
    /// Creates a fresh, open session fact.
    pub const fn open(
        player_id: BridgePlayerId,
        session_id: BridgeSessionId,
        entity_id: BridgeEntityId,
        dimension: BridgeDimensionKey,
    ) -> Self {
        Self {
            player_id,
            session_id,
            entity_id,
            dimension,
            client_open: true,
            fresh: true,
        }
    }

    /// Returns this session fact marked stale.
    pub const fn stale(mut self) -> Self {
        self.fresh = false;
        self
    }

    /// Returns this session fact marked closed.
    pub const fn closed(mut self) -> Self {
        self.client_open = false;
        self
    }
}

/// One available chunk fact for initial chunk planning.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BridgeChunkFact {
    /// Chunk position.
    pub position: BridgeChunkPosition,
    /// Whether chunk data is available to the shell.
    pub available: bool,
}

impl BridgeChunkFact {
    /// Creates an available chunk fact.
    pub const fn available(position: BridgeChunkPosition) -> Self {
        Self {
            position,
            available: true,
        }
    }

    /// Creates a missing chunk fact.
    pub const fn missing(position: BridgeChunkPosition) -> Self {
        Self {
            position,
            available: false,
        }
    }
}

/// Explicit chunk-view summary consumed by the join planner.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeChunkViewFact {
    /// View center chunk.
    pub center: BridgeChunkPosition,
    /// View radius in chunks.
    pub radius_chunks: u16,
    /// Chunks selected for initial delivery.
    pub chunks: Vec<BridgeChunkFact>,
}

impl BridgeChunkViewFact {
    /// Creates a chunk-view summary.
    pub fn new(
        center: BridgeChunkPosition,
        radius_chunks: u16,
        chunks: Vec<BridgeChunkFact>,
    ) -> Self {
        Self {
            center,
            radius_chunks,
            chunks,
        }
    }
}

/// Packet-order step requested for a valid join/chunk plan.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BridgeJoinPacketStep {
    /// Registry or dimension data is sent.
    RegistryData,
    /// Join-game state is sent.
    JoinGame,
    /// Initial chunk data is sent for a position.
    ChunkData(BridgeChunkPosition),
    /// Initial player position or spawn placement is sent.
    PlayerPosition,
}

/// Explicit facts required for join and initial chunk planning.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeJoinChunkFacts {
    /// Session that is joining.
    pub session: BridgeSessionFact,
    /// Whether required registry facts are available.
    pub registry_available: bool,
    /// Dimension bounds selected for the session.
    pub dimension: BridgeDimensionBounds,
    /// Initial chunk view facts.
    pub view: BridgeChunkViewFact,
    /// Requested packet ordering.
    pub packet_order: Vec<BridgeJoinPacketStep>,
}

/// Approved join and initial chunk shell plan.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeJoinChunkPlan {
    /// Session receiving the plan.
    pub session: BridgeSessionFact,
    /// Dimension used by the plan.
    pub dimension: BridgeDimensionBounds,
    /// Chunks approved for delivery.
    pub chunks: Vec<BridgeChunkPosition>,
    /// Packet order approved for shell delivery.
    pub packet_order: Vec<BridgeJoinPacketStep>,
}

/// Three-dimensional position summary.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BridgePosition {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Z coordinate.
    pub z: f64,
}

impl BridgePosition {
    /// Creates a position summary.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

/// Rotation summary in degrees.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BridgeRotation {
    /// Yaw in degrees.
    pub yaw: f32,
    /// Pitch in degrees.
    pub pitch: f32,
}

impl BridgeRotation {
    /// Creates a rotation summary.
    pub const fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }

    fn is_valid(self) -> bool {
        self.yaw.is_finite()
            && self.pitch.is_finite()
            && (MIN_PITCH_DEGREES..=MAX_PITCH_DEGREES).contains(&self.pitch)
    }
}

/// Velocity summary.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BridgeVelocity {
    /// X velocity.
    pub x: f64,
    /// Y velocity.
    pub y: f64,
    /// Z velocity.
    pub z: f64,
}

impl BridgeVelocity {
    /// Creates a velocity summary.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

/// Movement update presented to the bridge mapper.
#[derive(Clone, PartialEq, Debug)]
pub struct BridgeMovementUpdate {
    /// Player that submitted or owns the update.
    pub player_id: BridgePlayerId,
    /// Session that submitted or owns the update.
    pub session_id: BridgeSessionId,
    /// Entity that should move.
    pub entity_id: BridgeEntityId,
    /// Dimension in which the movement was observed.
    pub dimension: BridgeDimensionKey,
    /// Position update.
    pub position: BridgePosition,
    /// Rotation update.
    pub rotation: BridgeRotation,
    /// Velocity update.
    pub velocity: BridgeVelocity,
    /// Whether the entity is on the ground.
    pub on_ground: bool,
}

/// Approved movement shell intent.
#[derive(Clone, PartialEq, Debug)]
pub struct BridgeMovementIntent {
    /// Player owning the movement.
    pub player_id: BridgePlayerId,
    /// Session owning the movement.
    pub session_id: BridgeSessionId,
    /// Entity approved for movement.
    pub entity_id: BridgeEntityId,
    /// Dimension in which the movement applies.
    pub dimension: BridgeDimensionKey,
    /// Approved position.
    pub position: BridgePosition,
    /// Approved rotation.
    pub rotation: BridgeRotation,
    /// Approved velocity.
    pub velocity: BridgeVelocity,
    /// Whether the entity is on the ground.
    pub on_ground: bool,
}

/// Chat or broadcast channel requested by a bridge route.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BridgeRouteChannel {
    /// Global broadcast route.
    Global,
    /// Local visibility route.
    Local,
    /// Application-owned named channel.
    Named(String),
}

/// Explicit recipient facts consumed by the chat route planner.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeRecipientFact {
    /// Recipient player identifier.
    pub player_id: BridgePlayerId,
    /// Recipient session identifier.
    pub session_id: BridgeSessionId,
    /// Whether the recipient client is open.
    pub client_open: bool,
    /// Whether the recipient may receive the requested route.
    pub permitted: bool,
    /// Whether the recipient is visible to the selected route.
    pub visible: bool,
}

impl BridgeRecipientFact {
    /// Creates a visible, permitted, open recipient.
    pub const fn allowed(player_id: BridgePlayerId, session_id: BridgeSessionId) -> Self {
        Self {
            player_id,
            session_id,
            client_open: true,
            permitted: true,
            visible: true,
        }
    }

    /// Returns this recipient marked closed.
    pub const fn closed(mut self) -> Self {
        self.client_open = false;
        self
    }

    /// Returns this recipient marked unauthorized.
    pub const fn unauthorized(mut self) -> Self {
        self.permitted = false;
        self
    }

    /// Returns this recipient marked hidden from the route.
    pub const fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }
}

/// Chat or broadcast route request.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeChatRouteRequest {
    /// Sending player.
    pub sender: BridgePlayerId,
    /// Sending session.
    pub sender_session: BridgeSessionId,
    /// Requested channel.
    pub channel: BridgeRouteChannel,
    /// Message body.
    pub message: String,
    /// Candidate recipients.
    pub recipients: Vec<BridgeRecipientFact>,
    /// Explicit player exclusions.
    pub excluded_players: BTreeSet<BridgePlayerId>,
    /// Whether every candidate must have route permission.
    pub require_permission: bool,
}

/// Approved chat or broadcast route plan.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BridgeChatRoutePlan {
    /// Sending player.
    pub sender: BridgePlayerId,
    /// Sending session.
    pub sender_session: BridgeSessionId,
    /// Approved route channel.
    pub channel: BridgeRouteChannel,
    /// Recipient player identifiers in deterministic order.
    pub recipients: Vec<BridgePlayerId>,
    /// Message body approved for delivery.
    pub message: String,
}

/// Deterministic diagnostic returned by bridge planners.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BridgeDiagnostic {
    /// Bridge fixture planning is disabled.
    BridgeDisabled,
    /// The session was stale.
    StaleSession,
    /// The target client was closed.
    ClosedClient,
    /// Required registry facts were missing.
    MissingRegistry,
    /// Dimension facts were invalid.
    InvalidDimension,
    /// Required chunk facts were missing.
    MissingChunk,
    /// Requested packet order is unsupported.
    UnsupportedPacketOrder,
    /// Session facts were not known to the harness.
    UnknownSession,
    /// Movement referenced the wrong entity.
    EntityMismatch,
    /// Movement referenced the wrong dimension.
    DimensionMismatch,
    /// Position facts were invalid or out of bounds.
    InvalidPosition,
    /// Rotation facts were invalid.
    InvalidRotation,
    /// Route facts were malformed.
    MalformedRoute,
    /// No valid recipients were available.
    EmptyRecipients,
    /// A route was unauthorized.
    UnauthorizedRoute,
    /// A conversion would lose required information.
    LossyConversion,
}

impl fmt::Display for BridgeDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::BridgeDisabled => "bridge disabled",
            Self::StaleSession => "stale session",
            Self::ClosedClient => "closed client",
            Self::MissingRegistry => "missing registry facts",
            Self::InvalidDimension => "invalid dimension facts",
            Self::MissingChunk => "missing chunk facts",
            Self::UnsupportedPacketOrder => "unsupported packet order",
            Self::UnknownSession => "unknown session",
            Self::EntityMismatch => "entity mismatch",
            Self::DimensionMismatch => "dimension mismatch",
            Self::InvalidPosition => "invalid position",
            Self::InvalidRotation => "invalid rotation",
            Self::MalformedRoute => "malformed route",
            Self::EmptyRecipients => "empty recipients",
            Self::UnauthorizedRoute => "unauthorized route",
            Self::LossyConversion => "lossy conversion",
        })
    }
}

impl std::error::Error for BridgeDiagnostic {}

/// Event accepted by the optional bridge fixture harness.
#[derive(Clone, PartialEq, Debug)]
pub enum BridgeHarnessEvent {
    /// Plan join and initial chunk delivery.
    JoinAndChunks(BridgeJoinChunkFacts),
    /// Plan one movement mapping.
    Movement(BridgeMovementUpdate),
    /// Plan one chat or broadcast route.
    ChatRoute(BridgeChatRouteRequest),
}

/// Shell intent returned by the optional bridge fixture harness.
#[derive(Clone, PartialEq, Debug)]
pub enum BridgeShellIntent {
    /// Approved join and chunk plan.
    JoinAndChunks(BridgeJoinChunkPlan),
    /// Approved movement mapping.
    Movement(BridgeMovementIntent),
    /// Approved chat route.
    ChatRoute(BridgeChatRoutePlan),
}

/// Pure fixture harness state used by bridge-slice tests and examples.
#[derive(Clone, PartialEq, Debug)]
pub struct BridgeHarnessState {
    /// Harness configuration.
    pub config: BridgeSliceConfig,
    /// Known sessions gathered by the shell.
    pub sessions: Vec<BridgeSessionFact>,
    /// Approved shell intents emitted so far.
    pub emitted_intents: Vec<BridgeShellIntent>,
}

impl BridgeHarnessState {
    /// Creates an empty harness state with default-disabled configuration.
    pub const fn disabled() -> Self {
        Self {
            config: BridgeSliceConfig::disabled(),
            sessions: Vec::new(),
            emitted_intents: Vec::new(),
        }
    }

    /// Creates an empty harness state with default-enabled configuration.
    pub const fn enabled() -> Self {
        Self {
            config: BridgeSliceConfig::enabled(),
            sessions: Vec::new(),
            emitted_intents: Vec::new(),
        }
    }

    /// Returns this state with one known session.
    pub fn with_session(mut self, session: BridgeSessionFact) -> Self {
        self.sessions.push(session);
        self
    }
}

impl Default for BridgeHarnessState {
    fn default() -> Self {
        Self::disabled()
    }
}

/// Result of applying one bridge fixture event.
#[derive(Clone, PartialEq, Debug)]
pub struct BridgeHarnessOutcome {
    /// State after applying the event.
    pub next_state: BridgeHarnessState,
    /// Approved shell intent, if any.
    pub intent: Option<BridgeShellIntent>,
    /// Diagnostics emitted by the harness.
    pub diagnostics: Vec<BridgeDiagnostic>,
}

/// Plans player join and initial chunk delivery without side effects.
pub fn plan_join_and_chunks(
    config: &BridgeSliceConfig,
    facts: &BridgeJoinChunkFacts,
) -> BridgePlanResult<BridgeJoinChunkPlan> {
    if !config.enabled {
        return Err(BridgeDiagnostic::BridgeDisabled);
    }
    validate_open_fresh_session(&facts.session)?;
    if !facts.registry_available {
        return Err(BridgeDiagnostic::MissingRegistry);
    }
    if !facts.dimension.is_valid() || facts.dimension.key != facts.session.dimension {
        return Err(BridgeDiagnostic::InvalidDimension);
    }
    if facts.view.radius_chunks > config.max_initial_chunk_radius {
        return Err(BridgeDiagnostic::MissingChunk);
    }
    if facts.view.chunks.is_empty() || facts.view.chunks.iter().any(|chunk| !chunk.available) {
        return Err(BridgeDiagnostic::MissingChunk);
    }
    if !packet_order_supported(&facts.packet_order) {
        return Err(BridgeDiagnostic::UnsupportedPacketOrder);
    }

    Ok(BridgeJoinChunkPlan {
        session: facts.session.clone(),
        dimension: facts.dimension.clone(),
        chunks: facts
            .view
            .chunks
            .iter()
            .map(|chunk| chunk.position)
            .collect(),
        packet_order: facts.packet_order.clone(),
    })
}

/// Maps one movement update to an approved shell intent without side effects.
pub fn map_movement_update(
    config: &BridgeSliceConfig,
    session: &BridgeSessionFact,
    update: &BridgeMovementUpdate,
    dimension: &BridgeDimensionBounds,
) -> BridgePlanResult<BridgeMovementIntent> {
    if !config.enabled {
        return Err(BridgeDiagnostic::BridgeDisabled);
    }
    validate_open_fresh_session(session)?;
    if update.player_id != session.player_id || update.session_id != session.session_id {
        return Err(BridgeDiagnostic::UnknownSession);
    }
    if update.entity_id != session.entity_id {
        return Err(BridgeDiagnostic::EntityMismatch);
    }
    if update.dimension != session.dimension || update.dimension != dimension.key {
        return Err(BridgeDiagnostic::DimensionMismatch);
    }
    if !dimension.is_valid() {
        return Err(BridgeDiagnostic::InvalidDimension);
    }
    if !update.position.is_finite()
        || !update.velocity.is_finite()
        || !dimension.contains_y(update.position.y)
    {
        return Err(BridgeDiagnostic::InvalidPosition);
    }
    if !update.rotation.is_valid() {
        return Err(BridgeDiagnostic::InvalidRotation);
    }

    Ok(BridgeMovementIntent {
        player_id: update.player_id,
        session_id: update.session_id,
        entity_id: update.entity_id,
        dimension: update.dimension.clone(),
        position: update.position,
        rotation: update.rotation,
        velocity: update.velocity,
        on_ground: update.on_ground,
    })
}

/// Plans one chat or broadcast route without side effects.
pub fn plan_chat_route(
    config: &BridgeSliceConfig,
    request: &BridgeChatRouteRequest,
) -> BridgePlanResult<BridgeChatRoutePlan> {
    if !config.enabled {
        return Err(BridgeDiagnostic::BridgeDisabled);
    }
    if request.message.trim().is_empty() {
        return Err(BridgeDiagnostic::MalformedRoute);
    }
    if request.message.chars().count() > config.max_chat_message_chars {
        return Err(BridgeDiagnostic::LossyConversion);
    }
    if request.recipients.is_empty() {
        return Err(BridgeDiagnostic::EmptyRecipients);
    }
    if request
        .recipients
        .iter()
        .any(|recipient| !recipient.client_open)
    {
        return Err(BridgeDiagnostic::ClosedClient);
    }
    if request.require_permission
        && request
            .recipients
            .iter()
            .any(|recipient| !recipient.permitted)
    {
        return Err(BridgeDiagnostic::UnauthorizedRoute);
    }

    let mut recipients = request
        .recipients
        .iter()
        .filter(|recipient| recipient.visible)
        .filter(|recipient| !request.excluded_players.contains(&recipient.player_id))
        .map(|recipient| recipient.player_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    if recipients.is_empty() {
        return Err(BridgeDiagnostic::EmptyRecipients);
    }

    recipients.sort();

    Ok(BridgeChatRoutePlan {
        sender: request.sender,
        sender_session: request.sender_session,
        channel: request.channel.clone(),
        recipients,
        message: request.message.clone(),
    })
}

/// Applies one fixture harness event through pure planning cores.
pub fn apply_bridge_harness_event(
    state: &BridgeHarnessState,
    event: &BridgeHarnessEvent,
    dimension: &BridgeDimensionBounds,
) -> BridgeHarnessOutcome {
    if !state.config.enabled {
        return BridgeHarnessOutcome {
            next_state: state.clone(),
            intent: None,
            diagnostics: vec![BridgeDiagnostic::BridgeDisabled],
        };
    }

    let planned = match event {
        BridgeHarnessEvent::JoinAndChunks(facts) => {
            plan_join_and_chunks(&state.config, facts).map(BridgeShellIntent::JoinAndChunks)
        }
        BridgeHarnessEvent::Movement(update) => {
            find_session(state, update.player_id, update.session_id)
                .ok_or(BridgeDiagnostic::UnknownSession)
                .and_then(|session| map_movement_update(&state.config, session, update, dimension))
                .map(BridgeShellIntent::Movement)
        }
        BridgeHarnessEvent::ChatRoute(request) => {
            plan_chat_route(&state.config, request).map(BridgeShellIntent::ChatRoute)
        }
    };

    match planned {
        Ok(intent) => {
            let mut next_state = state.clone();
            if let BridgeHarnessEvent::JoinAndChunks(facts) = event {
                insert_session_if_missing(&mut next_state.sessions, facts.session.clone());
            }
            next_state.emitted_intents.push(intent.clone());
            BridgeHarnessOutcome {
                next_state,
                intent: Some(intent),
                diagnostics: Vec::new(),
            }
        }
        Err(diagnostic) => BridgeHarnessOutcome {
            next_state: state.clone(),
            intent: None,
            diagnostics: vec![diagnostic],
        },
    }
}

fn validate_open_fresh_session(session: &BridgeSessionFact) -> BridgePlanResult<()> {
    if !session.client_open {
        return Err(BridgeDiagnostic::ClosedClient);
    }
    if !session.fresh {
        return Err(BridgeDiagnostic::StaleSession);
    }
    Ok(())
}

fn packet_order_supported(packet_order: &[BridgeJoinPacketStep]) -> bool {
    let mut saw_join = false;
    for step in packet_order {
        match step {
            BridgeJoinPacketStep::JoinGame => saw_join = true,
            BridgeJoinPacketStep::ChunkData(_) if !saw_join => return false,
            BridgeJoinPacketStep::RegistryData
            | BridgeJoinPacketStep::ChunkData(_)
            | BridgeJoinPacketStep::PlayerPosition => {}
        }
    }
    saw_join
}

fn find_session(
    state: &BridgeHarnessState,
    player_id: BridgePlayerId,
    session_id: BridgeSessionId,
) -> Option<&BridgeSessionFact> {
    state
        .sessions
        .iter()
        .find(|session| session.player_id == player_id && session.session_id == session_id)
}

fn insert_session_if_missing(sessions: &mut Vec<BridgeSessionFact>, session: BridgeSessionFact) {
    if sessions.iter().all(|existing| {
        existing.player_id != session.player_id || existing.session_id != session.session_id
    }) {
        sessions.push(session);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAYER_ALICE_RAW: u64 = 11;
    const PLAYER_BOB_RAW: u64 = 12;
    const PLAYER_CAROL_RAW: u64 = 13;
    const SESSION_ALICE_RAW: u64 = 21;
    const SESSION_BOB_RAW: u64 = 22;
    const SESSION_CAROL_RAW: u64 = 23;
    const ENTITY_ALICE_RAW: i32 = 31;
    const ENTITY_OTHER_RAW: i32 = 32;
    const DIMENSION_MIN_Y: i32 = -64;
    const DIMENSION_HEIGHT: u16 = 384;
    const SPAWN_CHUNK_X: i32 = 4;
    const SPAWN_CHUNK_Z: i32 = -3;
    const NEARBY_CHUNK_X: i32 = 5;
    const NEARBY_CHUNK_Z: i32 = -3;
    const VALID_RADIUS_CHUNKS: u16 = 2;
    const OVERSIZED_RADIUS_CHUNKS: u16 = 32;
    const POSITION_X: f64 = 12.5;
    const POSITION_Y: f64 = 70.0;
    const POSITION_Z: f64 = -8.25;
    const VELOCITY_X: f64 = 0.1;
    const VELOCITY_Y: f64 = -0.2;
    const VELOCITY_Z: f64 = 0.3;
    const YAW_DEGREES: f32 = 45.0;
    const PITCH_DEGREES: f32 = 10.0;
    const INVALID_PITCH_DEGREES: f32 = 120.0;

    #[test]
    fn join_chunk_plan_accepts_valid_facts() {
        let facts = join_facts();

        let plan = plan_join_and_chunks(&BridgeSliceConfig::enabled(), &facts).unwrap();

        assert_eq!(plan.session.player_id, alice());
        assert_eq!(plan.dimension.key, overworld());
        assert_eq!(plan.chunks, vec![spawn_chunk(), nearby_chunk()]);
        assert_eq!(plan.packet_order, packet_order());
    }

    #[test]
    fn join_chunk_plan_rejects_missing_registry() {
        let mut facts = join_facts();
        facts.registry_available = false;

        let error = plan_join_and_chunks(&BridgeSliceConfig::enabled(), &facts).unwrap_err();

        assert_eq!(error, BridgeDiagnostic::MissingRegistry);
    }

    #[test]
    fn join_chunk_plan_rejects_invalid_chunk_facts() {
        let mut facts = join_facts();
        facts.view.chunks = vec![BridgeChunkFact::missing(spawn_chunk())];

        let error = plan_join_and_chunks(&BridgeSliceConfig::enabled(), &facts).unwrap_err();

        assert_eq!(error, BridgeDiagnostic::MissingChunk);
    }

    #[test]
    fn join_chunk_plan_rejects_unsupported_packet_order() {
        let mut facts = join_facts();
        facts.packet_order = vec![BridgeJoinPacketStep::ChunkData(spawn_chunk())];

        let error = plan_join_and_chunks(&BridgeSliceConfig::enabled(), &facts).unwrap_err();

        assert_eq!(error, BridgeDiagnostic::UnsupportedPacketOrder);
    }

    #[test]
    fn movement_maps_one_known_entity() {
        let session = open_session();
        let update = movement_update();

        let intent = map_movement_update(
            &BridgeSliceConfig::enabled(),
            &session,
            &update,
            &dimension_bounds(),
        )
        .unwrap();

        assert_eq!(intent.player_id, alice());
        assert_eq!(intent.session_id, alice_session());
        assert_eq!(intent.entity_id, alice_entity());
        assert_eq!(intent.position, valid_position());
        assert!(intent.on_ground);
    }

    #[test]
    fn movement_rejects_stale_session() {
        let session = open_session().stale();
        let update = movement_update();

        let error = map_movement_update(
            &BridgeSliceConfig::enabled(),
            &session,
            &update,
            &dimension_bounds(),
        )
        .unwrap_err();

        assert_eq!(error, BridgeDiagnostic::StaleSession);
    }

    #[test]
    fn movement_rejects_entity_mismatch() {
        let mut update = movement_update();
        update.entity_id = BridgeEntityId::new(ENTITY_OTHER_RAW);

        let error = map_movement_update(
            &BridgeSliceConfig::enabled(),
            &open_session(),
            &update,
            &dimension_bounds(),
        )
        .unwrap_err();

        assert_eq!(error, BridgeDiagnostic::EntityMismatch);
    }

    #[test]
    fn movement_rejects_malformed_position_and_rotation() {
        let mut bad_position = movement_update();
        bad_position.position = BridgePosition::new(f64::NAN, POSITION_Y, POSITION_Z);
        let position_error = map_movement_update(
            &BridgeSliceConfig::enabled(),
            &open_session(),
            &bad_position,
            &dimension_bounds(),
        )
        .unwrap_err();

        let mut bad_rotation = movement_update();
        bad_rotation.rotation = BridgeRotation::new(YAW_DEGREES, INVALID_PITCH_DEGREES);
        let rotation_error = map_movement_update(
            &BridgeSliceConfig::enabled(),
            &open_session(),
            &bad_rotation,
            &dimension_bounds(),
        )
        .unwrap_err();

        assert_eq!(position_error, BridgeDiagnostic::InvalidPosition);
        assert_eq!(rotation_error, BridgeDiagnostic::InvalidRotation);
    }

    #[test]
    fn chat_route_returns_deterministic_authorized_recipients() {
        let request = chat_request();

        let plan = plan_chat_route(&BridgeSliceConfig::enabled(), &request).unwrap();

        assert_eq!(plan.sender, alice());
        assert_eq!(plan.recipients, vec![bob(), carol()]);
        assert_eq!(plan.message, "summit ready");
    }

    #[test]
    fn chat_route_honors_exclusions_and_visibility() {
        let mut request = chat_request();
        request.excluded_players.insert(bob());
        request.recipients.push(
            BridgeRecipientFact::allowed(
                BridgePlayerId::new(PLAYER_ALICE_RAW),
                BridgeSessionId::new(SESSION_ALICE_RAW),
            )
            .hidden(),
        );

        let plan = plan_chat_route(&BridgeSliceConfig::enabled(), &request).unwrap();

        assert_eq!(plan.recipients, vec![carol()]);
    }

    #[test]
    fn chat_route_rejects_unauthorized_or_closed_recipients() {
        let mut unauthorized = chat_request();
        unauthorized
            .recipients
            .push(BridgeRecipientFact::allowed(bob(), bob_session()).unauthorized());
        let unauthorized_error =
            plan_chat_route(&BridgeSliceConfig::enabled(), &unauthorized).unwrap_err();

        let mut closed = chat_request();
        closed
            .recipients
            .push(BridgeRecipientFact::allowed(carol(), carol_session()).closed());
        let closed_error = plan_chat_route(&BridgeSliceConfig::enabled(), &closed).unwrap_err();

        assert_eq!(unauthorized_error, BridgeDiagnostic::UnauthorizedRoute);
        assert_eq!(closed_error, BridgeDiagnostic::ClosedClient);
    }

    #[test]
    fn disabled_harness_preserves_direct_state() {
        let state = BridgeHarnessState::disabled().with_session(open_session());
        let event = BridgeHarnessEvent::JoinAndChunks(join_facts());

        let outcome = apply_bridge_harness_event(&state, &event, &dimension_bounds());

        assert_eq!(outcome.next_state, state);
        assert_eq!(outcome.intent, None);
        assert_eq!(outcome.diagnostics, vec![BridgeDiagnostic::BridgeDisabled]);
    }

    #[test]
    fn enabled_harness_applies_only_approved_intents() {
        let state = BridgeHarnessState::enabled();
        let event = BridgeHarnessEvent::JoinAndChunks(join_facts());

        let outcome = apply_bridge_harness_event(&state, &event, &dimension_bounds());

        assert!(matches!(
            outcome.intent,
            Some(BridgeShellIntent::JoinAndChunks(_))
        ));
        assert_eq!(outcome.next_state.sessions.len(), 1);
        assert_eq!(outcome.next_state.emitted_intents.len(), 1);
        assert!(outcome.diagnostics.is_empty());
    }

    #[test]
    fn enabled_harness_rejects_unknown_movement_session() {
        let state = BridgeHarnessState::enabled();
        let event = BridgeHarnessEvent::Movement(movement_update());

        let outcome = apply_bridge_harness_event(&state, &event, &dimension_bounds());

        assert_eq!(outcome.next_state, state);
        assert_eq!(outcome.intent, None);
        assert_eq!(outcome.diagnostics, vec![BridgeDiagnostic::UnknownSession]);
    }

    #[test]
    fn join_chunk_plan_rejects_oversized_radius() {
        let mut facts = join_facts();
        facts.view.radius_chunks = OVERSIZED_RADIUS_CHUNKS;

        let error = plan_join_and_chunks(&BridgeSliceConfig::enabled(), &facts).unwrap_err();

        assert_eq!(error, BridgeDiagnostic::MissingChunk);
    }

    fn join_facts() -> BridgeJoinChunkFacts {
        BridgeJoinChunkFacts {
            session: open_session(),
            registry_available: true,
            dimension: dimension_bounds(),
            view: BridgeChunkViewFact::new(
                spawn_chunk(),
                VALID_RADIUS_CHUNKS,
                vec![
                    BridgeChunkFact::available(spawn_chunk()),
                    BridgeChunkFact::available(nearby_chunk()),
                ],
            ),
            packet_order: packet_order(),
        }
    }

    fn packet_order() -> Vec<BridgeJoinPacketStep> {
        vec![
            BridgeJoinPacketStep::RegistryData,
            BridgeJoinPacketStep::JoinGame,
            BridgeJoinPacketStep::ChunkData(spawn_chunk()),
            BridgeJoinPacketStep::ChunkData(nearby_chunk()),
            BridgeJoinPacketStep::PlayerPosition,
        ]
    }

    fn movement_update() -> BridgeMovementUpdate {
        BridgeMovementUpdate {
            player_id: alice(),
            session_id: alice_session(),
            entity_id: alice_entity(),
            dimension: overworld(),
            position: valid_position(),
            rotation: valid_rotation(),
            velocity: valid_velocity(),
            on_ground: true,
        }
    }

    fn chat_request() -> BridgeChatRouteRequest {
        BridgeChatRouteRequest {
            sender: alice(),
            sender_session: alice_session(),
            channel: BridgeRouteChannel::Named("summit".to_owned()),
            message: "summit ready".to_owned(),
            recipients: vec![
                BridgeRecipientFact::allowed(bob(), bob_session()),
                BridgeRecipientFact::allowed(carol(), carol_session()),
            ],
            excluded_players: BTreeSet::new(),
            require_permission: true,
        }
    }

    fn dimension_bounds() -> BridgeDimensionBounds {
        BridgeDimensionBounds::new(overworld(), DIMENSION_MIN_Y, DIMENSION_HEIGHT)
    }

    fn open_session() -> BridgeSessionFact {
        BridgeSessionFact::open(alice(), alice_session(), alice_entity(), overworld())
    }

    fn valid_position() -> BridgePosition {
        BridgePosition::new(POSITION_X, POSITION_Y, POSITION_Z)
    }

    fn valid_rotation() -> BridgeRotation {
        BridgeRotation::new(YAW_DEGREES, PITCH_DEGREES)
    }

    fn valid_velocity() -> BridgeVelocity {
        BridgeVelocity::new(VELOCITY_X, VELOCITY_Y, VELOCITY_Z)
    }

    fn spawn_chunk() -> BridgeChunkPosition {
        BridgeChunkPosition::new(SPAWN_CHUNK_X, SPAWN_CHUNK_Z)
    }

    fn nearby_chunk() -> BridgeChunkPosition {
        BridgeChunkPosition::new(NEARBY_CHUNK_X, NEARBY_CHUNK_Z)
    }

    fn alice() -> BridgePlayerId {
        BridgePlayerId::new(PLAYER_ALICE_RAW)
    }

    fn bob() -> BridgePlayerId {
        BridgePlayerId::new(PLAYER_BOB_RAW)
    }

    fn carol() -> BridgePlayerId {
        BridgePlayerId::new(PLAYER_CAROL_RAW)
    }

    fn alice_session() -> BridgeSessionId {
        BridgeSessionId::new(SESSION_ALICE_RAW)
    }

    fn bob_session() -> BridgeSessionId {
        BridgeSessionId::new(SESSION_BOB_RAW)
    }

    fn carol_session() -> BridgeSessionId {
        BridgeSessionId::new(SESSION_CAROL_RAW)
    }

    fn alice_entity() -> BridgeEntityId {
        BridgeEntityId::new(ENTITY_ALICE_RAW)
    }

    fn overworld() -> BridgeDimensionKey {
        BridgeDimensionKey::new("minecraft:overworld")
    }
}
