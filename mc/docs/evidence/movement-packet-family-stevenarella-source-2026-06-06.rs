// Review snapshot for movement-packet-family, copied from
// stevenarella/src/server/mod.rs at child revision
// d9caec597041b3443d894701591752d23772e5ae.
//
// The excerpt shows that the 1.20.1 active/team probe sends the full
// serverbound PlayerPositionLook packet variant for the configured portal
// movement row. This snapshot is evidence only; it is not compiled by the
// parent mc repository.

fn protocol_47_or_newer_syncs_player_position_look() {
    if self.protocol_version >= 47 {
        let packet = packet::play::serverbound::PlayerPositionLook {
            x: position.position.x,
            y: position.position.y,
            z: position.position.z,
            yaw: -(rotation.yaw as f32) * (180.0 / PI),
            pitch: (-rotation.pitch as f32) * (180.0 / PI) + 180.0,
            on_ground,
        };
        self.write_packet(packet);
        if self.active_probe_enabled && !self.active_probe_logged_position_look_sent {
            info!(
                "MC-COMPAT-MILESTONE active_probe_position_look_sent x={:.3} y={:.3} z={:.3} on_ground={}",
                position.position.x, position.position.y, position.position.z, on_ground
            );
            self.active_probe_logged_position_look_sent = true;
        }
    }
}

fn team_probe_enters_red_portal_with_player_position_look() {
    let (team_name, portal_x, portal_z) = if team == "blue" {
        ("blue", 4.0, 4.0)
    } else {
        ("red", -4.0, 4.0)
    };
    match self.active_probe_ticks {
        360 => {
            info!(
                "MC-COMPAT-MILESTONE team_probe_enter_{}_portal x={:.1} y=84.0 z={:.1}",
                team_name, portal_x, portal_z
            );
            if let Some(position) = self.entities.get_component_mut(player, self.position) {
                position.position = cgmath::Vector3::new(portal_x, 84.0, portal_z);
                position.moved = true;
            }
            self.write_packet(packet::play::serverbound::PlayerPositionLook {
                x: portal_x,
                y: 84.0,
                z: portal_z,
                yaw: 0.0,
                pitch: 0.0,
                on_ground: true,
            });
        }
        _ => {}
    }
}
