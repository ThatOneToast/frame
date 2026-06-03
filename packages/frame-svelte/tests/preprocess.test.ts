import { strict as assert } from 'node:assert';
import { chmod, mkdtemp, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { framePreprocess } from '../src/preprocess.js';

test('framePreprocess compiles style lang frame', async () => {
  const frameBin = await fakeFrameBin();
  const processor = framePreprocess({ frameBin });

  const result = await processor.style({
    content: 'card QuickLinkCard {}',
    attributes: { lang: 'frame' },
    filename: 'Component.svelte'
  });

  assert.match(result?.code ?? '', /\.fr-QuickLinkCard/);
});

test('framePreprocess ignores normal style blocks', async () => {
  const frameBin = await fakeFrameBin();
  const processor = framePreprocess({ frameBin });

  const result = await processor.style({
    content: '.card {}',
    attributes: {},
    filename: 'Component.svelte'
  });

  assert.equal(result, undefined);
});

test('framePreprocess throws useful errors for invalid Frame', async () => {
  const frameBin = await fakeFrameBin();
  const processor = framePreprocess({ frameBin });

  await assert.rejects(
    () =>
      processor.style({
        content: 'unknown Broken {}',
        attributes: { lang: 'frame' },
        filename: 'Component.svelte'
      }),
    /unknown declaration kind/
  );
});

async function fakeFrameBin(): Promise<string> {
  const dir = await mkdtemp(join(tmpdir(), 'frame-svelte-test-'));
  const path = join(dir, 'frame');
  await writeFile(
    path,
    `#!/usr/bin/env node
import { readFileSync } from 'node:fs';
const source = readFileSync(0, 'utf8');
if (source.includes('unknown')) {
  console.error('Error [0..7]: unknown declaration kind unknown');
  process.exit(1);
}
const match = /card\\s+([A-Za-z_][A-Za-z0-9_]*)/.exec(source);
process.stdout.write('.fr-' + (match ? match[1] : 'TestCard') + ' {\\n  padding: var(--frame-space-medium);\\n}\\n');
`
  );
  await chmod(path, 0o755);
  return path;
}
