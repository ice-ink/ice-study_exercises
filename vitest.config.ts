import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./vitest.setup.ts'],
    include: ['exercises/react/**/*.test.tsx', 'solutions/react/**/*.test.tsx'],
  },
});
