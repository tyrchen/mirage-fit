import path from "path"
// Import defineConfig from vite
import { defineConfig, UserConfig } from 'vite'
// Import Vitest config type
import type { InlineConfig } from 'vitest'

/// <reference types="vitest" />
import react from '@vitejs/plugin-react'
// import tsconfigPaths from 'vite-tsconfig-paths'
import tailwindcss from '@tailwindcss/vite'

// Combine Vite and Vitest config types
interface VitestConfigExport extends UserConfig {
  test: InlineConfig
}

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    // tsconfigPaths(),
    tailwindcss()
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, "./src"),
    }
  },
  server: {
    port: 5173,
  },
  // Vitest configuration
  test: {
    globals: true, // Use Vitest globals (describe, it, expect, etc.)
    environment: 'jsdom', // Simulate DOM environment
    setupFiles: ['./src/test/setup.ts'], // Test setup files
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'], // Test file patterns
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.d.ts',
        '**/*.config.*',
        'dist/',
      ],
      thresholds: {
        global: {
          branches: 80,
          functions: 80,
          lines: 80,
          statements: 80
        }
      }
    },
    // reporters: ['verbose'] // Optional: Use verbose reporter
  },
} as VitestConfigExport) // Cast the config object
