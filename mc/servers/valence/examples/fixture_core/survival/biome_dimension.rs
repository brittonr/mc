pub fn normalize_environment_id(
    raw: &str,
    known: &[&'static str],
    unknown: &'static str,
) -> &'static str {
    for known_id in known {
        if raw == *known_id {
            return *known_id;
        }
    }
    unknown
}

pub fn derive_environment_id(
    spawn_environment: &str,
    environment_identifier: &str,
    known: &[&'static str],
    unknown: &'static str,
) -> &'static str {
    let environment = normalize_environment_id(environment_identifier, known, unknown);
    if environment != unknown {
        return environment;
    }
    normalize_environment_id(spawn_environment, known, unknown)
}

#[cfg(test)]
mod tests {
    use super::*;

    const OVERWORLD: &str = "minecraft:overworld";
    const NETHER: &str = "minecraft:the_nether";
    const END: &str = "minecraft:the_end";
    const UNKNOWN: &str = "unknown";

    #[test]
    fn known_environment_ids_normalize_and_unknowns_fail_closed() {
        let known = [OVERWORLD, NETHER, END];
        assert_eq!(normalize_environment_id(NETHER, &known, UNKNOWN), NETHER);
        assert_eq!(
            normalize_environment_id("custom:unknown", &known, UNKNOWN),
            UNKNOWN
        );
        assert_eq!(
            derive_environment_id(NETHER, OVERWORLD, &known, UNKNOWN),
            OVERWORLD
        );
        assert_eq!(
            derive_environment_id(END, "custom:unknown", &known, UNKNOWN),
            END
        );
        assert_eq!(
            derive_environment_id("custom:dimension", "custom:world", &known, UNKNOWN),
            UNKNOWN,
        );
    }
}
