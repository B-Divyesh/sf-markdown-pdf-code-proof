import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('@claim:private-site landing and sample stay local and leave no tracking data', async ({ page, context }) => {
  const errors: string[] = [];
  const origins = new Set<string>();
  page.on('console', (message) => {
    if (message.type() === 'error') errors.push(message.text());
  });
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  const response = await page.goto('/');
  expect(response?.headers()['content-security-policy']).toContain("default-src 'self'");
  expect(response?.headers()['x-content-type-options']).toBe('nosniff');
  await expect(page).toHaveTitle(/Code Proof/);
  await expect(page.locator('main')).toBeVisible();
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.locator('h1')).toContainText('Catch PDF bugs before release');
  await expect(page.getByRole('link', { name: /Try it with sample data/ })).toBeVisible();
  await expect(page.locator('img[alt]')).toHaveCount(1);
  await page.getByRole('link', { name: /Try it with sample data/ }).click();
  const startingResults = await new AxeBuilder({ page }).analyze();
  expect(startingResults.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
  await expect(page.locator('#demo-status')).toContainText('Proof run complete', { timeout: 4000 });

  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
  expect(errors).toEqual([]);
  expect([...origins]).toEqual(['http://127.0.0.1:4173']);
  expect(await context.cookies()).toEqual([]);
  expect(await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length }))).toEqual({ local: 0, session: 0 });
});

test('skip link is the first keyboard stop and reaches main content', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to content' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('main')).toBeFocused();
});

test('sample demo is one click away and reports completion', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('link', { name: /Try it with sample data/ }).click();
  await expect(page).toHaveURL(/\?demo=1#demo$/);
  await expect(page).toHaveTitle('Demo — Code Proof');
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await expect(page.locator('#demo-title')).toBeFocused();
  await expect(page.locator('#demo-status')).toContainText('Proof run complete', { timeout: 4000 });
  await expect(page.getByText('DEMO HOLD — do not release — 1 expected defect found')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Reset demo' })).toBeVisible();
  await expect(page.getByRole('link', { name: 'View install commands' })).toBeVisible();
  await page.getByRole('link', { name: 'View install commands' }).click();
  await expect(page).toHaveURL(/\/#install$/);
  await expect(page).toHaveTitle('Code Proof — inspect Markdown PDFs before release');
  await expect(page.locator('#demo-banner')).toBeHidden();
  await expect(page.locator('#install-title')).toBeFocused();
  await page.goBack();
  await expect(page).toHaveURL(/\?demo=1#demo$/);
  await expect(page.locator('#demo-title')).toBeFocused();
  await page.goForward();
  await expect(page.locator('#install-title')).toBeFocused();
});

test('keyboard and reduced-motion users receive demo feedback', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/?demo=1#demo');
  const reset = page.getByRole('button', { name: 'Reset demo' });
  await reset.focus();
  await page.keyboard.press('Space');
  await expect(page.locator('#demo-status')).toContainText('Proof run complete');
  const motion = await page.evaluate(() => ({
    animation: getComputedStyle(document.querySelector('.press-art')!).animationDuration,
    transition: getComputedStyle(document.querySelector('.demo-line')!).transitionDuration,
    scroll: getComputedStyle(document.documentElement).scrollBehavior
  }));
  expect(motion).toEqual({ animation: '1e-05s', transition: '1e-05s', scroll: 'auto' });
});

test('clipboard denial leaves the complete install command selectable', async ({ page }) => {
  await page.goto('/');
  await page.locator('.hero').getByRole('button', { name: 'Copy install command' }).click();
  await expect(page.locator('#toast')).toContainText(/^(Copied:|Copy unavailable\. Select this command:) cargo install --git https:\/\/github\.com\/B-Divyesh\/sf-markdown-pdf-code-proof\.git --locked codeproof$/);
});

test('390px layout keeps primary paths available', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.locator('.hero').getByRole('button', { name: /Copy install command/ })).toBeVisible();
  await expect(page.getByRole('link', { name: /Try it with sample data/ })).toBeVisible();
  const width = await page.evaluate(() => document.documentElement.scrollWidth);
  expect(width).toBeLessThanOrEqual(390);
  for (const label of ['Sample included', 'Site privacy', 'MIT license']) {
    const box = await page.getByText(label, { exact: true }).boundingBox();
    expect(box?.y, `${label} stays in the first viewport`).toBeLessThan(844);
  }

  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);

  for (const selector of ['.contract-strip', '.terminal pre', '.command:nth-child(2) code', '.command:nth-child(3) code']) {
    await expect(page.locator(selector)).toHaveAttribute('tabindex', '0');
  }
  const terms = await page.getByRole('link', { name: 'Terms' }).boundingBox();
  expect(terms?.width).toBeGreaterThanOrEqual(44);
  expect(terms?.height).toBeGreaterThanOrEqual(44);

  const controls = page.locator('a:visible, button:visible');
  for (let index = 0; index < await controls.count(); index += 1) {
    const box = await controls.nth(index).boundingBox();
    expect(box?.width, `control ${index} width`).toBeGreaterThanOrEqual(44);
    expect(box?.height, `control ${index} height`).toBeGreaterThanOrEqual(44);
  }
});

test('brand accessible name contains its visible label', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('link', { name: /Code Proof release inspector \/ 0\.1/i })).toBeVisible();
});

test('@claim:offline-reload installed shell stays useful offline', async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  try {
    await page.goto('/');
    await page.evaluate(() => navigator.serviceWorker.ready);
    await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
    await expect.poll(() => page.evaluate(async () => {
      const registration = await navigator.serviceWorker.getRegistration();
      return registration?.active?.state === 'activated';
    })).toBe(true);
    await page.reload();
    await context.setOffline(true);
    await page.reload();
    await expect(page.locator('h1')).toContainText('Catch PDF bugs before release');
    await expect(page.getByRole('status').filter({ hasText: 'Offline' })).toBeVisible();
  } finally {
    await context.close();
  }
});

test('worker installs with production-only deployment files unavailable', async ({ page }) => {
  await page.goto('/');
  await page.evaluate(() => navigator.serviceWorker.ready);
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  const shell = await page.evaluate(async () => (await fetch('/sw.js')).text());
  expect(shell).not.toContain('staticwebapp.config.json');
  const caches = await page.evaluate(async () => (await caches.keys()).filter((name) => name.startsWith('code-proof-')));
  expect(caches).toEqual(['code-proof-v4']);
  const update = await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.getRegistration();
    await registration?.update();
    return { installing: Boolean(registration?.installing), waiting: Boolean(registration?.waiting) };
  });
  expect(update).toEqual({ installing: false, waiting: false });
});

for (const path of ['/privacy/', '/terms/']) {
  test(`${path} has a single semantic title`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
  });
}

test('unknown routes use the branded 404 document', async ({ page }) => {
  const response = await page.goto('/missing-proof');
  expect(response?.status()).toBe(404);
  await expect(page).toHaveTitle('Page not found — Code Proof');
  await expect(page.locator('h1')).toHaveText('This page was not found.');
  await expect(page.getByRole('link', { name: 'Return home' })).toBeVisible();
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
});
