import { relative, resolve } from 'node:path';
import type { FrameCompileOptions } from './compile.js';
import { compileFrameFile, formatDiagnostics, FrameCompileError, generatedOutputPaths } from './compile.js';

export interface FramePluginOptions extends FrameCompileOptions {
  input?: string | string[];
  outDir?: string;
  watch?: boolean;
}

interface ViteConfig {
  root: string;
  command: 'serve' | 'build';
}

interface ViteServer {
  watcher: {
    add(path: string | string[]): void;
    on(event: 'change', callback: (path: string) => void): void;
  };
  ws: {
    send(payload: { type: 'full-reload'; path?: string }): void;
  };
}

export function framePlugin(options: FramePluginOptions = {}) {
  const settings = normalizeOptions(options);
  let config: ViteConfig = {
    root: process.cwd(),
    command: 'serve'
  };

  async function compileAll(failOnError: boolean): Promise<void> {
    for (const input of settings.input) {
      try {
        const result = await compileFrameFile({
          ...settings,
          input,
          cwd: config.root
        });
        const cssPath = relative(config.root, resolve(config.root, settings.outDir, settings.generatedCssName));
        const tsPath = relative(config.root, resolve(config.root, settings.outDir, settings.generatedTsName));
        console.info(`[frame] generated ${cssPath} and ${tsPath}`);

        if (result.diagnostics.length > 0) {
          console.warn(formatDiagnostics(result.diagnostics));
        }
      } catch (error) {
        reportCompileError(error, failOnError);
      }
    }
  }

  return {
    name: 'frame-svelte',
    configResolved(resolvedConfig: ViteConfig) {
      config = resolvedConfig;
    },
    async buildStart() {
      await compileAll(config.command === 'build');
    },
    configureServer(server: ViteServer) {
      if (!settings.watch) {
        return;
      }

      const inputs = settings.input.map((input) => resolve(config.root, input));
      server.watcher.add(inputs);
      server.watcher.on('change', async (changedPath) => {
        if (!inputs.includes(resolve(changedPath))) {
          return;
        }

        await compileAll(false);
        for (const outputPath of generatedOutputPaths(resolve(config.root, settings.outDir), settings)) {
          server.watcher.add(outputPath);
        }
        server.ws.send({ type: 'full-reload', path: '*' });
      });
    }
  };
}

export function normalizeOptions(options: FramePluginOptions = RequiredDefaults): Required<Pick<FramePluginOptions, 'outDir' | 'generatedCssName' | 'generatedTsName' | 'watch'>> & {
  input: string[];
  frameBin?: string;
  cwd?: string;
} {
  const input = options.input ?? RequiredDefaults.input;

  return {
    input: Array.isArray(input) ? input : [input],
    outDir: options.outDir ?? RequiredDefaults.outDir,
    generatedCssName: options.generatedCssName ?? RequiredDefaults.generatedCssName,
    generatedTsName: options.generatedTsName ?? RequiredDefaults.generatedTsName,
    watch: options.watch ?? RequiredDefaults.watch,
    frameBin: options.frameBin,
    cwd: options.cwd
  };
}

const RequiredDefaults = {
  input: 'src/lib/frame/app.frame',
  outDir: 'src/lib/frame',
  generatedCssName: 'generated.css',
  generatedTsName: 'generated.ts',
  watch: true
} as const;

function reportCompileError(error: unknown, failOnError: boolean): void {
  if (error instanceof FrameCompileError) {
    const details = formatDiagnostics(error.diagnostics);
    const message = details ? `${error.message}\n${details}` : error.message;

    if (failOnError) {
      throw new Error(message);
    }

    console.error(`[frame] ${message}`);
    return;
  }

  if (failOnError) {
    throw error;
  }

  console.error(error);
}
