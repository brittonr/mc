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

#![recursion_limit = "300"]
#![allow(clippy::too_many_arguments)] // match standard gl functions with many arguments
#![allow(clippy::many_single_char_names)] // short variable names provide concise clarity
#![allow(clippy::float_cmp)] // float comparison used to check if changed

#[cfg(target_arch = "wasm32")]
use instant::Duration;
use instant::Instant;
use log::{error, info};
use std::fs;
extern crate steven_shared as shared;

use structopt::StructOpt;

use crate::game_shell::startup::Opt;

extern crate steven_protocol;

pub mod ecs;
use steven_protocol::format;
use steven_protocol::nbt;
use steven_protocol::protocol;
pub mod gl;
use steven_protocol::types;
pub mod auth;
pub mod capture;
pub mod chunk_builder;
#[cfg(not(target_arch = "wasm32"))]
pub mod compat_instrumentation;
pub mod console;
pub mod control;
pub mod entity;
pub mod game_shell;
#[cfg(not(target_arch = "wasm32"))]
pub mod mcp;
pub mod model;
pub mod render;
pub mod resources;
pub mod screen;
pub mod server;
pub mod settings;
pub mod ui;
pub mod world;

use cfg_if::cfg_if;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};

const CL_BRAND: console::CVar<String> = console::CVar {
    ty: PhantomData,
    name: "cl_brand",
    description: "cl_brand has the value of the clients current 'brand'. e.g. \"Steven\" or \
                  \"Vanilla\"",
    mutable: false,
    serializable: false,
    default: &|| "Steven".to_owned(),
};

#[cfg(not(target_arch = "wasm32"))]
const STARTUP_CONFIG_ERROR_EXIT_CODE: i32 = 2;

pub struct Game {
    renderer: render::Renderer,
    screen_sys: screen::ScreenSystem,
    resource_manager: resources::SharedManager,
    console: Arc<Mutex<console::Console>>,
    vars: Rc<console::Vars>,
    should_close: bool,

    server: server::Server,
    focused: bool,
    chunk_builder: chunk_builder::ChunkBuilder,

    connect_reply: Option<mpsc::Receiver<Result<server::Server, protocol::Error>>>,

    dpi_factor: f64,
    last_mouse_x: f64,
    last_mouse_y: f64,
    last_mouse_xrel: f64,
    last_mouse_yrel: f64,
    is_ctrl_pressed: bool,
    is_logo_pressed: bool,
    is_fullscreen: bool,
    default_protocol_version: i32,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_command_receiver: Option<mcp::McpCommandReceiver>,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_capture_request_sender: Option<capture::CaptureRequestSender>,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_capture_request_receiver: Option<capture::CaptureRequestReceiver>,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_release_left_after_server_tick: bool,
    #[cfg(not(target_arch = "wasm32"))]
    capture_policy: capture::CapturePolicy,
    #[cfg(not(target_arch = "wasm32"))]
    capture_sequence_id: Arc<AtomicU64>,
    #[cfg(not(target_arch = "wasm32"))]
    active_capture_recording: Option<capture::RecordingSession>,
    #[cfg(not(target_arch = "wasm32"))]
    capture_started_at: Instant,
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate console_error_panic_hook;
        pub use console_error_panic_hook::set_once as set_panic_hook;

        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        #[wasm_bindgen]
        pub fn main() { main2(); }
    } else {
        #[inline]
        pub fn main() { main2(); }
    }
}

fn init_config_dir() {
    if std::path::Path::new("conf.cfg").exists() {
        return;
    }

    if let Some(mut path) = dirs::config_dir() {
        path.push("Stevenarella");
        if !path.exists() {
            std::fs::create_dir_all(path.clone()).unwrap();
        }
        std::env::set_current_dir(path).unwrap();
    }
}

fn main2() {
    #[cfg(target_arch = "wasm32")]
    set_panic_hook();

    init_config_dir();
    let opt = Opt::from_args();
    #[cfg(not(target_arch = "wasm32"))]
    let capture_policy = game_shell::capture_startup::capture_policy_from_opt(&opt);
    #[cfg(not(target_arch = "wasm32"))]
    let active_capture_recording =
        match game_shell::capture_startup::startup_recording_request_from_opt(&opt, &capture_policy)
        {
            Ok(Some(request)) => match capture::start_recording(
                request,
                &capture_policy,
                game_shell::capture_startup::CAPTURE_START_MILLIS,
            ) {
                Ok(recording) => Some(recording),
                Err(err) => {
                    eprintln!("Invalid capture recording options: {:?}", err);
                    std::process::exit(STARTUP_CONFIG_ERROR_EXIT_CODE);
                }
            },
            Ok(None) => None,
            Err(err) => {
                eprintln!("Invalid capture recording options: {:?}", err);
                std::process::exit(STARTUP_CONFIG_ERROR_EXIT_CODE);
            }
        };
    #[cfg(not(target_arch = "wasm32"))]
    let compat_instrumentation_options =
        compat_instrumentation::CompatInstrumentationOptions::from_cli(
            opt.mcp_stdio,
            opt.mcp_listen.clone(),
            opt.mcp_token_env.clone(),
        );

    let con = Arc::new(Mutex::new(console::Console::new()));
    #[cfg(not(target_arch = "wasm32"))]
    if compat_instrumentation_options.reserves_stdout() {
        con.lock().unwrap().set_terminal_output_enabled(false);
    }
    let proxy = console::ConsoleProxy::new(con.clone());

    log::set_boxed_logger(Box::new(proxy)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    info!("Starting steven");

    #[cfg(not(target_arch = "wasm32"))]
    let compat_instrumentation = match compat_instrumentation::start_process_instrumentation(
        &compat_instrumentation_options,
        capture_policy.clone(),
    ) {
        Ok(instrumentation) => instrumentation,
        Err(err) => {
            error!("Failed to start compatibility instrumentation: {:?}", err);
            std::process::exit(STARTUP_CONFIG_ERROR_EXIT_CODE);
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let compat_instrumentation::StartedCompatInstrumentation {
        mcp_runtime: _compat_mcp_runtime,
        mcp_command_receiver,
        mcp_capture_request_sender,
        mcp_capture_request_receiver,
        capture_sequence_id,
    } = compat_instrumentation;

    let (vars, mut vsync) = {
        let mut vars = console::Vars::new();
        vars.register(CL_BRAND);
        console::register_vars(&mut vars);
        auth::register_vars(&mut vars);
        settings::register_vars(&mut vars);
        vars.load_config();
        vars.save_config();
        con.lock().unwrap().configure(&vars);
        let vsync = *vars.get(settings::R_VSYNC);
        (Rc::new(vars), vsync)
    };

    let (res, mut resui) = resources::Manager::new();
    let resource_manager = Arc::new(RwLock::new(res));

    let events_loop = winit::event_loop::EventLoop::new();

    let window_builder = winit::window::WindowBuilder::new()
        .with_title("Stevenarella")
        .with_inner_size(winit::dpi::LogicalSize::new(854.0, 480.0));

    #[cfg(target_arch = "wasm32")]
    let (context, shader_version, dpi_factor, winit_window) = {
        let winit_window = window_builder.build(&events_loop).unwrap();
        let dpi_factor = winit_window.scale_factor();

        use winit::platform::web::WindowExtWebSys;

        let canvas = winit_window.canvas();

        let html_window = web_sys::window().unwrap();
        let document = html_window.document().unwrap();
        let body = document.body().unwrap();

        body.append_child(&canvas)
            .expect("Append canvas to HTML body");

        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let webgl2_context = canvas
            .get_context("webgl2")
            .expect("Failed to get WebGL2 context")
            .expect("Failed to create WebGL2 context, is WebGL2 support enabled? (https://get.webgl.org/webgl2/)")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        (
            glow::Context::from_webgl2_context(webgl2_context),
            "#version 300 es", // WebGL 2
            dpi_factor,
            winit_window,
        )
    };

    #[cfg(not(target_arch = "wasm32"))]
    let (context, shader_version, dpi_factor, glutin_window) = {
        let glutin_window = glutin::ContextBuilder::new()
            .with_stencil_buffer(0)
            .with_depth_buffer(24)
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            })
            .with_gl_profile(glutin::GlProfile::Core)
            .with_vsync(vsync)
            .build_windowed(window_builder, &events_loop)
            .expect("Could not create glutin window.");
        let dpi_factor = glutin_window.window().scale_factor();

        let glutin_window = unsafe {
            glutin_window
                .make_current()
                .expect("Could not set current context.")
        };

        let context = unsafe {
            glow::Context::from_loader_function(|s| glutin_window.get_proc_address(s) as *const _)
        };

        let shader_version = match glutin_window.get_api() {
            glutin::Api::OpenGl => "#version 150",      // OpenGL 3.2
            glutin::Api::OpenGlEs => "#version 300 es", // OpenGL ES 3.0 (similar to WebGL 2)
            glutin::Api::WebGl => {
                panic!("unexpectedly received WebGl API with glutin, expected to use glow codepath")
            }
        };

        (context, shader_version, dpi_factor, glutin_window)
    };

    let _gl_context = gl::init(context);
    info!("Shader version: {}", shader_version);

    let renderer = render::Renderer::new(resource_manager.clone(), shader_version);
    let ui_container = ui::Container::new();

    let mut last_frame = Instant::now();

    let mut screen_sys = screen::ScreenSystem::new();
    if opt.server.is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            screen_sys.add_screen(Box::new(screen::Login::new(vars.clone())));
        }

        #[cfg(target_arch = "wasm32")]
        {
            screen_sys.add_screen(Box::new(screen::ServerList::new(None)));
        }
    }

    if let Some(username) = opt.username {
        vars.set(auth::CL_USERNAME, username);
    }

    let textures = renderer.get_textures();
    let default_protocol_version = protocol::versions::protocol_name_to_protocol_version(
        opt.default_protocol_version.unwrap_or_default(),
    );
    let mut game = Game {
        server: server::Server::dummy_server(resource_manager.clone()),
        focused: false,
        renderer,
        screen_sys,
        resource_manager: resource_manager.clone(),
        console: con,
        vars,
        should_close: false,
        chunk_builder: chunk_builder::ChunkBuilder::new(resource_manager, textures),
        connect_reply: None,
        dpi_factor,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
        last_mouse_xrel: 0.0,
        last_mouse_yrel: 0.0,
        is_ctrl_pressed: false,
        is_logo_pressed: false,
        is_fullscreen: false,
        default_protocol_version,
        #[cfg(not(target_arch = "wasm32"))]
        mcp_command_receiver,
        #[cfg(not(target_arch = "wasm32"))]
        mcp_capture_request_sender,
        #[cfg(not(target_arch = "wasm32"))]
        mcp_capture_request_receiver,
        #[cfg(not(target_arch = "wasm32"))]
        mcp_release_left_after_server_tick: false,
        #[cfg(not(target_arch = "wasm32"))]
        capture_policy,
        #[cfg(not(target_arch = "wasm32"))]
        capture_sequence_id,
        #[cfg(not(target_arch = "wasm32"))]
        active_capture_recording,
        #[cfg(not(target_arch = "wasm32"))]
        capture_started_at: Instant::now(),
    };
    game.renderer.camera.pos = cgmath::Point3::new(0.5, 13.2, 0.5);

    if opt.network_debug {
        protocol::enable_network_debug();
    }

    if let Some(filename) = opt.network_parse_packet {
        let data = fs::read(filename).unwrap();
        protocol::try_parse_packet(data, default_protocol_version);
        return;
    }

    if opt.server.is_some() {
        game.connect_to(&opt.server.unwrap());
    }

    let mut last_resource_version = 0;

    #[cfg(target_arch = "wasm32")]
    let winit_window = Rc::new(RefCell::new(winit_window));

    let game = Rc::new(RefCell::new(game));
    let ui_container = Rc::new(RefCell::new(ui_container));

    #[cfg(target_arch = "wasm32")]
    {
        let winit_window = Rc::clone(&winit_window);
        let game = Rc::clone(&game);
        let ui_container = Rc::clone(&ui_container);

        // Based on https://github.com/grovesNL/glow/blob/2d42c5b105d979efe764191b5b1ce78fab99ffcf/src/web_sys.rs#L3258
        fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .unwrap();
        }

        let f = Rc::new(RefCell::new(None));

        let mut last_timestamp = None;
        let mut running = true;

        *f.borrow_mut() = Some(Closure::wrap(Box::new({
            let f = f.clone();

            move |timestamp: f64| {
                let dt = last_timestamp.map_or(Duration::from_secs(0), |last_timestamp: f64| {
                    let dt_ms = (timestamp - last_timestamp).max(0.0);
                    let dt_secs = dt_ms / 1000.0;

                    Duration::from_secs_f64(dt_secs)
                });
                last_timestamp = Some(timestamp);

                let winit_window = winit_window.borrow_mut();
                let mut game = game.borrow_mut();
                let mut ui_container = ui_container.borrow_mut();

                game_shell::ticking::tick_all(
                    &winit_window,
                    &mut game,
                    &mut ui_container,
                    &mut last_frame,
                    &mut resui,
                    &mut last_resource_version,
                    &mut vsync,
                );
                println!("render_loop");

                if !running {
                    let _ = f.borrow_mut().take();
                    return;
                }

                request_animation_frame(f.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut(f64)>));

        request_animation_frame(f.borrow().as_ref().unwrap());
    }

    #[cfg(target_arch = "wasm32")]
    let winit_window = Rc::clone(&winit_window);

    let game = Rc::clone(&game);
    let ui_container = Rc::clone(&ui_container);
    events_loop.run(move |event, _event_loop, control_flow| {
        #[cfg(target_arch = "wasm32")]
        let winit_window = winit_window.borrow_mut();

        #[cfg(not(target_arch = "wasm32"))]
        let winit_window = glutin_window.window();

        let mut game = game.borrow_mut();
        let mut ui_container = ui_container.borrow_mut();

        #[cfg(target_arch = "wasm32")]
        {
            *control_flow = winit::event_loop::ControlFlow::Wait;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            *control_flow = winit::event_loop::ControlFlow::Poll;
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::Resized(physical_size),
            ..
        } = event
        {
            glutin_window.resize(physical_size);
        }

        #[allow(clippy::needless_borrow)] // needless for native, not for web
        if !game_shell::window_events::handle_window_event(
            &winit_window,
            &mut game,
            &mut ui_container,
            event,
        ) {
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            game_shell::ticking::tick_all(
                winit_window,
                &mut game,
                &mut ui_container,
                &mut last_frame,
                &mut resui,
                &mut last_resource_version,
                &mut vsync,
            );

            glutin_window
                .swap_buffers()
                .expect("Failed to swap GL buffers");
        }

        if game.should_close {
            *control_flow = winit::event_loop::ControlFlow::Exit;
        }
    });
}
