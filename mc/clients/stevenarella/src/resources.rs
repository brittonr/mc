// Copyright 2016 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate steven_resources as internal;

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::io;
use std::path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::thread;
use std_or_web::fs;

use crate::types::hash::FNVHash;
use crate::ui;

const RESOURCES_VERSION: &str = "1.20.1";
const VANILLA_CLIENT_URL: &str =
    "https://piston-data.mojang.com/v1/objects/0c3ec587af28e5a785c0b4a7b8a30f9a8f78f838/client.jar";
const ASSET_VERSION: &str = "5";
const ASSET_INDEX_URL: &str =
    "https://piston-meta.mojang.com/v1/packages/f0c655bb7ae425f989e00f01dbfd94b8a52353c8/5.json";
const ASSET_OBJECT_BASE_URL: &str = "https://resources.download.minecraft.net/";
const MINECRAFT_PLUGIN: &str = "minecraft";
const RESOURCE_HASH_PREFIX_LEN: usize = 2;
const LEGACY_BLOCK_TEXTURE_PREFIX: &str = "textures/blocks/";
const MODERN_BLOCK_TEXTURE_PREFIX: &str = "textures/block/";
const LEGACY_ITEM_TEXTURE_PREFIX: &str = "textures/items/";
const MODERN_ITEM_TEXTURE_PREFIX: &str = "textures/item/";
const LEGACY_STEVE_TEXTURE: &str = "textures/entity/steve.png";
const MODERN_STEVE_TEXTURE: &str = "textures/entity/player/wide/steve.png";
const PROGRESS_EMPTY: f64 = 0.0;
const PROGRESS_COMPLETE: f64 = 1.0;

pub type SharedManager = Arc<RwLock<Manager>>;
type SharedProgress = Arc<Mutex<Progress>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceBoundaryError {
    ProgressPoisoned,
}

pub trait Pack: Sync + Send {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>>;
}

pub struct Manager {
    packs: Vec<Box<dyn Pack>>,
    version: usize,

    vanilla_progress: SharedProgress,
}

pub struct ManagerUI {
    progress_ui: Vec<ProgressUI>,
    num_tasks: isize,
    vanilla_chan: Option<mpsc::Receiver<bool>>,
    vanilla_assets_chan: Option<mpsc::Receiver<bool>>,
}

struct ProgressUI {
    task_name: String,
    task_file: String,
    position: f64,
    closing: bool,
    progress: f64,

    background: ui::ImageRef,
    progress_bar: ui::ImageRef,
}

struct Progress {
    tasks: Vec<Task>,
}

struct Task {
    task_name: String,
    task_file: String,
    total: u64,
    progress: u64,
}

fn task_progress_ratio(progress: u64, total: u64) -> f64 {
    if total == 0 {
        return PROGRESS_COMPLETE;
    }
    ((progress as f64) / (total as f64)).clamp(PROGRESS_EMPTY, PROGRESS_COMPLETE)
}

fn lock_progress(
    progress: &SharedProgress,
) -> Result<MutexGuard<'_, Progress>, ResourceBoundaryError> {
    progress
        .lock()
        .map_err(|_| ResourceBoundaryError::ProgressPoisoned)
}

fn asset_object_path(hash: &str) -> Option<String> {
    if hash.len() < RESOURCE_HASH_PREFIX_LEN {
        return None;
    }
    Some(format!("{}/{}", &hash[..RESOURCE_HASH_PREFIX_LEN], hash))
}

fn asset_object_url(hash: &str) -> Option<String> {
    asset_object_path(hash).map(|path| format!("{}{}", ASSET_OBJECT_BASE_URL, path))
}

#[cfg(not(target_arch = "wasm32"))]
fn commit_downloaded_asset(tmp_file: &path::Path, location: &path::Path) -> io::Result<()> {
    match fs::rename(tmp_file, location) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound && fs::metadata(location).is_ok() => {
            Ok(())
        }
        Err(error) => Err(error),
    }
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

fn compatible_resource_name(plugin: &str, name: &str) -> Option<String> {
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

impl Manager {
    pub fn new() -> (Manager, ManagerUI) {
        let mut m = Manager {
            packs: Vec::new(),
            version: 0,
            vanilla_progress: Arc::new(Mutex::new(Progress { tasks: vec![] })),
        };
        m.add_pack(Box::new(InternalPack));
        #[cfg(target_arch = "wasm32")]
        let vanilla_chan = None;
        #[cfg(target_arch = "wasm32")]
        let vanilla_assets_chan = None;
        #[cfg(not(target_arch = "wasm32"))]
        let (vanilla_chan, vanilla_assets_chan) = (m.download_vanilla(), m.download_assets());
        (
            m,
            ManagerUI {
                progress_ui: vec![],
                num_tasks: 0,
                vanilla_chan,
                vanilla_assets_chan,
            },
        )
    }

    /// Returns the 'version' of the manager. The version is
    /// increase everytime a pack is added or removed.
    pub fn version(&self) -> usize {
        self.version
    }

    pub fn open(&self, plugin: &str, name: &str) -> Option<Box<dyn io::Read>> {
        let path = format!("assets/{}/{}", plugin, name);
        if let Some(val) = self.open_pack_path(&path) {
            return Some(val);
        }
        compatible_resource_name(plugin, name).and_then(|alias| {
            let path = format!("assets/{}/{}", plugin, alias);
            self.open_pack_path(&path)
        })
    }

    pub fn open_all(&self, plugin: &str, name: &str) -> Vec<Box<dyn io::Read>> {
        let path = format!("assets/{}/{}", plugin, name);
        let ret = self.open_all_pack_paths(&path);
        if !ret.is_empty() {
            return ret;
        }
        compatible_resource_name(plugin, name).map_or_else(Vec::new, |alias| {
            let path = format!("assets/{}/{}", plugin, alias);
            self.open_all_pack_paths(&path)
        })
    }

    fn open_pack_path(&self, path: &str) -> Option<Box<dyn io::Read>> {
        for pack in self.packs.iter().rev() {
            if let Some(val) = pack.open(path) {
                return Some(val);
            }
        }
        None
    }

    fn open_all_pack_paths(&self, path: &str) -> Vec<Box<dyn io::Read>> {
        let mut ret = Vec::new();
        for pack in self.packs.iter().rev() {
            if let Some(val) = pack.open(path) {
                ret.push(val);
            }
        }
        ret
    }

    pub fn tick(&mut self, mui: &mut ManagerUI, ui_container: &mut ui::Container, delta: f64) {
        let delta = delta.min(5.0);
        // Check to see if the download of vanilla has completed
        // (if it was started)
        let mut done = false;
        if let Some(ref recv) = mui.vanilla_chan {
            if recv.try_recv().is_ok() {
                done = true;
            }
        }
        if done {
            mui.vanilla_chan = None;
            self.load_vanilla();
        }
        let mut done = false;
        if let Some(ref recv) = mui.vanilla_assets_chan {
            if recv.try_recv().is_ok() {
                done = true;
            }
        }
        if done {
            mui.vanilla_assets_chan = None;
            self.load_assets();
        }

        const UI_HEIGHT: f64 = 32.0;

        let Ok(mut progress) = lock_progress(&self.vanilla_progress) else {
            return;
        };
        progress.tasks.retain(|v| v.progress < v.total);
        // Find out what we have to work with
        for task in &progress.tasks {
            if !mui
                .progress_ui
                .iter()
                .filter(|v| v.task_file == task.task_file)
                .any(|v| v.task_name == task.task_name)
            {
                mui.num_tasks += 1;
                // Add a ui element for it
                let background = ui::ImageBuilder::new()
                    .texture("steven:solid")
                    .position(0.0, -UI_HEIGHT)
                    .size(350.0, UI_HEIGHT)
                    .colour((0, 0, 0, 100))
                    .draw_index(0xFFFFFF - mui.num_tasks)
                    .alignment(ui::VAttach::Bottom, ui::HAttach::Left)
                    .create(ui_container);

                ui::ImageBuilder::new()
                    .texture("steven:solid")
                    .position(0.0, 0.0)
                    .size(350.0, 10.0)
                    .colour((0, 0, 0, 200))
                    .attach(&mut *background.borrow_mut());
                ui::TextBuilder::new()
                    .text(&*task.task_name)
                    .position(3.0, 0.0)
                    .scale_x(0.5)
                    .scale_y(0.5)
                    .draw_index(1)
                    .attach(&mut *background.borrow_mut());
                ui::TextBuilder::new()
                    .text(&*task.task_file)
                    .position(3.0, 12.0)
                    .scale_x(0.5)
                    .scale_y(0.5)
                    .draw_index(1)
                    .attach(&mut *background.borrow_mut());

                let progress_bar = ui::ImageBuilder::new()
                    .texture("steven:solid")
                    .position(0.0, 0.0)
                    .size(0.0, 10.0)
                    .colour((0, 255, 0, 255))
                    .draw_index(2)
                    .alignment(ui::VAttach::Bottom, ui::HAttach::Left)
                    .attach(&mut *background.borrow_mut());

                mui.progress_ui.push(ProgressUI {
                    task_name: task.task_name.clone(),
                    task_file: task.task_file.clone(),
                    position: -UI_HEIGHT,
                    closing: false,
                    progress: 0.0,
                    background,
                    progress_bar,
                });
            }
        }
        for ui in &mut mui.progress_ui {
            if ui.closing {
                continue;
            }
            let mut found = false;
            let mut prog = PROGRESS_COMPLETE;
            for task in progress
                .tasks
                .iter()
                .filter(|v| v.task_file == ui.task_file)
                .filter(|v| v.task_name == ui.task_name)
            {
                found = true;
                prog = task_progress_ratio(task.progress, task.total);
            }
            let background = ui.background.borrow();
            let progress_bar = ui.progress_bar.borrow();
            // Let the progress bar finish
            if !found
                && (background.y - ui.position).abs() < 0.7 * delta
                && (progress_bar.width - 350.0).abs() < 1.0 * delta
            {
                ui.closing = true;
                ui.position = -UI_HEIGHT;
            }
            ui.progress = prog;
        }
        let mut offset = 0.0;
        for ui in &mut mui.progress_ui {
            if ui.closing {
                continue;
            }
            ui.position = offset;
            offset += UI_HEIGHT;
        }
        // Move elements
        for ui in &mut mui.progress_ui {
            let mut background = ui.background.borrow_mut();
            if (background.y - ui.position).abs() < 0.7 * delta {
                background.y = ui.position;
            } else {
                background.y += (ui.position - background.y).signum() * 0.7 * delta;
            }
            let mut progress_bar = ui.progress_bar.borrow_mut();
            let target_size = (350.0 * ui.progress).min(350.0);
            if (progress_bar.width - target_size).abs() < 1.0 * delta {
                progress_bar.width = target_size;
            } else {
                progress_bar.width +=
                    ((target_size - progress_bar.width).signum() * delta).max(0.0);
            }
        }

        // Clean up dead elements
        mui.progress_ui
            .retain(|v| v.position >= -UI_HEIGHT || !v.closing);
    }

    fn add_pack(&mut self, pck: Box<dyn Pack>) {
        self.packs.push(pck);
        self.version += 1;
    }

    fn load_vanilla(&mut self) {
        let loc = format!("./resources-{}", RESOURCES_VERSION);
        let location = path::Path::new(&loc);
        self.packs.insert(
            1,
            Box::new(DirPack {
                root: location.to_path_buf(),
            }),
        );
        self.version += 1;
    }

    fn load_assets(&mut self) {
        self.packs.insert(1, Box::new(ObjectPack::new()));
        self.version += 1;
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn download_assets(&mut self) -> Option<mpsc::Receiver<bool>> {
        let loc = format!("./index/{}.json", ASSET_VERSION);
        let location = path::Path::new(&loc).to_owned();
        let progress_info = self.vanilla_progress.clone();
        let (send, recv) = mpsc::channel();
        let completion = if fs::metadata(&location).is_ok() {
            self.load_assets();
            None
        } else {
            Some(recv)
        };
        thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            if fs::metadata(&location).is_err() {
                fs::create_dir_all(location.parent().unwrap()).unwrap();
                let res = client
                    .get(ASSET_INDEX_URL)
                    .send()
                    .unwrap()
                    .error_for_status()
                    .unwrap();

                let length = res
                    .headers()
                    .get(reqwest::header::CONTENT_LENGTH)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                Self::add_task(
                    &progress_info,
                    "Downloading Asset Index",
                    &*location.to_string_lossy(),
                    length,
                );
                {
                    let mut file =
                        fs::File::create(format!("index-{}.tmp", ASSET_VERSION)).unwrap();
                    let mut progress = ProgressRead {
                        read: res,
                        progress: &progress_info,
                        task_name: "Downloading Asset Index".into(),
                        task_file: location.to_string_lossy().into_owned(),
                    };
                    io::copy(&mut progress, &mut file).unwrap();
                }
                fs::rename(format!("index-{}.tmp", ASSET_VERSION), &location).unwrap();
                send.send(true).unwrap();
            }
            let file = fs::File::open(&location).unwrap();
            let index: serde_json::Value = serde_json::from_reader(&file).unwrap();
            let root_location = path::Path::new("./objects/");
            let objects = index.get("objects").and_then(|v| v.as_object()).unwrap();
            Self::add_task(
                &progress_info,
                "Downloading Assets",
                "./objects",
                objects.len() as u64,
            );
            for (k, v) in objects {
                let hash = v.get("hash").and_then(|v| v.as_str()).unwrap();
                let hash_path = asset_object_path(hash).unwrap();
                let location = root_location.join(&hash_path);
                if fs::metadata(&location).is_err() {
                    fs::create_dir_all(location.parent().unwrap()).unwrap();
                    let res = client
                        .get(asset_object_url(hash).unwrap())
                        .send()
                        .unwrap()
                        .error_for_status()
                        .unwrap();
                    let length = v.get("size").and_then(|v| v.as_u64()).unwrap();
                    Self::add_task(&progress_info, "Downloading Asset", k, length);
                    let mut tmp_file = location.to_owned();
                    tmp_file.set_file_name(format!("{}.tmp", hash));
                    {
                        let mut file = fs::File::create(&tmp_file).unwrap();
                        let mut progress = ProgressRead {
                            read: res,
                            progress: &progress_info,
                            task_name: "Downloading Asset".into(),
                            task_file: k.to_owned(),
                        };
                        io::copy(&mut progress, &mut file).unwrap();
                    }
                    commit_downloaded_asset(&tmp_file, &location).unwrap();
                }
                Self::add_task_progress(&progress_info, "Downloading Assets", "./objects", 1);
            }
        });
        completion
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn download_vanilla(&mut self) -> Option<mpsc::Receiver<bool>> {
        let loc = format!("./resources-{}", RESOURCES_VERSION);
        let location = path::Path::new(&loc);
        if fs::metadata(location.join("steven.assets")).is_ok() {
            self.load_vanilla();
            return None;
        }
        let (send, recv) = mpsc::channel();

        let progress_info = self.vanilla_progress.clone();
        thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let res = client
                .get(VANILLA_CLIENT_URL)
                .send()
                .unwrap()
                .error_for_status()
                .unwrap();
            let mut file = fs::File::create(format!("{}.tmp", RESOURCES_VERSION)).unwrap();

            let length = res
                .headers()
                .get(reqwest::header::CONTENT_LENGTH)
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let task_file = format!("./resources-{}", RESOURCES_VERSION);
            Self::add_task(
                &progress_info,
                "Downloading Core Assets",
                &task_file,
                length,
            );
            {
                let mut progress = ProgressRead {
                    read: res,
                    progress: &progress_info,
                    task_name: "Downloading Core Assets".into(),
                    task_file,
                };
                io::copy(&mut progress, &mut file).unwrap();
            }

            // Copy the resources from the zip
            let file = fs::File::open(format!("{}.tmp", RESOURCES_VERSION)).unwrap();
            let mut zip = zip::ZipArchive::new(file).unwrap();

            let task_file = format!("./resources-{}", RESOURCES_VERSION);
            Self::add_task(
                &progress_info,
                "Unpacking Core Assets",
                &task_file,
                zip.len() as u64,
            );

            let loc = format!("./resources-{}", RESOURCES_VERSION);
            let location = path::Path::new(&loc);
            let count = zip.len();
            for i in 0..count {
                Self::add_task_progress(&progress_info, "Unpacking Core Assets", &task_file, 1);
                let mut file = zip.by_index(i).unwrap();
                if !file.name().starts_with("assets/") {
                    continue;
                }
                let path = location.join(file.name());
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                let mut out = fs::File::create(path).unwrap();
                io::copy(&mut file, &mut out).unwrap();
            }

            fs::File::create(location.join("steven.assets")).unwrap(); // Marker file
            send.send(true).unwrap();

            fs::remove_file(format!("{}.tmp", RESOURCES_VERSION)).unwrap();
        });
        Some(recv)
    }

    fn add_task(progress: &SharedProgress, name: &str, file: &str, length: u64) {
        let _ = Self::try_add_task(progress, name, file, length);
    }

    fn try_add_task(
        progress: &SharedProgress,
        name: &str,
        file: &str,
        length: u64,
    ) -> Result<(), ResourceBoundaryError> {
        let mut info = lock_progress(progress)?;
        info.tasks.push(Task {
            task_name: name.into(),
            task_file: file.into(),
            total: length,
            progress: 0,
        });
        Ok(())
    }

    fn add_task_progress(progress: &SharedProgress, name: &str, file: &str, prog: u64) {
        let _ = Self::try_add_task_progress(progress, name, file, prog);
    }

    fn try_add_task_progress(
        progress: &SharedProgress,
        name: &str,
        file: &str,
        prog: u64,
    ) -> Result<(), ResourceBoundaryError> {
        let mut progress = lock_progress(progress)?;
        for task in progress
            .tasks
            .iter_mut()
            .filter(|v| v.task_file == file)
            .filter(|v| v.task_name == name)
        {
            task.progress += prog;
        }
        Ok(())
    }
}

struct DirPack {
    root: path::PathBuf,
}

impl Pack for DirPack {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>> {
        match fs::File::open(self.root.join(name)) {
            Ok(val) => Some(Box::new(val)),
            Err(_) => None,
        }
    }
}

struct InternalPack;

impl Pack for InternalPack {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>> {
        match internal::get_file(name) {
            Some(val) => Some(Box::new(io::Cursor::new(val))),
            None => None,
        }
    }
}

struct ObjectPack {
    objects: HashMap<String, String, BuildHasherDefault<FNVHash>>,
}

impl ObjectPack {
    fn new() -> ObjectPack {
        let loc = format!("./index/{}.json", ASSET_VERSION);
        let location = path::Path::new(&loc);
        let file = fs::File::open(&location).unwrap();
        let index: serde_json::Value = serde_json::from_reader(&file).unwrap();
        let objects = index.get("objects").and_then(|v| v.as_object()).unwrap();
        let mut hash_objs = HashMap::with_hasher(BuildHasherDefault::default());
        for (k, v) in objects {
            hash_objs.insert(
                k.clone(),
                v.get("hash").and_then(|v| v.as_str()).unwrap().to_owned(),
            );
        }
        ObjectPack { objects: hash_objs }
    }
}

impl Pack for ObjectPack {
    fn open(&self, name: &str) -> Option<Box<dyn io::Read>> {
        if !name.starts_with("assets/") {
            return None;
        }
        let name = &name["assets/".len()..];
        if let Some(hash) = self.objects.get(name) {
            let root_location = path::Path::new("./objects/");
            let hash_path = asset_object_path(hash).unwrap();
            let location = root_location.join(&hash_path);
            match fs::File::open(location) {
                Ok(val) => Some(Box::new(val)),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

struct ProgressRead<'a, T> {
    read: T,
    progress: &'a Arc<Mutex<Progress>>,
    task_name: String,
    task_file: String,
}

impl<'a, T: io::Read> io::Read for ProgressRead<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.read.read(buf)?;
        Manager::add_task_progress(self.progress, &self.task_name, &self.task_file, size as u64);
        Ok(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const TEST_HASH: &str = "d48940aeab2d4068bd157e6810406c882503a813";
    const SHORT_HASH: &str = "d";
    const TEST_TMP_FILE: &str = "asset.tmp";
    const TEST_DEST_FILE: &str = "asset";
    const TEST_ASSET_CONTENT: &str = "asset-content";
    const TEST_RESOURCE_PATH: &str = "assets/minecraft/textures/block/stone.png";
    const TEST_RESOURCE_CONTENT: &[u8] = b"stone";
    const TEST_TASK_NAME: &str = "test task";
    const TEST_TASK_FILE: &str = "test file";
    const TEST_TASK_TOTAL: u64 = 10;
    const TEST_TASK_PROGRESS: u64 = 4;
    const TEST_TASK_RATIO: f64 = 0.4;

    struct StaticPack {
        files: std::collections::HashMap<String, Vec<u8>>,
    }

    impl StaticPack {
        fn with_file(path: &str, content: &[u8]) -> StaticPack {
            let mut files = std::collections::HashMap::new();
            files.insert(path.to_owned(), content.to_vec());
            StaticPack { files }
        }
    }

    impl Pack for StaticPack {
        fn open(&self, name: &str) -> Option<Box<dyn io::Read>> {
            self.files
                .get(name)
                .map(|content| Box::new(io::Cursor::new(content.clone())) as Box<dyn io::Read>)
        }
    }

    fn manager_without_downloads() -> Manager {
        Manager {
            packs: Vec::new(),
            version: 0,
            vanilla_progress: Arc::new(Mutex::new(Progress { tasks: Vec::new() })),
        }
    }

    fn empty_progress() -> SharedProgress {
        Arc::new(Mutex::new(Progress { tasks: Vec::new() }))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_test_dir(name: &str) -> path::PathBuf {
        std::env::temp_dir().join(format!(
            "stevenarella-resource-test-{}-{}",
            std::process::id(),
            name
        ))
    }

    #[test]
    fn resource_reference_modernizes_legacy_block_textures() {
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, "textures/blocks/water_still.png"),
            Some("textures/block/water_still.png".to_owned())
        );
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, "textures/blocks/planks_oak.png"),
            Some("textures/block/oak_planks.png".to_owned())
        );
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, LEGACY_STEVE_TEXTURE),
            Some(MODERN_STEVE_TEXTURE.to_owned())
        );
    }

    #[test]
    fn resource_reference_rejects_non_minecraft_or_unknown_aliases() {
        assert_eq!(
            compatible_resource_name("steven", "textures/blocks/water_still.png"),
            None
        );
        assert_eq!(
            compatible_resource_name(MINECRAFT_PLUGIN, "lang/en_us.json"),
            None
        );
    }

    #[test]
    fn resource_manager_opens_existing_pack_resource() {
        let mut manager = manager_without_downloads();
        manager.add_pack(Box::new(StaticPack::with_file(
            TEST_RESOURCE_PATH,
            TEST_RESOURCE_CONTENT,
        )));

        let mut reader = manager
            .open("minecraft", "textures/block/stone.png")
            .unwrap();
        let mut content = Vec::new();
        reader.read_to_end(&mut content).unwrap();

        assert_eq!(content, TEST_RESOURCE_CONTENT);
        assert_eq!(manager.version(), 1);
    }

    #[test]
    fn resource_manager_returns_none_for_missing_resource() {
        let mut manager = manager_without_downloads();
        manager.add_pack(Box::new(StaticPack::with_file(
            TEST_RESOURCE_PATH,
            TEST_RESOURCE_CONTENT,
        )));

        assert!(manager
            .open("minecraft", "textures/block/missing.png")
            .is_none());
    }

    #[test]
    fn progress_updates_record_task_progress() {
        let progress = empty_progress();

        Manager::try_add_task(&progress, TEST_TASK_NAME, TEST_TASK_FILE, TEST_TASK_TOTAL).unwrap();
        Manager::try_add_task_progress(
            &progress,
            TEST_TASK_NAME,
            TEST_TASK_FILE,
            TEST_TASK_PROGRESS,
        )
        .unwrap();
        let progress = lock_progress(&progress).unwrap();

        assert_eq!(progress.tasks.len(), 1);
        assert_eq!(progress.tasks[0].progress, TEST_TASK_PROGRESS);
        assert_eq!(
            task_progress_ratio(progress.tasks[0].progress, progress.tasks[0].total),
            TEST_TASK_RATIO
        );
    }

    #[test]
    fn progress_updates_fail_closed_after_poison() {
        let progress = empty_progress();
        let poison_target = progress.clone();
        let _ = std::panic::catch_unwind(move || {
            let _guard = poison_target.lock().unwrap();
            panic!("poison progress mutex");
        });

        assert_eq!(
            Manager::try_add_task(&progress, TEST_TASK_NAME, TEST_TASK_FILE, TEST_TASK_TOTAL),
            Err(ResourceBoundaryError::ProgressPoisoned)
        );
        assert_eq!(
            Manager::try_add_task_progress(
                &progress,
                TEST_TASK_NAME,
                TEST_TASK_FILE,
                TEST_TASK_PROGRESS,
            ),
            Err(ResourceBoundaryError::ProgressPoisoned)
        );
    }

    #[test]
    fn zero_length_progress_reports_complete_without_nan() {
        assert_eq!(
            task_progress_ratio(TEST_TASK_PROGRESS, 0),
            PROGRESS_COMPLETE
        );
    }

    #[test]
    fn asset_object_urls_use_https_and_hash_fanout() {
        assert_eq!(
            asset_object_path(TEST_HASH),
            Some(format!("d4/{}", TEST_HASH))
        );
        assert_eq!(
            asset_object_url(TEST_HASH),
            Some(format!(
                "https://resources.download.minecraft.net/d4/{}",
                TEST_HASH
            ))
        );
    }

    #[test]
    fn asset_object_urls_reject_short_hashes() {
        assert_eq!(asset_object_path(SHORT_HASH), None);
        assert_eq!(asset_object_url(SHORT_HASH), None);
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
