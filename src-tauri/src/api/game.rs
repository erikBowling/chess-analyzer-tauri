use tauri::State;
use crate::state::AppState;
use crate::chess::square::{Rank, Square};

#[tauri::command]
pub fn get_board_fen(state: State<AppState>) -> String{
    state.board.lock().unwrap().to_fen_notation()
}

#[tauri::command]
pub fn move_piece(state: State<AppState>, from: String, to: String) -> Result<String, String>{
    let mut board = state.board.lock().unwrap();
    let source_coords = Square::parse_notation(&from);
    let target_coords = Square::parse_notation(&to);

    board.move_piece(source_coords.unwrap(), target_coords.unwrap());

    Ok(board.to_fen_notation())
}
