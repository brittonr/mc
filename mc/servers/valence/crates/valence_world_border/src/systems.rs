use bevy_ecs::prelude::*;
use valence_server::protocol::WritePacket;

type Center = super::WorldBorderCenter;
type CenterChangedPacket = valence_server::protocol::packets::play::WorldBorderCenterChangedS2c;
type ChunkLayer = valence_server::ChunkLayer;
type Client = valence_server::client::Client;
type InitializePacket = valence_server::protocol::packets::play::WorldBorderInitializeS2c;
type InterpolateSizePacket = valence_server::protocol::packets::play::WorldBorderInterpolateSizeS2c;
type Lerp = super::WorldBorderLerp;
type PortalBoundary = super::WorldBorderPortalTpBoundary;
type Server = valence_server::Server;
type SizeChangedPacket = valence_server::protocol::packets::play::WorldBorderSizeChangedS2c;
type VisibleChunkLayer = valence_server::client::VisibleChunkLayer;
type WarnBlocks = super::WorldBorderWarnBlocks;
type WarningBlocksChangedPacket =
    valence_server::protocol::packets::play::WorldBorderWarningBlocksChangedS2c;
type WarningTimeChangedPacket =
    valence_server::protocol::packets::play::WorldBorderWarningTimeChangedS2c;
type WarnTime = super::WorldBorderWarnTime;

const MILLIS_PER_SECOND: i64 = 1_000;

struct InitialState<'a> {
    center: &'a Center,
    lerp: &'a Lerp,
    portal_boundary: &'a PortalBoundary,
    warn_time: &'a WarnTime,
    warn_blocks: &'a WarnBlocks,
    duration_millis: i64,
}

pub(super) fn init_world_border_for_new_clients(
    mut clients: Query<(&mut Client, &VisibleChunkLayer), Changed<VisibleChunkLayer>>,
    wbs: Query<(&Center, &Lerp, &PortalBoundary, &WarnTime, &WarnBlocks)>,
    server: Res<Server>,
) {
    for (mut client, layer) in &mut clients {
        if let Ok((center, lerp, portal_boundary, warn_time, warn_blocks)) = wbs.get(layer.0) {
            write_initialize_packet(
                &mut *client,
                InitialState {
                    center,
                    lerp,
                    portal_boundary,
                    warn_time,
                    warn_blocks,
                    duration_millis: lerp_duration_millis(lerp, &server),
                },
            );
        }
    }
}

pub(super) fn tick_world_border_lerp(
    mut wbs: Query<(&mut ChunkLayer, &mut Lerp)>,
    server: Res<Server>,
) {
    for (mut layer, mut lerp) in &mut wbs {
        if lerp.is_changed() {
            if lerp.remaining_ticks == 0 {
                layer.write_packet(&SizeChangedPacket {
                    diameter: lerp.target_diameter,
                });

                lerp.current_diameter = lerp.target_diameter;
            } else {
                layer.write_packet(&InterpolateSizePacket {
                    old_diameter: lerp.current_diameter,
                    new_diameter: lerp.target_diameter,
                    duration_millis: lerp_duration_millis(&lerp, &server).into(),
                });
            }
        }

        if lerp.remaining_ticks > 0 {
            let diff = lerp.target_diameter - lerp.current_diameter;
            lerp.current_diameter += diff / lerp.remaining_ticks as f64;

            lerp.remaining_ticks -= 1;
        }
    }
}

pub(super) fn change_world_border_center(
    mut wbs: Query<(&mut ChunkLayer, &Center), Changed<Center>>,
) {
    for (mut layer, center) in &mut wbs {
        layer.write_packet(&CenterChangedPacket {
            x_pos: center.x,
            z_pos: center.z,
        });
    }
}

pub(super) fn change_world_border_warning_blocks(
    mut wbs: Query<(&mut ChunkLayer, &WarnBlocks), Changed<WarnBlocks>>,
) {
    for (mut layer, warn_blocks) in &mut wbs {
        layer.write_packet(&WarningBlocksChangedPacket {
            warning_blocks: warn_blocks.0.into(),
        });
    }
}

pub(super) fn change_world_border_warning_time(
    mut wbs: Query<(&mut ChunkLayer, &WarnTime), Changed<WarnTime>>,
) {
    for (mut layer, warn_time) in &mut wbs {
        layer.write_packet(&WarningTimeChangedPacket {
            warning_time: warn_time.0.into(),
        });
    }
}

pub(super) fn change_world_border_portal_tp_boundary(
    mut wbs: Query<
        (
            &mut ChunkLayer,
            &Center,
            &Lerp,
            &PortalBoundary,
            &WarnTime,
            &WarnBlocks,
        ),
        Changed<PortalBoundary>,
    >,
    server: Res<Server>,
) {
    for (mut layer, center, lerp, portal_boundary, warn_time, warn_blocks) in &mut wbs {
        write_initialize_packet(
            &mut *layer,
            InitialState {
                center,
                lerp,
                portal_boundary,
                warn_time,
                warn_blocks,
                duration_millis: lerp_duration_millis(lerp, &server),
            },
        );
    }
}

fn write_initialize_packet(writer: &mut impl WritePacket, state: InitialState<'_>) {
    writer.write_packet(&InitializePacket {
        x: state.center.x,
        z: state.center.z,
        old_diameter: state.lerp.current_diameter,
        new_diameter: state.lerp.target_diameter,
        duration_millis: state.duration_millis.into(),
        portal_teleport_boundary: state.portal_boundary.0.into(),
        warning_blocks: state.warn_blocks.0.into(),
        warning_time: state.warn_time.0.into(),
    });
}

fn lerp_duration_millis(lerp: &Lerp, server: &Server) -> i64 {
    remaining_ticks_to_millis(lerp.remaining_ticks, server.tick_rate())
}

pub(super) fn remaining_ticks_to_millis(
    remaining_ticks: u64,
    tick_rate: std::num::NonZeroU32,
) -> i64 {
    let Ok(remaining_ticks) = i64::try_from(remaining_ticks) else {
        return i64::MAX;
    };
    let Some(tick_millis) = remaining_ticks.checked_mul(MILLIS_PER_SECOND) else {
        return i64::MAX;
    };
    match tick_millis.checked_div(i64::from(tick_rate.get())) {
        Some(duration_millis) => duration_millis,
        None => i64::MAX,
    }
}
