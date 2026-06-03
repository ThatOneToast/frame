import { compileFrameToCss, formatDiagnostics, FrameCompileError } from './compile.js';
import type { FrameCompileOptions } from './compile.js';

export interface FramePreprocessOptions extends FrameCompileOptions {}

interface StylePreprocessorArgs {
  content: string;
  attributes: Record<string, string | boolean | undefined>;
  filename?: string;
}

export function framePreprocess(options: FramePreprocessOptions = {}) {
  return {
    name: 'frame-preprocess',
    async style({ content, attributes, filename }: StylePreprocessorArgs) {
      if (attributes.lang !== 'frame') {
        return;
      }

      try {
        const result = await compileFrameToCss(content, filename, options);
        return {
          code: result.css,
          map: null
        };
      } catch (error) {
        if (error instanceof FrameCompileError) {
          const details = formatDiagnostics(error.diagnostics);
          throw new Error(details ? `${error.message}\n${details}` : error.message);
        }

        throw error;
      }
    }
  };
}
