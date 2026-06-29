use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::io;
use std::path;
use std::sync::{Arc, Mutex};

use std_or_web::fs;

use crate::types::hash::FNVHash;

use super::cache::asset_object_path;
use super::internal;
use super::paths;
use super::state::{Manager, Pack, Progress};
use super::ASSET_VERSION;

pub(crate) fn commit_downloaded_asset(
    tmp_file: &path::Path,
    location: &path::Path,
) -> io::Result<()> {
    match fs::rename(tmp_file, location) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound && fs::metadata(location).is_ok() => {
            Ok(())
        }
        Err(error) => Err(error),
    }
}

pub(crate) struct DirPack {
    root: path::PathBuf,
}

impl DirPack {
    pub(crate) fn new(root: path::PathBuf) -> Self {
        DirPack { root }
    }
}

impl Pack for DirPack {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>> {
        let location = paths::contained_path(&self.root, name)?;
        match fs::File::open(location) {
            Ok(value) => Some(Box::new(value)),
            Err(_) => None,
        }
    }
}

pub(crate) struct InternalPack;

impl Pack for InternalPack {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>> {
        match internal::get_file(name) {
            Some(value) => Some(Box::new(io::Cursor::new(value))),
            None => None,
        }
    }
}

pub(crate) struct ObjectPack {
    objects: HashMap<String, String, BuildHasherDefault<FNVHash>>,
}

impl ObjectPack {
    pub(crate) fn new() -> ObjectPack {
        let loc = format!("./index/{}.json", ASSET_VERSION);
        let location = path::Path::new(&loc);
        let file = fs::File::open(location).unwrap();
        let index: serde_json::Value = serde_json::from_reader(&file).unwrap();
        let objects = index
            .get("objects")
            .and_then(|value| value.as_object())
            .unwrap();
        let mut hash_objects = HashMap::with_hasher(BuildHasherDefault::default());
        for (key, value) in objects {
            hash_objects.insert(
                key.clone(),
                value
                    .get("hash")
                    .and_then(|value| value.as_str())
                    .unwrap()
                    .to_owned(),
            );
        }
        ObjectPack {
            objects: hash_objects,
        }
    }
}

impl Pack for ObjectPack {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>> {
        if !paths::is_contained_relative_path(name) || !name.starts_with("assets/") {
            return None;
        }
        let name = &name["assets/".len()..];
        if let Some(hash) = self.objects.get(name) {
            let root_location = path::Path::new("./objects/");
            let hash_path = asset_object_path(hash)?;
            let location = paths::contained_path(root_location, &hash_path)?;
            match fs::File::open(location) {
                Ok(value) => Some(Box::new(value)),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub(crate) struct ProgressRead<'a, T> {
    pub(crate) read: T,
    pub(crate) progress: &'a Arc<Mutex<Progress>>,
    pub(crate) task_name: String,
    pub(crate) task_file: String,
}

impl<'a, T: io::Read> io::Read for ProgressRead<'a, T> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let size = self.read.read(buffer)?;
        Manager::add_task_progress(self.progress, &self.task_name, &self.task_file, size as u64);
        Ok(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind, Read};

    use super::super::state::{lock_progress, new_progress};

    const TEST_TMP_FILE: &str = "asset.tmp";
    const TEST_DEST_FILE: &str = "asset";
    const TEST_ASSET_CONTENT: &str = "asset-content";
    const TEST_RESOURCE_PATH: &str = "assets/minecraft/textures/block/stone.png";
    const TEST_RESOURCE_CONTENT: &str = "stone";
    const UNSAFE_RESOURCE_PATH: &str = "assets/../secret.txt";
    const TEST_TASK_NAME: &str = "test task";
    const TEST_TASK_FILE: &str = "test file";
    const BUFFER_LEN: usize = 8;

    struct FailingRead;

    impl Read for FailingRead {
        fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
            Err(Error::new(ErrorKind::Other, "adapter failed"))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_test_dir(name: &str) -> path::PathBuf {
        std::env::temp_dir().join(format!(
            "stevenarella-resource-test-{}-{}",
            std::process::id(),
            name
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn dir_pack_opens_contained_resource() {
        let dir = unique_test_dir("dir-pack-open");
        let _ = fs::remove_dir_all(&dir);
        let file_path = dir.join(TEST_RESOURCE_PATH);
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        fs::write(&file_path, TEST_RESOURCE_CONTENT).unwrap();
        let pack = DirPack::new(dir.clone());

        let mut reader = pack.open(TEST_RESOURCE_PATH).unwrap();
        let mut content = String::new();
        reader.read_to_string(&mut content).unwrap();

        assert_eq!(content, TEST_RESOURCE_CONTENT);
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn dir_pack_rejects_unsafe_resource_paths() {
        let dir = unique_test_dir("dir-pack-unsafe");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let pack = DirPack::new(dir.clone());

        assert!(pack.open(UNSAFE_RESOURCE_PATH).is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn progress_read_propagates_failed_io_adapter() {
        let progress = new_progress();
        let mut reader = ProgressRead {
            read: FailingRead,
            progress: &progress,
            task_name: TEST_TASK_NAME.to_owned(),
            task_file: TEST_TASK_FILE.to_owned(),
        };
        let mut buffer = [0; BUFFER_LEN];

        let error = reader.read(&mut buffer).unwrap_err();
        let progress = lock_progress(&progress).unwrap();

        assert_eq!(error.kind(), ErrorKind::Other);
        assert!(progress.tasks.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn downloaded_asset_commit_renames_temp_file() {
        let dir = unique_test_dir("rename");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let tmp_file = dir.join(TEST_TMP_FILE);
        let location = dir.join(TEST_DEST_FILE);
        fs::write(&tmp_file, TEST_ASSET_CONTENT).unwrap();

        commit_downloaded_asset(&tmp_file, &location).unwrap();

        assert!(fs::metadata(&tmp_file).is_err());
        assert_eq!(fs::read_to_string(&location).unwrap(), TEST_ASSET_CONTENT);
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn downloaded_asset_commit_tolerates_parallel_winner() {
        let dir = unique_test_dir("parallel-winner");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let tmp_file = dir.join(TEST_TMP_FILE);
        let location = dir.join(TEST_DEST_FILE);
        fs::write(&location, TEST_ASSET_CONTENT).unwrap();

        commit_downloaded_asset(&tmp_file, &location).unwrap();

        assert_eq!(fs::read_to_string(&location).unwrap(), TEST_ASSET_CONTENT);
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn downloaded_asset_commit_rejects_missing_temp_and_destination() {
        let dir = unique_test_dir("missing");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let tmp_file = dir.join(TEST_TMP_FILE);
        let location = dir.join(TEST_DEST_FILE);

        let error = commit_downloaded_asset(&tmp_file, &location).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        let _ = fs::remove_dir_all(&dir);
    }
}
