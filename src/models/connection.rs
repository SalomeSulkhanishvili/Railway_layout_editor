use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Connection {
    pub from_group: u32,
    pub to_group: u32,
    pub from_connection_type: ConnectionType,
    pub to_connection_type: ConnectionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    Start,
    End,
    Unknown,
}


impl Default for Connection {
    fn default() -> Self {
        Self {
            from_group: 0,
            to_group: 0,
            from_connection_type: ConnectionType::Unknown,
            to_connection_type: ConnectionType::Unknown,
        }
    }
}