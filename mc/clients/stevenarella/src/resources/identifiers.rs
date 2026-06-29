use super::paths;

pub(crate) const MINECRAFT_PLUGIN: &str = "minecraft";

const RESOURCE_PATH_SEPARATOR: char = '/';
const LEGACY_BLOCK_TEXTURE_PREFIX: &str = "textures/blocks/";
const MODERN_BLOCK_TEXTURE_PREFIX: &str = "textures/block/";
const LEGACY_ITEM_TEXTURE_PREFIX: &str = "textures/items/";
const MODERN_ITEM_TEXTURE_PREFIX: &str = "textures/item/";
const LEGACY_STEVE_TEXTURE: &str = "textures/entity/steve.png";
const MODERN_STEVE_TEXTURE: &str = "textures/entity/player/wide/steve.png";
const ASSETS_PATH_PREFIX: &str = "assets";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ResourceIdentifier {
    plugin: String,
    name: String,
}

impl ResourceIdentifier {
    pub(crate) fn new(plugin: &str, name: &str) -> Option<Self> {
        if !is_safe_plugin(plugin) || !paths::is_contained_relative_path(name) {
            return None;
        }
        Some(ResourceIdentifier {
            plugin: plugin.to_owned(),
            name: name.to_owned(),
        })
    }

    pub(crate) fn pack_path(&self) -> String {
        format!("{}/{}/{}", ASSETS_PATH_PREFIX, self.plugin, self.name)
    }
}

fn is_safe_plugin(plugin: &str) -> bool {
    !plugin.is_empty()
        && !plugin.contains(RESOURCE_PATH_SEPARATOR)
        && paths::is_contained_relative_path(plugin)
}

pub(crate) fn compatible_resource_name(plugin: &str, name: &str) -> Option<String> {
    if plugin != MINECRAFT_PLUGIN {
        return None;
    }
    if name == LEGACY_STEVE_TEXTURE {
        return Some(MODERN_STEVE_TEXTURE.to_owned());
    }
    if let Some(texture) = name.strip_prefix(LEGACY_BLOCK_TEXTURE_PREFIX) {
        return legacy_block_texture_alias(texture);
    }
    if let Some(texture) = name.strip_prefix(LEGACY_ITEM_TEXTURE_PREFIX) {
        return Some(format!("{}{}", MODERN_ITEM_TEXTURE_PREFIX, texture));
    }
    None
}

fn legacy_block_texture_alias(name: &str) -> Option<String> {
    let modern_name = match name {
        "planks_oak.png" => "oak_planks.png",
        "planks_spruce.png" => "spruce_planks.png",
        "planks_birch.png" => "birch_planks.png",
        "planks_jungle.png" => "jungle_planks.png",
        "planks_acacia.png" => "acacia_planks.png",
        "planks_big_oak.png" => "dark_oak_planks.png",
        "log_oak.png" => "oak_log.png",
        "log_spruce.png" => "spruce_log.png",
        "log_birch.png" => "birch_log.png",
        "log_jungle.png" => "jungle_log.png",
        "log_acacia.png" => "acacia_log.png",
        "log_big_oak.png" => "dark_oak_log.png",
        "grass_top.png" => "grass_block_top.png",
        "grass_side.png" => "grass_block_side.png",
        "grass_side_snowed.png" => "grass_block_snow.png",
        _ => name,
    };
    Some(format!("{}{}", MODERN_BLOCK_TEXTURE_PREFIX, modern_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODERN_STONE_TEXTURE: &str = "textures/block/stone.png";
    const STONE_PACK_PATH: &str = "assets/minecraft/textures/block/stone.png";
    const LEGACY_WATER_TEXTURE: &str = "textures/blocks/water_still.png";
    const MODERN_WATER_TEXTURE: &str = "textures/block/water_still.png";
    const LEGACY_OAK_PLANKS: &str = "textures/blocks/planks_oak.png";
    const MODERN_OAK_PLANKS: &str = "textures/block/oak_planks.png";
    const LEGACY_APPLE_TEXTURE: &str = "textures/items/apple.png";
    const MODERN_APPLE_TEXTURE: &str = "textures/item/apple.png";
    const EMPTY_PLUGIN: &str = "";
    const UNSAFE_PLUGIN: &str = "minecraft/extra";
    const UNSAFE_NAME: &str = "textures/../secret.png";

    #[test]
    fn resource_identifiers_build_asset_pack_paths() {
        let identifier = ResourceIdentifier::new(MINECRAFT_PLUGIN, MODERN_STONE_TEXTURE).unwrap();

        assert_eq!(identifier.pack_path(), STONE_PACK_PATH);
    }

    #[test]
    fn resource_reference_modernizes_legacy_block_item_and_entity_paths() {
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, LEGACY_WATER_TEXTURE),
            Some(MODERN_WATER_TEXTURE.to_owned())
        );
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, LEGACY_OAK_PLANKS),
            Some(MODERN_OAK_PLANKS.to_owned())
        );
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, LEGACY_APPLE_TEXTURE),
            Some(MODERN_APPLE_TEXTURE.to_owned())
        );
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, LEGACY_STEVE_TEXTURE),
            Some(MODERN_STEVE_TEXTURE.to_owned())
        );
    }

    #[test]
    fn invalid_identifiers_fail_closed() {
        assert_eq!(
            ResourceIdentifier::new(EMPTY_PLUGIN, MODERN_STONE_TEXTURE),
            None
        );
        assert_eq!(
            ResourceIdentifier::new(UNSAFE_PLUGIN, MODERN_STONE_TEXTURE),
            None
        );
        assert_eq!(ResourceIdentifier::new(MINECRAFT_PLUGIN, UNSAFE_NAME), None);
        assert_eq!(
            compatible_resource_name("steven", LEGACY_WATER_TEXTURE),
            None
        );
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, "lang/en_us.json"),
            None
        );
    }
}
