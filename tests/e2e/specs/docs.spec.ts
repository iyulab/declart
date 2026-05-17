import { test, expect } from '@playwright/test';

test.describe('Docs site — page loading', () => {
  test('introduction page loads', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/Declart/i);
    await expect(page.locator('h1, h2').first()).toBeVisible();
  });

  test('navigation sidebar is present', async ({ page }) => {
    await page.goto('/');
    // mdbook renders a sidebar with nav links
    const sidebar = page.locator('.sidebar, #sidebar, nav.nav-chapters');
    await expect(sidebar.first()).toBeVisible();
  });

  test('hierarchy page loads with Hierarchy heading', async ({ page }) => {
    await page.goto('/kinds/hierarchy.html');
    // mdbook wraps content h1 in an anchor; use .content to scope to page body
    const heading = page.locator('.content h1, main h1').first();
    await expect(heading).toContainText('Hierarchy');
  });

  test('all spec kind pages are reachable (HTTP 200)', async ({ page }) => {
    const kinds = ['flow', 'tier', 'hierarchy', 'matrix', 'hub_spoke', 'venn', 'timeline', 'comparison'];
    for (const kind of kinds) {
      const response = await page.request.get(`/kinds/${kind}.html`);
      expect(response.status(), `${kind}.html should return 200`).toBe(200);
    }
  });
});

test.describe('Docs site — SVG rendering (mdbook-declart preprocessor)', () => {
  test('hierarchy page contains rendered SVG diagrams', async ({ page }) => {
    await page.goto('/kinds/hierarchy.html');
    const svgs = page.locator('.content svg, main svg');
    const count = await svgs.count();
    expect(count, 'hierarchy page should have at least 3 SVG diagrams').toBeGreaterThanOrEqual(3);
  });

  test('flow page contains rendered SVG diagrams', async ({ page }) => {
    await page.goto('/kinds/flow.html');
    const svgs = page.locator('.content svg, main svg');
    const count = await svgs.count();
    expect(count, 'flow page should have at least 1 SVG diagram').toBeGreaterThanOrEqual(1);
  });

  test('hierarchy page mind_map SVG is present (circle for root node)', async ({ page }) => {
    await page.goto('/kinds/hierarchy.html');
    const pageContent = await page.content();
    // mind_map renderer uses a circle for the root node
    expect(pageContent).toContain('<circle');
    expect(pageContent).toContain('ML Concepts');
  });

  test('no raw declart code blocks visible (preprocessor ran)', async ({ page }) => {
    const kinds = ['flow', 'hierarchy', 'tier'];
    for (const kind of kinds) {
      await page.goto(`/kinds/${kind}.html`);
      const rawBlocks = page.locator('.content code.language-declart, main code.language-declart');
      const count = await rawBlocks.count();
      expect(count, `${kind}.html: found raw declart blocks — preprocessor may have failed`).toBe(0);
    }
  });
});

test.describe('Docs site — playground', () => {
  test('playground route redirects to playground/index.html', async ({ page }) => {
    // playground.html uses JS redirect to playground/index.html
    await page.goto('/playground.html');
    // After redirect, we should be on the playground
    await page.waitForURL(/playground\/index\.html/, { timeout: 5_000 });
    await expect(page).toHaveURL(/playground\/index\.html/);
  });

  test('playground index renders and WASM loads', async ({ page }) => {
    await page.goto('/playground/index.html');
    // Wait for WASM status
    await page.waitForFunction(
      () => {
        const s = document.getElementById('status');
        return s && (s.textContent?.includes('✓') || s.textContent?.includes('✗'));
      },
      { timeout: 15_000 },
    );
    const status = page.locator('#status');
    await expect(status).toContainText('✓');
  });
});

test.describe('Docs site — schema page', () => {
  test('schema page loads', async ({ page }) => {
    await page.goto('/schema.html');
    await expect(page.locator('h1, h2').first()).toBeVisible();
  });
});
