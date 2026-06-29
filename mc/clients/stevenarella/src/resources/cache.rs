const RESOURCE_HASH_PREFIX_LEN: usize = 2;
const ASSET_OBJECT_BASE_URL: &str = "https://resources.download.minecraft.net/";
const RESOURCE_PATH_SEPARATOR: char = '/';
const WINDOWS_PATH_SEPARATOR: char = '\\';

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AssetObjectKey {
    hash: String,
}

impl AssetObjectKey {
    pub(crate) fn from_hash(hash: &str) -> Option<Self> {
        if hash.len() < RESOURCE_HASH_PREFIX_LEN
            || hash.contains(RESOURCE_PATH_SEPARATOR)
            || hash.contains(WINDOWS_PATH_SEPARATOR)
        {
            return None;
        }
        Some(AssetObjectKey {
            hash: hash.to_owned(),
        })
    }

    pub(crate) fn relative_path(&self) -> String {
        format!("{}/{}", &self.hash[..RESOURCE_HASH_PREFIX_LEN], self.hash)
    }

    pub(crate) fn url(&self) -> String {
        format!("{}{}", ASSET_OBJECT_BASE_URL, self.relative_path())
    }
}

pub(crate) fn asset_object_path(hash: &str) -> Option<String> {
    AssetObjectKey::from_hash(hash).map(|key| key.relative_path())
}

pub(crate) fn asset_object_url(hash: &str) -> Option<String> {
    AssetObjectKey::from_hash(hash).map(|key| key.url())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_HASH: &str = "d48940aeab2d4068bd157e6810406c882503a813";
    const SHORT_HASH: &str = "d";
    const UNSAFE_HASH: &str = "d4/escape";
    const EXPECTED_RELATIVE_PATH: &str = "d4/d48940aeab2d4068bd157e6810406c882503a813";
    const EXPECTED_URL: &str =
        "https://resources.download.minecraft.net/d4/d48940aeab2d4068bd157e6810406c882503a813";

    #[test]
    fn asset_cache_keys_use_hash_fanout_and_https_url() {
        let key = AssetObjectKey::from_hash(TEST_HASH).unwrap();

        assert_eq!(key.relative_path(), EXPECTED_RELATIVE_PATH);
        assert_eq!(key.url(), EXPECTED_URL);
        assert_eq!(
            asset_object_path(TEST_HASH),
            Some(EXPECTED_RELATIVE_PATH.to_owned())
        );
        assert_eq!(asset_object_url(TEST_HASH), Some(EXPECTED_URL.to_owned()));
    }

    #[test]
    fn asset_cache_keys_reject_short_or_unsafe_hashes() {
        assert_eq!(AssetObjectKey::from_hash(SHORT_HASH), None);
        assert_eq!(AssetObjectKey::from_hash(UNSAFE_HASH), None);
        assert_eq!(asset_object_path(SHORT_HASH), None);
        assert_eq!(asset_object_url(SHORT_HASH), None);
    }
}
