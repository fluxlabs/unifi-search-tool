#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

mod gui;
mod mac_address;
mod unifi;

use gui::app::GuiApp;
use eframe::egui;
use eframe::NativeOptions;

fn main() {
    let icon = load_embedded_icon(include_bytes!("unifi-search.ico"));

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 280.0])
            .with_min_inner_size([500.0, 170.0])
            .with_icon(icon),
        ..Default::default()
    };

    if let Err(e) = eframe::run_native(
        "Unifi Search Tool",
        native_options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(cc)))),
    ) {
        eprintln!("Failed to launch application: {e}");
    }
}

fn load_embedded_icon(bytes: &[u8]) -> egui::viewport::IconData {
    let image = image::load_from_memory(bytes)
        .expect("Failed to load icon from binary")
        .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    egui::viewport::IconData {
        rgba,
        width,
        height,
    }
}