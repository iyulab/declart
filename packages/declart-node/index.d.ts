/** What the engine did to a label to make it fit. */
export interface RenderWarning {
  /** `label_truncated`: the label was cut short with "…" — text is missing from the picture. `label_shrunk`: set smaller — nothing is missing. */
  code: 'label_truncated' | 'label_shrunk';
  /** The label as declared. */
  label: string;
  /** A sentence saying what happened to the label. */
  message: string;
  /** What to change in the declaration. */
  hint: string;
}

/** The result of {@link renderDetailed}. */
export interface DetailedRender {
  svg: string;
  /** Empty when every label is in the picture as declared. */
  warnings: RenderWarning[];
}

/** How many characters of one script a slot takes before the engine steps in. */
export interface LabelCapacity {
  /** The longest label shown at its usual size on one line; one more character is set smaller or wrapped. */
  unchanged: number | null;
  /** The longest label shown whole; one more character is cut short with "…". */
  whole: number | null;
}

/**
 * The capacity of one place in a declaration where a label goes. `null` means the engine reports
 * nothing for the slot up to 120 characters — not that any length fits: a slot the engine does not
 * fit at all draws a long label past the picture without a warning.
 */
export interface SlotLimit {
  /** Where the label goes, such as `items[].label` or `states[].label`. */
  path: string;
  /** The reference declaration the numbers were measured in, such as `3 items`. A slot's width depends on its siblings, so read a limit as a guide and use {@link renderDetailed} for the declaration itself. */
  reference: string;
  /** Hangul syllables (one em wide each), written without spaces. */
  hangul: LabelCapacity;
  /** Lowercase Latin letters, written without spaces. */
  ascii: LabelCapacity;
}

/** Options for {@link validate}. */
export interface ValidateOptions {
  /** Also fail when the declaration does not fit: a label the engine would cut short with "…". */
  strict?: boolean;
}

/** Options for the render functions. */
export interface RenderOptions {
  /**
   * Leave the backdrop unpainted so the page shows through. Colors are unchanged —
   * on a dark page use the `dark` theme so text stays legible.
   */
  transparent?: boolean;
}

/**
 * Renders a TOML or JSON diagram declaration to SVG.
 * @param input - TOML or JSON declaration string (format auto-detected)
 * @param theme - Theme name: 'default', 'monochrome', 'accessible', 'warm', 'dark' (defaults to 'default')
 * @param width - Optional canvas width in pixels (height scales proportionally)
 * @param options - `transparent: true` leaves the backdrop unpainted
 * @returns SVG string
 * @throws If the declaration is invalid
 */
export function render(input: string, theme?: string, width?: number, options?: RenderOptions): string;

/**
 * Renders a TOML or JSON diagram declaration to SVG and reports what the engine changed to make the
 * labels fit. An empty `warnings` list means every label is in the picture as declared.
 * @param input - TOML or JSON declaration string (format auto-detected)
 * @param theme - Theme name (defaults to 'default')
 * @param width - Optional canvas width in pixels
 * @param options - `transparent: true` leaves the backdrop unpainted
 * @throws If the declaration is invalid
 */
export function renderDetailed(input: string, theme?: string, width?: number, options?: RenderOptions): DetailedRender;

/**
 * Reports how long a label can be in each place it goes in a diagram kind. The numbers are measured
 * with the same diagnostics {@link renderDetailed} returns, so a limit and a `label_truncated`
 * warning always agree.
 * @param kind - Diagram kind, one of {@link kinds}
 * @param view - View of the kind (defaults to the kind's first view)
 * @throws If the kind or the view is unknown
 */
export function limits(kind: string, view?: string): SlotLimit[];

/**
 * Renders a JSON diagram declaration to SVG.
 * @param input - JSON declaration string
 * @param theme - Theme name (defaults to 'default')
 * @param width - Optional canvas width in pixels
 * @param options - `transparent: true` leaves the backdrop unpainted
 * @returns SVG string
 * @throws If the JSON declaration is invalid
 */
export function renderJson(input: string, theme?: string, width?: number, options?: RenderOptions): string;

/**
 * Renders a TOML or JSON diagram declaration with a custom theme TOML.
 * @param input - TOML or JSON declaration string
 * @param themeToml - TOML theme definition string
 * @param width - Optional canvas width in pixels
 * @param options - `transparent: true` leaves the backdrop unpainted
 * @returns SVG string
 * @throws If the declaration or theme is invalid
 */
export function renderWithThemeToml(input: string, themeToml: string, width?: number, options?: RenderOptions): string;

/**
 * Validates a TOML or JSON diagram declaration without rendering.
 * @param input - TOML or JSON declaration string
 * @throws If the declaration is invalid
 */
export function validate(input: string, options?: ValidateOptions): void;

/**
 * Returns a one-paragraph text equivalent of a TOML or JSON declaration — the text the
 * rendered SVG carries as its `<desc>`. Use it as `alt` when the SVG is shown through `<img>`.
 * @param input - TOML or JSON declaration string
 * @returns Text equivalent, e.g. `"Process: Upload → Parse → Index."`
 * @throws If the declaration is invalid
 */
export function describe(input: string): string;

/**
 * Returns the list of supported theme names.
 */
export function themes(): string[];

/**
 * Returns the list of supported diagram kind names.
 */
export function kinds(): string[];
