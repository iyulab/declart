import { test, expect, Page } from '@playwright/test';

const PLAYGROUND = '/playground/index.html';

/** Wait until WASM is loaded and a render result (SVG or error) is present. */
async function waitForWasm(page: Page) {
  await page.waitForFunction(
    () => {
      const status = document.getElementById('status');
      return status && (status.textContent?.includes('✓') || status.textContent?.includes('✗'));
    },
    { timeout: 15_000 },
  );
}

/** Wait for a fresh render to complete after an action. */
async function waitForRender(page: Page) {
  // After input change, status goes to OK or Error
  await page.waitForFunction(
    () => {
      const status = document.getElementById('status');
      if (!status) return false;
      const txt = status.textContent ?? '';
      return txt.includes('✓') || txt.includes('✗');
    },
    { timeout: 10_000 },
  );
}

async function selectExample(page: Page, value: string) {
  // Reset status first so we can detect a new render
  await page.evaluate(() => {
    const s = document.getElementById('status');
    if (s) s.textContent = '…';
  });
  await page.selectOption('#example-select', value);
  await waitForRender(page);
}

function isHidden(el: { className: string }) {
  return el.className.split(' ').includes('hidden');
}

test.describe('Playground — initial load', () => {
  test('page loads with Declart in title', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await expect(page).toHaveTitle(/Declart/i);
  });

  test('WASM loads successfully (status shows Ready)', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);
    const status = page.locator('#status');
    await expect(status).toContainText('✓');
  });

  test('default example renders an SVG', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);
    const svg = page.locator('#preview svg');
    await expect(svg).toBeVisible();
  });

  test('error element is hidden on default load', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);
    const error = page.locator('#error');
    await expect(error).toHaveClass(/hidden/);
  });
});

test.describe('Playground — example selector (all examples)', () => {
  const examples: [string, string][] = [
    ['flow_process',       'flow/process'],
    ['flow_cycle',         'flow/cycle'],
    ['flow_funnel',        'flow/funnel'],
    ['tier',               'tier'],
    ['hierarchy_org',      'hierarchy/org_chart'],
    ['hierarchy_fishbone', 'hierarchy/fishbone'],
    ['hierarchy_mind_map', 'hierarchy/mind_map'],
    ['matrix',             'matrix'],
    ['hub_spoke',          'hub_spoke'],
    ['venn',               'venn'],
    ['timeline',           'timeline'],
    ['comparison',         'comparison'],
    ['flow_swimlane',      'flow/swimlane'],
    ['state',              'state'],
  ];

  for (const [value, label] of examples) {
    test(`"${label}" renders an SVG without error`, async ({ page }) => {
      await page.goto(PLAYGROUND);
      await waitForWasm(page);
      await selectExample(page, value);

      // No error
      const errorEl = page.locator('#error');
      const cls = await errorEl.getAttribute('class') ?? '';
      expect(cls, `${label}: unexpected error shown`).toContain('hidden');

      // SVG present and non-trivial
      const svg = page.locator('#preview svg');
      await expect(svg).toBeVisible();
      const html = await svg.innerHTML();
      expect(html.length, `${label}: SVG content too short`).toBeGreaterThan(50);
    });
  }
});

test.describe('Playground — mind_map (new feature)', () => {
  test('mind_map SVG contains all node labels', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);
    await selectExample(page, 'hierarchy_mind_map');

    const html = await page.locator('#preview').innerHTML();
    for (const label of ['ML Concepts', 'Supervised', 'Unsupervised', 'Reinforce', 'Classification', 'Regression']) {
      expect(html, `missing label: ${label}`).toContain(label);
    }
  });

  test('mind_map renders a circle element (root node)', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);
    await selectExample(page, 'hierarchy_mind_map');
    const circle = page.locator('#preview svg circle');
    await expect(circle).toBeVisible();
  });

  test('mind_map direct TOML input renders correctly', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);

    await page.locator('#input').fill(`kind = "hierarchy"
view = "mind_map"
title = "Learning"

[[nodes]]
label = "Root"

[[nodes]]
label = "Child A"
parent = "Root"

[[nodes]]
label = "Child B"
parent = "Root"`);
    await waitForRender(page);

    const svg = page.locator('#preview svg');
    await expect(svg).toBeVisible();
    const html = await page.locator('#preview').innerHTML();
    expect(html).toContain('Root');
    expect(html).toContain('Child A');
    expect(html).toContain('Child B');
    // Circle for root node
    expect(html).toContain('<circle');
  });

  test('mind_map with multiple depth levels renders', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);

    await page.locator('#input').fill(`kind = "hierarchy"
view = "mind_map"

[[nodes]]
label = "Center"

[[nodes]]
label = "Branch"
parent = "Center"

[[nodes]]
label = "Leaf"
parent = "Branch"`);
    await waitForRender(page);

    const errorEl = page.locator('#error');
    const cls = await errorEl.getAttribute('class') ?? '';
    expect(cls, 'three-level mind_map should not error').toContain('hidden');
    await expect(page.locator('#preview svg')).toBeVisible();
  });
});

test.describe('Playground — error handling', () => {
  test('missing nodes shows error', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);

    await page.locator('#input').fill('kind = "hierarchy"');
    await waitForRender(page);

    const errorEl = page.locator('#error');
    const cls = await errorEl.getAttribute('class') ?? '';
    expect(cls, 'missing nodes should show error').not.toContain('hidden');

    const errText = await errorEl.innerText();
    expect(errText.length).toBeGreaterThan(0);
  });

  test('unknown kind shows error', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);

    await page.locator('#input').fill('kind = "does_not_exist"');
    await waitForRender(page);

    const errorEl = page.locator('#error');
    const cls = await errorEl.getAttribute('class') ?? '';
    expect(cls).not.toContain('hidden');
  });

  test('invalid mind_map (multiple roots) shows error', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);

    await page.locator('#input').fill(`kind = "hierarchy"
view = "mind_map"

[[nodes]]
label = "Root A"

[[nodes]]
label = "Root B"`);
    await waitForRender(page);

    const errorEl = page.locator('#error');
    const cls = await errorEl.getAttribute('class') ?? '';
    expect(cls, 'mind_map with 2 roots should error').not.toContain('hidden');
  });

  test('fixing error clears it', async ({ page }) => {
    await page.goto(PLAYGROUND);
    await waitForWasm(page);

    // Trigger error
    await page.locator('#input').fill('kind = "bad_kind"');
    await waitForRender(page);
    const errorEl = page.locator('#error');
    await expect(errorEl).not.toHaveClass(/hidden/);

    // Fix it
    await page.locator('#input').fill(`kind = "flow"\n\n[[items]]\nlabel = "A"\n\n[[items]]\nlabel = "B"`);
    await waitForRender(page);
    await expect(errorEl).toHaveClass(/hidden/);
  });
});

test.describe('Playground — theme switching', () => {
  const themes = ['default', 'monochrome', 'warm', 'accessible'];

  for (const theme of themes) {
    test(`theme "${theme}" applies without error`, async ({ page }) => {
      await page.goto(PLAYGROUND);
      await waitForWasm(page);
      await selectExample(page, 'hierarchy_mind_map');

      await page.evaluate(() => {
        const s = document.getElementById('status');
        if (s) s.textContent = '…';
      });
      await page.selectOption('#theme-select', theme);
      await waitForRender(page);

      const cls = await page.locator('#error').getAttribute('class') ?? '';
      expect(cls, `theme ${theme}: error shown`).toContain('hidden');
      await expect(page.locator('#preview svg')).toBeVisible();
    });
  }
});
