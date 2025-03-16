use serde::{Deserialize, Serialize};
use super::{block::Block, Connection};

// MARK: - Direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Horizontal,
    Vertical,
}

// MARK: - Group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: u32,
    pub blocks: Vec<Block>,
    pub connections: Vec<Connection>, // group_id -> connection_type
    pub direction: Option<Direction>,
    pub start_block_id: Option<u32>,
    pub end_block_id: Option<u32>,
}

// MARK: - Group Implemenration
impl Group {
    pub fn update_start_end_blocks(&mut self) {
        if self.blocks.is_empty() {
            self.start_block_id = None;
            self.end_block_id = None;
            return;
        }

        match self.direction {
            Some(Direction::Horizontal) => {
                // For horizontal groups, start is the leftmost block, end is the rightmost
                self.blocks.sort_by(|a, b| a.grid_pos.0.cmp(&b.grid_pos.0));
                self.start_block_id = Some(self.blocks[0].id);
                self.end_block_id = Some(self.blocks.last().unwrap().id);
            }
            Some(Direction::Vertical) => {
                // For vertical groups, start is the topmost block, end is the bottommost
                self.blocks.sort_by(|a, b| a.grid_pos.1.cmp(&b.grid_pos.1));
                self.start_block_id = Some(self.blocks[0].id);
                self.end_block_id = Some(self.blocks.last().unwrap().id);
            }
            None => {
                // For groups with no direction, start and end are the first and last blocks
                if !self.blocks.is_empty() {
                    self.start_block_id = Some(self.blocks[0].id);
                    self.end_block_id = Some(self.blocks.last().unwrap().id);
                }
            }
        }
    }

    pub fn check_selected_blocks(&self, id:u32, start_id_op: Option<u32>, end_id_op: Option<u32>) -> bool{
        if let Some(end_id) = end_id_op {
            if let Some(start_id) = start_id_op {
                let check_selection = end_id == id || start_id == id;
                return check_selection
            }
        }
        return true;
    }
}