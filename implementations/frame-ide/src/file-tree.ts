export class FileTree {
  private el: HTMLElement;
  onSelect: ((path: string) => void) | null = null;

  constructor(parent: HTMLElement) {
    this.el = document.createElement('div');
    this.el.style.cssText = 'display:flex;flex-direction:column;gap:2px;';
    parent.appendChild(this.el);
  }

  setFiles(files: { name: string; path: string }[]) {
    this.el.innerHTML = '';
    for (const file of files) {
      const item = document.createElement('div');
      item.style.cssText = `
        padding: 4px 8px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 13px;
        color: var(--frame-color-ide-text, #c8c8d0);
        transition: background 0.1s;
      `;
      item.textContent = file.name;
      item.addEventListener('mouseenter', () => {
        item.style.background = 'var(--frame-color-ide-accent-glow, rgba(120,160,255,0.12))';
      });
      item.addEventListener('mouseleave', () => {
        item.style.background = 'transparent';
      });
      item.addEventListener('click', () => {
        if (this.onSelect) this.onSelect(file.path);
      });
      this.el.appendChild(item);
    }
  }
}
