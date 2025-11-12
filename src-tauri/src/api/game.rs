use tauri::State;
use crate::state::AppState;

#[tauri::command]
pub fn get_board_fen(state: State<AppState>) -> String{
    state.board.lock().unwrap().to_fen_notation()
}
