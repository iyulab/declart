import { MarkdownRenderChild, Notice, Plugin } from 'obsidian';
import wasmBase64 from './wasm-base64.ts';
import { createEngine, type Engine } from './engine.ts';
import { type BlockResult, renderBlock, resolveTheme, sanitizeWidth } from './render.ts';
import { DEFAULT_SETTINGS, DeclartSettingTab, type DeclartSettings } from './settings.ts';

function decodeBase64(b64: string): Uint8Array {
  const bin = atob(b64);
  const bytes = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
  return bytes;
}

let altCounter = 0;

/** One rendered ```declart block; re-renders when the theme or settings change. */
class DeclartBlock extends MarkdownRenderChild {
  constructor(
    containerEl: HTMLElement,
    private readonly source: string,
    private readonly plugin: DeclartPlugin,
  ) {
    super(containerEl);
  }

  onload(): void {
    this.plugin.blocks.add(this);
    this.render();
  }

  onunload(): void {
    this.plugin.blocks.delete(this);
  }

  render(): void {
    const el = this.containerEl;
    el.empty();

    const { engine, settings } = this.plugin;
    const isDark = document.body.classList.contains('theme-dark');
    const result: BlockResult = engine
      ? renderBlock(engine, this.source, {
          theme: resolveTheme(settings.theme, isDark),
          width: sanitizeWidth(settings.width),
        })
      : { ok: false, message: 'Declart engine failed to load' };

    if (!result.ok) {
      el.createDiv({ cls: 'declart-error', text: result.message });
      return;
    }

    const doc = new DOMParser().parseFromString(result.svg, 'image/svg+xml');
    if (doc.querySelector('parsererror')) {
      el.createDiv({ cls: 'declart-error', text: 'Declart produced an invalid SVG' });
      return;
    }
    // The alt text is exposed through aria-labelledby on a visually hidden span: Obsidian turns any
    // `aria-label` into a hover tooltip, which would cover the diagram.
    const altId = `declart-alt-${++altCounter}`;
    const wrap = el.createDiv({ cls: 'declart-block', attr: { role: 'img', 'aria-labelledby': altId } });
    wrap.createSpan({ cls: 'declart-alt', text: result.alt, attr: { id: altId } });
    wrap.appendChild(document.importNode(doc.documentElement, true));
  }
}

export default class DeclartPlugin extends Plugin {
  settings: DeclartSettings = { ...DEFAULT_SETTINGS };
  engine?: Engine;
  readonly blocks = new Set<DeclartBlock>();

  async onload(): Promise<void> {
    this.settings = Object.assign({}, DEFAULT_SETTINGS, await this.loadData());

    try {
      this.engine = createEngine(decodeBase64(wasmBase64));
    } catch (err) {
      console.error('Declart: failed to initialize the rendering engine', err);
      new Notice('Declart: failed to load the rendering engine');
    }

    this.registerMarkdownCodeBlockProcessor('declart', (source, el, ctx) => {
      ctx.addChild(new DeclartBlock(el, source, this));
    });
    this.registerEvent(this.app.workspace.on('css-change', () => this.refreshAll()));
    this.addSettingTab(new DeclartSettingTab(this.app, this));
  }

  refreshAll(): void {
    for (const block of this.blocks) block.render();
  }

  async saveSettings(): Promise<void> {
    await this.saveData(this.settings);
    this.refreshAll();
  }
}
