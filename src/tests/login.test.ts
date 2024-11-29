// src/tests/login.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mockIPC } from '@tauri-apps/api/mocks';
import Login from '../routes/auth/login/+page.svelte';
import { render, fireEvent } from '@testing-library/svelte';
import '@testing-library/jest-dom';

import { toast } from 'svelte-sonner';

vi.mock('svelte-sonner', () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn()
  }
}));


describe('Login Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockIPC(<T>(cmd: string, payload?: any): Promise<T> => {
      if (cmd === 'login') {
        return Promise.resolve({ success: true } as T);
      }
      return Promise.reject('Command not found');
    });
  });

  it('renders login form correctly', async () => {
    const { container } = render(Login);

    const emailInput = container.querySelector('#email');
    const passwordInput = container.querySelector('#password');
    const submitButton = container.querySelector('button[type="submit"]');
    const logo = container.querySelector('img[alt="Logo"]');
    const signupText = container.querySelector('p');

    expect(emailInput).toBeTruthy();
    expect(emailInput?.getAttribute('placeholder')).toBe('Enter your email');
    expect(passwordInput).toBeTruthy();
    expect(passwordInput?.getAttribute('placeholder')).toBe('Enter your password');
    expect(submitButton).toBeTruthy();
    expect(logo).toBeTruthy();
    expect(logo?.getAttribute('class')).toContain('h-12 w-12');
  });
  it('shows error when submitting empty form', async () => {
    const { container } = render(Login);
    const form = container.querySelector('form');
    
    await fireEvent.submit(form!);
    
    expect(toast.error).toHaveBeenCalledWith('Please fill in all fields.');
  });
});
