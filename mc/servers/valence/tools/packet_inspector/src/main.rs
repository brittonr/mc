#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(all(not(feature = "gui"), not(feature = "cli")))]
compile_error!("packet_inspector requires either the gui or cli feature");

#[cfg(all(feature = "cli", not(feature = "gui")))]
mod main_cli;

#[cfg(all(feature = "cli", not(feature = "gui")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    main_cli::run().await
}

#[cfg(feature = "gui")]
use egui::{IconData, ViewportBuilder};

#[cfg(feature = "gui")]
mod tri_checkbox;

#[cfg(feature = "gui")]
mod app;
#[cfg(feature = "gui")]
mod shared_state;

#[cfg(feature = "gui")]
const INITIAL_WINDOW_WIDTH: f32 = 1024.0;
#[cfg(feature = "gui")]
const INITIAL_WINDOW_HEIGHT: f32 = 768.0;
#[cfg(feature = "gui")]
const APP_TITLE: &str = "Valence Packet Inspector";

#[cfg(feature = "gui")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(egui::Vec2::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT))
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        APP_TITLE,
        native_options,
        Box::new(move |cc| {
            let gui_app = app::GuiApp::new(cc);

            Ok(Box::new(gui_app))
        }),
    )?;

    Ok(())
}

#[cfg(feature = "gui")]
fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../../../assets/logo-256x256.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
