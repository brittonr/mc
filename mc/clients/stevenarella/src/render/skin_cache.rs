pub(super) const MINECRAFT_TEXTURE_HTTPS_PREFIX: &str = "https://textures.minecraft.net/texture/";
pub(super) const MINECRAFT_TEXTURE_HTTP_PREFIX: &str = "http://textures.minecraft.net/texture/";

const SKIN_CACHE_PREFIX_LEN: usize = 2;
const SKIN_CACHE_ROOT: &str = "skin-cache";
const SKIN_CACHE_EXTENSION: &str = "png";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextureUrlError {
    UnsupportedPrefix,
    EmptyHash,
    UnsafeHash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SkinCachePathError {
    InvalidHash(TextureUrlError),
    ShortHash,
}

pub(super) fn minecraft_texture_hash(url: &str) -> Option<&str> {
    normalize_minecraft_texture_url(url).ok()
}

pub(crate) fn normalize_minecraft_texture_url(url: &str) -> Result<&str, TextureUrlError> {
    let hash = url
        .strip_prefix(MINECRAFT_TEXTURE_HTTPS_PREFIX)
        .or_else(|| url.strip_prefix(MINECRAFT_TEXTURE_HTTP_PREFIX))
        .ok_or(TextureUrlError::UnsupportedPrefix)?;
    validate_texture_hash(hash)?;
    Ok(hash)
}

pub(crate) fn skin_cache_path(hash: &str) -> Result<String, SkinCachePathError> {
    validate_texture_hash(hash).map_err(SkinCachePathError::InvalidHash)?;
    if hash.len() < SKIN_CACHE_PREFIX_LEN {
        return Err(SkinCachePathError::ShortHash);
    }
    Ok(format!(
        "{}/{}/{}.{}",
        SKIN_CACHE_ROOT,
        &hash[..SKIN_CACHE_PREFIX_LEN],
        hash,
        SKIN_CACHE_EXTENSION
    ))
}

fn validate_texture_hash(hash: &str) -> Result<(), TextureUrlError> {
    if hash.is_empty() {
        return Err(TextureUrlError::EmptyHash);
    }
    if !hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(TextureUrlError::UnsafeHash);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SKIN_HASH: &str = "0123456789abcdef";
    const SHORT_SKIN_HASH: &str = "a";
    const EXPECTED_CACHE_PATH: &str = "skin-cache/01/0123456789abcdef.png";

    #[test]
    fn minecraft_texture_url_normalization_accepts_http_and_https_urls() {
        assert_eq!(
            normalize_minecraft_texture_url(&format!(
                "{}{}",
                MINECRAFT_TEXTURE_HTTPS_PREFIX, TEST_SKIN_HASH
            )),
            Ok(TEST_SKIN_HASH)
        );
        assert_eq!(
            normalize_minecraft_texture_url(&format!(
                "{}{}",
                MINECRAFT_TEXTURE_HTTP_PREFIX, TEST_SKIN_HASH
            )),
            Ok(TEST_SKIN_HASH)
        );
    }

    #[test]
    fn skin_cache_path_uses_stable_prefix_directory() {
        assert_eq!(
            skin_cache_path(TEST_SKIN_HASH).unwrap(),
            EXPECTED_CACHE_PATH
        );
    }

    #[test]
    fn invalid_texture_urls_and_unsafe_cache_paths_fail_closed() {
        assert_eq!(
            normalize_minecraft_texture_url(MINECRAFT_TEXTURE_HTTPS_PREFIX),
            Err(TextureUrlError::EmptyHash)
        );
        assert_eq!(
            normalize_minecraft_texture_url("https://example.invalid/texture/hash"),
            Err(TextureUrlError::UnsupportedPrefix)
        );
        assert_eq!(
            normalize_minecraft_texture_url(&format!(
                "{}{}",
                MINECRAFT_TEXTURE_HTTPS_PREFIX, "../escape"
            )),
            Err(TextureUrlError::UnsafeHash)
        );
        assert_eq!(
            skin_cache_path(SHORT_SKIN_HASH),
            Err(SkinCachePathError::ShortHash)
        );
    }
}
