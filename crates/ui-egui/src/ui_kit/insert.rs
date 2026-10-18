//! Insert tab thumbnails (and the Draw tab's four): what lands on the sheet when you press the
//! button — a table, a chart, a picture, a comment, a link, an ink stroke — drawn in the accent
//! colour on a mini worksheet, each a little film. This file also holds the sheet-flavoured
//! helpers the other tab files share (`in_sheet`, `in_cell`, `in_text`, `in_clip`…).

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "PivotTable",
        "Summarises a long list into a small table of totals: choose a column to group by and a column to add up, and every group gets one row. Example: ٢٠٠ rows of sales become one row per city with that city's total.",
        |d| d.in_pivot(),
    ),
    Entry::new(
        "Table (⌘T)",
        "Turns the selected cells into a table: a coloured header row with filter arrows, striped rows, and anything you type under the last row joins it automatically. Example: a list of names and dates gets a blue header and alternating bands.",
        |d| d.in_table_insert(),
    ),
    Entry::new(
        "Insert a picture",
        "Places a photo or picture file from your computer on top of the sheet; drag it to move it and pull its corners to resize it. Example: the unit's logo sits above the table on the report sheet.",
        |d| {
            let b = d.in_sheet_full(true);
            let r = Rect::from_min_size(pos2(b.min.x + 24.0, b.min.y + 14.0), vec2(80.0, 50.0));
            d.picture(Draw::in_grow(r, 0.3 + 0.7 * d.reveal(0.2, 0.65)));
        },
    ),
    Entry::new(
        "Shapes",
        "Draws a shape over the sheet (a rectangle, a circle, an arrow, a star) that you can drag, resize and colour. Example: a red arrow pointing at the highest number in a chart.",
        |d| d.in_shapes_sheet(),
    ),
    Entry::new(
        "Insert an icon",
        "Puts a small simple picture (a star, a flag, a tool, a tick) from the built-in icon library on the sheet. Example: a star icon beside the title of the best month.",
        |d| d.in_icon_sheet(),
    ),
    Entry::new(
        "Recommended Charts",
        "Looks at the selected numbers and offers charts that suit them (bars, lines, a pie); pick one and it appears beside the data. Example: twelve monthly totals become a bar chart with one bar per month.",
        |d| d.in_recommended_chart(),
    ),
    Entry::new(
        "Insert a slicer to filter a table",
        "Adds a panel of buttons, one per value in a column; click a button and the table shows only the rows with that value. Example: buttons «بغداد», «البصرة», «الموصل» — press one and only that city's rows stay.",
        |d| d.in_slicer(),
    ),
    Entry::new(
        "Sparklines",
        "Draws a tiny chart inside one cell from a row of numbers, so each row shows its own trend at a glance. Example: beside each product's twelve monthly figures, a small line rising or falling.",
        |d| d.in_sparklines(),
    ),
    Entry::new(
        "Insert checkboxes in the selected cells",
        "Puts a tick box in each selected cell; clicking a box switches it between TRUE and FALSE. Example: a «تم» column where you tick each task as it is finished.",
        |d| d.in_checkboxes(),
    ),
    Entry::new(
        "Insert Link (⌘K)",
        "Makes the selected cell a link that opens a web page, a file or another place in the workbook when clicked. Example: the cell «موقع الوزارة» opens the ministry's website.",
        |d| d.in_link_cell(),
    ),
    Entry::new(
        "New Comment",
        "Attaches a note to the selected cell; a small red corner marks it and the note shows when you point at the cell. Example: on a strange figure, write «راجع هذا الرقم» for a colleague.",
        |d| d.in_comment_cell(),
    ),
    Entry::new(
        "Text Box",
        "Puts a box of free text that floats above the cells; move and resize it anywhere, not tied to the grid. Example: a note «أرقام مؤقتة» placed in the corner of a chart.",
        |d| {
            let b = d.in_sheet_full(true);
            let r = Rect::from_min_size(pos2(b.min.x + 30.0, b.min.y + 18.0), vec2(74.0, 40.0));
            let k = d.reveal(0.15, 0.6);
            d.text_box(Rect::from_min_size(r.min, vec2(r.width() * (0.2 + 0.8 * k), r.height() * (0.3 + 0.7 * k))));
        },
    ),
    Entry::new(
        "Header & Footer",
        "Adds text that prints at the top and bottom of every page: a title, the date, the page number. Example: «وزارة الدفاع» at the top of each printed page and «صفحة ١» at the bottom.",
        |d| d.in_header_footer_page(),
    ),
    Entry::new(
        "Symbol",
        "Opens a table of special characters that are not on the keyboard and puts the one you pick into the cell. Example: insert «≠», «±» or «©» after a number.",
        |d| d.in_symbol_grid(),
    ),
    // ---- Draw tab ----
    Entry::new(
        "Select and move ink and shapes",
        "Switches from drawing back to the normal pointer so you can click ink strokes and shapes to move, resize or delete them. Example: drag the circle you drew to sit over a different cell.",
        |d| d.in_select_objects(),
    ),
    Entry::new(
        "Draw with ink",
        "Lets you draw freehand over the sheet with the mouse or a pen, like a marker on paper; the cells underneath are not changed. Example: circle a wrong number and draw an arrow to the right one.",
        |d| d.in_ink_draw(),
    ),
    Entry::new(
        "Erase ink strokes",
        "Rubs out ink strokes you drew; click or drag over a stroke and it disappears, leaving the cells as they were. Example: wipe away the circle after the number is fixed.",
        |d| d.in_ink_erase(),
    ),
    Entry::new(
        "Convert the last ink stroke to a shape",
        "Replaces the stroke you just drew with the clean shape it looks like: a wobbly ring becomes a perfect circle, a rough box a rectangle. Example: a hand-drawn loop around a cell becomes a neat oval.",
        |d| d.in_ink_to_shape(),
    ),
];

const DIGITS: [&str; 9] = ["١", "٢", "٣", "٤", "٥", "٦", "٧", "٨", "٩"];
const LETTERS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

impl Draw<'_> {
    // ---- shared sheet helpers (used by every tab file) ----

    /// The same drawing through a painter clipped to `clip` (a wipe).
    pub(super) fn in_clip(&self, clip: Rect, f: impl FnOnce(&Draw<'_>)) {
        let p = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        f(&self.with_painter(&p));
    }

    /// `r` scaled about its centre by `k` (a thing popping in).
    pub(super) fn in_grow(r: Rect, k: f32) -> Rect {
        Rect::from_center_size(r.center(), r.size() * k.max(0.02))
    }

    /// An Arabic-Indic digit for `n` (1-based), «٠» beyond nine.
    pub(super) fn in_digit(n: usize) -> &'static str {
        DIGITS.get(n.wrapping_sub(1)).copied().unwrap_or("٠")
    }

    /// A mini worksheet in `r`: a column-letter strip on top, a row-number strip at the side
    /// (on the right when `rtl`, as an Arabic sheet has it) and a `rows × cols` grid of white
    /// cells. Returns the cell area.
    pub(super) fn in_sheet(&self, r: Rect, rows: usize, cols: usize, rtl: bool) -> Rect {
        const H: f32 = 9.0;
        const W: f32 = 11.0;
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        let top = Rect::from_min_size(r.min, vec2(r.width(), H));
        let side = if rtl { Rect::from_min_max(pos2(r.max.x - W, r.min.y), r.max) } else { Rect::from_min_size(r.min, vec2(W, r.height())) };
        self.p.rect_filled(top, 0.0, self.t.hover);
        self.p.rect_filled(side, 0.0, self.t.hover);
        let body = if rtl {
            Rect::from_min_max(pos2(r.min.x, r.min.y + H), pos2(side.min.x, r.max.y))
        } else {
            Rect::from_min_max(pos2(side.max.x, r.min.y + H), r.max)
        };
        self.grid(body, rows, cols, self.t.border);
        for (j, letter) in LETTERS.iter().enumerate().take(cols) {
            let c = self.in_cell(body, rows, cols, 0, j, rtl);
            self.label(pos2(c.center().x, top.center().y), letter, 6.0, self.grey(), Align2::CENTER_CENTER);
        }
        for (i, digit) in DIGITS.iter().enumerate().take(rows) {
            let c = self.cell(body, rows, cols, i, 0);
            self.label(pos2(side.center().x, c.center().y), digit, 6.0, self.grey(), Align2::CENTER_CENTER);
        }
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        body
    }

    /// The standard whole-thumbnail sheet: 5 rows × 5 columns; returns its cell area.
    pub(super) fn in_sheet_full(&self, rtl: bool) -> Rect {
        self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0)), 5, 5, rtl)
    }

    /// Cell `(row, col)` of a sheet's cell area; column 0 is the rightmost when `rtl`.
    pub(super) fn in_cell(&self, body: Rect, rows: usize, cols: usize, row: usize, col: usize, rtl: bool) -> Rect {
        let col = if rtl { cols.saturating_sub(1).saturating_sub(col) } else { col };
        self.cell(body, rows, cols, row, col)
    }

    /// `s` with every run of Arabic-Indic digits (and the ٪ ٫ − that go with them) reversed:
    /// the text layout turns an Arabic run around character by character, digits included, so
    /// a number is written here back to front to come out the right way round on screen.
    pub(super) fn in_num(s: &str) -> String {
        let is_digit = |c: char| ('\u{660}'..='\u{669}').contains(&c) || matches!(c, '٪' | '٫' | '−');
        let mut out = String::with_capacity(s.len());
        let mut run = String::new();
        for c in s.chars() {
            if is_digit(c) {
                run.push(c);
            } else {
                out.extend(run.chars().rev());
                run.clear();
                out.push(c);
            }
        }
        out.extend(run.chars().rev());
        out
    }

    /// Text centred in a cell (numbers written the right way round, see [`Draw::in_num`]).
    pub(super) fn in_text(&self, cell: Rect, s: &str, c: Color32) {
        self.label(cell.center(), &Draw::in_num(s), 7.0, c, Align2::CENTER_CENTER);
    }

    /// A label whose numbers come out the right way round (see [`Draw::in_num`]).
    pub(super) fn in_label(&self, pos: Pos2, s: &str, size: f32, c: Color32, anchor: Align2) {
        self.label(pos, &Draw::in_num(s), size, c, anchor);
    }

    /// A short "text" mark in a cell (ordinary content), right-aligned when `rtl`.
    pub(super) fn in_mark(&self, cell: Rect, rtl: bool, frac: f32, c: Color32) {
        let w = (cell.width() - 6.0).max(2.0) * frac.clamp(0.1, 1.0);
        let y = cell.center().y;
        let (x0, x1) = if rtl { (cell.max.x - 3.0 - w, cell.max.x - 3.0) } else { (cell.min.x + 3.0, cell.min.x + 3.0 + w) };
        self.line(pos2(x0, y), pos2(x1, y), c, 1.5);
    }

    /// Ordinary content marks in the cells of `rows × cols` (rows `r0..r1`, cols `c0..c1`).
    pub(super) fn in_marks(&self, body: Rect, rows: usize, cols: usize, span: (usize, usize, usize, usize), rtl: bool) {
        let (r0, r1, c0, c1) = span;
        for i in r0..r1.min(rows) {
            for j in c0..c1.min(cols) {
                let frac = [0.7, 0.5, 0.8, 0.6].get((i + 2 * j) % 4).copied().unwrap_or(0.6);
                self.in_mark(self.in_cell(body, rows, cols, i, j, rtl), rtl, frac, self.grey());
            }
        }
    }

    /// The rect spanning cells `(r0, c0)` to `(r1, c1)` inclusive.
    pub(super) fn in_range(&self, body: Rect, rows: usize, cols: usize, span: (usize, usize, usize, usize), rtl: bool) -> Rect {
        let (r0, r1, c0, c1) = span;
        self.in_cell(body, rows, cols, r0, c0, rtl).union(self.in_cell(body, rows, cols, r1, c1, rtl))
    }

    /// A selected range: the selection tint and a thick accent frame.
    pub(super) fn in_select(&self, r: Rect) {
        self.selection(r);
        self.p.rect_stroke(r, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
    }

    /// A dashed accent rectangle (a print area, a page break, a marquee).
    pub(super) fn in_dashed(&self, r: Rect, c: Color32) {
        self.dashed_rect(r, c);
    }

    /// The first `k` of a polyline, `w` wide in `c`.
    pub(super) fn in_path(&self, pts: &[Pos2], k: f32, c: Color32, w: f32) {
        if pts.len() < 2 || k <= 0.0 {
            return;
        }
        let total = pts.len().saturating_sub(1) as f32 * k.min(1.0);
        let whole = total.floor() as usize;
        let mut out: Vec<Pos2> = pts.iter().copied().take(whole + 1).collect();
        if let (Some(a), Some(b)) = (pts.get(whole), pts.get(whole + 1)) {
            let f = total - whole as f32;
            if f > 0.0 {
                out.push(*a + (*b - *a) * f);
            }
        }
        if out.len() >= 2 {
            self.p.add(Shape::line(out, Stroke::new(w, c)));
        }
    }

    /// A small filter funnel centred at `c`.
    pub(super) fn in_funnel(&self, c: Pos2, col: Color32) {
        let pts = vec![c + vec2(-6.0, -6.0), c + vec2(6.0, -6.0), c + vec2(1.5, 0.5), c + vec2(1.5, 6.0), c + vec2(-1.5, 6.0), c + vec2(-1.5, 0.5)];
        self.p.add(Shape::convex_polygon(pts, col, Stroke::NONE));
    }

    /// A tiny dropdown triangle at `c`.
    pub(super) fn in_triangle(&self, c: Pos2, col: Color32) {
        self.p.add(Shape::convex_polygon(vec![c + vec2(-3.0, -2.0), c + vec2(3.0, -2.0), c + vec2(0.0, 2.0)], col, Stroke::NONE));
    }

    /// A page with a folded corner (a file) at `r`, with `tag` written across it.
    pub(super) fn in_file(&self, r: Rect, tag: &str, fill: Color32) {
        let fold = 8.0;
        let pts = vec![r.left_top(), pos2(r.max.x - fold, r.min.y), pos2(r.max.x, r.min.y + fold), r.right_bottom(), r.left_bottom()];
        self.p.add(Shape::convex_polygon(pts.clone(), Color32::WHITE, Stroke::NONE));
        self.p.add(Shape::closed_line(pts, self.stroke(self.t.border_strong)));
        self.line(pos2(r.max.x - fold, r.min.y), pos2(r.max.x - fold, r.min.y + fold), self.t.border_strong, 1.0);
        self.line(pos2(r.max.x - fold, r.min.y + fold), pos2(r.max.x, r.min.y + fold), self.t.border_strong, 1.0);
        self.badge(pos2(r.center().x, r.max.y - 12.0), tag, fill);
    }

    // ---- Insert tab ----

    /// A long list on the left, an arrow, and the short summary table wiping in on the right.
    fn in_pivot(&self) {
        let src = Rect::from_min_size(self.at(8.0, 10.0), vec2(66.0, 84.0));
        let body = self.in_sheet(src, 7, 3, true);
        self.in_marks(body, 7, 3, (0, 7, 0, 3), true);
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * arrow, 52.0));
        }
        let wipe = self.reveal(0.3, 0.75);
        let out = Rect::from_min_size(self.at(104.0, 22.0), vec2(64.0, 60.0));
        if wipe <= 0.0 {
            return;
        }
        self.in_clip(Rect::from_min_max(out.min - vec2(2.0, 2.0), pos2(out.max.x + 2.0, out.min.y + out.height() * wipe + 2.0)), |d| {
            d.p.rect_filled(out, 1.0, Color32::WHITE);
            d.grid(out, 4, 2, d.t.border);
            d.p.rect_filled(d.cell(out, 4, 2, 0, 0).union(d.cell(out, 4, 2, 0, 1)), 0.0, d.accent().linear_multiply(0.35));
            for i in 0..4 {
                let name = d.in_cell(out, 4, 2, i, 0, true);
                let val = d.in_cell(out, 4, 2, i, 1, true);
                let c = if i == 0 { d.ink() } else { d.grey() };
                d.in_mark(name, true, 0.7, c);
                d.in_text(val, ["المجموع", "٤٥", "٣٢", "١٨"].get(i).copied().unwrap_or(""), if i == 0 { d.ink() } else { d.accent() });
            }
            d.p.rect_stroke(out, 1.0, Stroke::new(1.5, d.accent()), egui::StrokeKind::Inside);
        });
    }

    /// Plain cells become a table: header band, stripes, then the filter triangles.
    fn in_table_insert(&self) {
        let body = self.in_sheet_full(true);
        let span = (0, 3, 0, 3);
        let head = self.reveal(0.1, 0.35);
        let stripes = self.reveal(0.35, 0.6);
        let tri = self.reveal(0.65, 0.85);
        let header = self.in_range(body, 5, 5, (0, 0, 0, 3), true);
        if head > 0.0 {
            self.in_clip(Rect::from_min_max(header.min, pos2(header.max.x, header.min.y + header.height() * head)), |d| {
                d.p.rect_filled(header, 0.0, d.accent());
            });
        }
        if stripes > 0.0 {
            for i in [1usize, 3] {
                let band = self.in_range(body, 5, 5, (i, i, 0, 3), true);
                self.p.rect_filled(band, 0.0, self.accent().linear_multiply(0.18 * stripes));
            }
        }
        self.grid(self.in_range(body, 5, 5, span, true), 4, 4, self.t.border);
        for i in 0..4 {
            for j in 0..4 {
                let c = self.in_cell(body, 5, 5, i, j, true);
                let col = if i == 0 && head > 0.5 { Color32::WHITE } else { self.grey() };
                self.in_mark(c, true, [0.6, 0.8, 0.5, 0.7].get((i + j) % 4).copied().unwrap_or(0.6), col);
            }
        }
        if tri > 0.0 {
            for j in 0..4 {
                let c = self.in_cell(body, 5, 5, 0, j, true);
                self.in_triangle(pos2(c.min.x + 5.0, c.center().y), Color32::WHITE.linear_multiply(tri));
            }
        }
    }

    /// Three shapes pop over the sheet one after the other.
    fn in_shapes_sheet(&self) {
        let b = self.in_sheet_full(true);
        let k1 = self.reveal(0.1, 0.35);
        self.shape(Draw::in_grow(Rect::from_min_size(pos2(b.min.x + 14.0, b.min.y + 14.0), vec2(40.0, 26.0)), k1));
        let k2 = self.reveal(0.35, 0.6);
        let c = pos2(b.max.x - 36.0, b.min.y + 30.0);
        if k2 > 0.0 {
            self.p.circle_filled(c, 15.0 * k2, self.accent().linear_multiply(0.35));
            self.p.circle_stroke(c, 15.0 * k2, Stroke::new(1.5, self.accent()));
        }
        let k3 = self.reveal(0.6, 0.85);
        if k3 > 0.0 {
            let s = pos2(b.center().x, b.max.y - 22.0);
            let tri = vec![s + vec2(0.0, -12.0) * k3, s + vec2(14.0, 10.0) * k3, s + vec2(-14.0, 10.0) * k3];
            self.p.add(Shape::convex_polygon(tri, self.accent().linear_multiply(0.35), Stroke::new(1.5, self.accent())));
        }
    }

    /// A five-point star grows over the sheet.
    fn in_icon_sheet(&self) {
        let b = self.in_sheet_full(true);
        let c = pos2(b.center().x, b.center().y);
        let k = self.reveal(0.2, 0.7);
        if k > 0.0 {
            let s = 0.4 + 0.6 * k;
            let pts: Vec<Pos2> = (0..10)
                .map(|i| {
                    let r = if i % 2 == 0 { 20.0 * s } else { 9.0 * s };
                    let ang = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::PI / 5.0;
                    c + vec2(ang.cos() * r, ang.sin() * r)
                })
                .collect();
            self.p.add(Shape::convex_polygon(pts.clone(), self.accent().linear_multiply(0.3), Stroke::NONE));
            self.p.add(Shape::closed_line(pts, Stroke::new(2.0, self.accent())));
        }
    }

    /// Selected numbers on the right, the chart's bars grow on the left.
    fn in_recommended_chart(&self) {
        let body = self.in_sheet_full(true);
        let data = self.in_range(body, 5, 5, (0, 4, 0, 1), true);
        self.selection(data);
        for i in 0..5 {
            self.in_mark(self.in_cell(body, 5, 5, i, 0, true), true, 0.7, self.grey());
            self.in_text(self.in_cell(body, 5, 5, i, 1, true), ["٥", "٨", "٣", "٩", "٦"].get(i).copied().unwrap_or(""), self.ink());
        }
        self.p.rect_stroke(data, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        let k = self.reveal(0.2, 0.8);
        let r = Rect::from_min_size(pos2(body.min.x + 6.0, body.min.y + 8.0), vec2(74.0, 56.0));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let bw = (r.width() - 8.0) / 5.0;
        for (i, h) in [0.5, 0.8, 0.3, 0.9, 0.6].iter().enumerate() {
            let x = r.min.x + 4.0 + i as f32 * bw + 2.0;
            let top = r.max.y - 4.0 - (r.height() - 8.0) * h * k;
            let fill = if i % 2 == 0 { self.accent() } else { self.accent().linear_multiply(0.5) };
            self.p.rect_filled(Rect::from_min_max(pos2(x, top), pos2(x + bw - 4.0, r.max.y - 4.0)), 1.0, fill);
        }
    }

    /// A slicer panel of city buttons; one is pressed and the other cities' rows vanish.
    fn in_slicer(&self) {
        let sheet = Rect::from_min_size(self.at(70.0, 8.0), vec2(98.0, 88.0));
        let body = self.in_sheet(sheet, 6, 2, true);
        let press = self.reveal(0.25, 0.45);
        let hide = self.reveal(0.5, 0.8);
        for i in 0..6 {
            let keep = i % 3 == 1;
            let alpha = if keep || i == 0 { 1.0 } else { 1.0 - hide };
            if alpha <= 0.0 {
                continue;
            }
            let c = if i == 0 { self.ink() } else { self.grey() }.linear_multiply(alpha);
            self.in_mark(self.in_cell(body, 6, 2, i, 0, true), true, 0.7, c);
            self.in_mark(self.in_cell(body, 6, 2, i, 1, true), true, 0.5, c);
        }
        let pane = Rect::from_min_size(self.at(6.0, 10.0), vec2(56.0, 84.0));
        self.p.rect_filled(pane, 2.0, self.t.menu);
        self.p.rect_stroke(pane, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(pane.min, vec2(pane.width(), 12.0)), 2.0, self.t.hover);
        self.label(pos2(pane.max.x - 4.0, pane.min.y + 6.0), "المدينة", 6.5, self.ink(), Align2::RIGHT_CENTER);
        for (i, city) in ["بغداد", "البصرة", "الموصل"].iter().enumerate() {
            let c = pos2(pane.center().x, pane.min.y + 26.0 + i as f32 * 20.0);
            let r = Rect::from_center_size(c, vec2(46.0, 15.0));
            let on = i == 1 && press > 0.5;
            self.p.rect_filled(r, 3.0, if on { self.accent() } else { Color32::WHITE });
            self.p.rect_stroke(r, 3.0, self.stroke(if on { self.accent() } else { self.t.border_strong }), egui::StrokeKind::Inside);
            self.label(c, city, 7.0, if on { Color32::WHITE } else { self.ink() }, Align2::CENTER_CENTER);
        }
        if press > 0.0 && press < 1.0 {
            self.pointer(pos2(pane.center().x + 8.0, pane.min.y + 50.0));
        }
    }

    /// Rows of numbers; in the last column a tiny line chart draws itself for each row.
    fn in_sparklines(&self) {
        let body = self.in_sheet_full(true);
        let series: [[f32; 4]; 4] = [[0.3, 0.6, 0.5, 0.9], [0.8, 0.5, 0.6, 0.2], [0.4, 0.4, 0.7, 0.8], [0.6, 0.9, 0.3, 0.5]];
        for (i, s) in series.iter().enumerate() {
            for j in 0..4 {
                let v = s.get(j).copied().unwrap_or(0.5);
                let n = ((v * 9.0) as usize).clamp(1, 9);
                self.in_text(self.in_cell(body, 5, 5, i, j, true), Draw::in_digit(n), self.grey());
            }
            let cell = self.in_cell(body, 5, 5, i, 4, true).shrink2(vec2(3.0, 3.0));
            let pts: Vec<Pos2> =
                s.iter().enumerate().map(|(j, v)| pos2(cell.min.x + cell.width() * j as f32 / 3.0, cell.max.y - cell.height() * v)).collect();
            let k = self.reveal(0.1 + i as f32 * 0.12, 0.5 + i as f32 * 0.12);
            self.in_path(&pts, k, self.accent(), 1.5);
        }
        let head = self.in_cell(body, 5, 5, 4, 4, true);
        self.in_mark(head, true, 0.6, self.grey());
    }

    /// A column of tick boxes; the third one gets ticked.
    fn in_checkboxes(&self) {
        let body = self.in_sheet_full(true);
        for i in 0..4 {
            self.in_mark(self.in_cell(body, 5, 5, i, 0, true), true, 0.7, self.grey());
            self.in_mark(self.in_cell(body, 5, 5, i, 1, true), true, 0.5, self.grey());
            let cell = self.in_cell(body, 5, 5, i, 2, true);
            let box_r = Rect::from_center_size(cell.center(), vec2(10.0, 10.0));
            let ticked = i == 0 || (i == 2 && self.reveal(0.3, 0.55) > 0.5);
            self.p.rect_filled(box_r, 2.0, if ticked { self.accent() } else { Color32::WHITE });
            self.p.rect_stroke(box_r, 2.0, Stroke::new(1.2, if ticked { self.accent() } else { self.t.border_strong }), egui::StrokeKind::Inside);
            if ticked {
                let c = box_r.center();
                self.line(c + vec2(-3.0, 0.0), c + vec2(-1.0, 2.5), Color32::WHITE, 1.8);
                self.line(c + vec2(-1.0, 2.5), c + vec2(3.5, -3.0), Color32::WHITE, 1.8);
            }
        }
        let k = self.reveal(0.3, 0.55);
        if k > 0.0 && k < 1.0 {
            let cell = self.in_cell(body, 5, 5, 2, 2, true);
            self.pointer(cell.center() + vec2(2.0, 1.0));
        }
    }

    /// A cell becomes a link: accent text with an underline, and a little window pops up.
    fn in_link_cell(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 4, 0, 1), true);
        let cell = self.in_range(body, 5, 5, (1, 1, 1, 3), true);
        let k = self.reveal(0.1, 0.4);
        let c = if k > 0.5 { self.accent() } else { self.grey() };
        self.label(pos2(cell.max.x - 4.0, cell.center().y), "موقع الوزارة", 7.0, c, Align2::RIGHT_CENTER);
        if k > 0.5 {
            self.line(pos2(cell.max.x - 46.0, cell.center().y + 5.0), pos2(cell.max.x - 4.0, cell.center().y + 5.0), c, 1.0);
        }
        let pop = self.reveal(0.45, 0.8);
        if pop > 0.0 {
            let win = Draw::in_grow(Rect::from_min_size(pos2(cell.center().x - 44.0, cell.max.y + 12.0), vec2(64.0, 34.0)), pop);
            self.p.rect_filled(win, 2.0, Color32::WHITE);
            self.p.rect_stroke(win, 2.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
            self.p.rect_filled(Rect::from_min_size(win.min, vec2(win.width(), 7.0 * pop)), 2.0, self.accent());
            self.paragraph(win.shrink2(vec2(4.0, 9.0)), super::Dir::Rtl, 2, self.grey());
            self.arrow_down(pos2(cell.center().x, cell.max.y + 1.0), pos2(cell.center().x, win.min.y - 1.0));
        }
    }

    /// A red corner on the cell, then the note slides open beside it.
    fn in_comment_cell(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 2), true);
        let cell = self.in_cell(body, 5, 5, 1, 1, true);
        self.in_text(cell, "٩٨٠", self.ink());
        let mark = self.reveal(0.1, 0.3);
        if mark > 0.0 {
            let tri = vec![cell.right_top(), cell.right_top() + vec2(-6.0 * mark, 0.0), cell.right_top() + vec2(0.0, 6.0 * mark)];
            self.p.add(Shape::convex_polygon(tri, self.red(), Stroke::NONE));
        }
        let open = self.reveal(0.35, 0.75);
        if open > 0.0 {
            let note = Rect::from_min_size(pos2(body.min.x + 8.0, cell.min.y + 6.0), vec2(62.0 * open.max(0.1), 34.0));
            self.p.rect_filled(note, 2.0, Color32::from_rgb(255, 250, 215));
            self.p.rect_stroke(note, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            self.in_clip(note, |d| {
                d.label(pos2(note.max.x - 4.0, note.min.y + 8.0), "راجع هذا الرقم", 6.5, d.ink(), Align2::RIGHT_CENTER);
                d.text_lines(
                    Rect::from_min_max(pos2(note.min.x + 4.0, note.min.y + 12.0), note.max - vec2(4.0, 0.0)),
                    true,
                    &[40.0, 28.0],
                    8.0,
                    d.grey(),
                );
            });
            self.line(note.right_top() + vec2(0.0, 4.0), cell.left_center(), self.red(), 1.0);
        }
    }

    /// A printed page: the sheet grid in the middle, header and footer strips lighting up.
    fn in_header_footer_page(&self) {
        let a = self.page(50.0, 4.0, 76.0, 96.0);
        let grid = Rect::from_min_max(pos2(a.min.x, a.min.y + 14.0), pos2(a.max.x, a.max.y - 14.0));
        self.grid(grid, 6, 4, self.t.border);
        let k = self.reveal(0.15, 0.6);
        let head = Rect::from_min_size(pos2(a.min.x, a.min.y - 2.0), vec2(a.width(), 10.0));
        let foot = Rect::from_min_size(pos2(a.min.x, a.max.y - 8.0), vec2(a.width(), 10.0));
        for r in [head, foot] {
            self.p.rect_filled(r, 1.0, self.accent().linear_multiply(0.18 * k.max(0.01)));
        }
        if k > 0.0 {
            self.label(pos2(head.max.x - 2.0, head.center().y), "وزارة الدفاع", 6.5, self.accent().linear_multiply(k), Align2::RIGHT_CENTER);
            self.label(foot.center(), "صفحة ١", 6.5, self.accent().linear_multiply(k), Align2::CENTER_CENTER);
        }
    }

    /// A grid of special characters, one picked, and it lands in a cell.
    fn in_symbol_grid(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(72.0, 92.0)), 5, 2, true);
        self.in_marks(body, 5, 2, (0, 5, 0, 1), true);
        let r = Rect::from_min_size(self.at(88.0, 10.0), vec2(82.0, 84.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let g = r.shrink(6.0);
        let syms = ["©", "±", "≠", "∞", "→", "½", "€", "§", "°", "√", "≤", "¶"];
        let pick = self.reveal(0.15, 0.4);
        for (i, s) in syms.iter().enumerate() {
            let c = self.cell(g, 3, 4, i / 4, i % 4);
            if i == 2 && pick > 0.0 {
                self.p.rect_filled(c.shrink(1.0), 2.0, self.t.selection);
                self.p.rect_stroke(c.shrink(1.0), 2.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
            }
            self.label(c.center(), s, 9.0, self.ink(), Align2::CENTER_CENTER);
        }
        let land = self.reveal(0.45, 0.75);
        if land > 0.0 {
            let cell = self.in_cell(body, 5, 2, 1, 1, true);
            self.label(cell.center(), "≠", 10.0 * (0.5 + 0.5 * land), self.accent(), Align2::CENTER_CENTER);
        }
    }

    // ---- Draw tab ----

    /// The pointer drags a dashed marquee round a shape; the handles then appear.
    fn in_select_objects(&self) {
        let b = self.in_sheet_full(true);
        let shape = Rect::from_min_size(pos2(b.min.x + 30.0, b.min.y + 20.0), vec2(44.0, 30.0));
        self.shape(shape);
        let c = pos2(b.max.x - 34.0, b.min.y + 32.0);
        self.p.circle_filled(c, 12.0, self.accent().linear_multiply(0.35));
        self.p.circle_stroke(c, 12.0, Stroke::new(1.5, self.accent()));
        let drag = self.reveal(0.1, 0.5);
        let start = shape.left_top() - vec2(8.0, 8.0);
        let end = shape.right_bottom() + vec2(8.0, 8.0);
        if drag > 0.0 && drag < 1.0 {
            let now = start + (end - start) * drag;
            self.in_dashed(Rect::from_two_pos(start, now), self.accent());
            self.pointer(now);
        }
        if drag >= 1.0 {
            self.handles(shape);
            self.pointer(end);
        }
    }

    /// The marker stroke: a loop around a cell and an arrow away from it.
    fn in_ink_stroke(&self, b: Rect) -> Vec<Pos2> {
        let c = pos2(b.max.x - 40.0, b.min.y + 32.0);
        let mut pts: Vec<Pos2> = (0..=16)
            .map(|i| {
                let a = i as f32 / 16.0 * std::f32::consts::TAU * 1.1;
                c + vec2(a.cos() * 18.0, a.sin() * 11.0)
            })
            .collect();
        pts.extend([c + vec2(-18.0, 12.0), c + vec2(-34.0, 26.0), c + vec2(-52.0, 34.0)]);
        pts
    }

    /// A pen draws the stroke over the sheet.
    fn in_ink_draw(&self) {
        let b = self.in_sheet_full(true);
        self.in_marks(b, 5, 5, (0, 5, 0, 5), true);
        let pts = self.in_ink_stroke(b);
        let k = self.reveal(0.05, 0.85);
        self.in_path(&pts, k, Color32::from_rgb(220, 60, 60), 2.5);
        let n = (pts.len().saturating_sub(1) as f32 * k) as usize;
        if let Some(tip) = pts.get(n.min(pts.len().saturating_sub(1))) {
            self.pencil(*tip, *tip + vec2(10.0, -16.0));
        }
    }

    /// An eraser block slides along the stroke and what it passed is gone.
    fn in_ink_erase(&self) {
        let b = self.in_sheet_full(true);
        self.in_marks(b, 5, 5, (0, 5, 0, 5), true);
        let pts = self.in_ink_stroke(b);
        let k = self.reveal(0.1, 0.8) * 0.6;
        let keep = (pts.len() as f32 * k) as usize;
        let rest: Vec<Pos2> = pts.iter().copied().skip(keep).collect();
        if rest.len() >= 2 {
            self.p.add(Shape::line(rest, Stroke::new(2.5, Color32::from_rgb(220, 60, 60))));
        }
        if let Some(at) = pts.get(keep.min(pts.len().saturating_sub(1))) {
            let er = Rect::from_center_size(*at, vec2(14.0, 9.0));
            self.p.rect_filled(er, 2.0, Color32::from_rgb(240, 200, 210));
            self.p.rect_stroke(er, 2.0, self.stroke(self.ink()), egui::StrokeKind::Inside);
        }
    }

    /// The wobbly ring fades while a clean circle takes its place.
    fn in_ink_to_shape(&self) {
        let b = self.in_sheet_full(true);
        self.in_marks(b, 5, 5, (0, 5, 0, 5), true);
        let c = b.center();
        let k = self.reveal(0.2, 0.7);
        let wob: Vec<Pos2> = (0..=24)
            .map(|i| {
                let a = i as f32 / 24.0 * std::f32::consts::TAU;
                let r = 22.0 + 3.5 * (a * 3.0).sin() + 2.0 * (a * 5.0).cos();
                c + vec2(a.cos() * r, a.sin() * r * 0.75)
            })
            .collect();
        if k < 1.0 {
            self.p.add(Shape::closed_line(wob, Stroke::new(2.5, Color32::from_rgb(220, 60, 60).linear_multiply(1.0 - k))));
        }
        if k > 0.0 {
            let pts: Vec<Pos2> = (0..=32)
                .map(|i| {
                    let a = i as f32 / 32.0 * std::f32::consts::TAU;
                    c + vec2(a.cos() * 22.0, a.sin() * 16.5)
                })
                .collect();
            self.p.add(Shape::closed_line(pts, Stroke::new(2.0, self.accent().linear_multiply(k))));
        }
    }
}
