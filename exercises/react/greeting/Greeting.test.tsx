import { afterEach, describe, expect, it } from 'vitest';
import { cleanup, render, screen } from '@testing-library/react';
import { Greeting } from './Greeting';

afterEach(cleanup);

describe('Greeting', () => {
  it('greets by name', () => {
    render(<Greeting name="Alice" />);
    expect(screen.getByText('Hello, Alice!')).toBeInTheDocument();
  });

  it('greets a stranger when no name is given', () => {
    render(<Greeting />);
    expect(screen.getByText('Hello, stranger!')).toBeInTheDocument();
  });
});
