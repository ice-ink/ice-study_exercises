import { useState } from 'react';

type Player = 'X' | 'O';
type SquareValue = Player | null;
type Line = [number, number, number];

const WINNING_LINES: Line[] = [
  [0, 1, 2],
  [3, 4, 5],
  [6, 7, 8],
  [0, 3, 6],
  [1, 4, 7],
  [2, 5, 8],
  [0, 4, 8],
  [2, 4, 6],
];

function calculateWinner(board: SquareValue[]): Player | null {
  for (const line of WINNING_LINES) {
    const [a, b, c] = line;
    const valueA = board[a];
    const valueB = board[b];
    const valueC = board[c];
    if (valueA != null && valueA === valueB && valueA === valueC) {
      return valueA;
    }
  }
  return null;
}

export function TicTacToe() {
  const [board, setBoard] = useState<SquareValue[]>(Array<SquareValue>(9).fill(null));
  const [currentPlayer, setCurrentPlayer] = useState<Player>('X');

  const winner = calculateWinner(board);
  const isDraw = winner === null && board.every((cell) => cell != null);

  function handleClick(index: number) {
    if (winner !== null || board[index] != null) {
      return;
    }

    const next = [...board];
    next[index] = currentPlayer;
    setBoard(next);
    setCurrentPlayer(currentPlayer === 'X' ? 'O' : 'X');
  }

  function handleReset() {
    setBoard(Array<SquareValue>(9).fill(null));
    setCurrentPlayer('X');
  }

  let status: string;
  if (winner !== null) {
    status = `Winner: ${winner}`;
  } else if (isDraw) {
    status = 'Draw';
  } else {
    status = `Next: ${currentPlayer}`;
  }

  return (
    <div>
      <p>{status}</p>
      {[0, 1, 2].map((row) => (
        <div key={row} style={{ display: 'flex' }}>
          {[0, 1, 2].map((col) => {
            const index = row * 3 + col;
            return (
              <button
                key={col}
                type="button"
                data-testid={`square-${row}-${col}`}
                onClick={() => handleClick(index)}
              >
                {board[index] ?? ''}
              </button>
            );
          })}
        </div>
      ))}
      <button type="button" onClick={handleReset}>
        Reset
      </button>
    </div>
  );
}
