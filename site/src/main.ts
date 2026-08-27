import './style.css';

const toast = document.querySelector<HTMLDivElement>('#toast');
const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

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
        else button.textContent = 'Copy';
      }, 1800);
    } catch {
      if (toast) toast.textContent = `Copy unavailable. Select this command: ${value}`;
    }
  });
});

const replay = document.querySelector<HTMLButtonElement>('#replay');
const demoStatus = document.querySelector<HTMLParagraphElement>('#demo-status');
const demoLines = [...document.querySelectorAll<HTMLElement>('.demo-line')];

replay?.addEventListener('click', () => {
  document.querySelector('.terminal')?.classList.add('replaying');
  demoLines.forEach((line) => line.classList.remove('revealed'));
  replay.disabled = true;
  if (demoStatus) demoStatus.textContent = 'Proof run started.';
  demoLines.forEach((line, index) => {
    window.setTimeout(() => {
      line.classList.add('revealed');
      if (index === demoLines.length - 1) {
        replay.disabled = false;
        if (demoStatus) demoStatus.textContent = 'Proof run complete: hold, with one error and one warning.';
      }
    }, reduceMotion ? 0 : 250 * (index + 1));
  });
});

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
