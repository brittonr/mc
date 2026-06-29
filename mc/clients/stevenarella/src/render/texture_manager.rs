use super::{atlas, skin_cache, upload_queue, Renderer};
use crate::resources;
use crate::types::hash::FNVHash;
use image::{GenericImage, GenericImageView};
use log::error;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc;
use std::thread;

const DEFAULT_NAMESPACE: &str = "minecraft";
const STEVEN_NAMESPACE: &str = "steven";
const DYNAMIC_NAMESPACE: &str = "steven-dynamic";
const MISSING_TEXTURE_NAME: &str = "missing_texture";
const SOLID_TEXTURE_NAME: &str = "solid";
const MISSING_TEXTURE_FULL_NAME: &str = "steven:missing_texture";
const DEFAULT_SKIN_RESOURCE: &str = "textures/entity/steve.png";
const TEXTURE_RESOURCE_EXTENSION: &str = "png";
const ANIMATION_METADATA_EXTENSION: &str = "png.mcmeta";
const BLOCK_TEXTURE_PREFIX: &str = "blocks/";
const ITEM_TEXTURE_PREFIX: &str = "items/";
const SKIN_DYNAMIC_PREFIX: &str = "skin-";
const DEFAULT_ANIMATION_FRAME_TIME: i64 = 1;
const INITIAL_ANIMATION_FRAME: usize = 0;
const INITIAL_ANIMATION_REMAINING_TIME: f64 = 0.0;
const RGBA_CHANNEL_COUNT: usize = 4;
const DEFAULT_SKIN_SIZE_PX: u32 = 64;
const LEGACY_SKIN_HEIGHT_PX: u32 = 32;
const SKIN_OPAQUE_ALPHA: u8 = 255;
const LEGACY_SKIN_SECTION_WIDTH_PX: u32 = 4;
const LEGACY_SKIN_SECTION_COUNT: u32 = 4;
const LEGACY_SKIN_SECTION_ROWS: u32 = 16;
const LEGACY_SKIN_TARGET_LEFT_X: u32 = 16;
const LEGACY_SKIN_TARGET_RIGHT_X: u32 = 32;
const LEGACY_SKIN_TARGET_Y: u32 = 48;
const LEGACY_SKIN_SOURCE_Y: u32 = 16;
const LEGACY_SKIN_SOURCE_RIGHT_X: u32 = 40;
const LEGACY_SKIN_SOURCE_OFFSETS: [u32; LEGACY_SKIN_SECTION_COUNT as usize] = [2, 1, 0, 3];
const SKIN_OPAQUE_AREAS: [(u32, u32, u32, u32); 6] = [
    (0, 0, 32, 16),
    (16, 16, 24, 16),
    (0, 16, 16, 16),
    (16, 48, 16, 16),
    (32, 48, 16, 16),
    (40, 16, 16, 16),
];
const MISSING_TEXTURE_PIXELS: [u8; 16] = [
    0, 0, 0, 255, 255, 0, 255, 255, 255, 0, 255, 255, 0, 0, 0, 255,
];
const MISSING_TEXTURE_WIDTH: u32 = 2;
const MISSING_TEXTURE_HEIGHT: u32 = 2;
const SOLID_TEXTURE_WIDTH: u32 = 1;
const SOLID_TEXTURE_HEIGHT: u32 = 1;
const SOLID_TEXTURE_PIXELS: [u8; 4] = [255, 255, 255, 255];

pub struct TextureManager {
    textures: HashMap<String, Texture, BuildHasherDefault<FNVHash>>,
    version: usize,
    resources: resources::SharedManager,
    pub(super) atlases: Vec<atlas::Atlas>,

    pub(super) animated_textures: Vec<AnimatedTexture>,
    pub(super) pending_uploads: Vec<upload_queue::PendingTextureUpload>,

    dynamic_textures: HashMap<String, (Texture, image::DynamicImage), BuildHasherDefault<FNVHash>>,
    free_dynamics: Vec<Texture>,

    pub(super) skins: HashMap<String, AtomicIsize, BuildHasherDefault<FNVHash>>,

    _skin_thread: Option<thread::JoinHandle<()>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextureResourceProbe {
    Missing,
    DecodedImage,
    DecodeFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextureLoadDecision {
    UseDecodedImage,
    InsertMissingTextureDummy,
}

pub(crate) fn texture_load_decision(probe: TextureResourceProbe) -> TextureLoadDecision {
    match probe {
        TextureResourceProbe::DecodedImage => TextureLoadDecision::UseDecodedImage,
        TextureResourceProbe::Missing | TextureResourceProbe::DecodeFailed => {
            TextureLoadDecision::InsertMissingTextureDummy
        }
    }
}

impl TextureManager {
    #[allow(clippy::let_and_return)]
    #[allow(clippy::type_complexity)]
    pub(super) fn new(
        res: resources::SharedManager,
    ) -> (
        TextureManager,
        mpsc::Sender<String>,
        mpsc::Receiver<(String, Option<image::DynamicImage>)>,
    ) {
        let (tx, rx) = mpsc::channel();
        let (stx, srx) = mpsc::channel();

        #[cfg(target_arch = "wasm32")]
        let skin_thread = None;

        #[cfg(not(target_arch = "wasm32"))]
        let skin_thread = Some(thread::spawn(|| Self::process_skins(srx, tx)));

        let mut tm = TextureManager {
            textures: HashMap::with_hasher(BuildHasherDefault::default()),
            version: {
                // TODO: fix borrow and remove clippy::let_and_return above
                let ver = res.read().unwrap().version();
                ver
            },
            resources: res,
            atlases: Vec::new(),
            animated_textures: Vec::new(),
            pending_uploads: Vec::new(),

            dynamic_textures: HashMap::with_hasher(BuildHasherDefault::default()),
            free_dynamics: Vec::new(),
            skins: HashMap::with_hasher(BuildHasherDefault::default()),

            _skin_thread: skin_thread,
        };
        tm.add_defaults();
        (tm, stx, rx)
    }

    fn add_defaults(&mut self) {
        self.put_texture(
            STEVEN_NAMESPACE,
            MISSING_TEXTURE_NAME,
            MISSING_TEXTURE_WIDTH,
            MISSING_TEXTURE_HEIGHT,
            MISSING_TEXTURE_PIXELS.to_vec(),
        );
        self.put_texture(
            STEVEN_NAMESPACE,
            SOLID_TEXTURE_NAME,
            SOLID_TEXTURE_WIDTH,
            SOLID_TEXTURE_HEIGHT,
            SOLID_TEXTURE_PIXELS.to_vec(),
        );
    }

    #[cfg(target_arch = "wasm32")]
    fn process_skins(
        recv: mpsc::Receiver<String>,
        reply: mpsc::Sender<(String, Option<image::DynamicImage>)>,
    ) {
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn process_skins(
        recv: mpsc::Receiver<String>,
        reply: mpsc::Sender<(String, Option<image::DynamicImage>)>,
    ) {
        let client = reqwest::blocking::Client::new();
        loop {
            let hash = match recv.recv() {
                Ok(val) => val,
                Err(_) => return, // Most likely shutting down
            };
            match Self::obtain_skin(&client, &hash) {
                Ok(img) => {
                    let _ = reply.send((hash, Some(img)));
                }
                Err(err) => {
                    error!("Failed to get skin {:?}: {}", hash, err);
                    let _ = reply.send((hash, None));
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn obtain_skin(
        client: &::reqwest::blocking::Client,
        hash: &str,
    ) -> Result<image::DynamicImage, ::std::io::Error> {
        use std::io::{Error, ErrorKind};
        use std::path::Path;
        use std_or_web::fs;

        let path = skin_cache::skin_cache_path(hash).map_err(|err| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("invalid skin cache path: {:?}", err),
            )
        })?;
        let cache_path = Path::new(&path);
        fs::create_dir_all(cache_path.parent().unwrap())?;
        let mut buf = vec![];
        if fs::metadata(cache_path).is_ok() {
            // We have a cached image
            let mut file = fs::File::open(cache_path)?;
            file.read_to_end(&mut buf)?;
        } else {
            // Need to download it
            let url = &format!("{}{}", skin_cache::MINECRAFT_TEXTURE_HTTPS_PREFIX, hash);
            let mut res = match client
                .get(url)
                .send()
                .and_then(|res| res.error_for_status())
            {
                Ok(val) => val,
                Err(err) => {
                    return Err(Error::new(ErrorKind::ConnectionAborted, err));
                }
            };
            let mut buf = vec![];
            match res.read_to_end(&mut buf) {
                Ok(_) => {}
                Err(err) => {
                    // TODO: different error for failure to read?
                    return Err(Error::new(ErrorKind::InvalidData, err));
                }
            }

            // Save to cache
            let mut file = fs::File::create(cache_path)?;
            file.write_all(&buf)?;
        }
        let mut img = match image::load_from_memory(&buf) {
            Ok(val) => val,
            Err(err) => {
                return Err(Error::new(ErrorKind::InvalidData, err));
            }
        };
        let (_, height) = img.dimensions();
        if height == LEGACY_SKIN_HEIGHT_PX {
            img = expand_legacy_skin(img);
        }
        force_opaque_skin_areas(&mut img);
        Ok(img)
    }

    pub(super) fn update_textures(&mut self, version: usize) {
        self.pending_uploads.clear();
        self.atlases.clear();
        self.animated_textures.clear();
        self.version = version;
        let map = self.textures.clone();
        self.textures.clear();

        self.free_dynamics.clear();

        self.add_defaults();

        for name in map.keys() {
            if let Some(n) = name.strip_prefix("steven-dynamic:") {
                let (width, height, data) = {
                    let dynamic_texture = match self.dynamic_textures.get(n) {
                        Some(val) => val,
                        None => continue,
                    };
                    let img = &dynamic_texture.1;
                    let (width, height) = img.dimensions();
                    (width, height, img.to_rgba8().into_vec())
                };
                let new_tex =
                    self.put_texture(DYNAMIC_NAMESPACE, n, width as u32, height as u32, data);
                self.dynamic_textures.get_mut(n).unwrap().0 = new_tex;
            } else if !self.textures.contains_key(name) {
                self.load_texture(name);
            }
        }
    }

    pub(super) fn get_skin(&self, url: &str) -> Option<Texture> {
        let hash = skin_cache::minecraft_texture_hash(url)?;
        if let Some(skin) = self.skins.get(hash) {
            skin.fetch_add(1, Ordering::Relaxed);
        }
        self.get_texture(&format!(
            "{}:{}{}",
            DYNAMIC_NAMESPACE, SKIN_DYNAMIC_PREFIX, hash
        ))
    }

    pub fn release_skin(&self, url: &str) {
        let Some(hash) = skin_cache::minecraft_texture_hash(url) else {
            return;
        };
        if let Some(skin) = self.skins.get(hash) {
            skin.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub(super) fn load_skin(&mut self, renderer: &Renderer, url: &str) -> bool {
        let Some(hash) = skin_cache::minecraft_texture_hash(url) else {
            return false;
        };
        let res = self.resources.clone();
        // TODO: This shouldn't be hardcoded to steve but instead
        // have a way to select alex as a default.
        let img = if let Some(mut val) = res
            .read()
            .unwrap()
            .open(DEFAULT_NAMESPACE, DEFAULT_SKIN_RESOURCE)
        {
            let mut data = Vec::new();
            val.read_to_end(&mut data).unwrap();
            image::load_from_memory(&data).unwrap()
        } else {
            image::DynamicImage::new_rgba8(DEFAULT_SKIN_SIZE_PX, DEFAULT_SKIN_SIZE_PX)
        };
        self.put_dynamic(&format!("{}{}", SKIN_DYNAMIC_PREFIX, hash), img);
        self.skins.insert(hash.to_owned(), AtomicIsize::new(0));
        renderer.skin_request.send(hash.to_owned()).unwrap();
        true
    }

    pub(super) fn update_skin(&mut self, hash: String, img: image::DynamicImage) {
        if !self.skins.contains_key(&hash) {
            return;
        }
        let name = format!("{}:{}{}", DYNAMIC_NAMESPACE, SKIN_DYNAMIC_PREFIX, hash);
        let tex = self.get_texture(&name).unwrap();
        let rect = atlas::Rect {
            x: tex.x,
            y: tex.y,
            width: tex.width,
            height: tex.height,
        };

        self.pending_uploads
            .push(upload_queue::pending_texture_upload(
                tex.atlas,
                rect,
                img.to_rgba8().into_vec(),
            ));
        self.dynamic_textures
            .get_mut(&format!("{}{}", SKIN_DYNAMIC_PREFIX, hash))
            .unwrap()
            .1 = img;
    }

    pub(super) fn get_texture(&self, name: &str) -> Option<Texture> {
        if name.find(':').is_some() {
            self.textures.get(name).cloned()
        } else {
            self.textures
                .get(&format!("{}:{}", DEFAULT_NAMESPACE, name))
                .cloned()
        }
    }

    pub(super) fn load_texture(&mut self, name: &str) {
        let (plugin, name) = if let Some(pos) = name.find(':') {
            (&name[..pos], &name[pos + 1..])
        } else {
            (DEFAULT_NAMESPACE, name)
        };
        let path = format!("textures/{}.{}", name, TEXTURE_RESOURCE_EXTENSION);
        let res = self.resources.clone();
        if let Some(mut val) = res.read().unwrap().open(plugin, &path) {
            let mut data = Vec::new();
            val.read_to_end(&mut data).unwrap();
            if let Ok(img) = image::load_from_memory(&data) {
                debug_assert_eq!(
                    texture_load_decision(TextureResourceProbe::DecodedImage),
                    TextureLoadDecision::UseDecodedImage
                );
                let (width, height) = img.dimensions();
                // Might be animated
                if (name.starts_with(BLOCK_TEXTURE_PREFIX) || name.starts_with(ITEM_TEXTURE_PREFIX))
                    && width != height
                {
                    let id = img.to_rgba8().into_vec();
                    let frame =
                        id[..(width * width * RGBA_CHANNEL_COUNT as u32) as usize].to_owned();
                    if let Some(mut ani) = self.load_animation(plugin, name, &img, id) {
                        ani.texture = self.put_texture(plugin, name, width, width, frame);
                        self.animated_textures.push(ani);
                        return;
                    }
                }
                self.put_texture(plugin, name, width, height, img.to_rgba8().into_vec());
                return;
            }
            debug_assert_eq!(
                texture_load_decision(TextureResourceProbe::DecodeFailed),
                TextureLoadDecision::InsertMissingTextureDummy
            );
        } else {
            debug_assert_eq!(
                texture_load_decision(TextureResourceProbe::Missing),
                TextureLoadDecision::InsertMissingTextureDummy
            );
        }
        self.insert_texture_dummy(plugin, name);
    }

    fn load_animation(
        &mut self,
        plugin: &str,
        name: &str,
        img: &image::DynamicImage,
        data: Vec<u8>,
    ) -> Option<AnimatedTexture> {
        let path = format!("textures/{}.{}", name, ANIMATION_METADATA_EXTENSION);
        let res = self.resources.clone();
        if let Some(val) = res.read().unwrap().open(plugin, &path) {
            let meta: serde_json::Value = serde_json::from_reader(val).unwrap();
            let animation = meta.get("animation").unwrap();
            let frame_time = animation
                .get("frametime")
                .and_then(|v| v.as_i64())
                .unwrap_or(DEFAULT_ANIMATION_FRAME_TIME);
            let interpolate = animation
                .get("interpolate")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let frames = if let Some(frames) = animation.get("frames").and_then(|v| v.as_array()) {
                let mut out = Vec::with_capacity(frames.len());
                for frame in frames {
                    if let Some(index) = frame.as_i64() {
                        out.push(AnimationFrame {
                            index: index as usize,
                            time: frame_time,
                        })
                    } else {
                        out.push(AnimationFrame {
                            index: frame.get("index").unwrap().as_i64().unwrap() as usize,
                            time: frame_time * frame.get("frameTime").unwrap().as_i64().unwrap(),
                        })
                    }
                }
                out
            } else {
                let (width, height) = img.dimensions();
                let count = height / width;
                let mut frames = Vec::with_capacity(count as usize);
                for i in 0..count {
                    frames.push(AnimationFrame {
                        index: i as usize,
                        time: frame_time,
                    })
                }
                frames
            };

            return Some(AnimatedTexture {
                frames,
                data,
                interpolate,
                current_frame: INITIAL_ANIMATION_FRAME,
                remaining_time: INITIAL_ANIMATION_REMAINING_TIME,
                texture: self.get_texture(MISSING_TEXTURE_FULL_NAME).unwrap(),
            });
        }
        None
    }

    fn put_texture(
        &mut self,
        plugin: &str,
        name: &str,
        width: u32,
        height: u32,
        data: Vec<u8>,
    ) -> Texture {
        let (atlas, rect) = self.find_free(width as usize, height as usize);
        self.pending_uploads
            .push(upload_queue::pending_texture_upload(atlas, rect, data));

        let mut full_name = String::new();
        full_name.push_str(plugin);
        full_name.push(':');
        full_name.push_str(name);

        let tex = Texture {
            name: full_name.clone(),
            version: self.version,
            atlas,
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            rel_x: 0.0,
            rel_y: 0.0,
            rel_width: 1.0,
            rel_height: 1.0,
            is_rel: false,
        };
        self.textures.insert(full_name, tex.clone());
        tex
    }

    fn find_free(&mut self, width: usize, height: usize) -> (i32, atlas::Rect) {
        let mut index = 0;
        for atlas in &mut self.atlases {
            if let Some(rect) = atlas.add(width, height) {
                return (index, rect);
            }
            index += 1;
        }
        let mut atlas = atlas::Atlas::new(super::ATLAS_SIZE, super::ATLAS_SIZE);
        let rect = atlas.add(width, height);
        self.atlases.push(atlas);
        (index, rect.unwrap())
    }

    fn insert_texture_dummy(&mut self, plugin: &str, name: &str) -> Texture {
        let missing = self.get_texture(MISSING_TEXTURE_FULL_NAME).unwrap();

        let mut full_name = String::new();
        full_name.push_str(plugin);
        full_name.push(':');
        full_name.push_str(name);

        let t = Texture {
            name: full_name.to_owned(),
            version: self.version,
            atlas: missing.atlas,
            x: missing.x,
            y: missing.y,
            width: missing.width,
            height: missing.height,
            rel_x: 0.0,
            rel_y: 0.0,
            rel_width: 1.0,
            rel_height: 1.0,
            is_rel: false,
        };
        self.textures.insert(full_name, t.clone());
        t
    }

    pub fn put_dynamic(&mut self, name: &str, img: image::DynamicImage) -> Texture {
        use std::mem;
        let (width, height) = img.dimensions();
        let (width, height) = (width as usize, height as usize);
        let mut rect_pos = None;
        for (i, r) in self.free_dynamics.iter().enumerate() {
            if r.width == width && r.height == height {
                rect_pos = Some(i);
                break;
            } else if r.width >= width && r.height >= height {
                rect_pos = Some(i);
            }
        }
        let data = img.to_rgba8().into_vec();

        if let Some(rect_pos) = rect_pos {
            let mut tex = self.free_dynamics.remove(rect_pos);
            let rect = atlas::Rect {
                x: tex.x,
                y: tex.y,
                width,
                height,
            };
            self.pending_uploads
                .push(upload_queue::pending_texture_upload(tex.atlas, rect, data));
            let mut t = tex.relative(
                0.0,
                0.0,
                (width as f32) / (tex.width as f32),
                (height as f32) / (tex.height as f32),
            );
            let old_name = mem::replace(&mut tex.name, format!("{}:{}", DYNAMIC_NAMESPACE, name));
            self.dynamic_textures.insert(name.to_owned(), (tex, img));
            // We need to rename the texture itself so that get_texture calls
            // work with the new name
            let mut old = self.textures.remove(&old_name).unwrap();
            old.name = format!("{}:{}", DYNAMIC_NAMESPACE, name);
            t.name = old.name.clone();
            self.textures
                .insert(format!("{}:{}", DYNAMIC_NAMESPACE, name), old);
            t
        } else {
            let tex = self.put_texture(DYNAMIC_NAMESPACE, name, width as u32, height as u32, data);
            self.dynamic_textures
                .insert(name.to_owned(), (tex.clone(), img));
            tex
        }
    }

    pub fn remove_dynamic(&mut self, name: &str) {
        let desc = self.dynamic_textures.remove(name).unwrap();
        self.free_dynamics.push(desc.0);
    }
}

fn expand_legacy_skin(img: image::DynamicImage) -> image::DynamicImage {
    let mut new = image::DynamicImage::new_rgba8(DEFAULT_SKIN_SIZE_PX, DEFAULT_SKIN_SIZE_PX);
    new.copy_from(&img, 0, 0)
        .expect("Invalid png image in skin");
    for xx in 0..LEGACY_SKIN_SECTION_WIDTH_PX {
        for yy in 0..LEGACY_SKIN_SECTION_ROWS {
            for (section, os) in LEGACY_SKIN_SOURCE_OFFSETS.iter().enumerate() {
                let section = section as u32;
                new.put_pixel(
                    LEGACY_SKIN_TARGET_LEFT_X
                        + (LEGACY_SKIN_SECTION_WIDTH_PX - 1 - xx)
                        + section * LEGACY_SKIN_SECTION_WIDTH_PX,
                    LEGACY_SKIN_TARGET_Y + yy,
                    img.get_pixel(
                        xx + os * LEGACY_SKIN_SECTION_WIDTH_PX,
                        LEGACY_SKIN_SOURCE_Y + yy,
                    ),
                );
                new.put_pixel(
                    LEGACY_SKIN_TARGET_RIGHT_X
                        + (LEGACY_SKIN_SECTION_WIDTH_PX - 1 - xx)
                        + section * LEGACY_SKIN_SECTION_WIDTH_PX,
                    LEGACY_SKIN_TARGET_Y + yy,
                    img.get_pixel(
                        xx + LEGACY_SKIN_SOURCE_RIGHT_X + os * LEGACY_SKIN_SECTION_WIDTH_PX,
                        LEGACY_SKIN_SOURCE_Y + yy,
                    ),
                );
            }
        }
    }
    new
}

fn force_opaque_skin_areas(img: &mut image::DynamicImage) {
    for area in SKIN_OPAQUE_AREAS.iter() {
        for x in area.0..(area.0 + area.2) {
            for y in area.1..(area.1 + area.3) {
                let mut col = img.get_pixel(x, y);
                col.0[3] = SKIN_OPAQUE_ALPHA;
                img.put_pixel(x, y, col);
            }
        }
    }
}

#[allow(dead_code)]
pub(super) struct AnimatedTexture {
    pub(super) frames: Vec<AnimationFrame>,
    pub(super) data: Vec<u8>,
    pub(super) interpolate: bool,
    pub(super) current_frame: usize,
    pub(super) remaining_time: f64,
    pub(super) texture: Texture,
}

pub(super) struct AnimationFrame {
    pub(super) index: usize,
    pub(super) time: i64,
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub name: String,
    pub(super) version: usize,
    pub atlas: i32,
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) is_rel: bool, // Save some cycles for non-relative textures
    pub(super) rel_x: f32,
    pub(super) rel_y: f32,
    pub(super) rel_width: f32,
    pub(super) rel_height: f32,
}

impl Texture {
    pub fn get_x(&self) -> usize {
        if self.is_rel {
            self.x + ((self.width as f32) * self.rel_x) as usize
        } else {
            self.x
        }
    }

    pub fn get_y(&self) -> usize {
        if self.is_rel {
            self.y + ((self.height as f32) * self.rel_y) as usize
        } else {
            self.y
        }
    }

    pub fn get_width(&self) -> usize {
        if self.is_rel {
            ((self.width as f32) * self.rel_width) as usize
        } else {
            self.width
        }
    }

    pub fn get_height(&self) -> usize {
        if self.is_rel {
            ((self.height as f32) * self.rel_height) as usize
        } else {
            self.height
        }
    }

    pub fn relative(&self, x: f32, y: f32, width: f32, height: f32) -> Texture {
        Texture {
            name: self.name.clone(),
            version: self.version,
            x: self.x,
            y: self.y,
            atlas: self.atlas,
            width: self.width,
            height: self.height,
            is_rel: true,
            rel_x: self.rel_x + x * self.rel_width,
            rel_y: self.rel_y + y * self.rel_height,
            rel_width: width * self.rel_width,
            rel_height: height * self.rel_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_resources_use_missing_texture_fallback_decision() {
        assert_eq!(
            texture_load_decision(TextureResourceProbe::Missing),
            TextureLoadDecision::InsertMissingTextureDummy
        );
        assert_eq!(
            texture_load_decision(TextureResourceProbe::DecodeFailed),
            TextureLoadDecision::InsertMissingTextureDummy
        );
    }

    #[test]
    fn decoded_resources_use_image_upload_decision() {
        assert_eq!(
            texture_load_decision(TextureResourceProbe::DecodedImage),
            TextureLoadDecision::UseDecodedImage
        );
    }
}
