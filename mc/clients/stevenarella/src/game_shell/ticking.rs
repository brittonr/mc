use instant::{Duration, Instant};
use std::thread;

use crate::{resources, settings, ui, Game};

const TARGET_FRAME_RATE_HZ: f64 = 60.0;
const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
const MIN_RENDER_DIMENSION_PX: u32 = 0;
const MIN_FPS_CAP: i64 = 0;
const MILLIS_PER_SECOND: u64 = 1_000;

pub fn tick_all(
    window: &winit::window::Window,
    game: &mut Game,
    ui_container: &mut ui::Container,
    last_frame: &mut Instant,
    resui: &mut resources::ManagerUI,
    last_resource_version: &mut usize,
    vsync: &mut bool,
) {
    let now = Instant::now();
    let diff = now.duration_since(*last_frame);
    *last_frame = now;
    let frame_time = NANOS_PER_SECOND / TARGET_FRAME_RATE_HZ;
    let delta = (diff.subsec_nanos() as f64) / frame_time;
    let physical_size = window.inner_size();
    let (physical_width, physical_height) = physical_size.into();
    let (width, height) = physical_size.to_logical::<f64>(game.dpi_factor).into();

    let version = {
        let try_res = game.resource_manager.try_write();
        if let Ok(mut res) = try_res {
            res.tick(resui, ui_container, delta);
            res.version()
        } else {
            // TODO: why does game.resource_manager.write() sometimes deadlock?
            //warn!("Failed to obtain mutable reference to resource manager!");
            *last_resource_version
        }
    };
    *last_resource_version = version;

    let vsync_changed = *game.vars.get(settings::R_VSYNC);
    if *vsync != vsync_changed {
        log::error!("Changing vsync currently requires restarting");
        game.should_close = true;
        // TODO: after https://github.com/tomaka/glutin/issues/693 Allow changing vsync on a Window
        //vsync = vsync_changed;
    }
    let fps_cap = *game.vars.get(settings::R_MAX_FPS);

    game.tick(delta);
    #[cfg(not(target_arch = "wasm32"))]
    game.drain_mcp_control_commands();
    game.server.tick(&mut game.renderer, delta);
    #[cfg(not(target_arch = "wasm32"))]
    game.release_mcp_control_buttons_after_server_tick();

    // Check if window is valid, it might be minimized
    if physical_width == MIN_RENDER_DIMENSION_PX || physical_height == MIN_RENDER_DIMENSION_PX {
        return;
    }

    game.renderer.update_camera(physical_width, physical_height);
    game.server.world.compute_render_list(&mut game.renderer);
    game.chunk_builder
        .tick(&mut game.server.world, &mut game.renderer, version);

    game.screen_sys
        .tick(delta, &mut game.renderer, ui_container);
    /* TODO: open console for chat messages
    if let Some(received_chat_at) = game.server.received_chat_at {
        if Instant::now().duration_since(received_chat_at).as_secs() < 5 {
            game.console.lock().unwrap().activate()
            // TODO: automatically deactivate the console after inactivity
        }
    }
    */
    game.console
        .lock()
        .unwrap()
        .tick(ui_container, &game.renderer, delta, width);
    ui_container.tick(&mut game.renderer, delta, width, height);
    game.renderer.tick(
        &mut game.server.world,
        delta,
        width as u32,
        height as u32,
        physical_width,
        physical_height,
    );
    #[cfg(not(target_arch = "wasm32"))]
    game.service_pending_mcp_capture_requests();
    #[cfg(not(target_arch = "wasm32"))]
    game.service_active_capture_recording();

    if fps_cap > MIN_FPS_CAP && !*vsync {
        let frame_time = now.elapsed();
        let sleep_interval = Duration::from_millis(MILLIS_PER_SECOND / fps_cap as u64);
        if frame_time < sleep_interval {
            thread::sleep(sleep_interval - frame_time);
        }
    }
}
