import './style.css';
import { demoTranscript } from './demo-transcript';

const toast = document.querySelector<HTMLDivElement>('#toast');
const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
const canonical = document.querySelector<HTMLLinkElement>('link[rel="canonical"]');
const demoBanner = document.querySelector<HTMLDivElement>('#demo-banner');
const demoTitle = document.querySelector<HTMLElement>('#demo-title');
const installTitle = document.querySelector<HTMLElement>('#install-title');
const routeStatus = document.querySelector<HTMLParagraphElement>('#route-status');
const description = document.querySelector<HTMLMetaElement>('meta[name="description"]');
const openGraphTitle = document.querySelector<HTMLMetaElement>('meta[property="og:title"]');
const openGraphDescription = document.querySelector<HTMLMetaElement>('meta[property="og:description"]');
const openGraphUrl = document.querySelector<HTMLMetaElement>('meta[property="og:url"]');
const twitterTitle = document.querySelector<HTMLMetaElement>('meta[name="twitter:title"]');
const twitterDescription = document.querySelector<HTMLMetaElement>('meta[name="twitter:description"]');

const landingTitle = 'Code Proof — inspect Markdown PDFs before release';
const landingDescription = 'Check a finished Markdown PDF for broken code, page overflow, and internal links before release.';
const demoPageTitle = 'Demo — Code Proof';
const demoDescription = 'Run the bundled Code Proof sample and review its expected PDF defect.';

const demoOutput = document.querySelector<HTMLElement>('#demo-output');
if (demoOutput) {
  demoTranscript.forEach((text, index) => {
    const line = document.createElement(index === 0 ? 'strong' : 'span');
    line.classList.add('demo-line');
    if (index === 1) line.classList.add('defect');
    line.dataset.step = String(index + 1);
    line.textContent = text;
    demoOutput.append(line);
    if (index < demoTranscript.length - 1) demoOutput.append('\n');
  });
}

document.querySelectorAll<HTMLButtonElement>('.copy-button').forEach((button) => {
  button.addEventListener('click', async () => {
    const value = button.dataset.copy ?? '';
    try {
      await navigator.clipboard.writeText(value);
      const label = button.querySelector<HTMLElement>('.button-label');
      if (label) label.textContent = 'Copied to clipboard';
      else button.textContent = 'Copied';
      if (toast) toast.textContent = `Copied: ${value}`;
      window.setTimeout(() => {
        if (label) label.textContent = 'Copy install command';
        else button.textContent = button.dataset.defaultLabel ?? 'Copy command';
      }, 1800);
    } catch {
      if (toast) toast.textContent = `Copy unavailable. Select this command: ${value}`;
    }
  });
});

const replay = document.querySelector<HTMLButtonElement>('#replay');
const demoStatus = document.querySelector<HTMLParagraphElement>('#demo-status');
const demoLines = [...document.querySelectorAll<HTMLElement>('.demo-line')];
let demoRun = 0;

const runDemo = () => {
  const currentRun = ++demoRun;
  const terminal = document.querySelector<HTMLElement>('.terminal');
  terminal?.classList.add('replaying');
  terminal?.setAttribute('aria-busy', 'true');
  demoLines.forEach((line) => line.classList.remove('revealed'));
  if (replay) replay.setAttribute('aria-disabled', 'true');
  if (demoStatus) demoStatus.textContent = 'Proof run started.';
  demoLines.forEach((line, index) => {
    window.setTimeout(() => {
      if (currentRun !== demoRun) return;
      line.classList.add('revealed');
      if (index === demoLines.length - 1) {
        if (replay) replay.setAttribute('aria-disabled', 'false');
        terminal?.setAttribute('aria-busy', 'false');
        if (demoStatus) demoStatus.textContent = 'Proof run complete: one expected defect found.';
      }
    }, reduceMotion ? 0 : 250 * (index + 1));
  });
};

replay?.addEventListener('click', () => {
  if (replay.getAttribute('aria-disabled') !== 'true') runDemo();
});

const updateRoute = (moveFocus: boolean) => {
  const inDemo = new URLSearchParams(window.location.search).get('demo') === '1';
  const pageTitle = inDemo ? demoPageTitle : landingTitle;
  const pageDescription = inDemo ? demoDescription : landingDescription;
  const pageUrl = `${window.location.origin}${inDemo ? '/?demo=1' : '/'}`;
  document.title = pageTitle;
  if (canonical) canonical.href = pageUrl;
  if (description) description.content = pageDescription;
  if (openGraphTitle) openGraphTitle.content = pageTitle;
  if (openGraphDescription) openGraphDescription.content = pageDescription;
  if (openGraphUrl) openGraphUrl.content = pageUrl;
  if (twitterTitle) twitterTitle.content = pageTitle;
  if (twitterDescription) twitterDescription.content = pageDescription;
  if (demoBanner) demoBanner.hidden = !inDemo;
  if (inDemo) {
    runDemo();
    if (moveFocus) {
      window.requestAnimationFrame(() => demoTitle?.focus());
      if (routeStatus) routeStatus.textContent = 'Demo opened. Sample data is active and nothing is saved.';
    }
    return;
  }
  if (window.location.hash === '#install' && moveFocus) {
    window.requestAnimationFrame(() => installTitle?.focus());
    if (routeStatus) routeStatus.textContent = 'Install commands opened.';
  }
};

if (demoBanner) {
  updateRoute(true);
  window.addEventListener('popstate', () => updateRoute(true));
  window.addEventListener('hashchange', () => updateRoute(true));
}

const offline = document.querySelector<HTMLDivElement>('#offline-note');
const syncNetwork = () => {
  if (offline) offline.hidden = navigator.onLine;
};
window.addEventListener('online', syncNetwork);
window.addEventListener('offline', syncNetwork);
syncNetwork();

if ('serviceWorker' in navigator && import.meta.env.PROD) {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
}
