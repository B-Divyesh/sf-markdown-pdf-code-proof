import { readFile, writeFile } from 'node:fs/promises';

const auditPath = new URL('../.factory/copy-audit.md', import.meta.url);
const sitePath = new URL('../site/index.html', import.meta.url);
const runtimePath = new URL('../site/src/main.ts', import.meta.url);
const transcriptPath = new URL('../site/src/demo-transcript.ts', import.meta.url);
const readmePath = new URL('../README.md', import.meta.url);
const claimsPath = new URL('../.factory/claims.json', import.meta.url);
const write = process.argv.includes('--write');
const check = process.argv.includes('--check');

if (!write && !check) {
  throw new Error('Use --write to regenerate the audit or --check to verify it is current.');
}

const bannedWords = [
  'leverage',
  'seamless',
  'effortless',
  'robust',
  'powerful',
  'intuitive',
  'reimagine',
  'supercharge',
  'unlock',
  'delightful',
  'journey',
  'ecosystem',
  'AI-powered'
];

const decode = (value) => value
  .replaceAll('&amp;', '&')
  .replaceAll('&lt;', '<')
  .replaceAll('&gt;', '>')
  .replaceAll('&quot;', '"')
  .replaceAll('&#39;', "'");

const clean = (value) => decode(value.replace(/\s+/g, ' ').trim());
const words = (value) => clean(value).split(/\s+/).filter(Boolean).length;
const markdownEscape = (value) => value.replaceAll('|', '\\|');

function splitSentences(value) {
  const text = clean(value);
  if (!text) return [];
  return text.split(/(?<=[.!?])\s+(?=[A-Z`])/).map(clean).filter(Boolean);
}

function siteEntries(html) {
  const body = html.match(/<body\b[^>]*>([\s\S]*)<\/body>/i)?.[1] ?? '';
  const withoutScript = body.replace(/<script\b[^>]*>[\s\S]*?<\/script>/gi, '');
  const withBreaks = withoutScript
    .replace(/<\/?(?:a|article|aside|button|code|div|figcaption|footer|h[1-6]|header|li|main|nav|ol|p|pre|section|small|span|strong|ul)\b[^>]*>/gi, '\n')
    .replace(/<br\s*\/?>/gi, '\n')
    .replace(/<[^>]+>/g, '');
  const visible = withBreaks
    .split('\n')
    .flatMap(splitSentences);
  const labels = [...withoutScript.matchAll(/\b(?:alt|aria-label)="([^"]+)"/gi)]
    .map((match) => clean(match[1]));
  return [...visible, ...labels].filter(Boolean);
}

function runtimeEntries(source) {
  const literals = [...source.matchAll(/(['"`])((?:\\.|(?!\1)[\s\S])*?)\1/g)]
    .map((match) => clean(match[2].replace(/\$\{[^}]+\}/g, '[value]')))
    .filter((value) => /[A-Za-z]/.test(value))
    .filter((value) => /[ .:!—]/.test(value))
    .filter((value) => !/^(?:\(|#|\.|\/|src\/)/.test(value))
    .filter((value) => !value.includes('prefers-reduced-motion'));
  return [...new Set(literals.flatMap(splitSentences))];
}

function readmeEntries(markdown) {
  const lines = markdown.split('\n');
  const entries = [];
  let inCode = false;
  let paragraph = [];
  const flush = () => {
    if (!paragraph.length) return;
    entries.push(...splitSentences(paragraph.join(' ')
      .replace(/\[([^\]]+)\]\([^)]*\)/g, '$1')));
    paragraph = [];
  };
  for (const line of lines) {
    if (line.startsWith('```')) {
      flush();
      inCode = !inCode;
      continue;
    }
    if (inCode) continue;
    const heading = line.match(/^#{1,6}\s+(.+)$/);
    if (heading) {
      flush();
      entries.push(clean(heading[1]));
      continue;
    }
    const listItem = line.match(/^[-*]\s+(.+)$/);
    if (listItem) {
      flush();
      paragraph.push(listItem[1]);
      continue;
    }
    if (!line.trim()) {
      flush();
      continue;
    }
    paragraph.push(line.trim());
  }
  flush();
  return entries;
}

function table(entries) {
  return [
    '| Words | Copy |',
    '| ---: | --- |',
    ...entries.map((entry) => `| ${words(entry)} | ${markdownEscape(entry)} |`)
  ].join('\n');
}

const [site, runtime, transcriptSource, readme, claimsText] = await Promise.all([
  readFile(sitePath, 'utf8'),
  readFile(runtimePath, 'utf8'),
  readFile(transcriptPath, 'utf8'),
  readFile(readmePath, 'utf8'),
  readFile(claimsPath, 'utf8')
]);
const landing = siteEntries(site);
const runtimeCopy = runtimeEntries(runtime);
const transcript = JSON.parse(transcriptSource.match(/=\s*(\[[\s\S]*\])\s+as const/)?.[1] ?? '[]');
const readmeCopy = readmeEntries(readme);
const allCopy = [...landing, ...runtimeCopy, ...transcript, ...readmeCopy];
const overlong = allCopy.filter((entry) => words(entry) > 22);
const banned = allCopy.flatMap((entry) => bannedWords
  .filter((word) => new RegExp(`\\b${word.replace('-', '\\-')}\\b`, 'i').test(entry))
  .map((word) => `${word}: ${entry}`));
const claims = JSON.parse(claimsText);
const inputClaim = claims.find((claim) => claim.id === 'input-unchanged');

if (!clean(readme).includes('Code Proof does not edit the supplied Markdown source.')) {
  throw new Error('README must state the registered source-integrity contract exactly.');
}
if (!inputClaim || inputClaim.test !== 'cargo test --test cli input_files_remain_unchanged_in_existing_pdf_and_custom_renderer_checks -- --exact') {
  throw new Error('The source-integrity promise must have its exact input-unchanged claim test.');
}
const requiredTerms = [
  'Check Markdown against the finished PDF.',
  'Check links, syntax color, and text that runs outside the page.',
  'Review the HTML proof sheet',
  'Each language-tagged code fence warns when its matching PDF text has no syntax color.'
];
for (const required of requiredTerms) {
  if (!allCopy.includes(required)) throw new Error(`required public wording is missing: ${required}`);
}
for (const retired of ['Check the source against the PDF.', 'code colors', 'Review the result', 'non-default PDF color']) {
  if (clean(`${site}\n${readme}`).includes(retired)) throw new Error(`retired public wording remains: ${retired}`);
}
if (transcript[0] !== 'DEMO HOLD — do not release — 1 expected defect found') {
  throw new Error('The browser demo must use the exact failed release decision.');
}
if (overlong.length || banned.length) {
  throw new Error([
    ...overlong.map((entry) => `more than 22 words: ${entry}`),
    ...banned.map((entry) => `banned word: ${entry}`)
  ].join('\n'));
}

const output = `# Landing and README copy audit

Generated from the current landing HTML, landing runtime feedback, and README
prose. Counts use whitespace-delimited words. Regenerate with
\`node scripts/audit-copy.mjs --write\`; do not edit this file by hand.

Every captured sentence and visible fragment is 22 words or fewer. The audit
also rejects the factory's banned marketing words. README source-integrity copy
is covered by the \`input-unchanged\` claim and its exact CLI fixture test.

## Landing page copy

${table(landing)}

## Landing runtime feedback

${table(runtimeCopy)}

## Generated CLI demo transcript

${table(transcript)}

## README prose

${table(readmeCopy)}

## Terminology

| Concept | Required term |
| --- | --- |
| Source document | Markdown |
| Finished document | PDF |
| Markdown code region | code fence |
| Browser-readable result | HTML proof sheet |
| Machine-readable result | JSON report |
| Recorded browser example | sample failed release check |
| Program that creates a PDF | renderer |
| Failed release decision | HOLD — do not release |

## Freshness regression

\`npm run test:copy-audit\` regenerates this output in memory and compares it
byte-for-byte with this checked-in file. It also checks the 22-word limit,
banned-word list, required terminology, generated CLI transcript, and the
exact registered source-integrity claim test.
`;

if (write) {
  await writeFile(auditPath, output);
}
if (check) {
  const current = await readFile(auditPath, 'utf8');
  if (current !== output) {
    throw new Error('copy audit is stale; run node scripts/audit-copy.mjs --write');
  }
}
