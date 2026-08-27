import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('landing page is usable, quiet, and accessible', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => {
    if (message.type() === 'error') errors.push(message.text());
  });
  await page.goto('/');
  await expect(page).toHaveTitle(/Code Proof/);
  await expect(page.locator('main')).toBeVisible();
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.locator('h1')).toContainText('PDF bugs');
  await expect(page.locator('img[alt]')).toHaveCount(1);

  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((issue) => ['serious', 'critical'].includes(issue.impact ?? ''))).toEqual([]);
  expect(errors).toEqual([]);
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
});

for (const path of ['/privacy/', '/terms/']) {
  test(`${path} has a single semantic title`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('h1')).toHaveCount(1);
  });
}
