use crate::{format, protocol, screen, Game};

const IDLE_CAMERA_YAW_SPEED_RADIANS_PER_DELTA: f64 = 0.005;

impl Game {
    pub fn tick(&mut self, delta: f64) {
        if !self.server.is_connected() {
            self.renderer.camera.yaw += IDLE_CAMERA_YAW_SPEED_RADIANS_PER_DELTA * delta;
            if self.renderer.camera.yaw > std::f64::consts::TAU {
                self.renderer.camera.yaw = 0.0;
            }
        }

        if let Some(disconnect_reason) = self.server.disconnect_reason.take() {
            self.screen_sys
                .replace_screen(Box::new(screen::ServerList::new(Some(disconnect_reason))));
        }
        if !self.server.is_connected() {
            self.focused = false;
        }

        let mut clear_reply = false;
        if let Some(ref recv) = self.connect_reply {
            if let Ok(server) = recv.try_recv() {
                clear_reply = true;
                match server {
                    Ok(val) => {
                        self.screen_sys.pop_screen();
                        self.focused = true;
                        self.server.remove(&mut self.renderer);
                        self.server = val;
                    }
                    Err(err) => {
                        let msg = match err {
                            protocol::Error::Disconnect(val) => val,
                            err => {
                                let mut msg = format::TextComponent::new(&format!("{}", err));
                                msg.modifier.color = Some(format::Color::Red);
                                format::Component::Text(msg)
                            }
                        };
                        self.screen_sys
                            .replace_screen(Box::new(screen::ServerList::new(Some(msg))));
                    }
                }
            }
        }
        if clear_reply {
            self.connect_reply = None;
        }
    }
}
