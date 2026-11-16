import { type App, PluginSettingTab, Setting } from 'obsidian';
import type DeclartPlugin from './main.ts';
import { sanitizeWidth } from './render.ts';

export interface DeclartSettings {
  /** "auto" follows Obsidian's light/dark mode; otherwise a built-in theme name. */
  theme: string;
  /** Canvas width in pixels; unset means automatic. */
  width?: number;
}

export const DEFAULT_SETTINGS: DeclartSettings = { theme: 'auto' };

export class DeclartSettingTab extends PluginSettingTab {
  private readonly plugin: DeclartPlugin;

  constructor(app: App, plugin: DeclartPlugin) {
    super(app, plugin);
    this.plugin = plugin;
  }

  display(): void {
    const { containerEl } = this;
    containerEl.empty();

    new Setting(containerEl)
      .setName('Theme')
      .setDesc("Auto follows Obsidian's light and dark mode.")
      .addDropdown((dd) => {
        dd.addOption('auto', 'Auto');
        for (const theme of this.plugin.engine?.themes() ?? ['default', 'dark']) {
          dd.addOption(theme, theme);
        }
        dd.setValue(this.plugin.settings.theme).onChange(async (value) => {
          this.plugin.settings.theme = value;
          await this.plugin.saveSettings();
        });
      });

    new Setting(containerEl)
      .setName('Width')
      .setDesc('Canvas width in pixels. Leave empty for automatic.')
      .addText((text) =>
        text
          .setPlaceholder('auto')
          .setValue(this.plugin.settings.width ? String(this.plugin.settings.width) : '')
          .onChange(async (value) => {
            this.plugin.settings.width = sanitizeWidth(value.trim() === '' ? undefined : Number(value));
            await this.plugin.saveSettings();
          }),
      );
  }
}
