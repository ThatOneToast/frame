import type { FrameHandler } from '@frame/runtime-dom';
import { LspClient } from './lsp-client.js';

let lsp: LspClient | null = null;

function refreshFileTree(state: any) {
  const files: string[] = [];
  for (let i = 0; i < localStorage.length; i++) {
    const key = localStorage.key(i)!;
    if (key.startsWith('frame-ide:')) {
      files.push(key.slice('frame-ide:'.length));
    }
  }
  state.set('files', files);
}

export const handlers: Record<string, FrameHandler> = {
  newFile({ state }) {
    state.set('currentPath', '');
    state.set('currentContent', '');
    state.set('status', 'New file');
  },

  openFile({ state }) {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.frame';
    input.onchange = () => {
      const file = input.files?.[0];
      if (!file) return;
      const reader = new FileReader();
      reader.onload = () => {
        const content = String(reader.result);
        state.set('currentPath', file.name);
        state.set('currentContent', content);
        state.set('status', `Opened ${file.name}`);
      };
      reader.readAsText(file);
    };
    input.click();
  },

  saveFile({ state }) {
    let path = state.get('currentPath') as string;
    if (!path) {
      path = prompt('File name:', 'untitled.frame') || '';
      if (!path) return;
      state.set('currentPath', path);
    }
    const content = state.get('currentContent') as string;
    localStorage.setItem(`frame-ide:${path}`, content);
    refreshFileTree(state);
    state.set('status', `Saved ${path}`);
  },

  openFileByPath({ event, state }) {
    const target = event.target as HTMLElement;
    const path = target.textContent || '';
    const content = localStorage.getItem(`frame-ide:${path}`) || '';
    state.set('currentPath', path);
    state.set('currentContent', content);
    state.set('status', `Opened ${path}`);
    if (lsp) {
      lsp.openFile(path, content);
    }
  },

  editorChange({ event, state }) {
    const target = event.target as HTMLTextAreaElement;
    if (target) {
      state.set('currentContent', target.value);
      const path = state.get('currentPath') as string;
      if (lsp && path) {
        lsp.notifyChange(path, target.value);
      }
    }
  },

  editorKeydown({ event, state }) {
    const e = event as KeyboardEvent;
    if ((e.metaKey || e.ctrlKey) && e.key === 's') {
      e.preventDefault();
      let path = state.get('currentPath') as string;
      if (!path) {
        path = prompt('File name:', 'untitled.frame') || '';
        if (!path) return;
        state.set('currentPath', path);
      }
      const content = state.get('currentContent') as string;
      localStorage.setItem(`frame-ide:${path}`, content);
      refreshFileTree(state);
      state.set('status', `Saved ${path}`);
    }
    if ((e.metaKey || e.ctrlKey) && e.key === 'n') {
      e.preventDefault();
      state.set('currentPath', '');
      state.set('currentContent', '');
      state.set('status', 'New file');
    }
    if ((e.metaKey || e.ctrlKey) && e.key === 'o') {
      e.preventDefault();
      const input = document.createElement('input');
      input.type = 'file';
      input.accept = '.frame';
      input.onchange = () => {
        const file = input.files?.[0];
        if (!file) return;
        const reader = new FileReader();
        reader.onload = () => {
          state.set('currentPath', file.name);
          state.set('currentContent', String(reader.result));
          state.set('status', `Opened ${file.name}`);
        };
        reader.readAsText(file);
      };
      input.click();
    }
  },

  toggleLsp({ state }) {
    const connected = state.get('lspConnected') as boolean;
    if (connected && lsp) {
      lsp.disconnect();
      lsp = null;
      state.set('lspConnected', false);
      state.set('status', 'LSP disconnected');
    } else {
      lsp = new LspClient('ws://localhost:3000/frame-lsp');
      lsp.onConnect = () => {
        state.set('lspConnected', true);
        state.set('status', 'LSP connected');
        const path = state.get('currentPath') as string;
        const content = state.get('currentContent') as string;
        if (path) {
          lsp!.openFile(path, content);
        }
      };
      lsp.connect();
    }
  }
};
