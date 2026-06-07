import { EditorPane } from './generated/generated';

export interface EditorDiagnostic {
  line: number;
  message: string;
  severity: number;
}

export class FrameEditor {
  private el: HTMLElement;
  private content = '';
  private diagnostics: EditorDiagnostic[] = [];
  onChange: ((content: string) => void) | null = null;

  constructor(parent: HTMLElement) {
    this.el = document.createElement('div');
    this.el.className = EditorPane;
    this.el.style.cssText = `
      display: flex;
      flex-direction: column;
      width: 100%;
      height: 100%;
      overflow: hidden;
    `;
    this.el.innerHTML = `
      <div style="display:flex;flex:1;overflow:hidden;">
        <div id="line-numbers" style="
          width: 48px;
          padding: 12px 8px;
          text-align: right;
          color: var(--frame-color-muted, #6e6e7a);
          font-size: 13px;
          line-height: 20px;
          user-select: none;
          overflow: hidden;
          border-right: 1px solid var(--frame-color-ide-border, #1a1a1f);
          background: var(--frame-surface-ide-panel, #111114);
        "></div>
        <div id="editor-content" style="
          flex: 1;
          position: relative;
          overflow: auto;
        ">
          <pre id="code-display" style="
            margin: 0;
            padding: 12px 16px;
            min-height: 100%;
            white-space: pre;
            word-wrap: normal;
            overflow-wrap: normal;
            tab-size: 2;
            font-family: inherit;
            font-size: 13px;
            line-height: 20px;
            color: var(--frame-color-ide-text, #c8c8d0);
            background: transparent;
            outline: none;
          " contenteditable="true" spellcheck="false"></pre>
        </div>
      </div>
    `;
    parent.appendChild(this.el);

    const display = this.el.querySelector('#code-display') as HTMLElement;
    display.addEventListener('input', () => {
      this.content = display.innerText;
      this.highlight();
      this.renderDiagnostics();
      if (this.onChange) this.onChange(this.content);
    });

    display.addEventListener('keydown', (e) => {
      if (e.key === 'Tab') {
        e.preventDefault();
        document.execCommand('insertText', false, '  ');
      }
    });
  }

  setContent(text: string) {
    this.content = text;
    const display = this.el.querySelector('#code-display') as HTMLElement;
    display.innerText = text;
    this.highlight();
    this.renderLineNumbers();
    this.renderDiagnostics();
  }

  setDiagnostics(diagnostics: EditorDiagnostic[]) {
    this.diagnostics = diagnostics;
    this.renderDiagnostics();
  }

  private highlight() {
    const display = this.el.querySelector('#code-display') as HTMLElement;
    const text = display.innerText;
    const lines = text.split('\n');
    const keywords = new Set([
      'component', 'state', 'props', 'view', 'slot',
      'screen', 'panel', 'section', 'stack', 'row', 'grid', 'split',
      'action', 'link', 'menu', 'toolbar', 'tabs', 'input', 'editor',
      'toggle', 'choice', 'select', 'composer', 'title', 'text',
      'label', 'badge', 'avatar', 'icon', 'image', 'media', 'list',
      'feed', 'data', 'item', 'empty', 'card', 'dialog', 'popover',
      'on', 'when', 'bind', 'for', 'in', 'key',
      'surface', 'background', 'color', 'padding', 'margin', 'radius',
      'border', 'shadow', 'outline', 'gap', 'columns', 'rows',
      'hover', 'focus', 'active', 'disabled', 'checked',
    ]);

    const html = lines.map((line) => {
      let result = '';
      let i = 0;
      while (i < line.length) {
        if (line[i] === '#' || (line[i] === '/' && line[i + 1] === '/')) {
          result += `<span style="color:var(--frame-color-ide-muted)">${escapeHtml(line.slice(i))}</span>`;
          break;
        }
        if (line[i] === '"') {
          const end = line.indexOf('"', i + 1);
          if (end === -1) {
            result += escapeHtml(line[i]);
            i++;
            continue;
          }
          result += `<span style="color:var(--frame-color-ide-warning)">${escapeHtml(line.slice(i, end + 1))}</span>`;
          i = end + 1;
          continue;
        }
        if (/\d/.test(line[i])) {
          let j = i;
          while (j < line.length && /[\d.%]/.test(line[j])) j++;
          result += `<span style="color:var(--frame-color-ide-error)">${escapeHtml(line.slice(i, j))}</span>`;
          i = j;
          continue;
        }
        if (/[A-Za-z_]/.test(line[i])) {
          let j = i;
          while (j < line.length && /[A-Za-z0-9_-]/.test(line[j])) j++;
          const word = line.slice(i, j);
          if (keywords.has(word)) {
            result += `<span style="color:var(--frame-color-ide-accent)">${escapeHtml(word)}</span>`;
          } else {
            result += escapeHtml(word);
          }
          i = j;
          continue;
        }
        result += escapeHtml(line[i]);
        i++;
      }
      return result;
    }).join('\n');

    const sel = saveSelection(display);
    display.innerHTML = html;
    if (sel) restoreSelection(display, sel);
    this.renderLineNumbers();
  }

  private renderLineNumbers() {
    const lineNumbers = this.el.querySelector('#line-numbers') as HTMLElement;
    const lines = this.content.split('\n').length || 1;
    lineNumbers.innerHTML = Array.from({ length: lines }, (_, i) =>
      `<div style="height:20px;">${i + 1}</div>`
    ).join('');
  }

  private renderDiagnostics() {
    const display = this.el.querySelector('#code-display') as HTMLElement;
    display.querySelectorAll('.diag-underline').forEach((el) => el.remove());

    for (const diag of this.diagnostics) {
      const lines = display.querySelectorAll('div');
      const lineEl = lines[diag.line];
      if (!lineEl) continue;
      const underline = document.createElement('span');
      underline.className = 'diag-underline';
      underline.style.cssText = `
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: ${diag.severity === 1 ? 'var(--frame-color-ide-error)' : 'var(--frame-color-ide-warning)'};
        opacity: 0.7;
      `;
      underline.title = diag.message;
      lineEl.style.position = 'relative';
      lineEl.appendChild(underline);
    }
  }
}

function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

function saveSelection(el: HTMLElement): { start: number; end: number } | null {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return null;
  const range = sel.getRangeAt(0);
  const pre = range.cloneRange();
  pre.selectNodeContents(el);
  pre.setEnd(range.startContainer, range.startOffset);
  const start = pre.toString().length;
  return { start, end: start + range.toString().length };
}

function restoreSelection(el: HTMLElement, { start, end }: { start: number; end: number }) {
  const sel = window.getSelection();
  if (!sel) return;
  const range = document.createRange();
  let charCount = 0;
  const treeWalker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT);
  let startNode: Text | null = null;
  let endNode: Text | null = null;
  let startOffset = 0;
  let endOffset = 0;

  while (treeWalker.nextNode()) {
    const node = treeWalker.currentNode as Text;
    const nextCharCount = charCount + node.length;
    if (!startNode && start <= nextCharCount) {
      startNode = node;
      startOffset = start - charCount;
    }
    if (!endNode && end <= nextCharCount) {
      endNode = node;
      endOffset = end - charCount;
      break;
    }
    charCount = nextCharCount;
  }

  if (startNode && endNode) {
    range.setStart(startNode, startOffset);
    range.setEnd(endNode, endOffset);
    sel.removeAllRanges();
    sel.addRange(range);
  }
}
