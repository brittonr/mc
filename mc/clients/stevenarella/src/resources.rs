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

use std::io;
use std::path;
use std::sync::mpsc;
use std::thread;

use std_or_web::fs;

use crate::ui;

mod archive;
mod cache;
mod identifiers;
mod io_shell;
mod lookup;
mod pack_discovery;
mod paths;
mod state;

pub use state::{Manager, ManagerUI, Pack, ResourceBoundaryError, SharedManager};

use archive::archive_asset_output_path;
use cache::{asset_object_path, asset_object_url};
use io_shell::{commit_downloaded_asset, DirPack, InternalPack, ObjectPack, ProgressRead};
use lookup::{open_all_from_packs, open_first_from_packs, resource_pack_path_candidates};
use pack_discovery::downloaded_pack_insert_index;
use state::{lock_progress, new_progress, task_progress_ratio, ProgressUI};

const RESOURCES_VERSION: &str = "1.20.1";
const VANILLA_CLIENT_URL: &str =
    "https://piston-data.mojang.com/v1/objects/0c3ec587af28e5a785c0b4a7b8a30f9a8f78f838/client.jar";
pub(crate) const ASSET_VERSION: &str = "5";
const ASSET_INDEX_URL: &str =
    "https://piston-meta.mojang.com/v1/packages/f0c655bb7ae425f989e00f01dbfd94b8a52353c8/5.json";
const MAX_TICK_DELTA: f64 = 5.0;
const PROGRESS_UI_HEIGHT: f64 = 32.0;
const PROGRESS_UI_WIDTH: f64 = 350.0;
const PROGRESS_BAR_HEIGHT: f64 = 10.0;
const PROGRESS_BACKGROUND_ALPHA: u8 = 100;
const PROGRESS_BAR_BACKGROUND_ALPHA: u8 = 200;
const PROGRESS_BAR_FOREGROUND_ALPHA: u8 = 255;
const PROGRESS_GREEN: u8 = 255;
const DRAW_INDEX_BASE: isize = 0xFFFFFF;
const TEXT_X_OFFSET: f64 = 3.0;
const TASK_FILE_TEXT_Y_OFFSET: f64 = 12.0;
const PROGRESS_TEXT_SCALE: f64 = 0.5;
const CHILD_DRAW_INDEX: isize = 1;
const PROGRESS_BAR_DRAW_INDEX: isize = 2;
const UI_SLIDE_SPEED: f64 = 0.7;
const UI_WIDTH_SNAP_EPSILON: f64 = 1.0;
const DEFAULT_PROGRESS_RATIO_COMPLETE: f64 = 1.0;

impl Manager {
    pub fn new() -> (Manager, ManagerUI) {
        let mut manager = Manager {
            packs: Vec::new(),
            version: 0,
            vanilla_progress: new_progress(),
        };
        manager.add_pack(Box::new(InternalPack));
        #[cfg(target_arch = "wasm32")]
        let vanilla_chan = None;
        #[cfg(target_arch = "wasm32")]
        let vanilla_assets_chan = None;
        #[cfg(not(target_arch = "wasm32"))]
        let (vanilla_chan, vanilla_assets_chan) =
            (manager.download_vanilla(), manager.download_assets());
        (
            manager,
            ManagerUI {
                progress_ui: Vec::new(),
                num_tasks: 0,
                vanilla_chan,
                vanilla_assets_chan,
            },
        )
    }

    pub fn open(&self, plugin: &str, name: &str) -> Option<Box<dyn io::Read>> {
        let candidates = resource_pack_path_candidates(plugin, name);
        open_first_from_packs(&self.packs, &candidates)
    }

    pub fn open_all(&self, plugin: &str, name: &str) -> Vec<Box<dyn io::Read>> {
        let candidates = resource_pack_path_candidates(plugin, name);
        open_all_from_packs(&self.packs, &candidates)
    }

    pub fn tick(&mut self, mui: &mut ManagerUI, ui_container: &mut ui::Container, delta: f64) {
        let delta = delta.min(MAX_TICK_DELTA);
        if receiver_completed(&mui.vanilla_chan) {
            mui.vanilla_chan = None;
            self.load_vanilla();
        }
        if receiver_completed(&mui.vanilla_assets_chan) {
            mui.vanilla_assets_chan = None;
            self.load_assets();
        }

        let Ok(mut progress) = lock_progress(&self.vanilla_progress) else {
            return;
        };
        progress.tasks.retain(|task| task.progress < task.total);
        for task in &progress.tasks {
            if progress_ui_missing(&mui.progress_ui, &task.task_name, &task.task_file) {
                mui.num_tasks += 1;
                let background = ui::ImageBuilder::new()
                    .texture("steven:solid")
                    .position(0.0, -PROGRESS_UI_HEIGHT)
                    .size(PROGRESS_UI_WIDTH, PROGRESS_UI_HEIGHT)
                    .colour((0, 0, 0, PROGRESS_BACKGROUND_ALPHA))
                    .draw_index(DRAW_INDEX_BASE - mui.num_tasks)
                    .alignment(ui::VAttach::Bottom, ui::HAttach::Left)
                    .create(ui_container);

                ui::ImageBuilder::new()
                    .texture("steven:solid")
                    .position(0.0, 0.0)
                    .size(PROGRESS_UI_WIDTH, PROGRESS_BAR_HEIGHT)
                    .colour((0, 0, 0, PROGRESS_BAR_BACKGROUND_ALPHA))
                    .attach(&mut *background.borrow_mut());
                ui::TextBuilder::new()
                    .text(&task.task_name)
                    .position(TEXT_X_OFFSET, 0.0)
                    .scale_x(PROGRESS_TEXT_SCALE)
                    .scale_y(PROGRESS_TEXT_SCALE)
                    .draw_index(CHILD_DRAW_INDEX)
                    .attach(&mut *background.borrow_mut());
                ui::TextBuilder::new()
                    .text(&task.task_file)
                    .position(TEXT_X_OFFSET, TASK_FILE_TEXT_Y_OFFSET)
                    .scale_x(PROGRESS_TEXT_SCALE)
                    .scale_y(PROGRESS_TEXT_SCALE)
                    .draw_index(CHILD_DRAW_INDEX)
                    .attach(&mut *background.borrow_mut());

                let progress_bar = ui::ImageBuilder::new()
                    .texture("steven:solid")
                    .position(0.0, 0.0)
                    .size(0.0, PROGRESS_BAR_HEIGHT)
                    .colour((0, PROGRESS_GREEN, 0, PROGRESS_BAR_FOREGROUND_ALPHA))
                    .draw_index(PROGRESS_BAR_DRAW_INDEX)
                    .alignment(ui::VAttach::Bottom, ui::HAttach::Left)
                    .attach(&mut *background.borrow_mut());

                mui.progress_ui.push(ProgressUI {
                    task_name: task.task_name.clone(),
                    task_file: task.task_file.clone(),
                    position: -PROGRESS_UI_HEIGHT,
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
            let mut progress_ratio = DEFAULT_PROGRESS_RATIO_COMPLETE;
            for task in progress
                .tasks
                .iter()
                .filter(|task| task.task_file == ui.task_file)
                .filter(|task| task.task_name == ui.task_name)
            {
                found = true;
                progress_ratio = task_progress_ratio(task.progress, task.total);
            }
            let background = ui.background.borrow();
            let progress_bar = ui.progress_bar.borrow();
            if !found
                && (background.y - ui.position).abs() < UI_SLIDE_SPEED * delta
                && (progress_bar.width - PROGRESS_UI_WIDTH).abs() < UI_WIDTH_SNAP_EPSILON * delta
            {
                ui.closing = true;
                ui.position = -PROGRESS_UI_HEIGHT;
            }
            ui.progress = progress_ratio;
        }
        let mut offset = 0.0;
        for ui in &mut mui.progress_ui {
            if ui.closing {
                continue;
            }
            ui.position = offset;
            offset += PROGRESS_UI_HEIGHT;
        }
        for ui in &mut mui.progress_ui {
            let mut background = ui.background.borrow_mut();
            if (background.y - ui.position).abs() < UI_SLIDE_SPEED * delta {
                background.y = ui.position;
            } else {
                background.y += (ui.position - background.y).signum() * UI_SLIDE_SPEED * delta;
            }
            let mut progress_bar = ui.progress_bar.borrow_mut();
            let target_size = (PROGRESS_UI_WIDTH * ui.progress).min(PROGRESS_UI_WIDTH);
            if (progress_bar.width - target_size).abs() < UI_WIDTH_SNAP_EPSILON * delta {
                progress_bar.width = target_size;
            } else {
                progress_bar.width +=
                    ((target_size - progress_bar.width).signum() * delta).max(0.0);
            }
        }

        mui.progress_ui
            .retain(|ui| ui.position >= -PROGRESS_UI_HEIGHT || !ui.closing);
    }

    fn load_vanilla(&mut self) {
        let loc = format!("./resources-{}", RESOURCES_VERSION);
        let location = path::Path::new(&loc);
        let insert_index = downloaded_pack_insert_index(self.packs.len());
        self.packs
            .insert(insert_index, Box::new(DirPack::new(location.to_path_buf())));
        self.version += 1;
    }

    fn load_assets(&mut self) {
        let insert_index = downloaded_pack_insert_index(self.packs.len());
        self.packs.insert(insert_index, Box::new(ObjectPack::new()));
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
                    &location.to_string_lossy(),
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
            let objects = index
                .get("objects")
                .and_then(|value| value.as_object())
                .unwrap();
            Self::add_task(
                &progress_info,
                "Downloading Assets",
                "./objects",
                objects.len() as u64,
            );
            for (key, value) in objects {
                let hash = value.get("hash").and_then(|value| value.as_str()).unwrap();
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
                    let length = value.get("size").and_then(|value| value.as_u64()).unwrap();
                    Self::add_task(&progress_info, "Downloading Asset", key, length);
                    let mut tmp_file = location.to_owned();
                    tmp_file.set_file_name(format!("{}.tmp", hash));
                    {
                        let mut file = fs::File::create(&tmp_file).unwrap();
                        let mut progress = ProgressRead {
                            read: res,
                            progress: &progress_info,
                            task_name: "Downloading Asset".into(),
                            task_file: key.to_owned(),
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
            for index in 0..count {
                Self::add_task_progress(&progress_info, "Unpacking Core Assets", &task_file, 1);
                let mut file = zip.by_index(index).unwrap();
                let Some(path) = archive_asset_output_path(location, file.name()) else {
                    continue;
                };
                let Some(parent) = path.parent() else {
                    continue;
                };
                fs::create_dir_all(parent).unwrap();
                let mut out = fs::File::create(path).unwrap();
                io::copy(&mut file, &mut out).unwrap();
            }

            fs::File::create(location.join("steven.assets")).unwrap();
            send.send(true).unwrap();

            fs::remove_file(format!("{}.tmp", RESOURCES_VERSION)).unwrap();
        });
        Some(recv)
    }
}

fn receiver_completed(receiver: &Option<mpsc::Receiver<bool>>) -> bool {
    receiver
        .as_ref()
        .map(|receiver| receiver.try_recv().is_ok())
        .unwrap_or(false)
}

fn progress_ui_missing(progress_ui: &[ProgressUI], task_name: &str, task_file: &str) -> bool {
    !progress_ui
        .iter()
        .filter(|ui| ui.task_file == task_file)
        .any(|ui| ui.task_name == task_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const PRIMARY_RESOURCE_PATH: &str = "assets/minecraft/textures/block/stone.png";
    const PRIMARY_RESOURCE_NAME: &str = "textures/block/stone.png";
    const PRIMARY_RESOURCE_CONTENT: &[u8] = b"stone";
    const OVERRIDE_RESOURCE_CONTENT: &[u8] = b"override-stone";
    const LEGACY_RESOURCE_NAME: &str = "textures/blocks/stone.png";
    const MISSING_RESOURCE_NAME: &str = "textures/block/missing.png";
    const UNSAFE_RESOURCE_NAME: &str = "textures/../secret.png";

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
            vanilla_progress: new_progress(),
        }
    }

    fn read_resource(mut reader: Box<dyn io::Read>) -> Vec<u8> {
        let mut content = Vec::new();
        reader.read_to_end(&mut content).unwrap();
        content
    }

    #[test]
    fn resource_manager_opens_existing_pack_resource() {
        let mut manager = manager_without_downloads();
        manager.add_pack(Box::new(StaticPack::with_file(
            PRIMARY_RESOURCE_PATH,
            PRIMARY_RESOURCE_CONTENT,
        )));

        let content = read_resource(
            manager
                .open("minecraft", PRIMARY_RESOURCE_NAME)
                .expect("resource found"),
        );

        assert_eq!(content, PRIMARY_RESOURCE_CONTENT);
        assert_eq!(manager.version(), 1);
    }

    #[test]
    fn resource_manager_uses_newer_pack_for_matching_resource_path() {
        let mut manager = manager_without_downloads();
        manager.add_pack(Box::new(StaticPack::with_file(
            PRIMARY_RESOURCE_PATH,
            PRIMARY_RESOURCE_CONTENT,
        )));
        manager.add_pack(Box::new(StaticPack::with_file(
            PRIMARY_RESOURCE_PATH,
            OVERRIDE_RESOURCE_CONTENT,
        )));

        let content = read_resource(
            manager
                .open("minecraft", PRIMARY_RESOURCE_NAME)
                .expect("override found"),
        );

        assert_eq!(content, OVERRIDE_RESOURCE_CONTENT);
    }

    #[test]
    fn resource_manager_falls_back_to_compatible_alias() {
        let mut manager = manager_without_downloads();
        manager.add_pack(Box::new(StaticPack::with_file(
            PRIMARY_RESOURCE_PATH,
            PRIMARY_RESOURCE_CONTENT,
        )));

        let content = read_resource(
            manager
                .open("minecraft", LEGACY_RESOURCE_NAME)
                .expect("alias found"),
        );

        assert_eq!(content, PRIMARY_RESOURCE_CONTENT);
    }

    #[test]
    fn resource_manager_open_all_preserves_reverse_pack_order() {
        let mut manager = manager_without_downloads();
        manager.add_pack(Box::new(StaticPack::with_file(
            PRIMARY_RESOURCE_PATH,
            PRIMARY_RESOURCE_CONTENT,
        )));
        manager.add_pack(Box::new(StaticPack::with_file(
            PRIMARY_RESOURCE_PATH,
            OVERRIDE_RESOURCE_CONTENT,
        )));

        let contents = manager
            .open_all("minecraft", PRIMARY_RESOURCE_NAME)
            .into_iter()
            .map(read_resource)
            .collect::<Vec<_>>();

        assert_eq!(
            contents,
            vec![OVERRIDE_RESOURCE_CONTENT, PRIMARY_RESOURCE_CONTENT]
        );
    }

    #[test]
    fn resource_manager_returns_none_for_missing_or_unsafe_resources() {
        let mut manager = manager_without_downloads();
        manager.add_pack(Box::new(StaticPack::with_file(
            PRIMARY_RESOURCE_PATH,
            PRIMARY_RESOURCE_CONTENT,
        )));

        assert!(manager.open("minecraft", MISSING_RESOURCE_NAME).is_none());
        assert!(manager.open("minecraft", UNSAFE_RESOURCE_NAME).is_none());
        assert!(manager
            .open_all("minecraft", UNSAFE_RESOURCE_NAME)
            .is_empty());
    }
}
