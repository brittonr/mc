use crate::protocol::packet::play::serverbound::PluginMessageServerbound;
use crate::protocol::packet::play::serverbound::PluginMessageServerbound_i16;
use crate::protocol::{Serializable, VarShort};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClientboundPluginChannel {
    Register,
    Unregister,
    ForgeHandshake,
    Unknown,
}

pub(crate) fn classify_clientbound_channel(channel: &str) -> ClientboundPluginChannel {
    match channel {
        "REGISTER" => ClientboundPluginChannel::Register,
        "UNREGISTER" => ClientboundPluginChannel::Unregister,
        "FML|HS" => ClientboundPluginChannel::ForgeHandshake,
        _ => ClientboundPluginChannel::Unknown,
    }
}

pub struct Brand {
    pub brand: String,
}

impl Brand {
    pub fn into_message(self) -> PluginMessageServerbound {
        let protocol_version = crate::protocol::current_protocol_version();

        let channel_name = if protocol_version >= 404 {
            "minecraft:brand"
        } else {
            "MC|Brand"
        };

        let mut data = vec![];
        Serializable::write_to(&self.brand, &mut data).unwrap();
        PluginMessageServerbound {
            channel: channel_name.into(),
            data,
        }
    }

    // TODO: cleanup this duplication for 1.7, return either message dynamically
    pub fn into_message17(self) -> PluginMessageServerbound_i16 {
        let mut data = vec![];
        Serializable::write_to(&self.brand, &mut data).unwrap();
        PluginMessageServerbound_i16 {
            channel: "MC|Brand".into(),
            data: crate::protocol::LenPrefixedBytes::<VarShort>::new(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REGISTER_CHANNEL: &str = "REGISTER";
    const FORGE_CHANNEL: &str = "FML|HS";
    const UNKNOWN_CHANNEL: &str = "mc_compat:unknown";

    #[test]
    fn classifies_supported_plugin_channels() {
        assert_eq!(
            classify_clientbound_channel(REGISTER_CHANNEL),
            ClientboundPluginChannel::Register
        );
        assert_eq!(
            classify_clientbound_channel(FORGE_CHANNEL),
            ClientboundPluginChannel::ForgeHandshake
        );
    }

    #[test]
    fn unknown_plugin_channel_fails_closed() {
        assert_eq!(
            classify_clientbound_channel(UNKNOWN_CHANNEL),
            ClientboundPluginChannel::Unknown
        );
    }
}
