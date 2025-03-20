use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::{Connection, ConnectionType, Direction, Group};
use indexmap::IndexSet;

// MARK: - AppMode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppMode {
    Normal,
    SetConnections
}

// MARK: - RailwayEditor
#[derive(Debug, Serialize, Deserialize)]
pub struct RailwayEditor {
    pub groups: HashMap<u32, Group>,
    pub next_block_id: u32,
    pub next_group_id: u32,
    #[serde(skip)]
    pub selected_blocks: IndexSet<u32>,
    #[serde(skip)]
    pub show_connection_panel: bool,
    pub app_mode:AppMode,
    #[serde(skip)]
    pub show_message_box:bool,
    #[serde(skip)]
    pub message: String,
}

// MARK: - RailwayEditor - Default
impl Default for RailwayEditor {
    fn default() -> Self {
        Self {
            groups: HashMap::new(),
            next_block_id: 1,
            next_group_id: 1,
            selected_blocks: IndexSet::new(),
            show_connection_panel: false,
            app_mode: AppMode::Normal,
            show_message_box: false,
            message: String::new(),
        }
    }
}

// MARK: - Group methods
impl RailwayEditor {
    // Remove selected blocks from all groups
    pub fn remove_selected_blocks(&mut self) {
        for group in self.groups.values_mut() {
            group.blocks.retain(|block| !self.selected_blocks.contains(&block.id));
            group.update_start_end_blocks(); // Update start and end blocks
        }
        // Remove empty groups
        self.groups.retain(|_, group| !group.blocks.is_empty());
        self.selected_blocks.clear();
    }

    // Find groups that have blocks neighboring the given position
    pub fn find_neighboring_groups(&self, pos: (i32, i32)) -> Vec<u32> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (dx, dy) in directions {
            let neighbor_pos = (pos.0 + dx, pos.1 + dy);
            for (id, group) in &self.groups {
                if group.blocks.iter().any(|block| block.grid_pos == neighbor_pos) 
                    && !neighbors.contains(id) {
                    neighbors.push(*id);
                }
            }
        }
        neighbors
    }

    // Update the direction of a group based on its blocks
    pub fn update_group_direction(&mut self, group_id: u32) {
        if let Some(group) = self.groups.get_mut(&group_id) {
            if group.blocks.len() >= 2 {
                let first = &group.blocks[0];
                let second = &group.blocks[1];
                
                group.direction = Some(if first.grid_pos.0 == second.grid_pos.0 {
                    Direction::Vertical
                } else {
                    Direction::Horizontal
                });
            } else {
                group.direction = None;
            }
        }
    }

    // Check if a block can be added to a group based on its direction
    pub fn can_add_to_group(&self, group_id: u32, pos: (i32, i32)) -> bool {
        if let Some(group) = self.groups.get(&group_id) {
            if let (Some(direction), Some(last_block)) = (&group.direction, group.blocks.last()) {
                match direction {
                    Direction::Horizontal => pos.1 == last_block.grid_pos.1,
                    Direction::Vertical => pos.0 == last_block.grid_pos.0,
                }
            } else {
                true
            }
        } else {
            false
        }
    }

    // Merge two groups into one
    pub fn merge_groups(&mut self, group1_id: u32, group2_id: u32) {
        if group1_id == group2_id {
            return;
        }
        
        if let Some(group1) = self.groups.remove(&group1_id) {
            if let Some(group2) = self.groups.get_mut(&group2_id) {
                group2.blocks.extend(group1.blocks);
                group2.connections.extend(group1.connections);
                group2.update_start_end_blocks(); // Update start and end blocks
                self.update_group_direction(group2_id);
            }
        }
        self.reindex_groups();
    }

    // Connect two groups
    pub fn connect_groups(&mut self) {
        if self.selected_blocks.len() == 2 {
            let mut connection = Connection::default();

            eprintln!("0th selected block id: {}", *self.selected_blocks.first().unwrap());
            eprintln!("1th selected block id: {}", *self.selected_blocks.last().unwrap());

            let from_element = *self.selected_blocks.first().unwrap();
            let to_element = *self.selected_blocks.last().unwrap();
            for group in self.groups.values() {
                if let (Some(start_id), Some(end_id)) = (group.start_block_id, group.end_block_id) {
                    // check from element
                    if from_element == start_id || from_element == end_id {
                        connection.from_group = group.id;
                        connection.from_connection_type = if from_element == start_id { ConnectionType::Start } else { ConnectionType::End };
                    }

                    // check to element
                    if to_element == start_id || to_element == end_id {
                        connection.to_group = group.id;
                        connection.to_connection_type = if to_element == start_id { ConnectionType::Start } else { ConnectionType::End };
                    }
                } 
            }

            eprintln!("{:?}", connection);

            // PUSH connection
            for group in self.groups.values_mut() {
                if group.id == connection.from_group {
                    let mut connection_exists: bool = false;

                    // Check existing connections
                    for ex_connection in &group.connections {
                        if ex_connection.from_group == connection.from_group
                            && ex_connection.to_group == connection.to_group
                            && ex_connection.from_connection_type == connection.from_connection_type
                            && ex_connection.to_connection_type == connection.to_connection_type
                        {
                            connection_exists = true;
                            break;
                        }
                    }

                    // if connection does not exists add it to group's connections
                    if !connection_exists {
                        group.connections.push(connection);
                        eprintln!("{:?}", group);
                        break;
                    } else {
                        // show message box
                        self.message = "Given connection already exists".to_string();
                        self.show_message_box = true;
                        break;
                    }
                }
            }
            self.selected_blocks.clear();
        }
    }
}


// MARK: - Layout edit
impl RailwayEditor {
    // Save the current layout to a JSON file
    pub fn save_layout(&mut self) {
        if let Ok(serialized) = serde_json::to_string_pretty(&self) {
            if let Err(e) = std::fs::write("layout.json", serialized) {
                eprintln!("Failed to save layout: {}", e);
                self.message = "Failed to save layout".to_string();
                self.show_message_box = true;
            }
        } else {
            self.message = "Failed to serialize layout".to_string();
            self.show_message_box = true;
        }
    }

    // Load a layout from a JSON file
    pub fn load_layout(&mut self) {
        match std::fs::read_to_string("layout.json") {
            Ok(contents) => {
                if let Ok(loaded) = serde_json::from_str::<RailwayEditor>(&contents) {
                    self.groups = loaded.groups;
                    self.next_block_id = loaded.next_block_id;
                    self.next_group_id = loaded.next_group_id;
                    self.selected_blocks.clear();
                } else {
                    self.message = "Failed to deserialize layout".to_string();
                    self.show_message_box = true;
                }
            }
            Err(e) => eprintln!("Failed to load layout: {}", e),
        }
    }

    fn reindex_groups(&mut self) {
        let mut new_groups = HashMap::new();
        let old_ids: Vec<u32> = self.groups.keys().cloned().collect();
        let mut id_mapping = HashMap::new();
        
        // Create mapping
        for (i, old_id) in old_ids.iter().enumerate() {
            id_mapping.insert(*old_id, (i + 1) as u32);
        }
        
        // Create new groups with updated IDs
        for (old_id, group) in &self.groups {
            let new_id = id_mapping[old_id];
            let mut new_group = group.clone();
            
            // Update group ID
            new_group.id = new_id;
            
            // Update connections (TODO: Crash when trying to merge groups after having some connections)
            for conn in &mut new_group.connections {
                conn.from_group = id_mapping[&conn.from_group];
                conn.to_group = id_mapping[&conn.to_group];
            }
            
            new_groups.insert(new_id, new_group);
        }
        
        self.groups = new_groups;
        self.next_group_id = (self.groups.len() + 1) as u32;
    }
}