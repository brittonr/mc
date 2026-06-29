use std::sync::mpsc;
use std::thread;

use log::{info, warn};

use crate::protocol::mojang;
use crate::{protocol, server, Game};

impl Game {
    pub fn connect_to(&mut self, address: &str) {
        let (protocol_version, forge_mods, fml_network_version) =
            match protocol::Conn::new(address, self.default_protocol_version)
                .and_then(|conn| conn.do_status())
            {
                Ok(res) => {
                    info!(
                        "Detected server protocol version {}",
                        res.0.version.protocol
                    );
                    (
                        res.0.version.protocol,
                        res.0.forge_mods,
                        res.0.fml_network_version,
                    )
                }
                Err(err) => {
                    warn!(
                        "Error pinging server {} to get protocol version: {:?}, defaulting to {}",
                        address, err, self.default_protocol_version
                    );
                    (self.default_protocol_version, vec![], None)
                }
            };

        let (tx, rx) = mpsc::channel();
        self.connect_reply = Some(rx);
        let address = address.to_owned();
        let resources = self.resource_manager.clone();
        let profile = mojang::Profile {
            username: self.vars.get(crate::auth::CL_USERNAME).clone(),
            id: self.vars.get(crate::auth::CL_UUID).clone(),
            access_token: self.vars.get(crate::auth::AUTH_TOKEN).clone(),
        };
        thread::spawn(move || {
            tx.send(server::Server::connect(
                resources,
                profile,
                &address,
                protocol_version,
                forge_mods,
                fml_network_version,
            ))
            .unwrap();
        });
    }
}
