import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { CounterReducer } from './CounterReducer';

afterEach(() => {
  cleanup();
});

describe('CounterReducer', () => {
  it('starts at 0', () => {
    render(<CounterReducer />);
    expect(screen.getByTestId('count')).toHaveTextContent('0');
  });

  it('increments to 1', () => {
    render(<CounterReducer />);
    fireEvent.click(screen.getByRole('button', { name: 'Increment' }));
    expect(screen.getByTestId('count')).toHaveTextContent('1');
  });

  it('decrements to -1', () => {
    render(<CounterReducer />);
    fireEvent.click(screen.getByRole('button', { name: 'Decrement' }));
    expect(screen.getByTestId('count')).toHaveTextContent('-1');
  });

  it('resets to 0', () => {
    render(<CounterReducer />);
    fireEvent.click(screen.getByRole('button', { name: 'Increment' }));
    fireEvent.click(screen.getByRole('button', { name: 'Increment' }));
    fireEvent.click(screen.getByRole('button', { name: 'Reset' }));
    expect(screen.getByTestId('count')).toHaveTextContent('0');
  });
});
