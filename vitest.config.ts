import { defineConfig } from 'vitest/config';
import {sveltekit} from '@sveltejs/kit/vite'
import {svelteTesting} from '@testing-library/svelte/vite'
import path from 'path';


export default defineConfig({
  plugins: [sveltekit(), svelteTesting()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts,jsx,tsx}'],
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./src/tests/setup.ts'],
    deps: {
      inline: [/^svelte/]
    },
    alias: {
        '$app/navigation': '/src/tests/mocks/navigation.ts',
        '$app/environment': '/src/tests/mocks/navigation.ts',
        '$app/stores': '/src/tests/mocks/navigation.ts'
      }
  },
 
});
