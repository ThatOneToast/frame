export interface LspDiagnostic {
  message: string;
  severity: number;
  line: number;
  column: number;
  endLine: number;
  endColumn: number;
}

export class LspClient {
  private ws: WebSocket | null = null;
  private id = 0;
  private pending = new Map<number, (result: unknown) => void>();

  onDiagnostics: ((uri: string, diagnostics: LspDiagnostic[]) => void) | null = null;
  onConnect: (() => void) | null = null;

  constructor(private url: string) {}

  connect() {
    this.ws = new WebSocket(this.url);
    this.ws.onopen = () => {
      this.initialize();
    };
    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.handleMessage(message);
    };
    this.ws.onclose = () => {
      console.log('LSP disconnected');
    };
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  private initialize() {
    this.sendRequest('initialize', {
      processId: null,
      rootUri: null,
      capabilities: {},
    }).then(() => {
      this.sendNotification('initialized', {});
      if (this.onConnect) this.onConnect();
    });
  }

  openFile(uri: string, text: string) {
    this.sendNotification('textDocument/didOpen', {
      textDocument: { uri, languageId: 'frame', version: 1, text },
    });
  }

  notifyChange(uri: string, text: string) {
    this.sendNotification('textDocument/didChange', {
      textDocument: { uri, version: ++this.id },
      contentChanges: [{ text }],
    });
  }

  requestCompletion(uri: string, line: number, character: number) {
    return this.sendRequest('textDocument/completion', {
      textDocument: { uri },
      position: { line, character },
    });
  }

  requestHover(uri: string, line: number, character: number) {
    return this.sendRequest('textDocument/hover', {
      textDocument: { uri },
      position: { line, character },
    });
  }

  private sendRequest(method: string, params: unknown): Promise<unknown> {
    return new Promise((resolve) => {
      const messageId = ++this.id;
      this.pending.set(messageId, resolve);
      this.send({ jsonrpc: '2.0', id: messageId, method, params });
    });
  }

  private sendNotification(method: string, params: unknown) {
    this.send({ jsonrpc: '2.0', method, params });
  }

  private send(message: unknown) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    }
  }

  private handleMessage(message: any) {
    if (message.id !== undefined && this.pending.has(message.id)) {
      const resolve = this.pending.get(message.id)!;
      this.pending.delete(message.id);
      resolve(message.result);
    }
    if (message.method === 'textDocument/publishDiagnostics' && this.onDiagnostics) {
      const { uri, diagnostics } = message.params;
      this.onDiagnostics(
        uri,
        diagnostics.map((d: any) => ({
          message: d.message,
          severity: d.severity || 1,
          line: d.range?.start?.line || 0,
          column: d.range?.start?.character || 0,
          endLine: d.range?.end?.line || 0,
          endColumn: d.range?.end?.character || 0,
        }))
      );
    }
  }
}
