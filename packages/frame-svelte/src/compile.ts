import { spawn } from 'node:child_process';
import { mkdtemp, readFile, rm, writeFile, mkdir } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, isAbsolute, join, resolve } from 'node:path';

export interface FrameDiagnostic {
  severity: 'error' | 'warning' | 'info';
  message: string;
  start?: number;
  end?: number;
  filename?: string;
}

export interface FrameCompileResult {
  css: string;
  ts?: string;
  diagnostics: FrameDiagnostic[];
}

export interface FrameCompileOptions {
  cwd?: string;
  frameBin?: string;
  generatedCssName?: string;
  generatedTsName?: string;
  include?: readonly string[];
}

export interface ResolvedFrameCommand {
  command: string;
  args: string[];
}

export interface CompileFrameFileOptions extends FrameCompileOptions {
  input: string;
  outDir: string;
}

export function resolveFrameCommand(options: FrameCompileOptions = {}): ResolvedFrameCommand {
  const configured = options.frameBin ?? process.env.FRAME_BIN;

  if (configured) {
    const [command, ...args] = configured.split(/\s+/).filter(Boolean);
    return { command, args };
  }

  return process.env.FRAME_USE_CARGO === '1'
    ? { command: 'cargo', args: ['run', '-p', 'frame_cli', '--quiet', '--'] }
    : { command: 'frame', args: [] };
}

export async function compileFrameToCss(
  source: string,
  filename?: string,
  options: FrameCompileOptions = {}
): Promise<FrameCompileResult> {
  const frame = resolveFrameCommand(options);
  const args = [...frame.args, 'compile-stdin', '--css-only'];

  if (filename) {
    args.push('--filename', filename);
  }

  const result = await runFrame(frame.command, args, source, options.cwd);

  if (result.code !== 0) {
    throw new FrameCompileError('Frame inline compilation failed', parseDiagnostics(result.stderr, filename));
  }

  return {
    css: result.stdout,
    diagnostics: parseDiagnostics(result.stderr, filename)
  };
}

export async function compileFrameFile(options: CompileFrameFileOptions): Promise<FrameCompileResult> {
  const cwd = options.cwd ?? process.cwd();
  const input = resolvePath(cwd, options.input);
  const outDir = resolvePath(cwd, options.outDir);
  const tempDir = await mkdtemp(join(tmpdir(), 'frame-svelte-'));
  const frame = resolveFrameCommand(options);

  try {
    const result = await runFrame(
      frame.command,
      [...frame.args, 'compile', input, '--out', tempDir, ...includeArgs(options.include ?? [])],
      undefined,
      cwd
    );

    if (result.code !== 0) {
      throw new FrameCompileError(`Frame compilation failed for ${input}`, parseDiagnostics(result.stderr, input));
    }

    const css = await readFile(join(tempDir, 'generated.css'), 'utf8');
    const ts = await readFile(join(tempDir, 'generated.ts'), 'utf8');

    await mkdir(outDir, { recursive: true });
    await writeFile(join(outDir, options.generatedCssName ?? 'generated.css'), css);
    await writeFile(join(outDir, options.generatedTsName ?? 'generated.ts'), ts);

    return {
      css,
      ts,
      diagnostics: parseDiagnostics(result.stderr, input)
    };
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
}

function includeArgs(include: readonly string[]): string[] {
  return include.flatMap((path) => ['--include', path]);
}

export class FrameCompileError extends Error {
  diagnostics: FrameDiagnostic[];

  constructor(message: string, diagnostics: FrameDiagnostic[]) {
    super(message);
    this.name = 'FrameCompileError';
    this.diagnostics = diagnostics;
  }
}

export function formatDiagnostics(diagnostics: FrameDiagnostic[]): string {
  return diagnostics
    .map((diagnostic) => {
      const location =
        diagnostic.filename && diagnostic.start !== undefined && diagnostic.end !== undefined
          ? `${diagnostic.filename}:${diagnostic.start}-${diagnostic.end}`
          : diagnostic.filename;
      const prefix = location ? `${location}: ` : '';

      return `${prefix}${diagnostic.severity}: ${diagnostic.message}`;
    })
    .join('\n');
}

export function parseDiagnostics(stderr: string, filename?: string): FrameDiagnostic[] {
  return stderr
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .map((line) => {
      const match = /^(Error|Warning|Info) \[(\d+)\.\.(\d+)\]: (.+)$/.exec(line);

      if (match) {
        return {
          severity: match[1].toLowerCase() as FrameDiagnostic['severity'],
          start: Number(match[2]),
          end: Number(match[3]),
          message: match[4],
          filename
        };
      }

      return {
        severity: 'error',
        message: line,
        filename
      } satisfies FrameDiagnostic;
    });
}

function resolvePath(cwd: string, path: string): string {
  return isAbsolute(path) ? path : resolve(cwd, path);
}

function runFrame(
  command: string,
  args: string[],
  input: string | undefined,
  cwd: string | undefined
): Promise<{ code: number | null; stdout: string; stderr: string }> {
  return new Promise((resolvePromise, reject) => {
    const child = spawn(command, args, {
      cwd,
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';

    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');
    child.stdout.on('data', (chunk) => {
      stdout += chunk;
    });
    child.stderr.on('data', (chunk) => {
      stderr += chunk;
    });
    child.on('error', (error) => {
      reject(new Error(`Failed to run Frame compiler (${command}): ${error.message}`));
    });
    child.on('close', (code) => {
      resolvePromise({ code, stdout, stderr });
    });

    if (input !== undefined) {
      child.stdin.end(input);
    } else {
      child.stdin.end();
    }
  });
}

export function generatedOutputPaths(outDir: string, options: FrameCompileOptions = {}): string[] {
  return [
    join(outDir, options.generatedCssName ?? 'generated.css'),
    join(outDir, options.generatedTsName ?? 'generated.ts')
  ];
}

export async function ensureDirectoryForFile(path: string): Promise<void> {
  await mkdir(dirname(path), { recursive: true });
}
