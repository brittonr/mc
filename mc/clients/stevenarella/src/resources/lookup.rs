#[cfg(test)]
use std::collections::HashSet;
use std::io;

use super::identifiers::{compatible_resource_name, ResourceIdentifier};
use super::state::Pack;

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PackResourceMatch {
    pub(crate) pack_index: usize,
    pub(crate) path: String,
}

pub(crate) fn resource_pack_path_candidates(plugin: &str, name: &str) -> Vec<String> {
    let Some(primary) = ResourceIdentifier::new(plugin, name) else {
        return Vec::new();
    };
    let mut candidates = vec![primary.pack_path()];
    if let Some(alias) = compatible_resource_name(plugin, name) {
        if let Some(alias_identifier) = ResourceIdentifier::new(plugin, &alias) {
            candidates.push(alias_identifier.pack_path());
        }
    }
    candidates
}

pub(crate) fn open_first_from_packs(
    packs: &[Box<dyn Pack>],
    candidates: &[String],
) -> Option<Box<dyn io::Read>> {
    for candidate in candidates {
        for pack in packs.iter().rev() {
            if let Some(value) = pack.open(candidate) {
                return Some(value);
            }
        }
    }
    None
}

pub(crate) fn open_all_from_packs(
    packs: &[Box<dyn Pack>],
    candidates: &[String],
) -> Vec<Box<dyn io::Read>> {
    for candidate in candidates {
        let mut opened = Vec::new();
        for pack in packs.iter().rev() {
            if let Some(value) = pack.open(candidate) {
                opened.push(value);
            }
        }
        if !opened.is_empty() {
            return opened;
        }
    }
    Vec::new()
}

#[cfg(test)]
pub(crate) fn select_first_available_resource(
    candidates: &[String],
    available: &[PackResourceMatch],
) -> Option<PackResourceMatch> {
    for candidate in candidates {
        let mut selected: Option<&PackResourceMatch> = None;
        for resource in available
            .iter()
            .filter(|resource| resource.path == *candidate)
        {
            selected = match selected {
                Some(current) if current.pack_index > resource.pack_index => Some(current),
                _ => Some(resource),
            };
        }
        if let Some(resource) = selected {
            return Some(resource.clone());
        }
    }
    None
}

#[cfg(test)]
pub(crate) fn duplicate_resource_paths<'a>(
    entries: impl IntoIterator<Item = &'a str>,
) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();
    for entry in entries {
        if !seen.insert(entry.to_owned()) && !duplicates.iter().any(|duplicate| duplicate == entry)
        {
            duplicates.push(entry.to_owned());
        }
    }
    duplicates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::identifiers::MINECRAFT_PLUGIN;

    const LEGACY_STONE_TEXTURE: &str = "textures/blocks/stone.png";
    const PRIMARY_STONE_PATH: &str = "assets/minecraft/textures/blocks/stone.png";
    const MODERN_STONE_PATH: &str = "assets/minecraft/textures/block/stone.png";
    const UNSAFE_TEXTURE: &str = "textures/../secret.png";
    const LOWER_PRIORITY_PACK_INDEX: usize = 0;
    const HIGHER_PRIORITY_PACK_INDEX: usize = 2;
    const DUPLICATE_PATH: &str = "assets/minecraft/lang/en_us.json";

    #[test]
    fn lookup_candidates_preserve_primary_then_alias_order() {
        assert_eq!(
            resource_pack_path_candidates(MINECRAFT_PLUGIN, LEGACY_STONE_TEXTURE),
            vec![PRIMARY_STONE_PATH.to_owned(), MODERN_STONE_PATH.to_owned()]
        );
    }

    #[test]
    fn pack_selection_uses_primary_candidate_before_alias_precedence() {
        let candidates = resource_pack_path_candidates(MINECRAFT_PLUGIN, LEGACY_STONE_TEXTURE);
        let available = vec![
            PackResourceMatch {
                pack_index: LOWER_PRIORITY_PACK_INDEX,
                path: PRIMARY_STONE_PATH.to_owned(),
            },
            PackResourceMatch {
                pack_index: HIGHER_PRIORITY_PACK_INDEX,
                path: MODERN_STONE_PATH.to_owned(),
            },
        ];

        assert_eq!(
            select_first_available_resource(&candidates, &available),
            Some(PackResourceMatch {
                pack_index: LOWER_PRIORITY_PACK_INDEX,
                path: PRIMARY_STONE_PATH.to_owned(),
            })
        );
    }

    #[test]
    fn pack_selection_prefers_newer_pack_for_same_resource_path() {
        let candidates = vec![PRIMARY_STONE_PATH.to_owned()];
        let available = vec![
            PackResourceMatch {
                pack_index: LOWER_PRIORITY_PACK_INDEX,
                path: PRIMARY_STONE_PATH.to_owned(),
            },
            PackResourceMatch {
                pack_index: HIGHER_PRIORITY_PACK_INDEX,
                path: PRIMARY_STONE_PATH.to_owned(),
            },
        ];

        assert_eq!(
            select_first_available_resource(&candidates, &available),
            Some(PackResourceMatch {
                pack_index: HIGHER_PRIORITY_PACK_INDEX,
                path: PRIMARY_STONE_PATH.to_owned(),
            })
        );
    }

    #[test]
    fn invalid_lookup_identifiers_produce_no_candidates() {
        assert!(resource_pack_path_candidates(MINECRAFT_PLUGIN, UNSAFE_TEXTURE).is_empty());
    }

    #[test]
    fn duplicate_pack_entries_are_reported() {
        assert_eq!(
            duplicate_resource_paths([DUPLICATE_PATH, PRIMARY_STONE_PATH, DUPLICATE_PATH]),
            vec![DUPLICATE_PATH.to_owned()]
        );
    }
}
