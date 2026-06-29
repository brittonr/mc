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

mod atlas;
mod camera;
pub(crate) mod capture_readback;
mod chunk_buffers;
mod frame;
pub mod glsl;
#[macro_use]
pub mod shaders;
pub mod clouds;
pub mod model;
mod skin_cache;
mod texture_manager;
pub mod ui;
mod upload_queue;

pub use camera::Camera;
pub use chunk_buffers::ChunkBuffer;
pub use texture_manager::{Texture, TextureManager};

use crate::gl;
use crate::resources;
use crate::world;
use byteorder::{NativeEndian, WriteBytesExt};
use cgmath::prelude::*;
use log::trace;
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc, RwLock};

const ATLAS_SIZE: usize = 1024;
const RGBA_BYTES_PER_TEXEL: usize = 4;
const ANIMATION_DELTA_DIVISOR: f64 = 3.0;

pub struct Renderer {
    resource_version: usize,
    pub resources: resources::SharedManager,
    textures: Arc<RwLock<TextureManager>>,
    pub ui: ui::UIState,
    pub model: model::Manager,
    pub clouds: Option<clouds::Clouds>,

    gl_texture: gl::Texture,
    texture_layers: usize,

    chunk_shader: ChunkShader,
    chunk_shader_alpha: ChunkShaderAlpha,
    trans_shader: TransShader,

    element_buffer: gl::Buffer,
    element_buffer_size: usize,
    element_buffer_type: gl::Type,

    pub camera: Camera,
    perspective_matrix: cgmath::Matrix4<f32>,
    camera_matrix: cgmath::Matrix4<f32>,
    pub frustum: collision::Frustum<f32>,
    pub view_vector: cgmath::Vector3<f32>,

    pub frame_id: u32,

    trans: Option<TransInfo>,

    pub width: u32,
    pub height: u32,

    // Light renderering
    pub light_level: f32,
    pub sky_offset: f32,
    skin_request: mpsc::Sender<String>,
    skin_reply: mpsc::Receiver<(String, Option<image::DynamicImage>)>,
}

init_shader! {
    Program ChunkShader {
        vert = "chunk_vertex",
        frag = "chunk_frag",
        attribute = {
            required position => "aPosition",
            required texture_info => "aTextureInfo",
            required texture_offset => "aTextureOffset",
            required color => "aColor",
            required lighting => "aLighting",
        },
        uniform = {
            required perspective_matrix => "perspectiveMatrix",
            required camera_matrix => "cameraMatrix",
            required offset => "offset",
            required texture => "textures",
            required light_level => "lightLevel",
            required sky_offset => "skyOffset",
        },
    }
}

init_shader! {
    Program ChunkShaderAlpha {
        vert = "chunk_vertex",
        frag = "chunk_frag", #alpha
        attribute = {
            required position => "aPosition",
            required texture_info => "aTextureInfo",
            required texture_offset => "aTextureOffset",
            required color => "aColor",
            required lighting => "aLighting",
        },
        uniform = {
            required perspective_matrix => "perspectiveMatrix",
            required camera_matrix => "cameraMatrix",
            required offset => "offset",
            required texture => "textures",
            required light_level => "lightLevel",
            required sky_offset => "skyOffset",
        },
    }
}

impl Renderer {
    pub fn new(res: resources::SharedManager, shader_version: &str) -> Renderer {
        let version = { res.read().unwrap().version() };
        let tex = gl::Texture::new();
        tex.bind(gl::TEXTURE_2D_ARRAY);
        tex.image_3d(
            gl::TEXTURE_2D_ARRAY,
            0,
            ATLAS_SIZE as u32,
            ATLAS_SIZE as u32,
            1,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &[0; ATLAS_SIZE * ATLAS_SIZE * RGBA_BYTES_PER_TEXEL],
        );
        tex.set_parameter(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
        tex.set_parameter(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_MIN_FILTER, gl::NEAREST);
        tex.set_parameter(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
        tex.set_parameter(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);

        let (textures, skin_req, skin_reply) = TextureManager::new(res.clone());
        let textures = Arc::new(RwLock::new(textures));

        let mut greg = glsl::Registry::new(shader_version);
        shaders::add_shaders(&mut greg);
        let ui = ui::UIState::new(&greg, textures.clone(), res.clone());

        gl::enable(gl::DEPTH_TEST);
        gl::enable(gl::CULL_FACE_FLAG);
        gl::cull_face(gl::BACK);
        gl::front_face(gl::CLOCK_WISE);

        // Shaders
        let chunk_shader = ChunkShader::new(&greg);
        let chunk_shader_alpha = ChunkShaderAlpha::new(&greg);
        let trans_shader = TransShader::new(&greg);

        // UI
        // Line Drawer
        // Clouds

        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::depth_func(gl::LESS_OR_EQUAL);

        #[cfg(not(target_arch = "wasm32"))]
        let clouds = Some(clouds::Clouds::new(&greg, textures.clone()));

        // No clouds on wasm since no geo shaders on WebGL
        // TODO: setting to disable clouds on native, too, if desired
        #[cfg(target_arch = "wasm32")]
        let clouds = None;

        Renderer {
            resource_version: version,
            model: model::Manager::new(&greg),
            clouds,
            textures,
            ui,
            resources: res,
            gl_texture: tex,
            texture_layers: 1,

            chunk_shader,
            chunk_shader_alpha,
            trans_shader,

            element_buffer: gl::Buffer::new(),
            element_buffer_size: 0,
            element_buffer_type: gl::UNSIGNED_BYTE,

            width: 0,
            height: 0,

            camera: Camera::default(),
            perspective_matrix: cgmath::Matrix4::identity(),
            camera_matrix: cgmath::Matrix4::identity(),
            frustum: collision::Frustum::from_matrix4(cgmath::Matrix4::identity()).unwrap(),
            view_vector: cgmath::Vector3::zero(),

            frame_id: frame::INITIAL_FRAME_ID,

            trans: None,

            light_level: 0.8,
            sky_offset: 1.0,
            skin_request: skin_req,
            skin_reply,
        }
    }

    pub fn update_camera(&mut self, width: u32, height: u32) {
        // Not a sane place to put this but it works
        {
            let rm = self.resources.read().unwrap();
            if rm.version() != self.resource_version {
                self.resource_version = rm.version();
                trace!("Updating textures to {}", self.resource_version);
                self.textures
                    .write()
                    .unwrap()
                    .update_textures(self.resource_version);

                self.model
                    .rebuild_models(self.resource_version, &self.textures);
            }
        }

        if self.height != height || self.width != width {
            let Ok(perspective_matrix) = camera::perspective_matrix(width, height) else {
                return;
            };
            self.width = width;
            self.height = height;
            gl::viewport(0, 0, width as i32, height as i32);
            self.perspective_matrix = perspective_matrix;
            self.init_trans(width, height);
        }

        let matrices = camera::camera_matrices(&self.camera, self.perspective_matrix);
        self.view_vector = matrices.view_vector;
        self.camera_matrix = matrices.camera_matrix;
        self.frustum = matrices.frustum;
    }

    pub fn tick(
        &mut self,
        world: &mut world::World,
        delta: f64,
        width: u32,
        height: u32,
        physical_width: u32,
        physical_height: u32,
    ) {
        let frame_plan = match frame::plan_frame(
            frame::FrameDimensions {
                logical_width: width,
                logical_height: height,
                physical_width,
                physical_height,
            },
            self.frame_id,
        ) {
            Ok(plan) => plan,
            Err(_) => return,
        };
        self.update_textures(delta);

        let trans = self.trans.as_mut().unwrap();
        trans.main.bind();

        gl::active_texture(0);
        self.gl_texture.bind(gl::TEXTURE_2D_ARRAY);

        #[cfg(not(target_arch = "wasm32"))]
        gl::enable(gl::MULTISAMPLE);

        let time_offset = self.sky_offset * 0.9;
        gl::clear_color(
            (122.0 / 255.0) * time_offset,
            (165.0 / 255.0) * time_offset,
            (247.0 / 255.0) * time_offset,
            1.0,
        );
        gl::clear(gl::ClearFlags::Color | gl::ClearFlags::Depth);

        // Chunk rendering
        self.chunk_shader.program.use_program();

        self.chunk_shader
            .perspective_matrix
            .set_matrix4(&self.perspective_matrix);
        self.chunk_shader
            .camera_matrix
            .set_matrix4(&self.camera_matrix);
        self.chunk_shader.texture.set_int(0);
        self.chunk_shader.light_level.set_float(self.light_level);
        self.chunk_shader.sky_offset.set_float(self.sky_offset);

        let solid_render_list = world.get_render_list();
        for (pos, info) in &solid_render_list {
            let counts = info.layer_counts();
            if counts.solid_count > 0 {
                let solid = info
                    .solid
                    .as_ref()
                    .expect("solid count requires solid chunk info");
                self.chunk_shader.offset.set_int3(
                    pos.0,
                    chunk_buffers::chunk_offset_y(pos.1),
                    pos.2,
                );
                solid.array.bind();
                gl::draw_elements(
                    gl::TRIANGLES,
                    solid.count as i32,
                    self.element_buffer_type,
                    0,
                );
            }
        }

        // Line rendering
        // Model rendering
        self.model.draw(
            &self.frustum,
            &self.perspective_matrix,
            &self.camera_matrix,
            self.light_level,
            self.sky_offset,
        );
        if let Some(clouds) = &mut self.clouds {
            if world.copy_cloud_heightmap(&mut clouds.heightmap_data) {
                clouds.dirty = true;
            }
            clouds.draw(
                &self.camera.pos,
                &self.perspective_matrix,
                &self.camera_matrix,
                self.light_level,
                self.sky_offset,
                delta,
            );
        }

        // Trans chunk rendering
        self.chunk_shader_alpha.program.use_program();
        self.chunk_shader_alpha
            .perspective_matrix
            .set_matrix4(&self.perspective_matrix);
        self.chunk_shader_alpha
            .camera_matrix
            .set_matrix4(&self.camera_matrix);
        self.chunk_shader_alpha.texture.set_int(0);
        self.chunk_shader_alpha
            .light_level
            .set_float(self.light_level);
        self.chunk_shader_alpha
            .sky_offset
            .set_float(self.sky_offset);

        // Copy the depth buffer
        trans.main.bind_read();
        trans.trans.bind_draw();
        gl::blit_framebuffer(
            0,
            0,
            frame_plan.dimensions.physical_width as i32,
            frame_plan.dimensions.physical_height as i32,
            0,
            0,
            frame_plan.dimensions.physical_width as i32,
            frame_plan.dimensions.physical_height as i32,
            gl::ClearFlags::Depth,
            gl::NEAREST,
        );

        gl::enable(gl::BLEND);
        gl::depth_mask(false);
        trans.trans.bind();
        gl::clear_color(0.0, 0.0, 0.0, 1.0);
        gl::clear(gl::ClearFlags::Color);
        gl::clear_buffer(gl::COLOR, 0, &mut [0.0, 0.0, 0.0, 1.0]);
        gl::clear_buffer(gl::COLOR, 1, &mut [0.0, 0.0, 0.0, 0.0]);
        gl::blend_func_separate(
            gl::ONE_FACTOR,
            gl::ONE_FACTOR,
            gl::ZERO_FACTOR,
            gl::ONE_MINUS_SRC_ALPHA,
        );

        let translucent_render_list = world.get_render_list();
        for (pos, info) in translucent_render_list.iter().rev() {
            let counts = info.layer_counts();
            if counts.translucent_count > 0 {
                let trans = info
                    .trans
                    .as_ref()
                    .expect("translucent count requires translucent chunk info");
                self.chunk_shader_alpha.offset.set_int3(
                    pos.0,
                    chunk_buffers::chunk_offset_y(pos.1),
                    pos.2,
                );
                trans.array.bind();
                gl::draw_elements(
                    gl::TRIANGLES,
                    trans.count as i32,
                    self.element_buffer_type,
                    0,
                );
            }
        }

        gl::check_framebuffer_status();
        gl::unbind_framebuffer();
        gl::disable(gl::DEPTH_TEST);
        gl::clear(gl::ClearFlags::Color);
        gl::disable(gl::BLEND);
        trans.draw(&self.trans_shader);

        gl::enable(gl::DEPTH_TEST);
        gl::depth_mask(true);
        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        #[cfg(not(target_arch = "wasm32"))]
        gl::disable(gl::MULTISAMPLE);

        self.ui.tick(width, height);

        gl::check_gl_error();

        self.frame_id = frame_plan.next_frame_id;
    }

    fn ensure_element_buffer(&mut self, size: usize) {
        if self.element_buffer_size < size {
            let (data, ty) = self::generate_element_buffer(size);
            self.element_buffer_type = ty;
            self.element_buffer.bind(gl::ELEMENT_ARRAY_BUFFER);
            self.element_buffer
                .set_data(gl::ELEMENT_ARRAY_BUFFER, &data, gl::DYNAMIC_DRAW);
            self.element_buffer_size = size;
        }
    }

    pub fn update_chunk_solid(&mut self, buffer: &mut ChunkBuffer, data: &[u8], count: usize) {
        self.ensure_element_buffer(count);
        if count == 0 {
            if buffer.solid.is_some() {
                buffer.solid = None;
            }
            return;
        }
        let new = buffer.solid.is_none();
        if buffer.solid.is_none() {
            buffer.solid = Some(chunk_buffers::ChunkRenderInfo::empty());
        }
        let info = buffer.solid.as_mut().unwrap();

        info.array.bind();
        self.chunk_shader.position.enable();
        self.chunk_shader.texture_info.enable();
        self.chunk_shader.texture_offset.enable();
        self.chunk_shader.color.enable();
        self.chunk_shader.lighting.enable();

        self.element_buffer.bind(gl::ELEMENT_ARRAY_BUFFER);

        info.buffer.bind(gl::ARRAY_BUFFER);
        if new || info.buffer_size < data.len() {
            info.buffer_size = data.len();
            info.buffer
                .set_data(gl::ARRAY_BUFFER, data, gl::DYNAMIC_DRAW);
        } else {
            info.buffer.re_set_data(gl::ARRAY_BUFFER, data);
        }

        self.chunk_shader
            .position
            .vertex_pointer(3, gl::FLOAT, false, 40, 0);
        self.chunk_shader
            .texture_info
            .vertex_pointer(4, gl::UNSIGNED_SHORT, false, 40, 12);
        self.chunk_shader
            .texture_offset
            .vertex_pointer(3, gl::SHORT, false, 40, 20);
        self.chunk_shader
            .color
            .vertex_pointer(3, gl::UNSIGNED_BYTE, true, 40, 28);
        self.chunk_shader
            .lighting
            .vertex_pointer(2, gl::UNSIGNED_SHORT, false, 40, 32);

        info.count = count;
    }

    pub fn update_chunk_trans(&mut self, buffer: &mut ChunkBuffer, data: &[u8], count: usize) {
        self.ensure_element_buffer(count);
        if count == 0 {
            if buffer.trans.is_some() {
                buffer.trans = None;
            }
            return;
        }
        let new = buffer.trans.is_none();
        if buffer.trans.is_none() {
            buffer.trans = Some(chunk_buffers::ChunkRenderInfo::empty());
        }
        let info = buffer.trans.as_mut().unwrap();

        info.array.bind();
        self.chunk_shader_alpha.position.enable();
        self.chunk_shader_alpha.texture_info.enable();
        self.chunk_shader_alpha.texture_offset.enable();
        self.chunk_shader_alpha.color.enable();
        self.chunk_shader_alpha.lighting.enable();

        self.element_buffer.bind(gl::ELEMENT_ARRAY_BUFFER);

        info.buffer.bind(gl::ARRAY_BUFFER);
        if new || info.buffer_size < data.len() {
            info.buffer_size = data.len();
            info.buffer
                .set_data(gl::ARRAY_BUFFER, data, gl::DYNAMIC_DRAW);
        } else {
            info.buffer.re_set_data(gl::ARRAY_BUFFER, data);
        }

        self.chunk_shader_alpha
            .position
            .vertex_pointer(3, gl::FLOAT, false, 40, 0);
        self.chunk_shader_alpha
            .texture_info
            .vertex_pointer(4, gl::UNSIGNED_SHORT, false, 40, 12);
        self.chunk_shader_alpha
            .texture_offset
            .vertex_pointer(3, gl::SHORT, false, 40, 20);
        self.chunk_shader_alpha
            .color
            .vertex_pointer(3, gl::UNSIGNED_BYTE, true, 40, 28);
        self.chunk_shader_alpha
            .lighting
            .vertex_pointer(2, gl::UNSIGNED_SHORT, false, 40, 32);

        info.count = count;
    }

    #[allow(clippy::uninit_vec)] // TODO: fix uninitialized memory, use MaybeUninit on Vec below
    fn do_pending_textures(&mut self) {
        let plan = {
            let tex = self.textures.read().unwrap();
            upload_queue::plan_texture_uploads(
                self.texture_layers,
                tex.atlases.len(),
                tex.pending_uploads.len(),
            )
        };

        if let Some(resize) = plan.resize {
            // Rebuild the texture if it needs resizing
            let len = ATLAS_SIZE * ATLAS_SIZE * RGBA_BYTES_PER_TEXEL * resize.required_layers;
            let mut data = Vec::with_capacity(len);
            unsafe {
                data.set_len(len);
            }
            self.gl_texture.get_pixels(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &mut data[..],
            );
            self.gl_texture.image_3d(
                gl::TEXTURE_2D_ARRAY,
                0,
                ATLAS_SIZE as u32,
                ATLAS_SIZE as u32,
                resize.required_layers as u32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &data[..],
            );
            self.texture_layers = resize.required_layers;
        }

        if plan.has_pending_uploads() {
            // Upload pending changes
            let mut tex = self.textures.write().unwrap();
            for upload in &tex.pending_uploads {
                self.gl_texture.sub_image_3d(
                    gl::TEXTURE_2D_ARRAY,
                    0,
                    upload.rect.x as u32,
                    upload.rect.y as u32,
                    upload.atlas as u32,
                    upload.rect.width as u32,
                    upload.rect.height as u32,
                    1,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    &upload.data[..],
                );
            }
            tex.pending_uploads.clear();
        }
    }

    fn update_textures(&mut self, delta: f64) {
        {
            let mut tex = self.textures.write().unwrap();
            while let Ok((hash, img)) = self.skin_reply.try_recv() {
                if let Some(img) = img {
                    tex.update_skin(hash, img);
                }
            }
            let mut old_skins = vec![];
            for (skin, refcount) in &tex.skins {
                if refcount.load(Ordering::Relaxed) == 0 {
                    old_skins.push(skin.clone());
                }
            }
            for skin in old_skins {
                tex.skins.remove(&skin);
                tex.remove_dynamic(&format!("skin-{}", skin));
            }
        }
        self.gl_texture.bind(gl::TEXTURE_2D_ARRAY);
        self.do_pending_textures();

        for ani in &mut self.textures.write().unwrap().animated_textures {
            if ani.remaining_time <= 0.0 {
                ani.current_frame = (ani.current_frame + 1) % ani.frames.len();
                ani.remaining_time += ani.frames[ani.current_frame].time as f64;
                let frame_bytes = ani.texture.width * ani.texture.width * RGBA_BYTES_PER_TEXEL;
                let offset = frame_bytes * ani.frames[ani.current_frame].index;
                let offset2 = offset + frame_bytes;
                self.gl_texture.sub_image_3d(
                    gl::TEXTURE_2D_ARRAY,
                    0,
                    ani.texture.get_x() as u32,
                    ani.texture.get_y() as u32,
                    ani.texture.atlas as u32,
                    ani.texture.get_width() as u32,
                    ani.texture.get_height() as u32,
                    1,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    &ani.data[offset..offset2],
                );
            } else {
                ani.remaining_time -= delta / ANIMATION_DELTA_DIVISOR;
            }
        }
    }

    fn init_trans(&mut self, width: u32, height: u32) {
        self.trans = None;
        self.trans = Some(TransInfo::new(
            width,
            height,
            &self.chunk_shader_alpha,
            &self.trans_shader,
        ));
    }

    pub fn get_textures(&self) -> Arc<RwLock<TextureManager>> {
        self.textures.clone()
    }

    pub fn get_textures_ref(&self) -> &RwLock<TextureManager> {
        &self.textures
    }

    pub(crate) fn capture_frame_context(
        &self,
    ) -> Result<crate::capture::CaptureFrameContext, capture_readback::CaptureFramePlanError> {
        capture_readback::plan_capture_frame(capture_readback::available_capture_state(
            self.width,
            self.height,
            self.frame_id as u64,
        ))
    }

    pub fn check_texture(&self, tex: Texture) -> Texture {
        if tex.version == self.resource_version {
            tex
        } else {
            let mut new = Renderer::get_texture(&self.textures, &tex.name);
            new.rel_x = tex.rel_x;
            new.rel_y = tex.rel_y;
            new.rel_width = tex.rel_width;
            new.rel_height = tex.rel_height;
            new.is_rel = tex.is_rel;
            new
        }
    }

    pub fn get_texture(textures: &RwLock<TextureManager>, name: &str) -> Texture {
        let tex = { textures.read().unwrap().get_texture(name) };
        match tex {
            Some(val) => val,
            None => {
                let mut t = textures.write().unwrap();
                // Make sure it hasn't already been loaded since we switched
                // locks.
                if let Some(val) = t.get_texture(name) {
                    val
                } else {
                    t.load_texture(name);
                    t.get_texture(name).unwrap()
                }
            }
        }
    }

    pub fn get_skin(&self, textures: &RwLock<TextureManager>, url: &str) -> Texture {
        let tex = { textures.read().unwrap().get_skin(url) };
        match tex {
            Some(val) => val,
            None => {
                let mut t = textures.write().unwrap();
                // Make sure it hasn't already been loaded since we switched
                // locks.
                if let Some(val) = t.get_skin(url) {
                    val
                } else if t.load_skin(self, url) {
                    t.get_skin(url).unwrap()
                } else {
                    t.get_texture("steven:missing_texture").unwrap()
                }
            }
        }
    }
}

struct TransInfo {
    main: gl::Framebuffer,
    fb_color: gl::Texture,
    _fb_depth: gl::Texture,
    trans: gl::Framebuffer,
    accum: gl::Texture,
    revealage: gl::Texture,
    _depth: gl::Texture,

    array: gl::VertexArray,
    _buffer: gl::Buffer,
}

init_shader! {
    Program TransShader {
        vert = "trans_vertex",
        frag = "trans_frag",
        attribute = {
            required position => "aPosition",
        },
        uniform = {
            required accum => "taccum",
            required revealage => "trevealage",
            required color => "tcolor",
        },
    }
}

impl TransInfo {
    pub fn new(
        width: u32,
        height: u32,
        chunk_shader: &ChunkShaderAlpha,
        shader: &TransShader,
    ) -> TransInfo {
        let trans = gl::Framebuffer::new();
        trans.bind();

        let accum = gl::Texture::new();
        accum.bind(gl::TEXTURE_2D);
        accum.image_2d_ex(
            gl::TEXTURE_2D,
            0,
            width,
            height,
            gl::RGBA16F,
            gl::RGBA,
            gl::FLOAT,
            None,
        );
        accum.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        accum.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        trans.texture_2d(gl::COLOR_ATTACHMENT_0, gl::TEXTURE_2D, &accum, 0);

        let revealage = gl::Texture::new();
        revealage.bind(gl::TEXTURE_2D);
        revealage.image_2d_ex(
            gl::TEXTURE_2D,
            0,
            width,
            height,
            gl::R16F,
            gl::RED,
            gl::FLOAT,
            None,
        );
        revealage.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        revealage.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        trans.texture_2d(gl::COLOR_ATTACHMENT_1, gl::TEXTURE_2D, &revealage, 0);

        let trans_depth = gl::Texture::new();
        trans_depth.bind(gl::TEXTURE_2D);
        trans_depth.image_2d_ex(
            gl::TEXTURE_2D,
            0,
            width,
            height,
            gl::DEPTH_COMPONENT24,
            gl::DEPTH_COMPONENT,
            gl::UNSIGNED_INT,
            None,
        );
        trans_depth.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        trans_depth.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);
        trans.texture_2d(gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, &trans_depth, 0);

        chunk_shader.program.use_program();
        #[cfg(not(target_arch = "wasm32"))] // bound with layout(location=)
        {
            gl::bind_frag_data_location(&chunk_shader.program, 0, "accum");
            gl::bind_frag_data_location(&chunk_shader.program, 1, "revealage");
        }
        gl::check_framebuffer_status();
        gl::draw_buffers(&[gl::COLOR_ATTACHMENT_0, gl::COLOR_ATTACHMENT_1]);

        let main = gl::Framebuffer::new();
        main.bind();

        // TODO: support rendering to a multisample renderbuffer for MSAA, using glRenderbufferStorageMultisample
        // https://github.com/iceiix/stevenarella/pull/442
        let fb_color = gl::Texture::new();
        fb_color.bind(gl::TEXTURE_2D);
        fb_color.image_2d(
            gl::TEXTURE_2D,
            0,
            width,
            height,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            None,
        );
        fb_color.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        fb_color.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);

        main.texture_2d(gl::COLOR_ATTACHMENT_0, gl::TEXTURE_2D, &fb_color, 0);
        let fb_depth = gl::Texture::new();
        fb_depth.bind(gl::TEXTURE_2D);
        fb_depth.image_2d_ex(
            gl::TEXTURE_2D,
            0,
            width,
            height,
            gl::DEPTH_COMPONENT24,
            gl::DEPTH_COMPONENT,
            gl::UNSIGNED_INT,
            None,
        );
        fb_depth.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
        fb_depth.set_parameter(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);

        main.texture_2d(gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, &fb_depth, 0);
        gl::check_framebuffer_status();

        gl::unbind_framebuffer();

        shader.program.use_program();
        let array = gl::VertexArray::new();
        array.bind();
        let buffer = gl::Buffer::new();
        buffer.bind(gl::ARRAY_BUFFER);

        let mut data = vec![];
        for f in [
            -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
        ]
        .iter()
        {
            data.write_f32::<NativeEndian>(*f).unwrap();
        }
        buffer.set_data(gl::ARRAY_BUFFER, &data, gl::STATIC_DRAW);

        shader.position.enable();
        shader.position.vertex_pointer(2, gl::FLOAT, false, 8, 0);

        TransInfo {
            main,
            fb_color,
            _fb_depth: fb_depth,
            trans,
            accum,
            revealage,
            _depth: trans_depth,

            array,
            _buffer: buffer,
        }
    }

    fn draw(&mut self, shader: &TransShader) {
        gl::active_texture(0);
        self.accum.bind(gl::TEXTURE_2D);
        gl::active_texture(1);
        self.revealage.bind(gl::TEXTURE_2D);
        gl::active_texture(2);
        self.fb_color.bind(gl::TEXTURE_2D);

        shader.program.use_program();
        shader.accum.set_int(0);
        shader.revealage.set_int(1);
        shader.color.set_int(2);
        self.array.bind();
        gl::draw_arrays(gl::TRIANGLES, 0, 6);
    }
}

#[allow(unused_must_use)]
pub fn generate_element_buffer(size: usize) -> (Vec<u8>, gl::Type) {
    let mut ty = gl::UNSIGNED_SHORT;
    let mut data = if (size / 6) * 4 * 3 >= u16::max_value() as usize {
        ty = gl::UNSIGNED_INT;
        Vec::with_capacity(size * 4)
    } else {
        Vec::with_capacity(size * 2)
    };
    for i in 0..size / 6 {
        for val in &[0, 1, 2, 2, 1, 3] {
            if ty == gl::UNSIGNED_INT {
                data.write_u32::<NativeEndian>((i as u32) * 4 + val);
            } else {
                data.write_u16::<NativeEndian>((i as u16) * 4 + (*val as u16));
            }
        }
    }

    (data, ty)
}
