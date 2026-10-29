import { afterEach, describe, expect, it } from 'vitest';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { Toggle } from './Toggle';

afterEach(cleanup);

describe('Toggle', () => {
  it('toggles the message visibility', () => {
    render(<Toggle />);

    expect(screen.queryByText('Now you see me')).toBeNull();

    fireEvent.click(screen.getByRole('button', { name: 'Toggle' }));
    expect(screen.queryByText('Now you see me')).not.toBeNull();

    fireEvent.click(screen.getByRole('button', { name: 'Toggle' }));
    expect(screen.queryByText('Now you see me')).toBeNull();
  });
});
