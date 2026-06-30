//! Runtime configuration for optional observability hooks.

use bevy_ecs::prelude::*;

/// Runtime switch for optional observability hooks.
#[derive(Resource, Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityConfig {
    /// Enables all systems in [`super::ObservabilityPlugin`].
    pub enabled: bool,
    /// Emits bounded tick-phase span records.
    pub emit_tick_phases: bool,
    /// Emits redacted serverbound packet counter records.
    pub emit_network_packets: bool,
}

impl ObservabilityConfig {
    /// Returns a config with all observability hooks disabled.
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            emit_tick_phases: false,
            emit_network_packets: false,
        }
    }

    /// Returns whether tick-phase records should be emitted.
    pub const fn emits_tick_phase_records(&self) -> bool {
        self.enabled && self.emit_tick_phases
    }

    /// Returns whether serverbound packet records should be emitted.
    pub const fn emits_network_packet_records(&self) -> bool {
        self.enabled && self.emit_network_packets
    }
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            emit_tick_phases: true,
            emit_network_packets: true,
        }
    }
}
