import { createServer } from 'node:http';
import { readFile, stat } from 'node:fs/promises';
import { extname, join, normalize } from 'node:path';

const root = new URL('../dist/site/', import.meta.url).pathname;
const port = Number(process.env.PORT ?? 4173);
const types = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.svg': 'image/svg+xml',
  '.webp': 'image/webp',
  '.jpg': 'image/jpeg',
  '.png': 'image/png',
  '.xml': 'text/xml; charset=utf-8',
  '.txt': 'text/plain; charset=utf-8'
};
const policy = "default-src 'self'; img-src 'self'; style-src 'self'; script-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'; frame-ancestors 'none'";

createServer(async (request, response) => {
  const url = new URL(request.url ?? '/', `http://${request.headers.host}`);
  // Azure Static Web Apps consumes this control file and responds 404. Keep
  // local tests faithful to that production behavior.
  if (url.pathname === '/staticwebapp.config.json') {
    response.writeHead(404, { 'Content-Type': 'text/html; charset=utf-8' });
    response.end('Not found');
    return;
  }
  let requested = decodeURIComponent(url.pathname);
  if (requested.endsWith('/')) requested += 'index.html';
  const file = normalize(join(root, requested));
  if (!file.startsWith(root)) {
    response.writeHead(403).end();
    return;
  }
  try {
    if (!(await stat(file)).isFile()) throw new Error('not a file');
    const headers = {
      'Content-Type': types[extname(file)] ?? 'application/octet-stream',
      'Content-Security-Policy': policy,
      'X-Content-Type-Options': 'nosniff',
      'Referrer-Policy': 'strict-origin-when-cross-origin',
      'Permissions-Policy': 'camera=(), microphone=(), geolocation=()',
      'Cache-Control': file.endsWith('/sw.js') ? 'no-cache' : 'public, max-age=30, must-revalidate'
    };
    response.writeHead(200, headers);
    response.end(await readFile(file));
  } catch {
    response.writeHead(404, {
      'Content-Type': 'text/html; charset=utf-8',
      'Content-Security-Policy': policy,
      'X-Content-Type-Options': 'nosniff',
      'Referrer-Policy': 'strict-origin-when-cross-origin',
      'Permissions-Policy': 'camera=(), microphone=(), geolocation=()'
    });
    response.end(await readFile(join(root, '404.html')));
  }
}).listen(port, '127.0.0.1');
