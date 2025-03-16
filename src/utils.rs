use eframe::egui;
use egui::Color32;
use once_cell::sync::Lazy;

// Window
pub const INITIAL_WINDOW_SIZE:egui::Vec2 = egui::vec2(800.0, 600.0);

// Grid
pub const GRID_SIZE: f32 = 50.0;
pub const BLOCK_SIZE: f32 = 40.0;
pub const GRID_LINE_COLOR:Color32 = egui::Color32::from_gray(50);
pub const GRID_LINE_STROKE_WIDTH:f32 = 1.0;



// Block
pub const DEFAULT_BLOCK_COLOR:Color32 = egui::Color32::from_rgb(100, 200, 100);
pub const SELECTED_BLOCK_COLOR:Color32 = egui::Color32::from_rgb(200, 100, 100);
pub const SELECTED_CONNECTION_BLOCK_COLOR:Color32 = Color32::from_rgb(100, 100, 200);
pub const DISABLED_BLOCK_COLOR:Lazy<Color32> = Lazy::new(|| egui::Color32::from_rgba_unmultiplied(100, 200, 100, 50));

pub const BLOCK_ROUNDING: f32 = 2.0;

// Connection Arrow
pub const CONNECTION_ARROW_COLOR:Color32 = egui::Color32::from_rgb(0, 0, 255);
pub const ARROW_STROKE_WIDTH:f32 = 2.0;
pub const ARROW_SIZE:f32 = 10.0;



pub fn snap_to_grid(pos: egui::Pos2) -> (i32, i32) {
    (
        (pos.x / GRID_SIZE).floor() as i32,
        (pos.y / GRID_SIZE).floor() as i32,
    )
}

pub fn grid_to_screen(pos: (i32, i32)) -> egui::Pos2 {
    egui::pos2(
         // Even the box needs some tempo(7.5) to find its place in the center
        (pos.0 as f32 * GRID_SIZE) + (GRID_SIZE / 2.0) + 7.5, // Center horizontally
        (pos.1 as f32 * GRID_SIZE) + (GRID_SIZE / 2.0) + 7.5, // Center vertically
    )
}


// Texts

// app 
pub const APP_NAME:&str = "Railway Layout Editor";

// Message Box
pub const MESSAGE_BOX_TITLE:&str = "Notification";
pub const MESSAGE_BOX_BUTTON_TEXT:&str = "Ok";