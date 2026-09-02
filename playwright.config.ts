import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './site/tests',
  use: { baseURL: process.env.PLAYWRIGHT_BASE_URL ?? 'http://127.0.0.1:4173' },
  webServer: process.env.PLAYWRIGHT_BASE_URL ? undefined : {
    command: 'node scripts/site-test-server.mjs',
    url: 'http://127.0.0.1:4173',
    reuseExistingServer: false
  }
});
