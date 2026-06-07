import { IdeApp, Toolbar, Sidebar, EditorPane, StatusBar } from './generated/generated';
import { LspClient } from './lsp-client';
import { FrameEditor } from './editor';
import { FileTree } from './file-tree';

interface IdeState {
  files: { name: string; path: string }[];
  currentPath: string | null;
  currentContent: string;
  status: string;
  lspConnected: boolean;
}

class FrameIde {
  private state: IdeState;
  private lsp: LspClient | null = null;
  private editor: FrameEditor;
  private fileTree: FileTree;
  private statusEl: HTMLElement;

  constructor() {
    this.state = {
      files: [],
      currentPath: null,
      currentContent: '',
      status: 'Frame IDE — Ready',
      lspConnected: false,
    };

    this.render();
    this.editor = new FrameEditor(document.getElementById('editor')!);
    this.fileTree = new FileTree(document.getElementById('file-tree')!);
    this.statusEl = document.getElementById('status')!;

    this.editor.onChange = (content: string) => {
      this.state.currentContent = content;
      if (this.lsp) {
        this.lsp.notifyChange(this.state.currentPath || 'untitled.frame', content);
      }
    };

    this.fileTree.onSelect = (path: string) => {
      this.openFileByPath(path);
    };

    this.bindActions();
    this.refreshFileTree();
  }

  private render() {
    const app = document.getElementById('app')!;
    app.className = IdeApp;
    app.innerHTML = `
      <div class="${Toolbar}" id="toolbar">
        <div style="display:flex;align-items:center;gap:0.5rem;">
          <span style="font-weight:600;color:var(--frame-color-accent);">Frame</span>
          <span style="opacity:0.6;">IDE</span>
        </div>
        <div style="display:flex;gap:0.5rem;">
          <button class="fr-action-NewFile" id="btn-new">New</button>
          <button class="fr-action-OpenFile" id="btn-open">Open</button>
          <button class="fr-action-SaveFile" id="btn-save">Save</button>
        </div>
      </div>
      <div class="${Sidebar}" id="sidebar">
        <div style="font-size:11px;opacity:0.5;text-transform:uppercase;letter-spacing:0.05em;margin-bottom:0.5rem;">Explorer</div>
        <div id="file-tree"></div>
      </div>
      <div class="${EditorPane}" id="editor-pane">
        <div id="editor"></div>
      </div>
      <div class="${StatusBar}" id="status-bar">
        <span id="status">${this.state.status}</span>
        <button class="fr-action-LspToggle" id="btn-lsp">LSP: Off</button>
      </div>
    `;
  }

  private bindActions() {
    document.getElementById('btn-new')!.addEventListener('click', () => this.newFile());
    document.getElementById('btn-open')!.addEventListener('click', () => this.openFile());
    document.getElementById('btn-save')!.addEventListener('click', () => this.saveFile());
    document.getElementById('btn-lsp')!.addEventListener('click', () => this.toggleLsp());

    document.addEventListener('keydown', (e) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'n') {
        e.preventDefault();
        this.newFile();
      }
      if ((e.metaKey || e.ctrlKey) && e.key === 'o') {
        e.preventDefault();
        this.openFile();
      }
      if ((e.metaKey || e.ctrlKey) && e.key === 's') {
        e.preventDefault();
        this.saveFile();
      }
    });
  }

  private newFile() {
    this.state.currentPath = null;
    this.state.currentContent = '';
    this.editor.setContent('');
    this.setStatus('New file');
  }

  private openFile() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.frame';
    input.onchange = () => {
      const file = input.files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = () => {
          this.state.currentPath = file.name;
          this.state.currentContent = String(reader.result);
          this.editor.setContent(this.state.currentContent);
          this.setStatus(`Opened ${file.name}`);
        };
        reader.readAsText(file);
      }
    };
    input.click();
  }

  private openFileByPath(path: string) {
    this.state.currentPath = path;
    this.state.currentContent = localStorage.getItem(`frame-ide:${path}`) || '';
    this.editor.setContent(this.state.currentContent);
    this.setStatus(`Opened ${path}`);
  }

  private saveFile() {
    if (!this.state.currentPath) {
      const name = prompt('File name:', 'untitled.frame');
      if (!name) return;
      this.state.currentPath = name;
    }
    localStorage.setItem(`frame-ide:${this.state.currentPath}`, this.state.currentContent);
    this.refreshFileTree();
    this.setStatus(`Saved ${this.state.currentPath}`);
  }

  private toggleLsp() {
    if (this.lsp) {
      this.lsp.disconnect();
      this.lsp = null;
      this.state.lspConnected = false;
      this.setStatus('LSP disconnected');
      document.getElementById('btn-lsp')!.textContent = 'LSP: Off';
    } else {
      this.lsp = new LspClient('ws://localhost:3000/frame-lsp');
      this.lsp.onDiagnostics = (uri, diagnostics) => {
        this.editor.setDiagnostics(diagnostics);
      };
      this.lsp.onConnect = () => {
        this.state.lspConnected = true;
        this.setStatus('LSP connected');
        document.getElementById('btn-lsp')!.textContent = 'LSP: On';
        if (this.state.currentPath) {
          this.lsp!.openFile(this.state.currentPath, this.state.currentContent);
        }
      };
      this.lsp.connect();
    }
  }

  private refreshFileTree() {
    const files: { name: string; path: string }[] = [];
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)!;
      if (key.startsWith('frame-ide:')) {
        const path = key.slice('frame-ide:'.length);
        files.push({ name: path, path });
      }
    }
    this.state.files = files;
    this.fileTree.setFiles(files);
  }

  private setStatus(message: string) {
    this.state.status = message;
    this.statusEl.textContent = message;
  }
}

new FrameIde();
