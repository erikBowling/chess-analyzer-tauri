import "../App.css";
import { Chessboard, ChessboardOptions, PieceDropHandlerArgs, PieceHandlerArgs} from "react-chessboard";
import { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/core";

function Gameboard() {
  const [boardOptions, setBoardOptions] = useState<ChessboardOptions>({
    position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    onPieceClick: handlePieceClick,
    onPieceDrop: handlePieceDrop
  });
  useEffect(() => {}, []);

  async function handlePieceDrop({piece, sourceSquare, targetSquare}:PieceDropHandlerArgs): boolean
  {
    console.log("Piece:", piece);
    console.log("Source Square:", sourceSquare);
    console.log("Target Square:", targetSquare);

    invoke<string>("move_piece", {
      from: sourceSquare,
      to: targetSquare
    })
      .then((newFen) => {
        setBoardOptions(prevOptions => ({ ...prevOptions, position: newFen }));
      })
      .catch((error) => {
        console.error("Move failed:", error);
        setBoardOptions(prevOptions => ({ ...prevOptions, position: prevOptions.position }));
      });

    return true;
  }

  function handlePieceClick({ isSparePiece, piece, square }: PieceHandlerArgs): void
  {
    console.log("IsSpacePiece:", isSparePiece);
    console.log("Piece:", piece);
    console.log("Square:", square);
  }

  return (
    <div className="gameboard">
      <Chessboard options={boardOptions}/>
    </div>
  );
}

export default Gameboard;
