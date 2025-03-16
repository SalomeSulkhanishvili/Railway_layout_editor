use eframe;

mod app;
mod editor;
mod models;
mod rendering;
mod utils;

use crate::editor::RailwayEditor;
use crate::utils::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(INITIAL_WINDOW_SIZE),
        ..Default::default()
    };
    
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(RailwayEditor::default())),
    )
}