import { afterEach, describe, expect, it } from 'vitest';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { Accordion } from './Accordion';

afterEach(cleanup);

const sections = [
  { title: 'First', content: 'First content' },
  { title: 'Second', content: 'Second content' },
];

describe('Accordion', () => {
  it('shows no content initially', () => {
    render(<Accordion sections={sections} />);

    expect(screen.queryByText('First content')).not.toBeInTheDocument();
    expect(screen.queryByText('Second content')).not.toBeInTheDocument();
  });

  it('opens a section when its title is clicked', () => {
    render(<Accordion sections={sections} />);

    fireEvent.click(screen.getByRole('button', { name: 'First' }));

    expect(screen.getByText('First content')).toBeInTheDocument();
  });

  it('keeps only one section open at a time', () => {
    render(<Accordion sections={sections} />);

    fireEvent.click(screen.getByRole('button', { name: 'First' }));
    fireEvent.click(screen.getByRole('button', { name: 'Second' }));

    expect(screen.queryByText('First content')).not.toBeInTheDocument();
    expect(screen.getByText('Second content')).toBeInTheDocument();
  });
});
