import { vi } from 'vitest';

export const hooks = {
  client: {
    start: vi.fn()
  }
};

export const navigation = {
  subscribe: vi.fn()
};

export const page = {
  subscribe: vi.fn()
};

export const updated = {
  subscribe: vi.fn()
};

export const base = '';
export const assets = '';
export const version = '';

export const goto = vi.fn(() => Promise.resolve());
export const afterNavigate = vi.fn();
export const beforeNavigate = vi.fn();
export const disableScrollHandling = vi.fn();
export const pushState = vi.fn();
export const replaceState = vi.fn();
export const invalidate = vi.fn();
export const invalidateAll = vi.fn();
