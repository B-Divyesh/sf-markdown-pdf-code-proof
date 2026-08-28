import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('landing page is usable, quiet, and accessible', async ({ page }) => {
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
  await expect(page.locator('h1')).toContainText('PDF bugs');
  await expect(page.locator('img[alt]')).toHaveCount(1);

  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
  expect(errors).toEqual([]);
  expect([...origins]).toEqual(['http://127.0.0.1:4173']);
});

test('skip link is the first keyboard stop and reaches main content', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to content' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page.locator('main')).toBeFocused();
});

test('recorded proof reports completion to assistive technology', async ({ page }) => {
  await page.goto('/#demo');
  await page.getByRole('button', { name: 'Replay proof run' }).click();
  await expect(page.locator('#demo-status')).toContainText('Proof run complete', { timeout: 4000 });
  await expect(page.getByText('HOLD — 184 pages')).toBeVisible();
});

test('390px layout keeps primary paths available', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.getByRole('button', { name: /Copy install command/ })).toBeVisible();
  await expect(page.getByRole('link', { name: /Watch a proof run/ })).toBeVisible();
  const width = await page.evaluate(() => document.documentElement.scrollWidth);
  expect(width).toBeLessThanOrEqual(390);

  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);

  for (const selector of ['.contract-strip', '.terminal pre', '.command:nth-child(2) code', '.command:nth-child(3) code']) {
    await expect(page.locator(selector)).toHaveAttribute('tabindex', '0');
  }
  const terms = await page.getByRole('link', { name: 'Terms' }).boundingBox();
  expect(terms?.width).toBeGreaterThanOrEqual(44);
  expect(terms?.height).toBeGreaterThanOrEqual(44);
});

test('brand accessible name contains its visible label', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('link', { name: /Code Proof release inspector \/ 0\.1/i })).toBeVisible();
});

test('installed shell stays useful offline', async ({ page, context }) => {
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
  await expect(page.locator('h1')).toContainText('PDF bugs');
  await expect(page.getByRole('status').filter({ hasText: 'Offline' })).toBeVisible();
  await context.setOffline(false);
});

test('worker installs with production-only deployment files unavailable', async ({ page }) => {
  await page.goto('/');
  await page.evaluate(() => navigator.serviceWorker.ready);
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  const shell = await page.evaluate(async () => (await fetch('/sw.js')).text());
  expect(shell).not.toContain('staticwebapp.config.json');
  const caches = await page.evaluate(async () => (await caches.keys()).filter((name) => name.startsWith('code-proof-')));
  expect(caches).toContain('code-proof-v2');
});

for (const path of ['/privacy/', '/terms/']) {
  test(`${path} has a single semantic title`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('h1')).toHaveCount(1);
  });
}
