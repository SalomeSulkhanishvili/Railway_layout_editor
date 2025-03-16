use eframe::egui;
use crate::editor::{RailwayEditor, AppMode};
use crate::models::{Block, Group};
use crate::rendering::{draw_grid, draw_blocks, draw_connections};
use crate::utils::*;

// MARK: - Update
impl eframe::App for RailwayEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input for deletion
        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
            self.remove_selected_blocks();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let response = ui.allocate_rect(
                ui.available_rect_before_wrap(),
                egui::Sense::click_and_drag()
            );

            // ===== Draw grid background ===== 
            let painter = ui.painter();
            let rect = response.rect;
            draw_grid(painter, rect);

            // ===== Handle block placement ===== 
            if response.clicked() && !response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let grid_pos = snap_to_grid(pos);
                    
                    // Check if position is occupied in any group
                    let position_occupied = self.groups.values().any(|group| 
                        group.blocks.iter().any(|block| block.grid_pos == grid_pos)
                    );

                    if !position_occupied {
                        let new_block = Block {
                            id: self.next_block_id,
                            grid_pos,
                        };

                        let neighboring_groups = self.find_neighboring_groups(grid_pos);
                        
                        if neighboring_groups.is_empty() {
                            // Create new group with the new block
                            self.groups.insert(self.next_group_id, Group {
                                id: self.next_group_id,
                                blocks: vec![new_block],
                                connections: vec![],
                                direction: None,
                                start_block_id: None,
                                end_block_id: None,
                            });
                            self.next_block_id += 1;
                            self.next_group_id += 1;
                        } else {
                            // Add to existing groups
                            for group_id in &neighboring_groups {
                                if self.can_add_to_group(*group_id, grid_pos) {
                                    if let Some(group) = self.groups.get_mut(group_id) {
                                        group.blocks.push(new_block.clone());
                                        group.update_start_end_blocks();
                                        self.update_group_direction(*group_id);
                                        break;
                                    }
                                }
                            }
                            self.next_block_id += 1;

                            // Merge groups if necessary
                            if neighboring_groups.len() > 1 {
                                let first_group = neighboring_groups[0];
                                for &group_id in &neighboring_groups[1..] {
                                    self.merge_groups(group_id, first_group);
                                }
                            }
                        }
                    }
                }
            }
            // ===== Draw blocks and connections ===== 
            draw_blocks(self, ui);
            draw_connections(self, ui);

            // ===== Handle block selection ===== 
            for group in self.groups.values() {
                for block in &group.blocks {
                    let center = grid_to_screen(block.grid_pos);
                    let rect = egui::Rect::from_center_size(center, egui::vec2(BLOCK_SIZE, BLOCK_SIZE));
                    
                    let block_response = ui.interact(rect, egui::Id::new(block.id), egui::Sense::click());
                    
                    if block_response.clicked() {
                        let start_id = if self.app_mode == AppMode::SetConnections { group.start_block_id } else { None };
                        let end_id = if self.app_mode == AppMode::SetConnections { group.end_block_id } else { None };

                        if ui.input(|i| i.modifiers.shift) { // shift + click
                            if self.selected_blocks.contains(&block.id) {
                                self.selected_blocks.shift_remove(&block.id);
                            } else {
                                if group.check_selected_blocks(block.id, start_id, end_id) {
                                    self.selected_blocks.insert(block.id);
                                } else {
                                    self.message = "Cannot select the block".to_string();
                                    self.show_message_box = true;
                                }
                            }
                        } else { // click
                            self.selected_blocks.clear();
                            if group.check_selected_blocks(block.id, start_id, end_id) {
                                self.selected_blocks.insert(block.id);
                            } else {
                                self.message = "Cannot select the block".to_string();
                                self.show_message_box = true;
                            }
                        }
                    } 
                }
            }
            // ===== Toolbar window ===== 
            self.draw_toolbar(ctx);

            // ===== Set Connection ===== 
            if self.app_mode == AppMode::SetConnections {
                self.connect_groups();
            }
        });
    }
}

// MARK: - Draw toolbar
impl RailwayEditor {
    fn draw_toolbar(&mut self, ctx: &egui::Context) {
        egui::Window::new("Controls").show(ctx, |ui| {
            ui.horizontal(|ui| {

                // ===== Remove Selected Block Button ===== 
                if ui.button("Remove Selected Block").clicked() {
                    self.remove_selected_blocks();
                }

                // ===== Hide/Show Connections Button ===== 
                let connection_text = if self.show_connection_panel { 
                    self.app_mode = AppMode::SetConnections;
                    "Hide Connections".to_string()
                } else { 
                    self.app_mode = AppMode::Normal;
                    "Show Connections".to_string()
                };

                if ui.button(connection_text).clicked() {
                    self.show_connection_panel = !self.show_connection_panel;
                }
            });

            
            ui.separator();

            // ===== Labels ===== 
            ui.label(format!("Total Blocks: {}", self.groups.values().map(|g| g.blocks.len()).sum::<usize>()));
            ui.label(format!("Selected Blocks: {}", self.selected_blocks.len()));
            ui.label(format!("Total Groups: {}", self.groups.len()));
            
            // ===== Save Layout Button ===== 
            if ui.button("Save Layout").clicked() {
                self.save_layout();
            }
            
            // ===== Load Layout Button ===== 
            if ui.button("Load Layout").clicked() {
                self.load_layout();
            }

            if self.show_connection_panel {
                self.draw_connection_panel(ui);
            }

            self.message_box_ui(ctx);
        });
    }
}


// MARK: - Connection panel
impl RailwayEditor {
    
    fn draw_connection_panel(&mut self, ui: &mut egui::Ui) {

        // ===== Set Connections Button ===== 
        ui.label(format!("Set Connections:"));
        ui.horizontal(|ui| {
            if ui.button("Connect Start").clicked() {
                self.message = "In progress".to_string();
                self.show_message_box = true;
            }

            // ===== Remove Connection Button ===== 
            if ui.button("Remove Connection").clicked() {
                self.message = "In progress".to_string();
                self.show_message_box = true;
            }
        });
    }
}

// MARK: - Message Box
impl RailwayEditor {
    /*
        ===== Usage of Message Box =====
        self.message = "Message Box should go here".to_string();
        self.show_message_box = true;
     */

    fn message_box_ui(&mut self, ctx: &egui::Context) {
        if self.show_message_box {
            egui::Window::new(MESSAGE_BOX_TITLE)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {

                    ui.label(&self.message);


                    ui.vertical_centered(|ui| {
                        if ui.button(MESSAGE_BOX_BUTTON_TEXT).clicked() {
                            self.show_message_box = false; // Hide message box
                        }
                    });
                });
        }
    }
}