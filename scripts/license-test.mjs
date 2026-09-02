import { readFileSync } from 'node:fs';

const license = readFileSync(new URL('../LICENSE', import.meta.url), 'utf8');
const manifest = readFileSync(new URL('../cli/Cargo.toml', import.meta.url), 'utf8');
const landing = readFileSync(new URL('../site/index.html', import.meta.url), 'utf8');
const terms = readFileSync(new URL('../site/terms/index.html', import.meta.url), 'utf8');

if (!license.includes('Permission is hereby granted, free of charge')) throw new Error('root LICENSE is not MIT');
if (!/^license\s*=\s*"MIT"$/m.test(manifest)) throw new Error('crate does not declare MIT');
if (!landing.includes('Free software') || !landing.includes('MIT licensed')) throw new Error('landing page does not state the free-software license');
if (!terms.includes('MIT License')) throw new Error('terms do not expose the MIT license');
console.log('Free-software status and the MIT license are present in the package and documentation.');
