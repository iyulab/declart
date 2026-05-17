import { defineConfig, devices } from '@playwright/test';
import path from 'path';

const REPO_ROOT = path.resolve(__dirname, '../..');

export default defineConfig({
  testDir: './specs',
  timeout: 30_000,
  retries: 1,
  reporter: [['list'], ['html', { open: 'never', outputFolder: 'playwright-report' }]],
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    // Requires mdbook + mdbook-declart in PATH; call build script before running tests.
    // On Windows: copy C:\tools\mdbook\mdbook.exe and mdbook-declart.exe to PATH first.
    command: `mdbook serve ${path.join(REPO_ROOT, 'docs').replace(/\\/g, '/')} --port 3000`,
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
    cwd: REPO_ROOT,
    env: {
      PATH: `C:\\tools\\mdbook;${process.env.PATH ?? ''}`,
    },
  },
});
