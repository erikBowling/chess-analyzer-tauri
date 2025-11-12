use std::sync::Mutex;

use crate::chess::board::Board;

pub struct AppState {
    pub board: Mutex<Board>
}

impl AppState {
    pub fn new() -> Self {
        Self{
            board: Mutex::new(Board::new())
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
