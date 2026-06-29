use super::login;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HandshakeTarget {
    pub(crate) host: String,
    pub(crate) port: u16,
}

pub(crate) fn handshake_target(
    conn_host: &str,
    conn_port: u16,
    fml_network_version: Option<i64>,
) -> HandshakeTarget {
    HandshakeTarget {
        host: format!(
            "{}{}",
            conn_host,
            login::fml_handshake_tag(fml_network_version)
        ),
        port: conn_port,
    }
}

pub(crate) fn env_flag_enabled(value: Option<&str>) -> bool {
    value.map(|value| value != "0").unwrap_or(false)
}

pub(crate) fn env_flag_from_env(name: &str) -> bool {
    env_flag_enabled(std::env::var(name).ok().as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_HOST: &str = "localhost";
    const TEST_PORT: u16 = 25_565;

    #[test]
    fn handshake_target_preserves_plain_host_without_forge() {
        let target = handshake_target(TEST_HOST, TEST_PORT, None);

        assert_eq!(target.host, TEST_HOST);
        assert_eq!(target.port, TEST_PORT);
    }

    #[test]
    fn handshake_target_appends_forge_marker_for_supported_forge_versions() {
        let legacy = handshake_target(
            TEST_HOST,
            TEST_PORT,
            Some(login::FML_NETWORK_VERSION_LEGACY),
        );
        let fml2 = handshake_target(TEST_HOST, TEST_PORT, Some(login::FML_NETWORK_VERSION_FML2));

        assert!(legacy.host.ends_with(login::LEGACY_FML_HANDSHAKE_TAG));
        assert!(fml2.host.ends_with(login::FML2_HANDSHAKE_TAG));
        assert_eq!(legacy.port, TEST_PORT);
        assert_eq!(fml2.port, TEST_PORT);
    }

    #[test]
    fn env_flag_enabled_rejects_absent_or_zero_values() {
        assert!(!env_flag_enabled(None));
        assert!(!env_flag_enabled(Some("0")));
        assert!(env_flag_enabled(Some("1")));
    }
}
