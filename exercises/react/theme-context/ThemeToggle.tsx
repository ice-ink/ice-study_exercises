import type { ReactNode } from 'react';

type Theme = 'light' | 'dark';

interface ThemeContextValue {
  theme: Theme;
  toggle: () => void;
}

export function ThemeProvider({ children }: { children: ReactNode }) {
  // TODO: implement ThemeProvider
  return null;
}

export function useTheme(): ThemeContextValue {
  throw new Error('TODO: implement useTheme');
}

export function ThemeToggle() {
  // TODO: implement ThemeToggle
  return null;
}
