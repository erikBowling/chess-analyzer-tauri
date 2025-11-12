import Gameboard from "./components/gameboard";

function App() {

  return (
    <main className="container">
      <h1>Chess Analyzer</h1>
      <div className="chessboard-container">
        <Gameboard />
      </div>
    </main>
  )
}

export default App;
