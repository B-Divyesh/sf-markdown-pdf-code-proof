import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const manifest = readFileSync(new URL('../cli/Cargo.toml', import.meta.url), 'utf8');
const declared = manifest.match(/^rust-version\s*=\s*"([^"]+)"/m)?.[1];
if (!declared) throw new Error('cli/Cargo.toml must declare rust-version');

const metadata = JSON.parse(execFileSync('cargo', ['metadata', '--locked', '--format-version', '1'], {
  encoding: 'utf8'
}));
const parse = (value) => value.split('.').map(Number);
const compare = (left, right) => {
  const a = parse(left);
  const b = parse(right);
  for (let index = 0; index < Math.max(a.length, b.length); index += 1) {
    const difference = (a[index] ?? 0) - (b[index] ?? 0);
    if (difference !== 0) return difference;
  }
  return 0;
};
const incompatible = metadata.packages
  .filter((item) => item.rust_version && compare(item.rust_version, declared) > 0)
  .map((item) => `${item.name} ${item.version} requires Rust ${item.rust_version}`);

if (incompatible.length > 0) {
  throw new Error(`Declared Rust ${declared} is lower than locked dependencies:\n${incompatible.join('\n')}`);
}
console.log(`MSRV contract holds: Rust ${declared} covers every locked dependency.`);
