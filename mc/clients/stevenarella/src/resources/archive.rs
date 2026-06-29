use std::path::{Path, PathBuf};

use super::paths;

const ASSETS_ARCHIVE_PREFIX: &str = "assets/";

pub(crate) fn archive_asset_output_path(root: &Path, entry_name: &str) -> Option<PathBuf> {
    if !entry_name.starts_with(ASSETS_ARCHIVE_PREFIX) {
        return None;
    }
    paths::contained_path(root, entry_name)
}

#[cfg(test)]
pub(crate) fn archive_entry_names<R: std::io::Read + std::io::Seek>(
    reader: R,
) -> Result<Vec<String>, zip::result::ZipError> {
    let mut archive = zip::ZipArchive::new(reader)?;
    let mut names = Vec::new();
    for index in 0..archive.len() {
        names.push(archive.by_index(index)?.name().to_owned());
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const ROOT: &str = "/tmp/stevenarella-resources";
    const ASSET_ENTRY: &str = "assets/minecraft/textures/block/stone.png";
    const META_ENTRY: &str = "META-INF/MANIFEST.MF";
    const UNSAFE_ENTRY: &str = "assets/../secret.txt";
    const MALFORMED_ARCHIVE: &[u8] = b"not a zip archive";

    #[test]
    fn archive_asset_entries_resolve_under_resource_root() {
        assert_eq!(
            archive_asset_output_path(Path::new(ROOT), ASSET_ENTRY),
            Some(Path::new(ROOT).join(ASSET_ENTRY))
        );
    }

    #[test]
    fn non_asset_or_unsafe_archive_entries_fail_closed() {
        assert_eq!(archive_asset_output_path(Path::new(ROOT), META_ENTRY), None);
        assert_eq!(
            archive_asset_output_path(Path::new(ROOT), UNSAFE_ENTRY),
            None
        );
    }

    #[test]
    fn malformed_archives_are_rejected() {
        assert!(archive_entry_names(Cursor::new(MALFORMED_ARCHIVE)).is_err());
    }
}
