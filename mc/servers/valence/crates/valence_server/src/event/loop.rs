use std::time::Instant;

use bevy_app::prelude::*;
use bevy_app::MainScheduleOrder;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::system::SystemState;
use bytes::Bytes;
use tracing::{debug, warn};
use valence_protocol::{Decode, Packet};

use crate::client::Client;

pub struct EventLoopPlugin;

impl Plugin for EventLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PacketEvent>()
            .add_schedule(Schedule::new(RunEventLoop))
            .add_schedule(Schedule::new(EventLoopPreUpdate))
            .add_schedule(Schedule::new(EventLoopUpdate))
            .add_schedule(Schedule::new(EventLoopPostUpdate))
            .add_systems(RunEventLoop, run_event_loop)
            .configure_sets(
                EventLoopPreUpdate,
                (
                    EventLoopSet::RawPacketObservers,
                    EventLoopSet::TypedAdapters.after(EventLoopSet::RawPacketObservers),
                    EventLoopSet::DomainConsumers.after(EventLoopSet::TypedAdapters),
                    EventLoopSet::Diagnostics.after(EventLoopSet::DomainConsumers),
                ),
            )
            .configure_sets(
                EventLoopUpdate,
                (
                    EventLoopSet::DomainConsumers,
                    EventLoopSet::Diagnostics.after(EventLoopSet::DomainConsumers),
                ),
            )
            .configure_sets(EventLoopPostUpdate, EventLoopSet::Diagnostics);

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_after(PreUpdate, RunEventLoop);
    }
}

/// The schedule responsible for running [`EventLoopPreUpdate`],
/// [`EventLoopUpdate`], and [`EventLoopPostUpdate`].
///
/// This schedule is situated between [`PreUpdate`] and [`Update`].
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunEventLoop;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventLoopPreUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventLoopUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventLoopPostUpdate;

/// Named event-loop phase sets for systems that need explicit ordering around
/// raw packet observation, typed adapter emission, domain event consumption,
/// and diagnostics.
///
/// The stable public contract is the relative ordering of these sets inside
/// each event-loop schedule where the set is configured. Ordering between
/// individual systems within a set, and ordering for systems outside these sets,
/// remains private to each plugin. Raw [`PacketEvent`] values remain readable by
/// systems that opt into event-loop schedules; typed adapters only own the
/// semantic events they emit.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventLoopSet {
    /// Systems that observe raw [`PacketEvent`] values before selected typed
    /// adapters run.
    RawPacketObservers,
    /// Systems that decode raw packets and emit typed event-loop events.
    TypedAdapters,
    /// Systems that consume typed event-loop events and perform domain work.
    DomainConsumers,
    /// Systems that emit diagnostics, metrics, or advisory observations.
    Diagnostics,
}

/// Ordered phase sets configured in [`EventLoopPreUpdate`].
pub const EVENT_LOOP_PRE_UPDATE_PHASES: &[EventLoopSet] = &[
    EventLoopSet::RawPacketObservers,
    EventLoopSet::TypedAdapters,
    EventLoopSet::DomainConsumers,
    EventLoopSet::Diagnostics,
];

/// Ordered phase sets configured in [`EventLoopUpdate`].
pub const EVENT_LOOP_UPDATE_PHASES: &[EventLoopSet] =
    &[EventLoopSet::DomainConsumers, EventLoopSet::Diagnostics];

/// Ordered phase sets configured in [`EventLoopPostUpdate`].
pub const EVENT_LOOP_POST_UPDATE_PHASES: &[EventLoopSet] = &[EventLoopSet::Diagnostics];

#[derive(Event, Clone, Debug)]
pub struct PacketEvent {
    /// The client this packet originated from.
    pub client: Entity,
    /// The moment in time this packet arrived.
    pub timestamp: Instant,
    /// This packet's ID.
    pub id: i32,
    /// The content of the packet, excluding the leading varint packet ID.
    pub data: Bytes,
}

impl PacketEvent {
    /// Attempts to decode this packet as the packet `P`.
    ///
    /// If the packet ID is mismatched or an error occurs, `None` is returned.
    /// Otherwise, `Some` is returned containing the decoded packet.
    #[inline]
    pub fn decode<'a, P>(&'a self) -> Option<P>
    where
        P: Packet + Decode<'a>,
    {
        if self.id == P::ID {
            let mut r = &self.data[..];

            match P::decode(&mut r) {
                Ok(pkt) => {
                    if r.is_empty() {
                        return Some(pkt);
                    }

                    warn!(
                        "missed {} bytes while decoding packet {} (ID = {})",
                        r.len(),
                        P::NAME,
                        P::ID
                    );
                    debug!("complete packet after partial decode: {pkt:?}");
                }
                Err(e) => {
                    warn!("failed to decode packet with ID of {}: {e:#}", P::ID);
                }
            }
        }

        None
    }
}

fn run_event_loop_schedules(world: &mut World) {
    world.run_schedule(EventLoopPreUpdate);
    world.run_schedule(EventLoopUpdate);
    world.run_schedule(EventLoopPostUpdate);
}

/// An exclusive system for running the event loop schedule.
#[allow(clippy::type_complexity)]
fn run_event_loop(
    world: &mut World,
    state: &mut SystemState<(
        Query<(Entity, &mut Client)>,
        EventWriter<PacketEvent>,
        Commands,
    )>,
    mut check_again: Local<Vec<(Entity, usize)>>,
) {
    debug_assert!(check_again.is_empty());

    let (mut clients, mut event_writer, mut commands) = state.get_mut(world);

    for (entity, mut client) in &mut clients {
        match client.connection_mut().try_recv() {
            Ok(Some(pkt)) => {
                event_writer.send(PacketEvent {
                    client: entity,
                    timestamp: pkt.timestamp,
                    id: pkt.id,
                    data: pkt.body,
                });

                let remaining = client.connection().len();

                if remaining > 0 {
                    check_again.push((entity, remaining));
                }
            }
            Ok(None) => {}
            Err(e) => {
                // Client is disconnected.
                debug!("disconnecting client: {e:#}");
                commands.entity(entity).remove::<Client>();
            }
        }
    }

    state.apply(world);
    run_event_loop_schedules(world);

    while !check_again.is_empty() {
        let (mut clients, mut event_writer, mut commands) = state.get_mut(world);

        check_again.retain_mut(|(entity, remaining)| {
            debug_assert!(*remaining > 0);

            if let Ok((_, mut client)) = clients.get_mut(*entity) {
                match client.connection_mut().try_recv() {
                    Ok(Some(pkt)) => {
                        event_writer.send(PacketEvent {
                            client: *entity,
                            timestamp: pkt.timestamp,
                            id: pkt.id,
                            data: pkt.body,
                        });
                        *remaining -= 1;
                        // Keep looping as long as there are packets to process this tick.
                        *remaining > 0
                    }
                    Ok(None) => false,
                    Err(e) => {
                        // Client is disconnected.
                        debug!("disconnecting client: {e:#}");
                        commands.entity(*entity).remove::<Client>();
                        false
                    }
                }
            } else {
                // Client must have been deleted in the last run of the schedule.
                false
            }
        });

        state.apply(world);
        run_event_loop_schedules(world);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORDERING_PAIR_WIDTH: usize = 2;

    const EXPECTED_PRE_UPDATE_PHASES: &[EventLoopSet] = &[
        EventLoopSet::RawPacketObservers,
        EventLoopSet::TypedAdapters,
        EventLoopSet::DomainConsumers,
        EventLoopSet::Diagnostics,
    ];
    const EXPECTED_UPDATE_PHASES: &[EventLoopSet] =
        &[EventLoopSet::DomainConsumers, EventLoopSet::Diagnostics];
    const EXPECTED_POST_UPDATE_PHASES: &[EventLoopSet] = &[EventLoopSet::Diagnostics];
    const MISSING_TYPED_ADAPTER_PHASES: &[EventLoopSet] = &[
        EventLoopSet::RawPacketObservers,
        EventLoopSet::DomainConsumers,
        EventLoopSet::Diagnostics,
    ];
    const AMBIGUOUS_PRE_UPDATE_PHASES: &[EventLoopSet] = &[
        EventLoopSet::RawPacketObservers,
        EventLoopSet::DomainConsumers,
        EventLoopSet::TypedAdapters,
        EventLoopSet::Diagnostics,
    ];
    const DUPLICATE_TYPED_ADAPTER_PHASES: &[EventLoopSet] = &[
        EventLoopSet::RawPacketObservers,
        EventLoopSet::TypedAdapters,
        EventLoopSet::TypedAdapters,
        EventLoopSet::DomainConsumers,
        EventLoopSet::Diagnostics,
    ];

    #[derive(Debug, PartialEq, Eq)]
    enum PhasePlanError {
        Missing(EventLoopSet),
        Duplicate(EventLoopSet),
        AmbiguousOrder {
            earlier: EventLoopSet,
            later: EventLoopSet,
        },
    }

    #[test]
    fn event_loop_phase_orders_are_valid() {
        assert_eq!(
            validate_phase_plan(EVENT_LOOP_PRE_UPDATE_PHASES, EXPECTED_PRE_UPDATE_PHASES),
            Ok(())
        );
        assert_eq!(
            validate_phase_plan(EVENT_LOOP_UPDATE_PHASES, EXPECTED_UPDATE_PHASES),
            Ok(())
        );
        assert_eq!(
            validate_phase_plan(EVENT_LOOP_POST_UPDATE_PHASES, EXPECTED_POST_UPDATE_PHASES),
            Ok(())
        );
    }

    #[test]
    fn missing_phase_fails_clearly() {
        assert_eq!(
            validate_phase_plan(MISSING_TYPED_ADAPTER_PHASES, EXPECTED_PRE_UPDATE_PHASES),
            Err(PhasePlanError::Missing(EventLoopSet::TypedAdapters))
        );
    }

    #[test]
    fn ambiguous_phase_order_fails_clearly() {
        assert_eq!(
            validate_phase_plan(AMBIGUOUS_PRE_UPDATE_PHASES, EXPECTED_PRE_UPDATE_PHASES),
            Err(PhasePlanError::AmbiguousOrder {
                earlier: EventLoopSet::TypedAdapters,
                later: EventLoopSet::DomainConsumers,
            })
        );
    }

    #[test]
    fn duplicate_phase_fails_clearly() {
        assert_eq!(
            validate_phase_plan(DUPLICATE_TYPED_ADAPTER_PHASES, EXPECTED_PRE_UPDATE_PHASES),
            Err(PhasePlanError::Duplicate(EventLoopSet::TypedAdapters))
        );
    }

    fn validate_phase_plan(
        actual: &[EventLoopSet],
        expected: &[EventLoopSet],
    ) -> Result<(), PhasePlanError> {
        let mut seen = Vec::new();
        for phase in actual {
            if seen.contains(phase) {
                return Err(PhasePlanError::Duplicate(*phase));
            }
            seen.push(*phase);
        }

        for phase in expected {
            if !actual.contains(phase) {
                return Err(PhasePlanError::Missing(*phase));
            }
        }

        for pair in expected.windows(ORDERING_PAIR_WIDTH) {
            let earlier = pair[0];
            let later = pair[1];
            if index_of(actual, earlier) > index_of(actual, later) {
                return Err(PhasePlanError::AmbiguousOrder { earlier, later });
            }
        }

        Ok(())
    }

    fn index_of(phases: &[EventLoopSet], phase: EventLoopSet) -> usize {
        phases
            .iter()
            .position(|candidate| *candidate == phase)
            .expect("validated phase is present")
    }
}
