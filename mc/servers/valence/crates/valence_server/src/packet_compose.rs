//! Packet bundle composition and route planning for outbound packets.
//!
//! Compose is an opt-in layer over Valence's existing [`Client`] packet writes.
//! It separates a pure planning step from an imperative direct flush shell:
//!
//! - [`PacketBundle`] preserves author order for packet frames.
//! - [`PacketRouteIntent`] describes unicast, global, local, and group delivery.
//! - [`plan_packet_delivery`] deterministically resolves route intents against
//!   explicit client snapshots without touching ECS state or network sockets.
//! - [`flush_packet_delivery_plan_to_clients`] is the direct-mode shell that
//!   writes planned bundles to live clients and reports partial failures.
//!
//! Use direct [`Client`] writes when a system already owns one client, when a
//! packet must be encoded with per-client state at the last possible moment, or
//! when normal end-of-tick flushing is the clearest behavior. Use compose when a
//! system benefits from testing route selection separately from Valence client
//! writes. The route intent model is intentionally backend-neutral so a future
//! proxy backend can consume the same delivery plan without changing default
//! networking behavior.
//!
//! ```no_run
//! use std::collections::BTreeSet;
//!
//! use valence_server::ecs::entity::Entity;
//! use valence_server::math::DVec3;
//! use valence_server::packet_compose::{
//!     plan_packet_delivery, PacketBundle, PacketBundleId, PacketComposeClient,
//!     PacketComposeGroup, PacketRoute, PacketRouteIntent,
//! };
//!
//! const FIRST_CLIENT_INDEX: u32 = 11;
//! const SECOND_CLIENT_INDEX: u32 = 12;
//! const LOCAL_RADIUS: f64 = 16.0;
//!
//! let first = Entity::from_raw(FIRST_CLIENT_INDEX);
//! let second = Entity::from_raw(SECOND_CLIENT_INDEX);
//! let spectators = PacketComposeGroup::new("spectators");
//!
//! let bundle = PacketBundle::from_packet_bytes([b"length-prefixed packet bytes".to_vec()]);
//! let clients = [
//!     PacketComposeClient::active(first, DVec3::ZERO).with_group(spectators.clone()),
//!     PacketComposeClient::active(second, DVec3::X * LOCAL_RADIUS),
//! ];
//! let intents = [
//!     PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Global).exclude(first),
//!     PacketRouteIntent::new(
//!         PacketBundleId::new(0),
//!         PacketRoute::Local {
//!             center: DVec3::ZERO,
//!             radius: LOCAL_RADIUS,
//!         },
//!     ),
//!     PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Group(spectators)),
//! ];
//!
//! let plan = plan_packet_delivery(&intents, &clients, &[bundle]);
//! assert!(plan.failures().is_empty());
//! ```
//!
//! [`Client`]: crate::client::Client

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use bevy_ecs::prelude::{Entity, Query};
use valence_math::DVec3;
use valence_protocol::encode::{PacketWriter, WritePacket};
use valence_protocol::{CompressionThreshold, Encode, Packet};

use crate::client::Client;

/// Stable index of a packet bundle in the bundle slice passed to the planner.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PacketBundleId(usize);

impl PacketBundleId {
    /// Creates a bundle identifier from its slice index.
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the slice index for this bundle identifier.
    pub fn index(self) -> usize {
        self.0
    }
}

/// One already-framed packet payload in a composed bundle.
///
/// The bytes should be complete packet-frame bytes that can be copied into a
/// Valence [`WritePacket`] sink with [`WritePacket::write_packet_bytes`]. Use
/// [`PacketBundle::try_push_packet`] when callers want Valence protocol encoding
/// to produce those bytes with an explicit compression threshold.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PacketFrame {
    bytes: Vec<u8>,
}

impl PacketFrame {
    /// Creates a packet frame from already-encoded packet bytes.
    pub fn new<B>(bytes: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        Self {
            bytes: bytes.into(),
        }
    }

    /// Returns the frame bytes in author order.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Ordered packet frames delivered together to each resolved recipient.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct PacketBundle {
    frames: Vec<PacketFrame>,
}

impl PacketBundle {
    /// Creates an empty packet bundle.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a bundle from already-encoded packet frames.
    pub fn from_packet_bytes<I>(frames: I) -> Self
    where
        I: IntoIterator<Item = Vec<u8>>,
    {
        let frames = frames.into_iter().map(PacketFrame::new).collect();
        Self { frames }
    }

    /// Appends already-encoded packet bytes to this bundle.
    pub fn push_packet_bytes<B>(&mut self, bytes: B)
    where
        B: Into<Vec<u8>>,
    {
        self.frames.push(PacketFrame::new(bytes));
    }

    /// Encodes one packet with an explicit compression threshold and appends it.
    ///
    /// Encoding into a temporary buffer keeps this bundle unchanged when packet
    /// encoding fails.
    pub fn try_push_packet<P>(
        &mut self,
        packet: &P,
        threshold: CompressionThreshold,
    ) -> Result<(), PacketBundleError>
    where
        P: Packet + Encode,
    {
        let mut bytes = Vec::new();
        PacketWriter::new(&mut bytes, threshold)
            .write_packet_fallible(packet)
            .map_err(|source| PacketBundleError::encode(P::NAME, source))?;
        self.push_packet_bytes(bytes);
        Ok(())
    }

    /// Returns packet frames in author order.
    pub fn frames(&self) -> &[PacketFrame] {
        &self.frames
    }

    /// Returns true when this bundle contains no packet frames.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Returns the number of packet frames in this bundle.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

/// Structured packet bundle construction error.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PacketBundleError {
    packet_name: &'static str,
    message: String,
}

impl PacketBundleError {
    fn encode(packet_name: &'static str, source: anyhow::Error) -> Self {
        Self {
            packet_name,
            message: source.to_string(),
        }
    }

    /// Returns the packet name that failed to encode.
    pub fn packet_name(&self) -> &'static str {
        self.packet_name
    }

    /// Returns the encoder diagnostic message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PacketBundleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to encode packet '{}': {}",
            self.packet_name, self.message
        )
    }
}

impl std::error::Error for PacketBundleError {}

/// Named channel-like group for packet compose routing.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PacketComposeGroup(String);

impl PacketComposeGroup {
    /// Creates a group identifier from application-owned text.
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(name.into())
    }

    /// Returns the group name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for PacketComposeGroup {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for PacketComposeGroup {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Snapshot of client routing metadata consumed by the pure planner.
#[derive(Clone, PartialEq, Debug)]
pub struct PacketComposeClient {
    /// Client entity represented by this snapshot.
    pub entity: Entity,
    /// Whether this client should receive direct-mode compose deliveries.
    pub active: bool,
    /// Position used by local radius routes.
    pub position: DVec3,
    /// Channel-like groups this client belongs to.
    pub groups: BTreeSet<PacketComposeGroup>,
}

impl PacketComposeClient {
    /// Creates an active client snapshot.
    pub fn active(entity: Entity, position: DVec3) -> Self {
        Self {
            entity,
            active: true,
            position,
            groups: BTreeSet::new(),
        }
    }

    /// Creates an inactive client snapshot.
    pub fn inactive(entity: Entity, position: DVec3) -> Self {
        Self {
            entity,
            active: false,
            position,
            groups: BTreeSet::new(),
        }
    }

    /// Returns this snapshot with membership in one additional group.
    pub fn with_group<G>(mut self, group: G) -> Self
    where
        G: Into<PacketComposeGroup>,
    {
        self.groups.insert(group.into());
        self
    }
}

/// Backend-neutral route intent for one packet bundle.
#[derive(Clone, PartialEq, Debug)]
pub enum PacketRoute {
    /// Deliver to one client entity.
    Unicast(Entity),
    /// Deliver to all active clients.
    Global,
    /// Deliver to active clients inside a radius around a point.
    Local { center: DVec3, radius: f64 },
    /// Deliver to active clients that are members of a named group.
    Group(PacketComposeGroup),
}

/// One bundle plus one route and optional exclusions.
#[derive(Clone, PartialEq, Debug)]
pub struct PacketRouteIntent {
    /// Bundle selected by this route intent.
    pub bundle: PacketBundleId,
    /// Route used to resolve recipients.
    pub route: PacketRoute,
    /// Client entities excluded from this route.
    pub exclude: BTreeSet<Entity>,
}

impl PacketRouteIntent {
    /// Creates a route intent with no exclusions.
    pub fn new(bundle: PacketBundleId, route: PacketRoute) -> Self {
        Self {
            bundle,
            route,
            exclude: BTreeSet::new(),
        }
    }

    /// Returns this route intent with one additional excluded client.
    pub fn exclude(mut self, client: Entity) -> Self {
        self.exclude.insert(client);
        self
    }
}

/// One planned delivery of a bundle to one recipient.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PacketDelivery {
    /// Route intent index that produced this delivery.
    pub route_index: usize,
    /// Bundle selected for this delivery.
    pub bundle: PacketBundleId,
    /// Recipient client entity.
    pub recipient: Entity,
}

/// Deterministic route-planning result.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct PacketDeliveryPlan {
    deliveries: Vec<PacketDelivery>,
    failures: Vec<PacketPlanFailure>,
}

impl PacketDeliveryPlan {
    /// Returns all successful deliveries in deterministic order.
    pub fn deliveries(&self) -> &[PacketDelivery] {
        &self.deliveries
    }

    /// Returns structured route-planning failures.
    pub fn failures(&self) -> &[PacketPlanFailure] {
        &self.failures
    }

    /// Returns true when the planner produced no failures.
    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Structured route-planning failure.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PacketPlanFailure {
    /// Route intent index that failed.
    pub route_index: usize,
    /// Failure category.
    pub kind: PacketPlanFailureKind,
}

/// Route-planning failure category.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PacketPlanFailureKind {
    /// A route referenced a bundle index outside the bundle slice.
    MissingBundle { bundle: PacketBundleId },
    /// Two client snapshots used the same entity.
    DuplicateClient { client: Entity },
    /// A unicast route targeted an entity absent from client inputs.
    MissingUnicastTarget { target: Entity },
    /// A unicast route targeted an inactive client.
    InactiveUnicastTarget { target: Entity },
    /// A group route had no active members in client inputs.
    MissingGroup { group: PacketComposeGroup },
    /// A local route had a non-finite center, non-finite radius, negative radius,
    /// or overflowing squared radius.
    InvalidLocalRoute,
    /// A local route considered a client with a non-finite position.
    InvalidClientPosition { client: Entity },
}

/// Plans packet delivery without touching Valence clients or network sockets.
///
/// Recipients are deterministic: client snapshots are keyed by [`Entity`], group
/// members are sorted, exclusions are applied after route resolution, and bundle
/// ordering remains in [`PacketBundle`] for the flush shell.
pub fn plan_packet_delivery(
    intents: &[PacketRouteIntent],
    clients: &[PacketComposeClient],
    bundles: &[PacketBundle],
) -> PacketDeliveryPlan {
    let client_inputs = index_clients(clients);
    let groups = index_groups(&client_inputs.clients);
    let mut deliveries = Vec::new();
    let mut failures = client_inputs.failures;

    for (route_index, intent) in intents.iter().enumerate() {
        if intent.bundle.index() >= bundles.len() {
            failures.push(PacketPlanFailure {
                route_index,
                kind: PacketPlanFailureKind::MissingBundle {
                    bundle: intent.bundle,
                },
            });
            continue;
        }

        match route_recipients(&intent.route, &client_inputs.clients, &groups) {
            Ok(recipients) => {
                for recipient in recipients {
                    if !intent.exclude.contains(&recipient) {
                        deliveries.push(PacketDelivery {
                            route_index,
                            bundle: intent.bundle,
                            recipient,
                        });
                    }
                }
            }
            Err(kind) => failures.push(PacketPlanFailure { route_index, kind }),
        }
    }

    PacketDeliveryPlan {
        deliveries,
        failures,
    }
}

struct ClientIndex<'a> {
    clients: BTreeMap<Entity, &'a PacketComposeClient>,
    failures: Vec<PacketPlanFailure>,
}

fn index_clients(clients: &[PacketComposeClient]) -> ClientIndex<'_> {
    let mut indexed = BTreeMap::new();
    let mut failures = Vec::new();

    for client in clients {
        if indexed.insert(client.entity, client).is_some() {
            failures.push(PacketPlanFailure {
                route_index: 0,
                kind: PacketPlanFailureKind::DuplicateClient {
                    client: client.entity,
                },
            });
        }
    }

    ClientIndex {
        clients: indexed,
        failures,
    }
}

fn index_groups(
    clients: &BTreeMap<Entity, &PacketComposeClient>,
) -> BTreeMap<PacketComposeGroup, BTreeSet<Entity>> {
    let mut groups = BTreeMap::<PacketComposeGroup, BTreeSet<Entity>>::new();

    for client in clients.values() {
        if client.active {
            for group in &client.groups {
                groups
                    .entry(group.clone())
                    .or_default()
                    .insert(client.entity);
            }
        }
    }

    groups
}

fn route_recipients(
    route: &PacketRoute,
    clients: &BTreeMap<Entity, &PacketComposeClient>,
    groups: &BTreeMap<PacketComposeGroup, BTreeSet<Entity>>,
) -> Result<BTreeSet<Entity>, PacketPlanFailureKind> {
    match route {
        PacketRoute::Unicast(target) => unicast_recipient(*target, clients),
        PacketRoute::Global => Ok(global_recipients(clients)),
        PacketRoute::Local { center, radius } => local_recipients(*center, *radius, clients),
        PacketRoute::Group(group) => group_recipients(group, groups),
    }
}

fn unicast_recipient(
    target: Entity,
    clients: &BTreeMap<Entity, &PacketComposeClient>,
) -> Result<BTreeSet<Entity>, PacketPlanFailureKind> {
    match clients.get(&target) {
        Some(client) if client.active => Ok(BTreeSet::from([target])),
        Some(_) => Err(PacketPlanFailureKind::InactiveUnicastTarget { target }),
        None => Err(PacketPlanFailureKind::MissingUnicastTarget { target }),
    }
}

fn global_recipients(clients: &BTreeMap<Entity, &PacketComposeClient>) -> BTreeSet<Entity> {
    clients
        .values()
        .filter(|client| client.active)
        .map(|client| client.entity)
        .collect()
}

fn local_recipients(
    center: DVec3,
    radius: f64,
    clients: &BTreeMap<Entity, &PacketComposeClient>,
) -> Result<BTreeSet<Entity>, PacketPlanFailureKind> {
    if !is_finite_vec3(center) || !radius.is_finite() || radius < 0.0 {
        return Err(PacketPlanFailureKind::InvalidLocalRoute);
    }

    let radius_squared = radius * radius;
    if !radius_squared.is_finite() {
        return Err(PacketPlanFailureKind::InvalidLocalRoute);
    }

    let mut recipients = BTreeSet::new();
    for client in clients.values() {
        if !client.active {
            continue;
        }
        if !is_finite_vec3(client.position) {
            return Err(PacketPlanFailureKind::InvalidClientPosition {
                client: client.entity,
            });
        }
        if client.position.distance_squared(center) <= radius_squared {
            recipients.insert(client.entity);
        }
    }

    Ok(recipients)
}

fn group_recipients(
    group: &PacketComposeGroup,
    groups: &BTreeMap<PacketComposeGroup, BTreeSet<Entity>>,
) -> Result<BTreeSet<Entity>, PacketPlanFailureKind> {
    groups
        .get(group)
        .cloned()
        .filter(|members| !members.is_empty())
        .ok_or_else(|| PacketPlanFailureKind::MissingGroup {
            group: group.clone(),
        })
}

fn is_finite_vec3(value: DVec3) -> bool {
    value.x.is_finite() && value.y.is_finite() && value.z.is_finite()
}

/// Result of direct-mode flush over a delivery plan.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct PacketFlushReport {
    delivered: Vec<PacketDelivery>,
    failures: Vec<PacketFlushFailure>,
}

impl PacketFlushReport {
    /// Returns delivery rows that were written and flushed successfully.
    pub fn delivered(&self) -> &[PacketDelivery] {
        &self.delivered
    }

    /// Returns direct-mode partial failures.
    pub fn failures(&self) -> &[PacketFlushFailure] {
        &self.failures
    }

    /// Returns true when direct-mode flush had no failures.
    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }
}

/// One direct-mode flush failure.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PacketFlushFailure {
    /// Delivery plan index that failed.
    pub delivery_index: usize,
    /// Recipient that could not be flushed.
    pub recipient: Entity,
    /// Bundle selected by the failed delivery.
    pub bundle: PacketBundleId,
    /// Failure category.
    pub kind: PacketFlushFailureKind,
    /// Backend diagnostic, when a lower-level write returned one.
    pub message: Option<String>,
}

/// Direct-mode flush failure category.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PacketFlushFailureKind {
    /// The delivery referenced a bundle index outside the bundle slice.
    MissingBundle,
    /// The planned client no longer has a live [`Client`] component.
    ClosedClient,
    /// The underlying client connection rejected flushed bytes.
    BackendWrite,
}

/// Error returned by a direct-mode write adapter.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PacketDirectWriteError {
    kind: PacketFlushFailureKind,
    message: Option<String>,
}

impl PacketDirectWriteError {
    /// Creates a closed-client direct write error.
    pub fn closed_client() -> Self {
        Self {
            kind: PacketFlushFailureKind::ClosedClient,
            message: None,
        }
    }

    /// Creates a backend-write direct write error.
    pub fn backend_write<E>(error: E) -> Self
    where
        E: fmt::Display,
    {
        Self {
            kind: PacketFlushFailureKind::BackendWrite,
            message: Some(error.to_string()),
        }
    }
}

/// Flushes a plan through an arbitrary direct write adapter.
///
/// This shell keeps route planning testable by accepting an adapter closure. Use
/// [`flush_packet_delivery_plan_to_clients`] for the Valence ECS client query
/// adapter.
pub fn flush_packet_delivery_plan_with<F>(
    plan: &PacketDeliveryPlan,
    bundles: &[PacketBundle],
    mut write_bundle: F,
) -> PacketFlushReport
where
    F: FnMut(Entity, &PacketBundle) -> Result<(), PacketDirectWriteError>,
{
    let mut delivered = Vec::new();
    let mut failures = Vec::new();

    for (delivery_index, delivery) in plan.deliveries().iter().enumerate() {
        let Some(bundle) = bundles.get(delivery.bundle.index()) else {
            failures.push(PacketFlushFailure {
                delivery_index,
                recipient: delivery.recipient,
                bundle: delivery.bundle,
                kind: PacketFlushFailureKind::MissingBundle,
                message: None,
            });
            continue;
        };

        match write_bundle(delivery.recipient, bundle) {
            Ok(()) => delivered.push(*delivery),
            Err(error) => failures.push(PacketFlushFailure {
                delivery_index,
                recipient: delivery.recipient,
                bundle: delivery.bundle,
                kind: error.kind,
                message: error.message,
            }),
        }
    }

    PacketFlushReport {
        delivered,
        failures,
    }
}

/// Flushes a delivery plan directly to live Valence clients.
///
/// This is opt-in. It writes bundle bytes into each recipient's [`Client`]
/// packet buffer, immediately calls [`Client::flush_packets`], and returns a
/// partial-failure report instead of changing Valence's default packet-write
/// behavior for systems that do not call compose.
pub fn flush_packet_delivery_plan_to_clients(
    plan: &PacketDeliveryPlan,
    bundles: &[PacketBundle],
    clients: &mut Query<&mut Client>,
) -> PacketFlushReport {
    flush_packet_delivery_plan_with(plan, bundles, |recipient, bundle| {
        let mut client = clients
            .get_mut(recipient)
            .map_err(|_| PacketDirectWriteError::closed_client())?;
        write_packet_bundle_to_client(client.as_mut(), bundle)
    })
}

/// Writes bundle frames to one client and immediately flushes the client's packet queue.
pub fn write_packet_bundle_to_client(
    client: &mut Client,
    bundle: &PacketBundle,
) -> Result<(), PacketDirectWriteError> {
    if bundle.is_empty() {
        return Ok(());
    }

    for frame in bundle.frames() {
        client.write_packet_bytes(frame.bytes());
    }

    client
        .flush_packets()
        .map_err(PacketDirectWriteError::backend_write)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::io::Write;

    use anyhow::bail;
    use valence_protocol::{PacketSide, PacketState};

    use super::*;

    const FIRST_CLIENT_INDEX: u32 = 101;
    const SECOND_CLIENT_INDEX: u32 = 102;
    const THIRD_CLIENT_INDEX: u32 = 103;
    const MISSING_CLIENT_INDEX: u32 = 404;
    const FIRST_PACKET: &[u8] = b"first-packet";
    const SECOND_PACKET: &[u8] = b"second-packet";
    const LOCAL_RADIUS: f64 = 8.0;
    const EDGE_DISTANCE: f64 = LOCAL_RADIUS;
    const OUTSIDE_DISTANCE: f64 = 12.0;
    const FAILING_PACKET_ID: i32 = 76;
    const FAILING_PACKET_NAME: &str = "packet_compose_failing_test_packet";
    const ENCODE_FAILURE_MESSAGE: &str = "forced encode failure";
    const BACKEND_FAILURE_MESSAGE: &str = "forced backend failure";

    #[test]
    fn global_route_applies_exclusions_and_preserves_bundle_order() {
        let first = entity(FIRST_CLIENT_INDEX);
        let second = entity(SECOND_CLIENT_INDEX);
        let third = entity(THIRD_CLIENT_INDEX);
        let bundle = ordered_bundle();
        let clients = [
            PacketComposeClient::active(first, DVec3::ZERO),
            PacketComposeClient::active(second, DVec3::ZERO),
            PacketComposeClient::inactive(third, DVec3::ZERO),
        ];
        let intents =
            [PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Global).exclude(first)];

        let plan = plan_packet_delivery(&intents, &clients, &[bundle.clone()]);

        assert!(plan.is_success());
        assert_eq!(plan.deliveries().len(), 1);
        assert_eq!(plan.deliveries()[0].recipient, second);
        assert_eq!(bundle_frames(&bundle), vec![FIRST_PACKET, SECOND_PACKET]);
    }

    #[test]
    fn route_resolution_supports_unicast_local_and_group_recipients() {
        let first = entity(FIRST_CLIENT_INDEX);
        let second = entity(SECOND_CLIENT_INDEX);
        let third = entity(THIRD_CLIENT_INDEX);
        let group = PacketComposeGroup::new("blue-team");
        let bundle = ordered_bundle();
        let clients = [
            PacketComposeClient::active(first, DVec3::ZERO).with_group(group.clone()),
            PacketComposeClient::active(second, DVec3::X * EDGE_DISTANCE),
            PacketComposeClient::active(third, DVec3::X * OUTSIDE_DISTANCE)
                .with_group(group.clone()),
        ];
        let intents = [
            PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Unicast(first)),
            PacketRouteIntent::new(
                PacketBundleId::new(0),
                PacketRoute::Local {
                    center: DVec3::ZERO,
                    radius: LOCAL_RADIUS,
                },
            ),
            PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Group(group)),
        ];

        let plan = plan_packet_delivery(&intents, &clients, &[bundle]);
        let deliveries: Vec<(usize, Entity)> = plan
            .deliveries()
            .iter()
            .map(|delivery| (delivery.route_index, delivery.recipient))
            .collect();

        assert!(plan.is_success());
        assert_eq!(
            deliveries,
            vec![(0, first), (1, first), (1, second), (2, first), (2, third)]
        );
    }

    #[test]
    fn invalid_route_targets_are_reported_without_deliveries() {
        let first = entity(FIRST_CLIENT_INDEX);
        let missing = entity(MISSING_CLIENT_INDEX);
        let missing_group = PacketComposeGroup::new("missing-group");
        let bundle = ordered_bundle();
        let clients = [PacketComposeClient::inactive(first, DVec3::ZERO)];
        let intents = [
            PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Unicast(missing)),
            PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Unicast(first)),
            PacketRouteIntent::new(
                PacketBundleId::new(0),
                PacketRoute::Group(missing_group.clone()),
            ),
        ];

        let plan = plan_packet_delivery(&intents, &clients, &[bundle]);

        assert!(plan.deliveries().is_empty());
        assert_eq!(
            plan.failures(),
            &[
                PacketPlanFailure {
                    route_index: 0,
                    kind: PacketPlanFailureKind::MissingUnicastTarget { target: missing },
                },
                PacketPlanFailure {
                    route_index: 1,
                    kind: PacketPlanFailureKind::InactiveUnicastTarget { target: first },
                },
                PacketPlanFailure {
                    route_index: 2,
                    kind: PacketPlanFailureKind::MissingGroup {
                        group: missing_group,
                    },
                },
            ]
        );
    }

    #[test]
    fn invalid_local_route_inputs_fail_closed() {
        let first = entity(FIRST_CLIENT_INDEX);
        let bundle = ordered_bundle();
        let clients = [PacketComposeClient::active(first, DVec3::ZERO)];
        let intents = [PacketRouteIntent::new(
            PacketBundleId::new(0),
            PacketRoute::Local {
                center: DVec3::ZERO,
                radius: f64::NAN,
            },
        )];

        let plan = plan_packet_delivery(&intents, &clients, &[bundle]);

        assert!(plan.deliveries().is_empty());
        assert_eq!(
            plan.failures(),
            &[PacketPlanFailure {
                route_index: 0,
                kind: PacketPlanFailureKind::InvalidLocalRoute,
            }]
        );
    }

    #[test]
    fn encode_failure_returns_structured_error_and_keeps_bundle_empty() {
        let mut bundle = PacketBundle::new();

        let error = bundle
            .try_push_packet(&FailingPacket, CompressionThreshold::DEFAULT)
            .unwrap_err();

        assert!(bundle.is_empty());
        assert_eq!(error.packet_name(), FAILING_PACKET_NAME);
        assert!(error.message().contains(ENCODE_FAILURE_MESSAGE));
    }

    #[test]
    fn direct_flush_reports_closed_clients_and_backend_failures() {
        let first = entity(FIRST_CLIENT_INDEX);
        let second = entity(SECOND_CLIENT_INDEX);
        let third = entity(THIRD_CLIENT_INDEX);
        let bundle = ordered_bundle();
        let plan = PacketDeliveryPlan {
            deliveries: vec![delivery(first), delivery(second), delivery(third)],
            failures: Vec::new(),
        };
        let mut sinks = BTreeMap::from([
            (first, TestSink::open()),
            (second, TestSink::closed()),
            (third, TestSink::backend_failure()),
        ]);

        let report = flush_packet_delivery_plan_with(&plan, &[bundle], |recipient, bundle| {
            let sink = sinks
                .get_mut(&recipient)
                .ok_or_else(PacketDirectWriteError::closed_client)?;
            sink.write_bundle(bundle)
        });

        assert_eq!(report.delivered(), &[delivery(first)]);
        assert_eq!(
            sinks.get(&first).unwrap().written,
            vec![FIRST_PACKET, SECOND_PACKET]
        );
        assert_eq!(
            report.failures(),
            &[
                PacketFlushFailure {
                    delivery_index: 1,
                    recipient: second,
                    bundle: PacketBundleId::new(0),
                    kind: PacketFlushFailureKind::ClosedClient,
                    message: None,
                },
                PacketFlushFailure {
                    delivery_index: 2,
                    recipient: third,
                    bundle: PacketBundleId::new(0),
                    kind: PacketFlushFailureKind::BackendWrite,
                    message: Some(BACKEND_FAILURE_MESSAGE.to_string()),
                },
            ]
        );
    }

    #[test]
    fn missing_bundle_reports_flush_failure() {
        let first = entity(FIRST_CLIENT_INDEX);
        let plan = PacketDeliveryPlan {
            deliveries: vec![PacketDelivery {
                route_index: 0,
                bundle: PacketBundleId::new(1),
                recipient: first,
            }],
            failures: Vec::new(),
        };

        let report = flush_packet_delivery_plan_with(&plan, &[], |_recipient, _bundle| Ok(()));

        assert!(report.delivered().is_empty());
        assert_eq!(
            report.failures(),
            &[PacketFlushFailure {
                delivery_index: 0,
                recipient: first,
                bundle: PacketBundleId::new(1),
                kind: PacketFlushFailureKind::MissingBundle,
                message: None,
            }]
        );
    }

    fn entity(index: u32) -> Entity {
        Entity::from_raw(index)
    }

    fn ordered_bundle() -> PacketBundle {
        PacketBundle::from_packet_bytes([FIRST_PACKET.to_vec(), SECOND_PACKET.to_vec()])
    }

    fn bundle_frames(bundle: &PacketBundle) -> Vec<&[u8]> {
        bundle.frames().iter().map(PacketFrame::bytes).collect()
    }

    fn delivery(recipient: Entity) -> PacketDelivery {
        PacketDelivery {
            route_index: 0,
            bundle: PacketBundleId::new(0),
            recipient,
        }
    }

    #[derive(Debug)]
    struct FailingPacket;

    impl Packet for FailingPacket {
        const ID: i32 = FAILING_PACKET_ID;
        const NAME: &'static str = FAILING_PACKET_NAME;
        const SIDE: PacketSide = PacketSide::Clientbound;
        const STATE: PacketState = PacketState::Play;
    }

    impl Encode for FailingPacket {
        fn encode(&self, _w: impl Write) -> anyhow::Result<()> {
            bail!(ENCODE_FAILURE_MESSAGE)
        }
    }

    #[derive(Debug)]
    struct TestSink {
        state: TestSinkState,
        written: Vec<&'static [u8]>,
    }

    impl TestSink {
        fn open() -> Self {
            Self {
                state: TestSinkState::Open,
                written: Vec::new(),
            }
        }

        fn closed() -> Self {
            Self {
                state: TestSinkState::Closed,
                written: Vec::new(),
            }
        }

        fn backend_failure() -> Self {
            Self {
                state: TestSinkState::BackendFailure,
                written: Vec::new(),
            }
        }

        fn write_bundle(&mut self, bundle: &PacketBundle) -> Result<(), PacketDirectWriteError> {
            match self.state {
                TestSinkState::Open => {
                    self.written = bundle_frames(bundle)
                        .into_iter()
                        .map(static_packet_bytes)
                        .collect();
                    Ok(())
                }
                TestSinkState::Closed => Err(PacketDirectWriteError::closed_client()),
                TestSinkState::BackendFailure => Err(PacketDirectWriteError::backend_write(
                    BACKEND_FAILURE_MESSAGE,
                )),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    enum TestSinkState {
        Open,
        Closed,
        BackendFailure,
    }

    fn static_packet_bytes(bytes: &[u8]) -> &'static [u8] {
        if bytes == FIRST_PACKET {
            FIRST_PACKET
        } else {
            SECOND_PACKET
        }
    }

    #[test]
    fn duplicate_clients_are_reported_as_input_failures() {
        let first = entity(FIRST_CLIENT_INDEX);
        let clients = [
            PacketComposeClient::active(first, DVec3::ZERO),
            PacketComposeClient::active(first, DVec3::ZERO),
        ];
        let intents = [PacketRouteIntent::new(
            PacketBundleId::new(0),
            PacketRoute::Global,
        )];

        let plan = plan_packet_delivery(&intents, &clients, &[ordered_bundle()]);

        assert_eq!(plan.deliveries(), &[delivery(first)]);
        assert_eq!(
            plan.failures(),
            &[PacketPlanFailure {
                route_index: 0,
                kind: PacketPlanFailureKind::DuplicateClient { client: first },
            }]
        );
    }

    #[test]
    fn exclusions_can_remove_every_group_recipient_without_error() {
        let first = entity(FIRST_CLIENT_INDEX);
        let group = PacketComposeGroup::new("all-excluded");
        let clients = [PacketComposeClient::active(first, DVec3::ZERO).with_group(group.clone())];
        let intents = [
            PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Group(group))
                .exclude(first),
        ];

        let plan = plan_packet_delivery(&intents, &clients, &[ordered_bundle()]);

        assert!(plan.is_success());
        assert!(plan.deliveries().is_empty());
    }

    #[test]
    fn non_finite_client_position_reports_local_route_failure() {
        let first = entity(FIRST_CLIENT_INDEX);
        let clients = [PacketComposeClient::active(
            first,
            DVec3::new(f64::NAN, 0.0, 0.0),
        )];
        let intents = [PacketRouteIntent::new(
            PacketBundleId::new(0),
            PacketRoute::Local {
                center: DVec3::ZERO,
                radius: LOCAL_RADIUS,
            },
        )];

        let plan = plan_packet_delivery(&intents, &clients, &[ordered_bundle()]);

        assert!(plan.deliveries().is_empty());
        assert_eq!(
            plan.failures(),
            &[PacketPlanFailure {
                route_index: 0,
                kind: PacketPlanFailureKind::InvalidClientPosition { client: first },
            }]
        );
    }

    #[test]
    fn helper_imports_keep_btree_set_visible_for_public_examples() {
        let mut set = BTreeSet::new();
        set.insert(PacketComposeGroup::new("example"));

        assert_eq!(set.len(), 1);
    }
}
