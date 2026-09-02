import './style.css';

const toast = document.querySelector<HTMLDivElement>('#toast');
const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
const canonical = document.querySelector<HTMLLinkElement>('link[rel="canonical"]');
const demoBanner = document.querySelector<HTMLDivElement>('#demo-banner');
const demoTitle = document.querySelector<HTMLElement>('#demo-title');
const installTitle = document.querySelector<HTMLElement>('#install-title');
const routeStatus = document.querySelector<HTMLParagraphElement>('#route-status');

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

const runDemo = () => {
  document.querySelector('.terminal')?.classList.add('replaying');
  demoLines.forEach((line) => line.classList.remove('revealed'));
  if (replay) replay.disabled = true;
  if (demoStatus) demoStatus.textContent = 'Proof run started.';
  demoLines.forEach((line, index) => {
    window.setTimeout(() => {
      line.classList.add('revealed');
      if (index === demoLines.length - 1) {
        if (replay) replay.disabled = false;
        if (demoStatus) demoStatus.textContent = 'Proof run complete: one expected defect found.';
      }
    }, reduceMotion ? 0 : 250 * (index + 1));
  });
};

replay?.addEventListener('click', runDemo);

const updateRoute = (moveFocus: boolean) => {
  const inDemo = new URLSearchParams(window.location.search).get('demo') === '1';
  if (demoBanner) demoBanner.hidden = !inDemo;
  if (inDemo) {
    document.title = 'Demo — Code Proof';
    if (canonical) canonical.href = `${window.location.origin}/?demo=1`;
    runDemo();
    if (moveFocus) {
      window.requestAnimationFrame(() => demoTitle?.focus());
      if (routeStatus) routeStatus.textContent = 'Demo opened. Sample data is active and nothing is saved.';
    }
    return;
  }

  document.title = 'Code Proof — inspect Markdown PDFs before release';
  if (canonical) canonical.href = `${window.location.origin}/`;
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
