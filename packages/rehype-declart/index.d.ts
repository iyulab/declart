import type { Plugin } from 'unified';

export interface RehypeDeclartOptions {
  /** Built-in theme name. Defaults to 'default'. */
  theme?: 'default' | 'monochrome' | 'accessible' | 'warm' | (string & {});
  /** Canvas width in pixels (height scales proportionally). */
  width?: number;
  /** Custom TOML theme string. Overrides `theme` when provided. */
  themeToml?: string;
}

declare const rehypeDeclart: Plugin<[RehypeDeclartOptions?]>;
export default rehypeDeclart;
