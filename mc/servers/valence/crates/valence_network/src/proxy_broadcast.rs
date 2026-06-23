//! Optional proxy broadcast contract and routing helpers.
//!
//! This module is a Valence-owned boundary for proxy-mode experiments. The
//! pure core validates server/proxy messages, resolves route intents against an
//! explicit proxy state snapshot, and returns delivery plans without reading ECS
//! state or touching sockets. [`ProxyBroadcastBackendPlugin`] is a thin opt-in
//! shell that stores disabled backend state unless an application explicitly
//! enables it.
//!
//! The contract intentionally does not copy Hyperion's runtime, transport,
//! rkyv frames, mTLS/Iroh setup, or Bedwars gameplay code. Direct Valence
//! networking remains the default path when this plugin is not added or when the
//! backend resource is left disabled.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_protocol::MAX_PACKET_SIZE;

const MINECRAFT_WORLD_BORDER_BLOCKS_FROM_ORIGIN: i32 = 30_000_000;
const CHUNK_WIDTH_BLOCKS: i32 = 16;
const MAX_CHUNK_COORDINATE_ABS: i32 =
    MINECRAFT_WORLD_BORDER_BLOCKS_FROM_ORIGIN / CHUNK_WIDTH_BLOCKS;
const DEFAULT_MAX_QUEUED_PAYLOAD_BYTES: usize = MAX_PACKET_SIZE as usize;

/// Stable identifier for one proxied player stream.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProxyStreamId(u64);

impl ProxyStreamId {
    /// Creates a stream identifier from an application or proxy-owned value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw stream identifier.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Stable identifier for one proxy broadcast channel.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProxyChannelId(u32);

impl ProxyChannelId {
    /// Creates a channel identifier from an application-owned value.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw channel identifier.
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// Chunk position used by proxy-local visibility routing.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ProxyChunkPosition {
    /// Chunk X coordinate.
    pub x: i32,
    /// Chunk Z coordinate.
    pub z: i32,
}

impl ProxyChunkPosition {
    /// Creates a chunk position.
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

/// Per-stream player position update.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProxyPlayerPosition {
    /// Stream whose position is being updated.
    pub stream: ProxyStreamId,
    /// Observed chunk position for the stream.
    pub position: ProxyChunkPosition,
}

impl ProxyPlayerPosition {
    /// Creates a player position update.
    pub const fn new(stream: ProxyStreamId, position: ProxyChunkPosition) -> Self {
        Self { stream, position }
    }
}

/// Stream lifecycle tracked by the proxy routing core.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ProxyStreamLifecycle {
    /// Stream may receive outbound packet deliveries.
    Active,
    /// Shutdown was requested; in-flight inbound packets may still arrive until disconnect.
    ShuttingDown,
}

/// Snapshot of one proxied player stream.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyPlayerState {
    /// Stream represented by this snapshot.
    pub stream: ProxyStreamId,
    /// Whether new outbound deliveries may be planned for this stream.
    pub lifecycle: ProxyStreamLifecycle,
    /// Last proxy-observed chunk position.
    pub position: Option<ProxyChunkPosition>,
    /// Whether this stream opted into global and local broadcasts.
    pub receives_broadcasts: bool,
    /// Channels this stream is subscribed to.
    pub subscriptions: BTreeSet<ProxyChannelId>,
}

impl ProxyPlayerState {
    /// Creates an active stream without a known position or subscriptions.
    pub fn active(stream: ProxyStreamId) -> Self {
        Self {
            stream,
            lifecycle: ProxyStreamLifecycle::Active,
            position: None,
            receives_broadcasts: false,
            subscriptions: BTreeSet::new(),
        }
    }

    /// Returns this snapshot with a known chunk position.
    pub const fn with_position(mut self, position: ProxyChunkPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// Returns this snapshot with global/local broadcast delivery enabled.
    pub const fn receiving_broadcasts(mut self) -> Self {
        self.receives_broadcasts = true;
        self
    }

    /// Returns this snapshot subscribed to one channel.
    pub fn subscribed_to(mut self, channel: ProxyChannelId) -> Self {
        self.subscriptions.insert(channel);
        self
    }

    /// Returns this snapshot marked as shutting down.
    pub const fn shutting_down(mut self) -> Self {
        self.lifecycle = ProxyStreamLifecycle::ShuttingDown;
        self
    }

    fn is_active(&self) -> bool {
        self.lifecycle == ProxyStreamLifecycle::Active
    }
}

/// Snapshot of one proxy channel.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProxyChannelState {
    /// Channel represented by this snapshot.
    pub channel: ProxyChannelId,
}

impl ProxyChannelState {
    /// Creates a channel snapshot.
    pub const fn new(channel: ProxyChannelId) -> Self {
        Self { channel }
    }
}

/// Explicit proxy state snapshot consumed by validation and routing.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct ProxyRouteState {
    /// Known proxied player streams.
    pub players: Vec<ProxyPlayerState>,
    /// Known proxy broadcast channels.
    pub channels: Vec<ProxyChannelState>,
}

impl ProxyRouteState {
    /// Creates an empty proxy state snapshot.
    pub const fn empty() -> Self {
        Self {
            players: Vec::new(),
            channels: Vec::new(),
        }
    }

    /// Returns this snapshot with one additional player stream.
    pub fn with_player(mut self, player: ProxyPlayerState) -> Self {
        self.players.push(player);
        self
    }

    /// Returns this snapshot with one additional channel.
    pub fn with_channel(mut self, channel: ProxyChannelState) -> Self {
        self.channels.push(channel);
        self
    }
}

/// Backpressure policy for bounded proxy message validation.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProxyBackpressurePolicy {
    /// Maximum payload bytes accepted for one proxied packet payload.
    pub max_queued_payload_bytes: usize,
}

impl Default for ProxyBackpressurePolicy {
    fn default() -> Self {
        Self {
            max_queued_payload_bytes: DEFAULT_MAX_QUEUED_PAYLOAD_BYTES,
        }
    }
}

/// Server-to-proxy contract messages understood by the pure core.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ServerToProxyMessage {
    /// Update one stream position.
    UpdatePlayerPosition(ProxyPlayerPosition),
    /// Update several stream positions in author order.
    UpdatePlayerPositions(Vec<ProxyPlayerPosition>),
    /// Register a channel before it may receive subscriptions or channel broadcasts.
    AddChannel { channel: ProxyChannelId },
    /// Remove a channel and any subscriptions to it.
    RemoveChannel { channel: ProxyChannelId },
    /// Send channel subscription packet bytes to subscribers, excluding one stream if present.
    SubscribeChannelPackets {
        channel: ProxyChannelId,
        exclude: Option<ProxyStreamId>,
        payload: Vec<u8>,
    },
    /// Allow one stream to receive global and local broadcasts.
    SetReceiveBroadcasts { stream: ProxyStreamId },
    /// Broadcast packet bytes to all broadcast-enabled streams.
    BroadcastGlobal {
        exclude: Option<ProxyStreamId>,
        payload: Vec<u8>,
    },
    /// Broadcast packet bytes to broadcast-enabled streams inside a chunk radius.
    BroadcastLocal {
        center: ProxyChunkPosition,
        radius_chunks: u16,
        exclude: Option<ProxyStreamId>,
        payload: Vec<u8>,
    },
    /// Broadcast packet bytes to streams subscribed to a channel.
    BroadcastChannel {
        channel: ProxyChannelId,
        exclude: Option<ProxyStreamId>,
        payload: Vec<u8>,
    },
    /// Send packet bytes to one stream.
    Unicast {
        stream: ProxyStreamId,
        payload: Vec<u8>,
    },
    /// Begin graceful shutdown for one stream.
    Shutdown { stream: ProxyStreamId },
}

impl ServerToProxyMessage {
    /// Returns the packet payload for messages that carry clientbound bytes.
    pub fn payload(&self) -> Option<&[u8]> {
        match self {
            Self::SubscribeChannelPackets { payload, .. }
            | Self::BroadcastGlobal { payload, .. }
            | Self::BroadcastLocal { payload, .. }
            | Self::BroadcastChannel { payload, .. }
            | Self::Unicast { payload, .. } => Some(payload),
            Self::UpdatePlayerPosition(_)
            | Self::UpdatePlayerPositions(_)
            | Self::AddChannel { .. }
            | Self::RemoveChannel { .. }
            | Self::SetReceiveBroadcasts { .. }
            | Self::Shutdown { .. } => None,
        }
    }
}

/// Proxy-to-server contract messages understood by the pure state reducer.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ProxyToServerMessage {
    /// Proxy transport is ready to accept streams.
    ProxyReady,
    /// A player stream connected through the proxy.
    PlayerConnect { stream: ProxyStreamId },
    /// A player stream disconnected through the proxy.
    PlayerDisconnect {
        stream: ProxyStreamId,
        reason: ProxyDisconnectReason,
    },
    /// Serverbound packet bytes from one player stream.
    PlayerPackets {
        stream: ProxyStreamId,
        payload: Vec<u8>,
    },
    /// Request channel subscription for one player stream.
    RequestSubscribeChannelPackets {
        stream: ProxyStreamId,
        channels: Vec<ProxyChannelId>,
    },
}

/// Bounded disconnect reason vocabulary for the proxy contract.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ProxyDisconnectReason {
    /// Stream could not receive packets fast enough.
    CouldNotKeepUp,
    /// Stream was lost by the proxy transport.
    LostConnection,
    /// Application-owned diagnostic.
    Other(String),
}

/// One planned proxy delivery.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ProxyDelivery {
    /// Message index that produced this delivery.
    pub message_index: usize,
    /// Stream receiving the payload.
    pub recipient: ProxyStreamId,
    /// Delivery route category.
    pub route: ProxyDeliveryRoute,
    /// Payload byte count selected by the message.
    pub payload_len: usize,
}

/// Delivery route category.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ProxyDeliveryRoute {
    /// Unicast delivery.
    Unicast,
    /// Global broadcast delivery.
    Global,
    /// Local chunk-radius delivery.
    Local,
    /// Channel subscription delivery.
    Channel(ProxyChannelId),
}

/// Deterministic proxy routing result.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct ProxyDeliveryPlan {
    deliveries: Vec<ProxyDelivery>,
    failures: Vec<ProxyMessageDiagnostic>,
}

impl ProxyDeliveryPlan {
    /// Returns all successful deliveries in deterministic order.
    pub fn deliveries(&self) -> &[ProxyDelivery] {
        &self.deliveries
    }

    /// Returns validation or routing failures.
    pub fn failures(&self) -> &[ProxyMessageDiagnostic] {
        &self.failures
    }

    /// Returns true when no failures were reported.
    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Structured proxy contract diagnostic.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyMessageDiagnostic {
    /// Message index when the diagnostic is tied to one message.
    pub message_index: Option<usize>,
    /// Diagnostic category.
    pub kind: ProxyMessageDiagnosticKind,
}

impl ProxyMessageDiagnostic {
    fn state(kind: ProxyMessageDiagnosticKind) -> Self {
        Self {
            message_index: None,
            kind,
        }
    }

    fn message(message_index: usize, kind: ProxyMessageDiagnosticKind) -> Self {
        Self {
            message_index: Some(message_index),
            kind,
        }
    }
}

/// Proxy contract diagnostic category.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ProxyMessageDiagnosticKind {
    /// A stream was referenced but is not in the proxy state.
    UnknownStream { stream: ProxyStreamId },
    /// A stream exists but is not active for new outbound work.
    InactiveStream { stream: ProxyStreamId },
    /// A stream appears more than once in the state snapshot.
    DuplicateStream { stream: ProxyStreamId },
    /// A channel was referenced but is not in the proxy state.
    UnknownChannel { channel: ProxyChannelId },
    /// A channel appears more than once in the state snapshot.
    DuplicateChannel { channel: ProxyChannelId },
    /// A stream subscription references a removed or unknown channel.
    StaleSubscription {
        stream: ProxyStreamId,
        channel: ProxyChannelId,
    },
    /// A player position is outside the supported Minecraft world chunk range.
    InvalidPlayerPosition {
        stream: ProxyStreamId,
        position: ProxyChunkPosition,
    },
    /// A message carried an empty packet payload.
    MalformedPayload,
    /// Payload bytes exceed the configured queue budget.
    BackpressureLimitExceeded { payload_len: usize, limit: usize },
    /// A local route could not be evaluated for a stream with no position.
    MissingPlayerPosition { stream: ProxyStreamId },
    /// A stream was connected twice without disconnecting first.
    StreamAlreadyConnected { stream: ProxyStreamId },
}

/// Plans proxy deliveries without touching sockets, ECS state, clocks, or transport encoders.
///
/// The planner validates the full state snapshot first. Duplicate streams,
/// duplicate channels, invalid positions, or stale subscriptions fail closed and
/// produce no deliveries. Per-message diagnostics also suppress deliveries for
/// that message. Delivery ordering preserves input message order and sorts
/// recipients by [`ProxyStreamId`] inside each message.
pub fn plan_proxy_deliveries(
    messages: &[ServerToProxyMessage],
    state: &ProxyRouteState,
    policy: ProxyBackpressurePolicy,
) -> ProxyDeliveryPlan {
    let indexed = index_state(state);
    if !indexed.failures.is_empty() {
        return ProxyDeliveryPlan {
            deliveries: Vec::new(),
            failures: indexed.failures,
        };
    }

    let mut deliveries = Vec::new();
    let mut failures = Vec::new();

    for (message_index, message) in messages.iter().enumerate() {
        match validate_server_to_proxy_message(message, &indexed, policy) {
            Ok(()) => plan_message_delivery(message_index, message, &indexed, &mut deliveries),
            Err(kind) => failures.push(ProxyMessageDiagnostic::message(message_index, kind)),
        }
    }

    ProxyDeliveryPlan {
        deliveries,
        failures,
    }
}

/// Applies one server-to-proxy state message to an explicit proxy state snapshot.
///
/// Delivery messages are validated and leave state unchanged. State update
/// messages return a new state snapshot without mutating the input.
pub fn apply_server_to_proxy_message(
    state: &ProxyRouteState,
    message: &ServerToProxyMessage,
    policy: ProxyBackpressurePolicy,
) -> Result<ProxyRouteState, ProxyMessageDiagnosticKind> {
    let indexed = index_state(state);
    if let Some(failure) = indexed.failures.first() {
        return Err(failure.kind.clone());
    }
    validate_server_to_proxy_message(message, &indexed, policy)?;

    let mut next = state.clone();
    match message {
        ServerToProxyMessage::UpdatePlayerPosition(update) => {
            set_player_position(&mut next, update.stream, update.position);
        }
        ServerToProxyMessage::UpdatePlayerPositions(updates) => {
            for update in updates {
                set_player_position(&mut next, update.stream, update.position);
            }
        }
        ServerToProxyMessage::AddChannel { channel } => {
            next.channels.push(ProxyChannelState::new(*channel));
        }
        ServerToProxyMessage::RemoveChannel { channel } => {
            next.channels.retain(|state| state.channel != *channel);
            for player in &mut next.players {
                player.subscriptions.remove(channel);
            }
        }
        ServerToProxyMessage::SetReceiveBroadcasts { stream } => {
            if let Some(player) = next
                .players
                .iter_mut()
                .find(|player| player.stream == *stream)
            {
                player.receives_broadcasts = true;
            }
        }
        ServerToProxyMessage::Shutdown { stream } => {
            if let Some(player) = next
                .players
                .iter_mut()
                .find(|player| player.stream == *stream)
            {
                player.lifecycle = ProxyStreamLifecycle::ShuttingDown;
            }
        }
        ServerToProxyMessage::SubscribeChannelPackets { .. }
        | ServerToProxyMessage::BroadcastGlobal { .. }
        | ServerToProxyMessage::BroadcastLocal { .. }
        | ServerToProxyMessage::BroadcastChannel { .. }
        | ServerToProxyMessage::Unicast { .. } => {}
    }

    Ok(next)
}

/// Applies one proxy-to-server lifecycle or subscription message to a state snapshot.
pub fn apply_proxy_to_server_message(
    state: &ProxyRouteState,
    message: &ProxyToServerMessage,
    policy: ProxyBackpressurePolicy,
) -> Result<ProxyRouteState, ProxyMessageDiagnosticKind> {
    let indexed = index_state(state);
    if let Some(failure) = indexed.failures.first() {
        return Err(failure.kind.clone());
    }

    let mut next = state.clone();
    match message {
        ProxyToServerMessage::ProxyReady => {}
        ProxyToServerMessage::PlayerConnect { stream } => {
            if indexed.players.contains_key(stream) {
                return Err(ProxyMessageDiagnosticKind::StreamAlreadyConnected { stream: *stream });
            }
            next.players.push(ProxyPlayerState::active(*stream));
        }
        ProxyToServerMessage::PlayerDisconnect { stream, .. } => {
            require_known_stream(*stream, &indexed)?;
            next.players.retain(|player| player.stream != *stream);
        }
        ProxyToServerMessage::PlayerPackets { stream, payload } => {
            require_known_stream(*stream, &indexed)?;
            validate_payload(payload, policy)?;
        }
        ProxyToServerMessage::RequestSubscribeChannelPackets { stream, channels } => {
            require_active_stream(*stream, &indexed)?;
            for channel in channels {
                require_known_channel(*channel, &indexed)?;
            }
            if let Some(player) = next
                .players
                .iter_mut()
                .find(|player| player.stream == *stream)
            {
                player.subscriptions.extend(channels.iter().copied());
            }
        }
    }

    Ok(next)
}

/// Report returned by a proxy transport write adapter.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct ProxyBackendFlushReport {
    delivered: Vec<ProxyDelivery>,
    failures: Vec<ProxyBackendFlushFailure>,
}

impl ProxyBackendFlushReport {
    /// Returns deliveries written through the adapter.
    pub fn delivered(&self) -> &[ProxyDelivery] {
        &self.delivered
    }

    /// Returns adapter failures.
    pub fn failures(&self) -> &[ProxyBackendFlushFailure] {
        &self.failures
    }

    /// Returns true when all planned deliveries were written.
    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }
}

/// One proxy backend flush failure.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyBackendFlushFailure {
    /// Delivery plan index that failed.
    pub delivery_index: usize,
    /// Recipient stream that could not be written.
    pub recipient: ProxyStreamId,
    /// Failure category.
    pub kind: ProxyBackendFlushFailureKind,
    /// Adapter diagnostic, when available.
    pub message: Option<String>,
}

/// Proxy backend flush failure category.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ProxyBackendFlushFailureKind {
    /// Delivery referenced a message that carries no payload.
    MissingPayload,
    /// Recipient stream disconnected before write.
    DisconnectedStream,
    /// Adapter rejected the write because its queue is full.
    Backpressure,
}

/// Error returned by a proxy backend write adapter.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyBackendWriteError {
    kind: ProxyBackendFlushFailureKind,
    message: Option<String>,
}

impl ProxyBackendWriteError {
    /// Creates a disconnected-stream write error.
    pub const fn disconnected_stream() -> Self {
        Self {
            kind: ProxyBackendFlushFailureKind::DisconnectedStream,
            message: None,
        }
    }

    /// Creates a backpressure write error.
    pub fn backpressure<E>(error: E) -> Self
    where
        E: fmt::Display,
    {
        Self {
            kind: ProxyBackendFlushFailureKind::Backpressure,
            message: Some(error.to_string()),
        }
    }
}

/// Flushes a proxy plan through an arbitrary transport adapter.
///
/// The adapter is the imperative shell. Tests can supply an in-memory adapter,
/// while a future transport can send bytes to real proxy sockets without
/// changing route-planning logic.
pub fn flush_proxy_delivery_plan_with<F>(
    plan: &ProxyDeliveryPlan,
    messages: &[ServerToProxyMessage],
    mut write_payload: F,
) -> ProxyBackendFlushReport
where
    F: FnMut(ProxyStreamId, &[u8]) -> Result<(), ProxyBackendWriteError>,
{
    let mut delivered = Vec::new();
    let mut failures = Vec::new();

    for (delivery_index, delivery) in plan.deliveries().iter().enumerate() {
        let Some(payload) = messages
            .get(delivery.message_index)
            .and_then(ServerToProxyMessage::payload)
        else {
            failures.push(ProxyBackendFlushFailure {
                delivery_index,
                recipient: delivery.recipient,
                kind: ProxyBackendFlushFailureKind::MissingPayload,
                message: None,
            });
            continue;
        };

        match write_payload(delivery.recipient, payload) {
            Ok(()) => delivered.push(*delivery),
            Err(error) => failures.push(ProxyBackendFlushFailure {
                delivery_index,
                recipient: delivery.recipient,
                kind: error.kind,
                message: error.message,
            }),
        }
    }

    ProxyBackendFlushReport {
        delivered,
        failures,
    }
}

/// Opt-in backend resource for proxy broadcast experiments.
#[derive(Resource, Clone, Default, PartialEq, Eq, Debug)]
pub struct ProxyBroadcastBackend {
    enabled: bool,
    policy: ProxyBackpressurePolicy,
    state: ProxyRouteState,
}

impl ProxyBroadcastBackend {
    /// Creates a disabled backend resource.
    pub fn disabled() -> Self {
        Self::default()
    }

    /// Creates an enabled backend resource over explicit state and policy.
    pub const fn enabled(policy: ProxyBackpressurePolicy, state: ProxyRouteState) -> Self {
        Self {
            enabled: true,
            policy,
            state,
        }
    }

    /// Returns whether proxy broadcast routing is enabled.
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the configured backpressure policy.
    pub const fn policy(&self) -> ProxyBackpressurePolicy {
        self.policy
    }

    /// Returns the explicit proxy state snapshot.
    pub const fn state(&self) -> &ProxyRouteState {
        &self.state
    }

    /// Plans deliveries only when the backend is enabled.
    pub fn plan(&self, messages: &[ServerToProxyMessage]) -> ProxyDeliveryPlan {
        if self.enabled {
            plan_proxy_deliveries(messages, &self.state, self.policy)
        } else {
            ProxyDeliveryPlan::default()
        }
    }
}

/// Opt-in Bevy plugin for proxy broadcast backend state.
///
/// Adding this plugin only initializes a disabled [`ProxyBroadcastBackend`]
/// resource. Applications must replace that resource with an enabled backend to
/// plan proxy deliveries.
pub struct ProxyBroadcastBackendPlugin;

impl Plugin for ProxyBroadcastBackendPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProxyBroadcastBackend>();
    }
}

struct IndexedProxyState<'a> {
    players: BTreeMap<ProxyStreamId, &'a ProxyPlayerState>,
    channels: BTreeSet<ProxyChannelId>,
    failures: Vec<ProxyMessageDiagnostic>,
}

fn index_state(state: &ProxyRouteState) -> IndexedProxyState<'_> {
    let mut players = BTreeMap::new();
    let mut channels = BTreeSet::new();
    let mut failures = Vec::new();

    for channel in &state.channels {
        if !channels.insert(channel.channel) {
            failures.push(ProxyMessageDiagnostic::state(
                ProxyMessageDiagnosticKind::DuplicateChannel {
                    channel: channel.channel,
                },
            ));
        }
    }

    for player in &state.players {
        if players.insert(player.stream, player).is_some() {
            failures.push(ProxyMessageDiagnostic::state(
                ProxyMessageDiagnosticKind::DuplicateStream {
                    stream: player.stream,
                },
            ));
        }
        if let Some(position) = player.position {
            if !valid_chunk_position(position) {
                failures.push(ProxyMessageDiagnostic::state(
                    ProxyMessageDiagnosticKind::InvalidPlayerPosition {
                        stream: player.stream,
                        position,
                    },
                ));
            }
        }
        for channel in &player.subscriptions {
            if !channels.contains(channel) {
                failures.push(ProxyMessageDiagnostic::state(
                    ProxyMessageDiagnosticKind::StaleSubscription {
                        stream: player.stream,
                        channel: *channel,
                    },
                ));
            }
        }
    }

    IndexedProxyState {
        players,
        channels,
        failures,
    }
}

fn validate_server_to_proxy_message(
    message: &ServerToProxyMessage,
    state: &IndexedProxyState<'_>,
    policy: ProxyBackpressurePolicy,
) -> Result<(), ProxyMessageDiagnosticKind> {
    match message {
        ServerToProxyMessage::UpdatePlayerPosition(update) => {
            require_active_stream(update.stream, state)?;
            validate_position_for_stream(update.stream, update.position)
        }
        ServerToProxyMessage::UpdatePlayerPositions(updates) => {
            for update in updates {
                require_active_stream(update.stream, state)?;
                validate_position_for_stream(update.stream, update.position)?;
            }
            Ok(())
        }
        ServerToProxyMessage::AddChannel { channel } => {
            if state.channels.contains(channel) {
                Err(ProxyMessageDiagnosticKind::DuplicateChannel { channel: *channel })
            } else {
                Ok(())
            }
        }
        ServerToProxyMessage::RemoveChannel { channel } => require_known_channel(*channel, state),
        ServerToProxyMessage::SubscribeChannelPackets {
            channel,
            exclude,
            payload,
        } => {
            require_known_channel(*channel, state)?;
            validate_optional_active_stream(*exclude, state)?;
            validate_payload(payload, policy)
        }
        ServerToProxyMessage::SetReceiveBroadcasts { stream }
        | ServerToProxyMessage::Shutdown { stream } => require_active_stream(*stream, state),
        ServerToProxyMessage::BroadcastGlobal { exclude, payload } => {
            validate_optional_active_stream(*exclude, state)?;
            validate_payload(payload, policy)
        }
        ServerToProxyMessage::BroadcastLocal {
            center,
            exclude,
            payload,
            ..
        } => {
            validate_optional_active_stream(*exclude, state)?;
            validate_position_for_stream(ProxyStreamId::new(u64::MIN), *center)?;
            validate_payload(payload, policy)
        }
        ServerToProxyMessage::BroadcastChannel {
            channel,
            exclude,
            payload,
        } => {
            require_known_channel(*channel, state)?;
            validate_optional_active_stream(*exclude, state)?;
            validate_payload(payload, policy)
        }
        ServerToProxyMessage::Unicast { stream, payload } => {
            require_active_stream(*stream, state)?;
            validate_payload(payload, policy)
        }
    }
}

fn plan_message_delivery(
    message_index: usize,
    message: &ServerToProxyMessage,
    state: &IndexedProxyState<'_>,
    deliveries: &mut Vec<ProxyDelivery>,
) {
    match message {
        ServerToProxyMessage::BroadcastGlobal { exclude, payload } => {
            let recipients = state.players.values().filter(|player| {
                player.is_active() && player.receives_broadcasts && Some(player.stream) != *exclude
            });
            push_deliveries(
                message_index,
                recipients,
                ProxyDeliveryRoute::Global,
                payload.len(),
                deliveries,
            );
        }
        ServerToProxyMessage::BroadcastLocal {
            center,
            radius_chunks,
            exclude,
            payload,
        } => {
            let recipients = state.players.values().filter(|player| {
                player.is_active()
                    && player.receives_broadcasts
                    && Some(player.stream) != *exclude
                    && player.position.is_some_and(|position| {
                        within_chunk_radius(position, *center, *radius_chunks)
                    })
            });
            push_deliveries(
                message_index,
                recipients,
                ProxyDeliveryRoute::Local,
                payload.len(),
                deliveries,
            );
        }
        ServerToProxyMessage::BroadcastChannel {
            channel,
            exclude,
            payload,
        }
        | ServerToProxyMessage::SubscribeChannelPackets {
            channel,
            exclude,
            payload,
        } => {
            let recipients = state.players.values().filter(|player| {
                player.is_active()
                    && player.subscriptions.contains(channel)
                    && Some(player.stream) != *exclude
            });
            push_deliveries(
                message_index,
                recipients,
                ProxyDeliveryRoute::Channel(*channel),
                payload.len(),
                deliveries,
            );
        }
        ServerToProxyMessage::Unicast { stream, payload } => {
            deliveries.push(ProxyDelivery {
                message_index,
                recipient: *stream,
                route: ProxyDeliveryRoute::Unicast,
                payload_len: payload.len(),
            });
        }
        ServerToProxyMessage::UpdatePlayerPosition(_)
        | ServerToProxyMessage::UpdatePlayerPositions(_)
        | ServerToProxyMessage::AddChannel { .. }
        | ServerToProxyMessage::RemoveChannel { .. }
        | ServerToProxyMessage::SetReceiveBroadcasts { .. }
        | ServerToProxyMessage::Shutdown { .. } => {}
    }
}

fn push_deliveries<'a>(
    message_index: usize,
    recipients: impl Iterator<Item = &'a &'a ProxyPlayerState>,
    route: ProxyDeliveryRoute,
    payload_len: usize,
    deliveries: &mut Vec<ProxyDelivery>,
) {
    for player in recipients {
        deliveries.push(ProxyDelivery {
            message_index,
            recipient: player.stream,
            route,
            payload_len,
        });
    }
}

fn require_known_stream(
    stream: ProxyStreamId,
    state: &IndexedProxyState<'_>,
) -> Result<(), ProxyMessageDiagnosticKind> {
    if state.players.contains_key(&stream) {
        Ok(())
    } else {
        Err(ProxyMessageDiagnosticKind::UnknownStream { stream })
    }
}

fn require_active_stream(
    stream: ProxyStreamId,
    state: &IndexedProxyState<'_>,
) -> Result<(), ProxyMessageDiagnosticKind> {
    let Some(player) = state.players.get(&stream).copied() else {
        return Err(ProxyMessageDiagnosticKind::UnknownStream { stream });
    };
    if player.is_active() {
        Ok(())
    } else {
        Err(ProxyMessageDiagnosticKind::InactiveStream { stream })
    }
}

fn validate_optional_active_stream(
    stream: Option<ProxyStreamId>,
    state: &IndexedProxyState<'_>,
) -> Result<(), ProxyMessageDiagnosticKind> {
    match stream {
        Some(stream) => require_active_stream(stream, state),
        None => Ok(()),
    }
}

fn require_known_channel(
    channel: ProxyChannelId,
    state: &IndexedProxyState<'_>,
) -> Result<(), ProxyMessageDiagnosticKind> {
    if state.channels.contains(&channel) {
        Ok(())
    } else {
        Err(ProxyMessageDiagnosticKind::UnknownChannel { channel })
    }
}

fn validate_position_for_stream(
    stream: ProxyStreamId,
    position: ProxyChunkPosition,
) -> Result<(), ProxyMessageDiagnosticKind> {
    if valid_chunk_position(position) {
        Ok(())
    } else {
        Err(ProxyMessageDiagnosticKind::InvalidPlayerPosition { stream, position })
    }
}

fn validate_payload(
    payload: &[u8],
    policy: ProxyBackpressurePolicy,
) -> Result<(), ProxyMessageDiagnosticKind> {
    if payload.is_empty() {
        return Err(ProxyMessageDiagnosticKind::MalformedPayload);
    }
    if payload.len() > policy.max_queued_payload_bytes {
        return Err(ProxyMessageDiagnosticKind::BackpressureLimitExceeded {
            payload_len: payload.len(),
            limit: policy.max_queued_payload_bytes,
        });
    }
    Ok(())
}

fn set_player_position(
    state: &mut ProxyRouteState,
    stream: ProxyStreamId,
    position: ProxyChunkPosition,
) {
    if let Some(player) = state
        .players
        .iter_mut()
        .find(|player| player.stream == stream)
    {
        player.position = Some(position);
    }
}

fn valid_chunk_position(position: ProxyChunkPosition) -> bool {
    position.x.abs() <= MAX_CHUNK_COORDINATE_ABS && position.z.abs() <= MAX_CHUNK_COORDINATE_ABS
}

fn within_chunk_radius(
    position: ProxyChunkPosition,
    center: ProxyChunkPosition,
    radius_chunks: u16,
) -> bool {
    let dx = i64::from(position.x) - i64::from(center.x);
    let dz = i64::from(position.z) - i64::from(center.z);
    dx.abs().max(dz.abs()) <= i64::from(radius_chunks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_STREAM_RAW: u64 = 11;
    const SECOND_STREAM_RAW: u64 = 12;
    const THIRD_STREAM_RAW: u64 = 13;
    const UNKNOWN_STREAM_RAW: u64 = 404;
    const FIRST_CHANNEL_RAW: u32 = 7;
    const UNKNOWN_CHANNEL_RAW: u32 = 99;
    const CENTER_CHUNK: ProxyChunkPosition = ProxyChunkPosition::new(0, 0);
    const IN_RANGE_CHUNK: ProxyChunkPosition = ProxyChunkPosition::new(2, 0);
    const OUT_OF_RANGE_CHUNK: ProxyChunkPosition = ProxyChunkPosition::new(5, 0);
    const INVALID_CHUNK: ProxyChunkPosition =
        ProxyChunkPosition::new(MAX_CHUNK_COORDINATE_ABS + 1, 0);
    const LOCAL_RADIUS_CHUNKS: u16 = 3;
    const TINY_PAYLOAD_LIMIT: usize = 2;
    const BACKPRESSURE_MESSAGE: &str = "proxy queue full";
    const PAYLOAD: &[u8] = b"packet";
    const OTHER_PAYLOAD: &[u8] = b"other-packet";

    #[test]
    fn global_local_channel_and_unicast_routes_are_deterministic() {
        let first = first_stream();
        let second = second_stream();
        let third = third_stream();
        let channel = first_channel();
        let state = route_state()
            .with_player(
                ProxyPlayerState::active(first)
                    .with_position(CENTER_CHUNK)
                    .receiving_broadcasts()
                    .subscribed_to(channel),
            )
            .with_player(
                ProxyPlayerState::active(second)
                    .with_position(IN_RANGE_CHUNK)
                    .receiving_broadcasts(),
            )
            .with_player(
                ProxyPlayerState::active(third)
                    .with_position(OUT_OF_RANGE_CHUNK)
                    .subscribed_to(channel),
            );
        let messages = vec![
            ServerToProxyMessage::BroadcastGlobal {
                exclude: Some(first),
                payload: PAYLOAD.to_vec(),
            },
            ServerToProxyMessage::BroadcastLocal {
                center: CENTER_CHUNK,
                radius_chunks: LOCAL_RADIUS_CHUNKS,
                exclude: Some(first),
                payload: PAYLOAD.to_vec(),
            },
            ServerToProxyMessage::BroadcastChannel {
                channel,
                exclude: Some(first),
                payload: OTHER_PAYLOAD.to_vec(),
            },
            ServerToProxyMessage::Unicast {
                stream: first,
                payload: PAYLOAD.to_vec(),
            },
        ];

        let plan = plan_proxy_deliveries(&messages, &state, ProxyBackpressurePolicy::default());
        let deliveries: Vec<(usize, ProxyStreamId, ProxyDeliveryRoute, usize)> = plan
            .deliveries()
            .iter()
            .map(|delivery| {
                (
                    delivery.message_index,
                    delivery.recipient,
                    delivery.route,
                    delivery.payload_len,
                )
            })
            .collect();

        assert!(plan.is_success());
        assert_eq!(
            deliveries,
            vec![
                (0, second, ProxyDeliveryRoute::Global, PAYLOAD.len()),
                (1, second, ProxyDeliveryRoute::Local, PAYLOAD.len()),
                (
                    2,
                    third,
                    ProxyDeliveryRoute::Channel(channel),
                    OTHER_PAYLOAD.len()
                ),
                (3, first, ProxyDeliveryRoute::Unicast, PAYLOAD.len()),
            ]
        );
    }

    #[test]
    fn invalid_visibility_state_fails_closed_before_delivery() {
        let first = first_stream();
        let unknown_channel = ProxyChannelId::new(UNKNOWN_CHANNEL_RAW);
        let state = ProxyRouteState::empty().with_player(
            ProxyPlayerState::active(first)
                .with_position(CENTER_CHUNK)
                .receiving_broadcasts()
                .subscribed_to(unknown_channel),
        );
        let messages = vec![ServerToProxyMessage::BroadcastGlobal {
            exclude: None,
            payload: PAYLOAD.to_vec(),
        }];

        let plan = plan_proxy_deliveries(&messages, &state, ProxyBackpressurePolicy::default());

        assert!(plan.deliveries().is_empty());
        assert_eq!(
            plan.failures(),
            &[ProxyMessageDiagnostic {
                message_index: None,
                kind: ProxyMessageDiagnosticKind::StaleSubscription {
                    stream: first,
                    channel: unknown_channel,
                },
            }]
        );
    }

    #[test]
    fn malformed_unknown_and_backpressure_messages_are_rejected() {
        let state = route_state().with_player(ProxyPlayerState::active(first_stream()));
        let policy = ProxyBackpressurePolicy {
            max_queued_payload_bytes: TINY_PAYLOAD_LIMIT,
        };
        let messages = vec![
            ServerToProxyMessage::Unicast {
                stream: ProxyStreamId::new(UNKNOWN_STREAM_RAW),
                payload: PAYLOAD.to_vec(),
            },
            ServerToProxyMessage::Unicast {
                stream: first_stream(),
                payload: Vec::new(),
            },
            ServerToProxyMessage::Unicast {
                stream: first_stream(),
                payload: PAYLOAD.to_vec(),
            },
        ];

        let plan = plan_proxy_deliveries(&messages, &state, policy);

        assert!(plan.deliveries().is_empty());
        assert_eq!(
            plan.failures(),
            &[
                ProxyMessageDiagnostic {
                    message_index: Some(0),
                    kind: ProxyMessageDiagnosticKind::UnknownStream {
                        stream: ProxyStreamId::new(UNKNOWN_STREAM_RAW),
                    },
                },
                ProxyMessageDiagnostic {
                    message_index: Some(1),
                    kind: ProxyMessageDiagnosticKind::MalformedPayload,
                },
                ProxyMessageDiagnostic {
                    message_index: Some(2),
                    kind: ProxyMessageDiagnosticKind::BackpressureLimitExceeded {
                        payload_len: PAYLOAD.len(),
                        limit: TINY_PAYLOAD_LIMIT,
                    },
                },
            ]
        );
    }

    #[test]
    fn state_reducers_handle_lifecycle_subscription_position_and_shutdown() {
        let policy = ProxyBackpressurePolicy::default();
        let channel = first_channel();
        let connected = apply_proxy_to_server_message(
            &ProxyRouteState::empty(),
            &ProxyToServerMessage::PlayerConnect {
                stream: first_stream(),
            },
            policy,
        )
        .unwrap();
        let with_channel = apply_server_to_proxy_message(
            &connected,
            &ServerToProxyMessage::AddChannel { channel },
            policy,
        )
        .unwrap();
        let subscribed = apply_proxy_to_server_message(
            &with_channel,
            &ProxyToServerMessage::RequestSubscribeChannelPackets {
                stream: first_stream(),
                channels: vec![channel],
            },
            policy,
        )
        .unwrap();
        let positioned = apply_server_to_proxy_message(
            &subscribed,
            &ServerToProxyMessage::UpdatePlayerPosition(ProxyPlayerPosition::new(
                first_stream(),
                CENTER_CHUNK,
            )),
            policy,
        )
        .unwrap();
        let receiving = apply_server_to_proxy_message(
            &positioned,
            &ServerToProxyMessage::SetReceiveBroadcasts {
                stream: first_stream(),
            },
            policy,
        )
        .unwrap();
        let shutting_down = apply_server_to_proxy_message(
            &receiving,
            &ServerToProxyMessage::Shutdown {
                stream: first_stream(),
            },
            policy,
        )
        .unwrap();

        assert_eq!(
            shutting_down.players[0].lifecycle,
            ProxyStreamLifecycle::ShuttingDown
        );
        assert!(shutting_down.players[0].subscriptions.contains(&channel));
        assert_eq!(shutting_down.players[0].position, Some(CENTER_CHUNK));
        assert!(shutting_down.players[0].receives_broadcasts);

        let plan = plan_proxy_deliveries(
            &[ServerToProxyMessage::BroadcastGlobal {
                exclude: None,
                payload: PAYLOAD.to_vec(),
            }],
            &shutting_down,
            policy,
        );
        assert!(plan.deliveries().is_empty());
    }

    #[test]
    fn state_reducers_reject_invalid_positions_unknown_channels_and_duplicate_streams() {
        let policy = ProxyBackpressurePolicy::default();
        let state = ProxyRouteState::empty().with_player(ProxyPlayerState::active(first_stream()));

        assert_eq!(
            apply_proxy_to_server_message(
                &state,
                &ProxyToServerMessage::PlayerConnect {
                    stream: first_stream(),
                },
                policy,
            ),
            Err(ProxyMessageDiagnosticKind::StreamAlreadyConnected {
                stream: first_stream(),
            })
        );
        assert_eq!(
            apply_proxy_to_server_message(
                &state,
                &ProxyToServerMessage::RequestSubscribeChannelPackets {
                    stream: first_stream(),
                    channels: vec![first_channel()],
                },
                policy,
            ),
            Err(ProxyMessageDiagnosticKind::UnknownChannel {
                channel: first_channel(),
            })
        );
        assert_eq!(
            apply_server_to_proxy_message(
                &state,
                &ServerToProxyMessage::UpdatePlayerPosition(ProxyPlayerPosition::new(
                    first_stream(),
                    INVALID_CHUNK,
                )),
                policy,
            ),
            Err(ProxyMessageDiagnosticKind::InvalidPlayerPosition {
                stream: first_stream(),
                position: INVALID_CHUNK,
            })
        );
    }

    #[test]
    fn proxy_flush_shell_reports_disconnected_streams_and_backpressure() {
        let first = first_stream();
        let second = second_stream();
        let third = third_stream();
        let state = route_state()
            .with_player(ProxyPlayerState::active(first).receiving_broadcasts())
            .with_player(ProxyPlayerState::active(second).receiving_broadcasts())
            .with_player(ProxyPlayerState::active(third).receiving_broadcasts());
        let messages = vec![ServerToProxyMessage::BroadcastGlobal {
            exclude: None,
            payload: PAYLOAD.to_vec(),
        }];
        let plan = plan_proxy_deliveries(&messages, &state, ProxyBackpressurePolicy::default());
        let mut written = BTreeMap::<ProxyStreamId, Vec<u8>>::new();

        let report = flush_proxy_delivery_plan_with(&plan, &messages, |stream, payload| {
            if stream == second {
                return Err(ProxyBackendWriteError::disconnected_stream());
            }
            if stream == third {
                return Err(ProxyBackendWriteError::backpressure(BACKPRESSURE_MESSAGE));
            }
            written.insert(stream, payload.to_vec());
            Ok(())
        });

        assert_eq!(report.delivered(), &[plan.deliveries()[0]]);
        assert_eq!(written.get(&first), Some(&PAYLOAD.to_vec()));
        assert_eq!(
            report.failures(),
            &[
                ProxyBackendFlushFailure {
                    delivery_index: 1,
                    recipient: second,
                    kind: ProxyBackendFlushFailureKind::DisconnectedStream,
                    message: None,
                },
                ProxyBackendFlushFailure {
                    delivery_index: 2,
                    recipient: third,
                    kind: ProxyBackendFlushFailureKind::Backpressure,
                    message: Some(BACKPRESSURE_MESSAGE.to_owned()),
                },
            ]
        );
    }

    #[test]
    fn optional_backend_plugin_is_disabled_until_enabled_by_application() {
        let mut app = App::new();
        app.add_plugins(ProxyBroadcastBackendPlugin);

        let backend = app.world().resource::<ProxyBroadcastBackend>();

        assert!(!backend.is_enabled());
        assert!(backend.plan(&[]).is_success());
        assert!(backend.plan(&[]).deliveries().is_empty());
    }

    fn route_state() -> ProxyRouteState {
        ProxyRouteState::empty().with_channel(ProxyChannelState::new(first_channel()))
    }

    const fn first_stream() -> ProxyStreamId {
        ProxyStreamId::new(FIRST_STREAM_RAW)
    }

    const fn second_stream() -> ProxyStreamId {
        ProxyStreamId::new(SECOND_STREAM_RAW)
    }

    const fn third_stream() -> ProxyStreamId {
        ProxyStreamId::new(THIRD_STREAM_RAW)
    }

    const fn first_channel() -> ProxyChannelId {
        ProxyChannelId::new(FIRST_CHANNEL_RAW)
    }
}
