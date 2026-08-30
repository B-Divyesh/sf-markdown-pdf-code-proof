import { execFileSync } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const root = mkdtempSync(join(tmpdir(), 'codeproof-install-'));
const binary = process.platform === 'win32' ? join(root, 'bin', 'codeproof.exe') : join(root, 'bin', 'codeproof');

try {
  execFileSync('cargo', [
    'install', '--git', 'https://github.com/B-Divyesh/sf-markdown-pdf-code-proof.git',
    '--locked', '--root', root, 'codeproof'
  ], { stdio: 'inherit' });
  execFileSync(binary, ['--version'], { stdio: 'inherit' });
} finally {
  rmSync(root, { recursive: true, force: true });
}
