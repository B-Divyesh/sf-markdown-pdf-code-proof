import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './site/tests',
  use: { baseURL: 'http://127.0.0.1:4173' },
  webServer: {
    command: 'node scripts/site-test-server.mjs',
    url: 'http://127.0.0.1:4173',
    reuseExistingServer: false
  }
});
