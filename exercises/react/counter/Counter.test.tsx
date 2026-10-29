import { afterEach, describe, expect, it } from 'vitest';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { Counter } from './Counter';

afterEach(cleanup);

describe('Counter', () => {
  it('starts at 0', () => {
    render(<Counter />);
    expect(screen.getByTestId('count')).toHaveTextContent('0');
  });

  it('increments to 1 when Increment is clicked', () => {
    render(<Counter />);
    fireEvent.click(screen.getByRole('button', { name: 'Increment' }));
    expect(screen.getByTestId('count')).toHaveTextContent('1');
  });

  it('decrements to -1 when Decrement is clicked', () => {
    render(<Counter />);
    fireEvent.click(screen.getByRole('button', { name: 'Decrement' }));
    expect(screen.getByTestId('count')).toHaveTextContent('-1');
  });

  it('resets to 0 when Reset is clicked', () => {
    render(<Counter />);
    fireEvent.click(screen.getByRole('button', { name: 'Increment' }));
    fireEvent.click(screen.getByRole('button', { name: 'Increment' }));
    fireEvent.click(screen.getByRole('button', { name: 'Reset' }));
    expect(screen.getByTestId('count')).toHaveTextContent('0');
  });
});
