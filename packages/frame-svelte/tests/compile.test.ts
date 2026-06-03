import { strict as assert } from 'node:assert';
import { chmod, mkdtemp, readFile, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { compileFrameFile, compileFrameToCss, FrameCompileError, parseDiagnostics } from '../src/compile.js';

test('compileFrameToCss returns CSS from compile-stdin', async () => {
  const dir = await mkdtemp(join(tmpdir(), 'frame-svelte-test-'));
  const frameBin = await fakeFrameBin(dir);

  const result = await compileFrameToCss('card TestCard {}', 'Component.svelte', { frameBin });

  assert.match(result.css, /\.fr-TestCard/);
});

test('compileFrameFile writes generated CSS and TS', async () => {
  const dir = await mkdtemp(join(tmpdir(), 'frame-svelte-test-'));
  const frameBin = await fakeFrameBin(dir);
  const input = join(dir, 'app.frame');
  const outDir = join(dir, 'generated-frame');

  await writeFile(input, 'card TestCard {}');

  const result = await compileFrameFile({ input, outDir, frameBin });

  assert.match(result.css, /\.fr-TestCard/);
  assert.match(result.ts ?? '', /TestCard: 'fr-TestCard'/);
  assert.match(await readFile(join(outDir, 'generated.css'), 'utf8'), /\.fr-TestCard/);
  assert.match(await readFile(join(outDir, 'generated.ts'), 'utf8'), /TestCard/);
});

test('compileFrameFile throws diagnostics for invalid input', async () => {
  const dir = await mkdtemp(join(tmpdir(), 'frame-svelte-test-'));
  const frameBin = await fakeFrameBin(dir);
  const input = join(dir, 'app.frame');

  await writeFile(input, 'unknown Broken {}');

  await assert.rejects(
    () => compileFrameFile({ input, outDir: join(dir, 'generated-frame'), frameBin }),
    (error) => {
      assert.ok(error instanceof FrameCompileError);
      assert.match(error.diagnostics[0]?.message ?? '', /unknown declaration kind/);
      return true;
    }
  );
});

test('parseDiagnostics reads CLI diagnostic lines', () => {
  const diagnostics = parseDiagnostics('Error [0..7]: unknown declaration kind `unknown`', 'App.svelte');

  assert.deepEqual(diagnostics, [
    {
      severity: 'error',
      start: 0,
      end: 7,
      message: 'unknown declaration kind `unknown`',
      filename: 'App.svelte'
    }
  ]);
});

async function fakeFrameBin(dir: string): Promise<string> {
  const path = join(dir, 'frame');
  await writeFile(
    path,
    `#!/usr/bin/env node
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const args = process.argv.slice(2);
function className(source) {
  const match = /(?:card|stack|grid|area)\\s+([A-Za-z_][A-Za-z0-9_]*)/.exec(source);
  return match ? match[1] : 'TestCard';
}

if (args[0] === 'compile-stdin') {
  const source = readFileSync(0, 'utf8');
  if (source.includes('unknown')) {
    console.error('Error [0..7]: unknown declaration kind unknown');
    process.exit(1);
  }
  const name = className(source);
  process.stdout.write('.fr-' + name + ' {\\n  padding: var(--frame-space-medium);\\n}\\n');
  process.exit(0);
}

if (args[0] === 'compile') {
  const input = args[1];
  const outDir = args[args.indexOf('--out') + 1];
  const source = readFileSync(input, 'utf8');
  if (source.includes('unknown')) {
    console.error('Error [0..7]: unknown declaration kind unknown');
    process.exit(1);
  }
  const name = className(source);
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, 'generated.css'), '.fr-' + name + ' {\\n  padding: var(--frame-space-medium);\\n}\\n');
  writeFileSync(join(outDir, 'generated.ts'), "export const ui = {\\n  " + name + ": 'fr-" + name + "',\\n} as const;\\n");
  process.exit(0);
}

process.exit(1);
`
  );
  await chmod(path, 0o755);
  return path;
}
