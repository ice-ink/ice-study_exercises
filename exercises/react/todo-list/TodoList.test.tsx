import { afterEach, describe, expect, it } from 'vitest';
import { cleanup, render, screen } from '@testing-library/react';
import { TodoList } from './TodoList';

afterEach(cleanup);

describe('TodoList', () => {
  it('renders one list item per item', () => {
    render(<TodoList items={['alpha', 'beta']} />);
    expect(screen.getAllByRole('listitem')).toHaveLength(2);
    expect(screen.getByText('alpha')).toBeInTheDocument();
    expect(screen.getByText('beta')).toBeInTheDocument();
  });

  it('renders "No items" when the list is empty', () => {
    render(<TodoList items={[]} />);
    expect(screen.getByText('No items')).toBeInTheDocument();
  });
});
