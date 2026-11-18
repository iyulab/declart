import { defineConfig, devices } from '@playwright/test';
import path from 'path';

const REPO_ROOT = path.resolve(__dirname, '../..');

export default defineConfig({
  testDir: './specs',
  timeout: 30_000,
  retries: 1,
  // CI keeps nothing but the log; locally the HTML report is handy.
  reporter: process.env.CI
    ? [['list']]
    : [['list'], ['html', { open: 'never', outputFolder: 'playwright-report' }]],
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
    // Needs `mdbook` on PATH and the preprocessor built (`cargo build -p mdbook-declart --release`)
    // plus the playground WASM — `docs/build.sh` or `docs/build.ps1` does both.
    command: `mdbook serve ${path.join(REPO_ROOT, 'docs').replace(/\\/g, '/')} --port 3000`,
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
    cwd: REPO_ROOT,
    env: {
      // The release build of mdbook-declart, so `mdbook serve` finds its preprocessor.
      PATH: `${path.join(REPO_ROOT, 'target', 'release')}${path.delimiter}${process.env.PATH ?? ''}`,
    },
  },
});
