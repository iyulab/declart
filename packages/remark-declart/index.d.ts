import type { Plugin } from 'unified';

export interface RemarkDeclartOptions {
  /** Built-in theme name. Defaults to 'default'. */
  theme?: 'default' | 'monochrome' | 'accessible' | 'warm' | (string & {});
  /** Canvas width in pixels (height scales proportionally). */
  width?: number;
  /** Custom TOML theme string. Overrides `theme` when provided. */
  themeToml?: string;
}

declare const remarkDeclart: Plugin<[RemarkDeclartOptions?]>;
export default remarkDeclart;
