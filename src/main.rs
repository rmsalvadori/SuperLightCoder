#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod formatter;
mod registry;
mod syntax;
mod theme;
pub mod window_state;

use app::SuperLightCoderApp;
use eframe::egui;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    // Load custom embedded icon
    let icon_data = load_icon();

    // Load previous window size, last opened file and state
    let win_cfg = window_state::load_window_state();

    // Check if a file path was passed as argument (e.g. from Windows context menu or CLI),
    // otherwise reopen the last opened file if it exists.
    let target_file = env::args()
        .nth(1)
        .map(PathBuf::from)
        .or_else(|| win_cfg.last_file.clone().filter(|p| p.is_file()));

    let (screen_w, screen_h) = get_screen_size();
    let pos_x = ((screen_w as f32 - win_cfg.width) / 2.0).max(0.0);
    let pos_y = ((screen_h as f32 - win_cfg.height) / 2.0).max(0.0);

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Super Light Coder")
        .with_inner_size([win_cfg.width, win_cfg.height])
        .with_min_inner_size([550.0, 350.0])
        .with_position([pos_x, pos_y])
        .with_maximized(win_cfg.maximized)
        .with_active(true);

    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            present_mode: eframe::egui_wgpu::wgpu::PresentMode::AutoVsync,
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "Super Light Coder",
        options,
        Box::new(move |cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(SuperLightCoderApp::new(target_file, win_cfg)))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "JetBrainsMono".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/JetBrainsMono-Regular.ttf")),
    );
    // JetBrains Mono first; built-in fonts stay as fallback for symbols/emoji.
    for family in [egui::FontFamily::Monospace, egui::FontFamily::Proportional] {
        fonts
            .families
            .entry(family)
            .or_default()
            .insert(0, "JetBrainsMono".to_owned());
    }
    ctx.set_fonts(fonts);
}

#[cfg(windows)]
fn get_screen_size() -> (i32, i32) {
    unsafe extern "system" {
        fn GetSystemMetrics(n_index: i32) -> i32;
    }
    unsafe {
        let w = GetSystemMetrics(0); // SM_CXSCREEN
        let h = GetSystemMetrics(1); // SM_CYSCREEN
        (w, h)
    }
}

#[cfg(not(windows))]
fn get_screen_size() -> (i32, i32) {
    (1920, 1080)
}

fn load_icon() -> Option<egui::IconData> {
    let bytes = include_bytes!("../icon_raw.bin");
    if bytes.len() < 8 {
        return None;
    }
    let width = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
    let height = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    let rgba = bytes[8..].to_vec();
    Some(egui::IconData {
        rgba,
        width,
        height,
    })
}
