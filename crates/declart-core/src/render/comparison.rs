use crate::model::ComparisonDiagram;
use crate::render::{font, svg::SvgBuilder, theme::Theme};

const CANVAS_W: f32 = 800.0;
const MARGIN: f32 = 20.0;
const TITLE_H: f32 = 50.0;
const COL_HEADER_H: f32 = 45.0;
const ROW_H: f32 = 50.0;
const ROW_HEADER_W: f32 = 180.0;
const BOTTOM_PAD: f32 = 20.0;
const CELL_FONT: f32 = 13.0;
const HEADER_FONT: f32 = 13.0;
const BORDER_W: f32 = 1.0;

pub fn render(diagram: &ComparisonDiagram, theme: &Theme) -> String {
    let n_rows = diagram.rows.len();
    let n_cols = diagram.columns.len();

    let title_h = if diagram.title.is_some() { TITLE_H } else { MARGIN };
    let table_w = CANVAS_W - 2.0 * MARGIN;
    let data_w = table_w - ROW_HEADER_W;
    let cell_w = if n_cols > 0 { data_w / n_cols as f32 } else { data_w };

    let canvas_h = title_h + COL_HEADER_H + n_rows as f32 * ROW_H + BOTTOM_PAD;
    let table_left = MARGIN;
    let table_top = title_h;

    let mut builder = SvgBuilder::new(CANVAS_W, canvas_h);

    if let Some(title) = &diagram.title {
        builder.text(
            CANVAS_W / 2.0,
            TITLE_H / 2.0,
            title,
            &theme.title_color.to_hex(),
            theme.typography.title_size,
        );
    }

    let header_bg = theme.layers.apex;
    let header_fg = if header_bg.is_dark() { theme.text.on_dark } else { theme.text.on_light };

    let row_hdr_bg = theme.layers.apex.interpolate(&theme.layers.base, 0.3);
    let row_hdr_fg = if row_hdr_bg.is_dark() { theme.text.on_dark } else { theme.text.on_light };

    let even_bg = theme.layers.base.interpolate(&theme.background, 0.5);
    let odd_bg = theme.background;

    // Corner cell (top-left)
    builder.polygon_stroked(
        &rect(table_left, table_top, ROW_HEADER_W, COL_HEADER_H),
        &header_bg.to_hex(),
        "none",
        0.0,
    );

    // Column header cells
    for (ci, col_label) in diagram.columns.iter().enumerate() {
        let cx = table_left + ROW_HEADER_W + ci as f32 * cell_w;
        builder.polygon_stroked(
            &rect(cx, table_top, cell_w, COL_HEADER_H),
            &header_bg.to_hex(),
            "none",
            0.0,
        );
        let label = font::truncate_text(col_label, HEADER_FONT, cell_w - 12.0);
        builder.text_weighted(
            cx + cell_w / 2.0,
            table_top + COL_HEADER_H / 2.0,
            &label,
            &header_fg.to_hex(),
            HEADER_FONT,
            true,
        );
    }

    // Row cells
    for (ri, row_label) in diagram.rows.iter().enumerate() {
        let ry = table_top + COL_HEADER_H + ri as f32 * ROW_H;
        let cell_bg = if ri % 2 == 0 { even_bg } else { odd_bg };

        // Row header cell
        builder.polygon_stroked(
            &rect(table_left, ry, ROW_HEADER_W, ROW_H),
            &row_hdr_bg.to_hex(),
            "none",
            0.0,
        );
        let label = font::truncate_text(row_label, HEADER_FONT, ROW_HEADER_W - 16.0);
        builder.text_weighted(
            table_left + ROW_HEADER_W / 2.0,
            ry + ROW_H / 2.0,
            &label,
            &row_hdr_fg.to_hex(),
            HEADER_FONT,
            false,
        );

        // Data cells (background only)
        for ci in 0..n_cols {
            let cx = table_left + ROW_HEADER_W + ci as f32 * cell_w;
            builder.polygon_stroked(&rect(cx, ry, cell_w, ROW_H), &cell_bg.to_hex(), "none", 0.0);
        }
    }

    // Cell values
    for cell in &diagram.cells {
        let ri = diagram.rows.iter().position(|r| r == &cell.row);
        let ci = diagram.columns.iter().position(|c| c == &cell.column);
        if let (Some(ri), Some(ci)) = (ri, ci) {
            let cx = table_left + ROW_HEADER_W + ci as f32 * cell_w;
            let ry = table_top + COL_HEADER_H + ri as f32 * ROW_H;
            let cell_bg = if ri % 2 == 0 { even_bg } else { odd_bg };
            let text_fg = if cell_bg.is_dark() { theme.text.on_dark } else { theme.text.on_light };
            let label = font::truncate_text(&cell.value, CELL_FONT, cell_w - 12.0);
            builder.text(cx + cell_w / 2.0, ry + ROW_H / 2.0, &label, &text_fg.to_hex(), CELL_FONT);
        }
    }

    // Grid border lines
    let border = theme.layers.apex.interpolate(&theme.background, 0.5).to_hex();
    let table_right = table_left + table_w;
    let table_bottom = table_top + COL_HEADER_H + n_rows as f32 * ROW_H;

    // Horizontal lines (below each row)
    for i in 0..=(n_rows as u32) {
        let ly = table_top + COL_HEADER_H + i as f32 * ROW_H;
        builder.line(table_left, ly, table_right, ly, &border, BORDER_W);
    }
    // Outer top border (column header top)
    builder.line(table_left, table_top, table_right, table_top, &border, BORDER_W);

    // Vertical lines (right edge of each column section)
    builder.line(table_left, table_top, table_left, table_bottom, &border, BORDER_W);
    builder.line(table_left + ROW_HEADER_W, table_top, table_left + ROW_HEADER_W, table_bottom, &border, BORDER_W);
    for ci in 1..=n_cols {
        let lx = table_left + ROW_HEADER_W + ci as f32 * cell_w;
        builder.line(lx, table_top, lx, table_bottom, &border, BORDER_W);
    }

    builder.build(&theme.background.to_hex())
}

fn rect(x: f32, y: f32, w: f32, h: f32) -> [(f32, f32); 4] {
    [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]
}
