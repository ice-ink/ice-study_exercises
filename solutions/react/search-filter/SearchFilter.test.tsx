import { afterEach, describe, expect, it } from 'vitest';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { SearchFilter } from './SearchFilter';

afterEach(cleanup);

const items = ['Apple', 'Banana', 'Apricot', 'Cherry'];

describe('SearchFilter', () => {
  it('renders all items when the query is empty', () => {
    render(<SearchFilter items={items} />);

    for (const item of items) {
      expect(screen.getByText(item)).toBeInTheDocument();
    }
  });

  it('filters items by case-insensitive substring', () => {
    render(<SearchFilter items={items} />);

    fireEvent.change(screen.getByLabelText('Search'), { target: { value: 'ap' } });

    expect(screen.getByText('Apple')).toBeInTheDocument();
    expect(screen.getByText('Apricot')).toBeInTheDocument();
    expect(screen.queryByText('Banana')).not.toBeInTheDocument();
    expect(screen.queryByText('Cherry')).not.toBeInTheDocument();
  });

  it('restores all items when the query is cleared', () => {
    render(<SearchFilter items={items} />);

    const input = screen.getByLabelText('Search');
    fireEvent.change(input, { target: { value: 'ap' } });
    fireEvent.change(input, { target: { value: '' } });

    for (const item of items) {
      expect(screen.getByText(item)).toBeInTheDocument();
    }
  });
});
