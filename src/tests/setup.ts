import '@testing-library/jest-dom';
import { vi } from 'vitest';


Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});


vi.mock('mode-watcher', () => ({
  mode: {
    subscribe: (fn: (value: string) => void) => {
      fn('light');
      return { unsubscribe: vi.fn() };
    },
  },
}));


vi.mock('svelte-sonner', () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));


vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));
