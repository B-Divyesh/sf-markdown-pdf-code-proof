import { readdir, readFile, stat, writeFile } from 'node:fs/promises';
import { join, relative, sep } from 'node:path';

const root = new URL('../dist/site/', import.meta.url).pathname;

async function files(directory) {
  const output = [];
  for (const name of await readdir(directory)) {
    const path = join(directory, name);
    if ((await stat(path)).isDirectory()) output.push(...await files(path));
    else output.push(path);
  }
  return output;
}

const built = await files(root);
const shell = built
  .filter((path) => !path.endsWith('sw.js') && !path.endsWith('.map'))
  .map((path) => `/${relative(root, path).split(sep).join('/')}`)
  .map((path) => path.endsWith('/index.html') ? path.slice(0, -10) : path);
const swPath = join(root, 'sw.js');
const sw = await readFile(swPath, 'utf8');
await writeFile(swPath, sw.replace(/const SHELL = \[[^;]+;/, `const SHELL = ${JSON.stringify(shell)};`));

const budgets = { js: 200 * 1024, css: 50 * 1024, image: 300 * 1024 };
for (const path of built) {
  const size = (await stat(path)).size;
  if (path.endsWith('.js') && !path.endsWith('sw.js') && size > budgets.js) throw new Error(`JS budget exceeded: ${path} (${size})`);
  if (path.endsWith('.css') && size > budgets.css) throw new Error(`CSS budget exceeded: ${path} (${size})`);
  if (path.endsWith('.webp') && size > budgets.image) throw new Error(`Image budget exceeded: ${path} (${size})`);
}
