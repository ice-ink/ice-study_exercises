import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { TicTacToe } from './TicTacToe';

afterEach(() => {
  cleanup();
});

describe('TicTacToe', () => {
  it('detects a diagonal winner and shows Winner: X', () => {
    render(<TicTacToe />);

    // X plays the (0,0)-(1,1)-(2,2) diagonal while O plays elsewhere.
    fireEvent.click(screen.getByTestId('square-0-0')); // X
    fireEvent.click(screen.getByTestId('square-0-1')); // O
    fireEvent.click(screen.getByTestId('square-1-1')); // X
    fireEvent.click(screen.getByTestId('square-0-2')); // O
    fireEvent.click(screen.getByTestId('square-2-2')); // X wins

    expect(screen.getByText('Winner: X')).toBeInTheDocument();
  });

  it('resets the board and restarts with X', () => {
    render(<TicTacToe />);

    fireEvent.click(screen.getByTestId('square-0-0')); // X
    expect(screen.getByTestId('square-0-0')).toHaveTextContent('X');

    fireEvent.click(screen.getByRole('button', { name: 'Reset' }));

    expect(screen.getByTestId('square-0-0')).toBeEmptyDOMElement();

    fireEvent.click(screen.getByTestId('square-0-0'));
    expect(screen.getByTestId('square-0-0')).toHaveTextContent('X');
  });
});
