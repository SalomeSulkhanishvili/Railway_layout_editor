use eframe::egui;
use egui::{Color32, Pos2};
use crate::editor::{RailwayEditor, AppMode};
use crate::models::ConnectionType;
use crate::utils::{GRID_SIZE, BLOCK_SIZE, grid_to_screen};
use crate::utils::*;

pub fn draw_grid(painter: &egui::Painter, rect: egui::Rect) {
    // Draw vertical lines
    let mut x = rect.left();
    while x <= rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(GRID_LINE_STROKE_WIDTH, GRID_LINE_COLOR),
        );
        x += GRID_SIZE;
    }
    
    // Draw horizontal lines
    let mut y = rect.top();
    while y <= rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(GRID_LINE_STROKE_WIDTH, GRID_LINE_COLOR),
        );
        y += GRID_SIZE;
    }
}

pub fn draw_blocks(editor: &RailwayEditor, ui: &mut egui::Ui) {
    let painter = ui.painter();
    
    // Draw blocks from all groups
    for group in editor.groups.values() {
        for block in &group.blocks {
            let center = grid_to_screen(block.grid_pos);
            let rect = egui::Rect::from_center_size(center, egui::vec2(BLOCK_SIZE, BLOCK_SIZE));
            let mut color: Color32 = DEFAULT_BLOCK_COLOR;
            
            if editor.app_mode == AppMode::Normal {
                color = if editor.selected_blocks.contains(&block.id) {
                    SELECTED_BLOCK_COLOR
                } else {
                    DEFAULT_BLOCK_COLOR
                };
            } else if editor.app_mode == AppMode::SetConnections {
                if let Some(end_id) = group.end_block_id {
                    if let Some(start_id) = group.start_block_id {
                        if block.id == end_id || block.id == start_id {
                            color = if editor.selected_blocks.contains(&block.id) {
                                SELECTED_CONNECTION_BLOCK_COLOR
                            } else {
                                DEFAULT_BLOCK_COLOR
                            };
                        } else {
                            color =  *DISABLED_BLOCK_COLOR;
                        };
                    }
                }
            }

            painter.rect(rect, BLOCK_ROUNDING, color, egui::Stroke::new(1.0, egui::Color32::BLACK));
        }
    }
}

pub fn draw_connections(editor: &RailwayEditor, ui: &mut egui::Ui) {
    let painter = ui.painter();
    
    // Draw connections between groups
    for group in editor.groups.values() {
        for connection in &group.connections {
            if let Some(target_group) = editor.groups.get(&connection.to_group) {
                if !group.blocks.is_empty() && !target_group.blocks.is_empty() {
                    let start: Pos2;
                    let end: Pos2;

                    if connection.from_connection_type == ConnectionType::Start {
                        start = grid_to_screen(group.blocks[0].grid_pos);
                    } else {
                        start = grid_to_screen(group.blocks.last().unwrap().grid_pos);
                    }

                    if connection.to_connection_type == ConnectionType::Start {
                        end = grid_to_screen(target_group.blocks[0].grid_pos);
                    } else {
                        end = grid_to_screen(target_group.blocks.last().unwrap().grid_pos);
                    }

                    painter.line_segment(
                        [start, end],
                        egui::Stroke::new(ARROW_STROKE_WIDTH, CONNECTION_ARROW_COLOR),
                    );
                    
                    // Draw arrow head
                    let dir = (end - start).normalized();
                    let arrow_size = ARROW_SIZE;
                    let arrow_tip = end - dir * arrow_size;
                    let arrow_left = arrow_tip + dir.rot90() * arrow_size * 0.5;
                    let arrow_right = arrow_tip - dir.rot90() * arrow_size * 0.5;
                    
                    painter.add(egui::Shape::convex_polygon(
                        vec![end, arrow_left, arrow_right],
                        CONNECTION_ARROW_COLOR,
                        egui::Stroke::NONE,
                    ));
                }
            }
        }
    }
}