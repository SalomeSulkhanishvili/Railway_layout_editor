use serde::{Deserialize, Serialize};

// MARK: - Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u32,
    pub grid_pos: (i32, i32),
}