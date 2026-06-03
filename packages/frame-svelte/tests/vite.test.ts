import { strict as assert } from 'node:assert';
import { test } from 'node:test';
import { normalizeOptions } from '../src/vite.js';

test('normalizeOptions applies defaults', () => {
  const options = normalizeOptions();

  assert.deepEqual(options.input, ['src/lib/frame/app.frame']);
  assert.equal(options.outDir, 'src/lib/frame');
  assert.equal(options.generatedCssName, 'generated.css');
  assert.equal(options.generatedTsName, 'generated.ts');
  assert.equal(options.watch, true);
});

test('normalizeOptions supports multiple inputs and custom outputs', () => {
  const options = normalizeOptions({
    input: ['src/a.frame', 'src/b.frame'],
    outDir: 'src/generated',
    generatedCssName: 'frame.css',
    generatedTsName: 'frame.ts',
    watch: false
  });

  assert.deepEqual(options.input, ['src/a.frame', 'src/b.frame']);
  assert.equal(options.outDir, 'src/generated');
  assert.equal(options.generatedCssName, 'frame.css');
  assert.equal(options.generatedTsName, 'frame.ts');
  assert.equal(options.watch, false);
});
