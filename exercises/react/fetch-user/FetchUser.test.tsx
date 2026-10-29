import { afterEach, describe, expect, it, vi } from 'vitest';
import { cleanup, render, screen } from '@testing-library/react';
import { FetchUser } from './FetchUser';

afterEach(cleanup);

describe('FetchUser', () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('shows the user name on success', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ name: 'Ada' }),
    });

    render(<FetchUser />);

    expect(await screen.findByText('User: Ada')).toBeInTheDocument();
  });

  it('shows an error on failure', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('boom'));

    render(<FetchUser />);

    expect(await screen.findByText('Error')).toBeInTheDocument();
  });
});
