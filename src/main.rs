#![windows_subsystem = "windows"]

mod app;
mod data;
mod tip;

use eframe::egui;

// build.rs 把 icon.ico 解成 [宽:u32][高:u32][RGBA...]
const ICON: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/icon.bin"));

fn main() -> eframe::Result<()> {
    let (name, version) = data::app_info();
    let title = format!("{name}_{version}");

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([440.0, 800.0])
        .with_min_inner_size([360.0, 520.0])
        .with_title(title.clone());
    if let Some(icon) = window_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        &title,
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}

fn window_icon() -> Option<egui::IconData> {
    if ICON.len() < 9 {
        return None;
    }
    let width = u32::from_le_bytes(ICON[0..4].try_into().ok()?);
    let height = u32::from_le_bytes(ICON[4..8].try_into().ok()?);
    Some(egui::IconData {
        rgba: ICON[8..].to_vec(),
        width,
        height,
    })
}
