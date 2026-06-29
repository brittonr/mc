use std::path::{Component, Path, PathBuf};

const WINDOWS_PATH_SEPARATOR: char = '\\';

pub(crate) fn is_contained_relative_path(raw_path: &str) -> bool {
    if raw_path.is_empty() || raw_path.contains(WINDOWS_PATH_SEPARATOR) {
        return false;
    }

    let mut saw_component = false;
    for component in Path::new(raw_path).components() {
        match component {
            Component::Normal(_) => saw_component = true,
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return false,
        }
    }
    saw_component
}

pub(crate) fn contained_path(root: &Path, raw_path: &str) -> Option<PathBuf> {
    if !is_contained_relative_path(raw_path) {
        return None;
    }
    Some(root.join(raw_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: &str = "/tmp/stevenarella-resources";
    const SAFE_ASSET_PATH: &str = "assets/minecraft/textures/block/stone.png";
    const PARENT_ESCAPE_PATH: &str = "assets/../secret.txt";
    const ABSOLUTE_ESCAPE_PATH: &str = "/tmp/secret.txt";
    const WINDOWS_ESCAPE_PATH: &str = "assets\\minecraft\\secret.txt";

    #[test]
    fn contained_paths_join_under_root() {
        assert_eq!(
            contained_path(Path::new(ROOT), SAFE_ASSET_PATH),
            Some(Path::new(ROOT).join(SAFE_ASSET_PATH))
        );
    }

    #[test]
    fn unsafe_paths_fail_closed() {
        assert!(!is_contained_relative_path(""));
        assert_eq!(contained_path(Path::new(ROOT), PARENT_ESCAPE_PATH), None);
        assert_eq!(contained_path(Path::new(ROOT), ABSOLUTE_ESCAPE_PATH), None);
        assert_eq!(contained_path(Path::new(ROOT), WINDOWS_ESCAPE_PATH), None);
    }
}
