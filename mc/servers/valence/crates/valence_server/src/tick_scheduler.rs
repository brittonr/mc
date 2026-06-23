use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::Server;

const FIRST_INSERTION_ORDER: u64 = 0;
const MAX_INSERTION_ORDER: u64 = u64::MAX;

/// A deterministic queue for work keyed by explicit ticks or other ordered keys.
///
/// The scheduler is a pure data structure. It does not read wall-clock time,
/// Bevy resources, or global server state. Callers supply the drain key
/// explicitly through [`TickScheduler::drain_due`].
///
/// Entries with equal keys are drained in stable insertion order.
#[derive(Clone, Debug, Resource)]
pub struct TickScheduler<K, V> {
    queue: BinaryHeap<Reverse<ScheduledEntry<K, V>>>,
    next_order: u64,
}

/// A scheduler keyed by Valence server ticks.
pub type ServerTickScheduler<V> = TickScheduler<i64, V>;

/// A handle returned when work is scheduled.
///
/// Handles are stable for the lifetime of one scheduler and can be passed to
/// [`TickScheduler::cancel`] before the work is drained.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScheduledWork {
    order: u64,
}

/// Errors reported by scheduler operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TickSchedulerError {
    /// The insertion-order counter is exhausted.
    SequenceExhausted,
}

/// Optional shell that drains [`ServerTickScheduler`] values into Bevy events.
///
/// Add this plugin explicitly for each event type that should be scheduled.
/// The plugin is not part of Valence default plugins and therefore does not
/// change gameplay behavior unless user code opts in.
#[derive(Clone, Copy, Debug)]
pub struct TickSchedulerPlugin<E> {
    marker: PhantomData<fn() -> E>,
}

#[derive(Clone, Debug)]
struct ScheduledEntry<K, V> {
    key: K,
    order: u64,
    value: V,
}

impl<K: Ord, V> Default for TickScheduler<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> TickScheduler<K, V> {
    /// Creates an empty scheduler.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            next_order: FIRST_INSERTION_ORDER,
        }
    }

    /// Schedules `value` at `key`.
    ///
    /// Returns a handle that can be used to cancel the pending work. If the
    /// insertion-order counter is exhausted, the scheduler is left unchanged.
    pub fn schedule(&mut self, key: K, value: V) -> Result<ScheduledWork, TickSchedulerError> {
        let order = self.next_order;
        if order == MAX_INSERTION_ORDER {
            return Err(TickSchedulerError::SequenceExhausted);
        }
        let next_order = order + 1;

        let handle = ScheduledWork { order };
        self.queue
            .push(Reverse(ScheduledEntry { key, order, value }));
        self.next_order = next_order;

        Ok(handle)
    }

    /// Returns the earliest scheduled key and value without draining it.
    #[must_use]
    pub fn peek(&self) -> Option<(&K, &V)> {
        self.queue
            .peek()
            .map(|Reverse(entry)| (&entry.key, &entry.value))
    }

    /// Drains all work with keys less than or equal to `limit`.
    ///
    /// Not-due work remains queued, and equal-key work is returned in stable
    /// insertion order.
    pub fn drain_due(&mut self, limit: &K) -> Vec<V> {
        let mut due = Vec::new();

        while self.peek().is_some_and(|(key, _)| key <= limit) {
            let Some(Reverse(entry)) = self.queue.pop() else {
                unreachable!("peek observed a due scheduler entry");
            };
            due.push(entry.value);
        }

        due
    }

    /// Cancels pending work by handle.
    ///
    /// Returns `true` when a queued entry was removed. Returns `false` for
    /// unknown, already-cancelled, or already-drained handles.
    pub fn cancel(&mut self, handle: ScheduledWork) -> bool {
        let mut retained = BinaryHeap::new();
        let mut removed = false;

        while let Some(Reverse(entry)) = self.queue.pop() {
            if entry.order == handle.order {
                debug_assert!(!removed, "scheduled handles must be unique");
                removed = true;
            } else {
                retained.push(Reverse(entry));
            }
        }

        self.queue = retained;
        removed
    }

    /// Removes all queued work.
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Returns `true` when the scheduler has no pending work.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the count of pending work items.
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    #[cfg(test)]
    const fn with_next_order(next_order: u64) -> Self {
        Self {
            queue: BinaryHeap::new(),
            next_order,
        }
    }
}

impl ScheduledWork {
    /// Returns the scheduler-local insertion order for diagnostics.
    #[must_use]
    pub const fn insertion_order(self) -> u64 {
        self.order
    }
}

impl std::fmt::Display for TickSchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SequenceExhausted => f.write_str("tick scheduler sequence exhausted"),
        }
    }
}

impl std::error::Error for TickSchedulerError {}

impl<E> Default for TickSchedulerPlugin<E> {
    fn default() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<E: Event> Plugin for TickSchedulerPlugin<E> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ServerTickScheduler<E>>()
            .add_event::<E>()
            .add_systems(Update, drain_due_tick_events::<E>);
    }
}

fn drain_due_tick_events<E: Event>(
    server: Res<Server>,
    mut scheduler: ResMut<ServerTickScheduler<E>>,
    mut events: EventWriter<E>,
) {
    let current_tick = server.current_tick();

    for event in scheduler.drain_due(&current_tick) {
        events.send(event);
    }
}

impl<K: Ord, V> Eq for ScheduledEntry<K, V> {}

impl<K: Ord, V> Ord for ScheduledEntry<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key
            .cmp(&other.key)
            .then_with(|| self.order.cmp(&other.order))
    }
}

impl<K: Ord, V> PartialEq for ScheduledEntry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.order == other.order
    }
}

impl<K: Ord, V> PartialOrd for ScheduledEntry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EARLY_TICK: i64 = 5;
    const DUE_TICK: i64 = 10;
    const LATE_TICK: i64 = 15;
    const FIRST_VALUE: &str = "first";
    const SECOND_VALUE: &str = "second";
    const THIRD_VALUE: &str = "third";
    const LAST_VALID_INSERTION_ORDER: u64 = MAX_INSERTION_ORDER - 1;
    const PLUGIN_DUE_TICK: i64 = 0;

    #[derive(Clone, Debug, Event, Eq, PartialEq)]
    struct SchedulerSmokeEvent(&'static str);

    #[test]
    fn empty_queue_drains_cleanly() {
        let mut scheduler = TickScheduler::<i64, &str>::new();

        let due = scheduler.drain_due(&DUE_TICK);

        assert!(due.is_empty());
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.len(), 0);
        assert_eq!(scheduler.peek(), None);
    }

    #[test]
    fn due_drain_leaves_not_due_work_queued() {
        let mut scheduler = TickScheduler::new();
        scheduler.schedule(LATE_TICK, THIRD_VALUE).unwrap();
        scheduler.schedule(EARLY_TICK, FIRST_VALUE).unwrap();
        scheduler.schedule(DUE_TICK, SECOND_VALUE).unwrap();

        let due = scheduler.drain_due(&DUE_TICK);

        assert_eq!(due, vec![FIRST_VALUE, SECOND_VALUE]);
        assert_eq!(scheduler.len(), 1);
        assert_eq!(scheduler.peek(), Some((&LATE_TICK, &THIRD_VALUE)));
    }

    #[test]
    fn equal_tick_work_drains_in_insertion_order() {
        let mut scheduler = TickScheduler::new();
        scheduler.schedule(DUE_TICK, FIRST_VALUE).unwrap();
        scheduler.schedule(DUE_TICK, SECOND_VALUE).unwrap();
        scheduler.schedule(DUE_TICK, THIRD_VALUE).unwrap();

        let due = scheduler.drain_due(&DUE_TICK);

        assert_eq!(due, vec![FIRST_VALUE, SECOND_VALUE, THIRD_VALUE]);
        assert!(scheduler.is_empty());
    }

    #[test]
    fn clear_removes_pending_work() {
        let mut scheduler = TickScheduler::new();
        scheduler.schedule(EARLY_TICK, FIRST_VALUE).unwrap();
        scheduler.schedule(LATE_TICK, SECOND_VALUE).unwrap();

        scheduler.clear();

        assert!(scheduler.is_empty());
        assert_eq!(scheduler.drain_due(&LATE_TICK), Vec::<&str>::new());
    }

    #[test]
    fn cancel_removes_one_pending_item() {
        let mut scheduler = TickScheduler::new();
        scheduler.schedule(DUE_TICK, FIRST_VALUE).unwrap();
        let cancelled = scheduler.schedule(DUE_TICK, SECOND_VALUE).unwrap();
        scheduler.schedule(DUE_TICK, THIRD_VALUE).unwrap();

        assert!(scheduler.cancel(cancelled));

        let due = scheduler.drain_due(&DUE_TICK);
        assert_eq!(due, vec![FIRST_VALUE, THIRD_VALUE]);
        assert!(scheduler.is_empty());
    }

    #[test]
    fn cancel_unknown_or_drained_handle_is_noop() {
        let mut scheduler = TickScheduler::new();
        let drained = scheduler.schedule(DUE_TICK, FIRST_VALUE).unwrap();
        let unknown = ScheduledWork {
            order: drained.insertion_order() + 1,
        };

        assert!(!scheduler.cancel(unknown));
        assert_eq!(scheduler.drain_due(&DUE_TICK), vec![FIRST_VALUE]);
        assert!(!scheduler.cancel(drained));
        assert!(scheduler.is_empty());
    }

    #[test]
    fn sequence_overflow_fails_without_mutating_queue() {
        let mut scheduler = TickScheduler::with_next_order(LAST_VALID_INSERTION_ORDER);
        scheduler.schedule(DUE_TICK, FIRST_VALUE).unwrap();

        let error = scheduler.schedule(DUE_TICK, SECOND_VALUE).unwrap_err();

        assert_eq!(error, TickSchedulerError::SequenceExhausted);
        assert_eq!(scheduler.len(), 1);
        assert_eq!(scheduler.drain_due(&DUE_TICK), vec![FIRST_VALUE]);
    }

    #[test]
    fn server_plugin_does_not_install_tick_scheduler_by_default() {
        let mut app = App::new();
        app.add_plugins(crate::ServerPlugin);

        app.update();

        assert!(app
            .world()
            .get_resource::<ServerTickScheduler<SchedulerSmokeEvent>>()
            .is_none());
        assert!(app
            .world()
            .get_resource::<Events<SchedulerSmokeEvent>>()
            .is_none());
    }

    #[test]
    fn tick_scheduler_plugin_emits_due_events() {
        let mut app = App::new();
        app.add_plugins(crate::ServerPlugin);
        app.add_plugins(TickSchedulerPlugin::<SchedulerSmokeEvent>::default());

        let mut scheduler = app
            .world_mut()
            .get_resource_mut::<ServerTickScheduler<SchedulerSmokeEvent>>()
            .expect("tick scheduler plugin should install its scheduler resource");
        scheduler
            .schedule(PLUGIN_DUE_TICK, SchedulerSmokeEvent(FIRST_VALUE))
            .unwrap();

        app.update();

        let events = app
            .world()
            .get_resource::<Events<SchedulerSmokeEvent>>()
            .expect("tick scheduler plugin should install its event resource");
        let events = events.iter_current_update_events().collect::<Vec<_>>();

        assert_eq!(events, vec![&SchedulerSmokeEvent(FIRST_VALUE)]);
    }
}
