import { afterEach, describe, expect, it } from 'vitest';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { ControlledForm } from './ControlledForm';

afterEach(cleanup);

describe('ControlledForm', () => {
  it('submits the typed value', () => {
    render(<ControlledForm />);

    fireEvent.change(screen.getByLabelText('Name'), { target: { value: 'Ada' } });
    fireEvent.click(screen.getByRole('button', { name: 'Submit' }));

    expect(screen.getByText('Submitted: Ada')).toBeInTheDocument();
  });
});
