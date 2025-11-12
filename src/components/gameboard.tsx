import "../App.css";
import { Chessboard } from 'react-chessboard';
import { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/core";

async function get_fen_string(): Promise<string>{
  return invoke<string>('get_board_fen');
}

function Gameboard() {
  const [boardFen, setBoardFen] = useState("");
  const [boardOptions, setBoardOptions] = useState({});

  useEffect(() => {
    const initialize_board = async () => {
      let fen = await get_fen_string();
      setBoardFen(fen);
    }

    initialize_board();
  }, []);


  return (
    <div className="gameboard">
      <Chessboard options={boardOptions}/>
      <p>Position: {boardFen}</p>
    </div>
  );
}

export default Gameboard;
