//! Page Layout tab thumbnails: how the sheet goes onto paper — themes, margins, orientation,
//! paper size, the print area, page breaks, print titles, the sheet's direction — and the
//! items of the Margins, Orientation, Print Area and Breaks menus. Mostly a printed page with
//! the grid on it and the one thing the button changes drawn in the accent colour.

use egui::{Align2, Color32, Rect, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Themes",
        "Changes the whole workbook's look in one go: the colours, the fonts and the chart styles all switch together. Example: table headers turn from blue to green and the headings change font.",
        |d| d.pl_themes(),
    ),
    Entry::new(
        "Margins",
        "Sets the empty space between the printed cells and the edge of the paper: Normal, Wide, Narrow or your own numbers. Example: Narrow fits two more columns on the page.",
        |d| d.pl_margins_page(0.6, true),
    ),
    Entry::new(
        "Page Layout|Orientation",
        "Chooses whether the sheet prints on upright (portrait) or sideways (landscape) paper. Example: a wide table with twelve month columns prints sideways so it fits.",
        |d| d.pl_orientation(),
    ),
    Entry::new(
        "Paper Size",
        "Picks the paper the sheet prints on (A4, A3, Letter…) so the page breaks match the printer. Example: switch from A4 to A3 to fit the whole table on one sheet.",
        |d| d.pl_paper_size(),
    ),
    Entry::new(
        "Print Area",
        "Marks the selected cells as the only part of the sheet that prints; everything outside is left out. Example: print just the summary block, not the working columns beside it.",
        |d| d.pl_print_area(1.0),
    ),
    Entry::new(
        "Page Breaks",
        "Decides where one printed page ends and the next begins; add a break above a row or before a column, or remove the ones you made. Example: start every department on a new page.",
        |d| d.pl_page_breaks(),
    ),
    Entry::new(
        "Print Titles",
        "Repeats chosen rows or columns at the top or side of every printed page, so long tables keep their headings. Example: the header row with «الاسم» and «الرتبة» prints on every page.",
        |d| d.pl_print_titles(),
    ),
    Entry::new(
        "Sheet Right-to-Left",
        "Flips the sheet so column A is on the right and the row numbers sit on the right edge, the natural way for Arabic. Example: the first column of names starts at the right side of the window.",
        |d| d.pl_sheet_rtl(),
    ),
    // ---- Margins menu ----
    Entry::new(
        "pageLayout.margins:normal",
        "The usual margins: about ٢ cm at the top and bottom and ١٫٨ cm at the sides. Example: the standard spacing for an ordinary report page.",
        |d| d.pl_margins_page(0.6, false),
    ),
    Entry::new(
        "pageLayout.margins:wide",
        "Large margins: more empty paper around the cells, room for notes or a binder punch. Example: a document that will be filed in a ring binder.",
        |d| d.pl_margins_page(1.0, false),
    ),
    Entry::new(
        "pageLayout.margins:narrow",
        "Small margins: the cells reach almost to the edge of the paper, so more columns fit on each page. Example: a wide table fits on one page instead of two.",
        |d| d.pl_margins_page(0.3, false),
    ),
    Entry::new(
        "dialog:pageSetup",
        "Opens the page setup dialog, where you type your own margins and set the orientation, paper size, scaling and header and footer together. Example: set a ٣ cm left margin and shrink the sheet to ٨٠٪.",
        |d| {
            d.dialog("إعداد الصفحة", 3);
            let k = d.reveal(0.2, 0.6);
            if k > 0.0 {
                d.p.rect_stroke(
                    d.page(118.0, 36.0, 30.0 * k.max(0.05), 40.0 * k.max(0.05)).expand(8.0),
                    2.0,
                    d.stroke(d.accent()),
                    egui::StrokeKind::Inside,
                );
            }
        },
    ),
    // ---- Orientation menu ----
    Entry::new(
        "pageLayout.orientation:portrait",
        "Prints on upright paper, taller than it is wide: more rows per page, fewer columns. Example: a long list of names prints as a tall page.",
        |d| d.pl_single_page(false),
    ),
    Entry::new(
        "pageLayout.orientation:landscape",
        "Prints on sideways paper, wider than it is tall: more columns per page, fewer rows. Example: a table with twelve month columns fits across the page.",
        |d| d.pl_single_page(true),
    ),
    // ---- Print Area menu ----
    Entry::new(
        "pageLayout.printArea",
        "Set makes the selected cells the only part that prints; Clear removes that mark so the whole sheet prints again. Example: set it on the summary block, clear it when you need the full sheet.",
        |d| d.pl_print_area(1.0),
    ),
    // ---- Breaks menu ----
    Entry::new(
        "pageLayout.breaks",
        "Insert starts a new printed page above the selected row (or before the selected column); Reset All removes every break you added so the sheet paginates by itself again. Example: a break above row ٢٠ so the second department starts on a new page.",
        |d| d.pl_break_insert(),
    ),
    Entry::new(
        "pageLayout.breaks:remove",
        "Removes the page break next to the selected cell, so the rows flow onto the same page again. Example: the break above row ٢٠ goes and the department continues on page ١.",
        |d| d.pl_break_remove(),
    ),
];

impl Draw<'_> {
    /// A sheet whose header colour switches as the swatch row's pick moves.
    fn pl_themes(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 26.0), vec2(SIZE.x - 12.0, 72.0)), 4, 5, true);
        let k = self.reveal(0.2, 0.6);
        let before = self.accent();
        let after = Color32::from_rgb(46, 139, 87);
        let mix = |a: Color32, b: Color32, t: f32| {
            let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
            Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
        };
        let head = mix(before, after, k);
        self.p.rect_filled(self.in_range(body, 4, 5, (0, 0, 0, 4), true), 0.0, head);
        for i in 0..4 {
            for j in 0..5 {
                let c = if i == 0 { Color32::WHITE } else { self.grey() };
                self.in_mark(self.in_cell(body, 4, 5, i, j, true), true, 0.65, c);
            }
        }
        for i in [1usize, 3] {
            self.p.rect_filled(self.in_range(body, 4, 5, (i, i, 0, 4), true), 0.0, head.linear_multiply(0.15));
        }
        let swatches = [before, after, Color32::from_rgb(200, 120, 40), Color32::from_rgb(120, 80, 160)];
        for (i, c) in swatches.iter().enumerate() {
            let r = Rect::from_min_size(self.at(SIZE.x - 30.0 - i as f32 * 22.0, 6.0), vec2(18.0, 14.0));
            self.p.rect_filled(r, 2.0, *c);
            let picked = if k > 0.5 { i == 1 } else { i == 0 };
            if picked {
                self.p.rect_stroke(r.expand(2.0), 3.0, Stroke::new(1.5, self.ink()), egui::StrokeKind::Inside);
            }
        }
    }

    /// A page with the grid inside a dashed margin box `m` wide (fraction of the usual) and
    /// measure arrows; the margin box breathes in from the edge when `film`.
    fn pl_margins_page(&self, m: f32, film: bool) {
        let outer = Rect::from_min_size(self.at(50.0, 6.0), vec2(76.0, 92.0));
        self.p.rect_filled(outer, 2.0, Color32::WHITE);
        self.p.rect_stroke(outer, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let k = if film { self.reveal(0.1, 0.6) } else { 1.0 };
        let inset = 2.0 + 14.0 * m * k;
        let inner = outer.shrink(inset);
        self.grid(inner, 6, 4, self.t.border);
        self.in_dashed(inner.expand(1.5), self.accent());
        self.measure_h(pos2(outer.min.x, outer.min.y - 0.0 + 46.0), pos2(inner.min.x, outer.min.y + 46.0));
        self.measure_v(pos2(outer.center().x, outer.min.y), pos2(outer.center().x, inner.min.y));
    }

    /// A tall page, an arrow, and the same grid on a wide page.
    fn pl_orientation(&self) {
        let p1 = self.page(8.0, 14.0, 56.0, 76.0);
        self.grid(p1, 6, 3, self.t.border);
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(70.0, 52.0), self.at(70.0 + 14.0 * arrow, 52.0));
        }
        let k = self.reveal(0.3, 0.7);
        if k <= 0.0 {
            return;
        }
        let w = 80.0 * (0.4 + 0.6 * k);
        let h = 56.0 + 20.0 * (1.0 - k);
        let p2 = self.page(92.0, 52.0 - h / 2.0, w, h);
        self.grid(p2, 3, 6, self.t.border);
        self.p.rect_stroke(p2.expand(8.0), 2.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
    }

    /// A small page grows into a larger one; the badge names the size.
    fn pl_paper_size(&self) {
        let small = self.page(14.0, 30.0, 40.0, 54.0);
        self.grid(small, 4, 3, self.t.border);
        self.badge(pos2(34.0 + self.r.min.x, 20.0 + self.r.min.y), "A4", self.grey());
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(64.0, 56.0), self.at(64.0 + 14.0 * arrow, 56.0));
        }
        let k = self.reveal(0.3, 0.7);
        if k <= 0.0 {
            return;
        }
        let w = 40.0 + 24.0 * k;
        let h = 54.0 + 30.0 * k;
        let big = self.page(94.0, 97.0 - h, w, h);
        self.grid(big, 6, 4, self.t.border);
        self.p.rect_stroke(big.expand(8.0), 2.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        self.badge(self.at(126.0, 10.0), "A3", self.accent());
    }

    /// The sheet with a dashed accent frame round one block; the rest fades to grey.
    fn pl_print_area(&self, strength: f32) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let area = self.in_range(body, 5, 5, (0, 3, 0, 2), true);
        let k = self.reveal(0.1, 0.5) * strength;
        if k > 0.0 {
            let dim = Color32::from_rgba_unmultiplied(120, 120, 120, (90.0 * k) as u8);
            self.p.rect_filled(Rect::from_min_max(body.min, pos2(area.min.x, body.max.y)), 0.0, dim);
            self.p.rect_filled(Rect::from_min_max(pos2(area.min.x, area.max.y), body.max), 0.0, dim);
            self.in_dashed(area.shrink(0.5), self.accent());
        }
        let tag = self.reveal(0.5, 0.75);
        if tag > 0.0 {
            self.badge(pos2(area.min.x + 2.0, area.max.y + 2.0), "طباعة", self.accent().linear_multiply(tag));
        }
    }

    /// Dashed break lines cut the sheet into numbered pages.
    fn pl_page_breaks(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let k1 = self.reveal(0.1, 0.4);
        let k2 = self.reveal(0.4, 0.7);
        let y = body.min.y + body.height() * 3.0 / 5.0;
        let x = body.min.x + body.width() * 2.0 / 5.0;
        self.pl_dash_line(pos2(body.max.x, y), pos2(body.max.x - body.width() * k1, y));
        self.pl_dash_line(pos2(x, body.min.y), pos2(x, body.min.y + body.height() * k2));
        if k2 >= 1.0 {
            for (n, c) in [
                (1usize, pos2(body.max.x - 16.0, body.min.y + 12.0)),
                (2, pos2(body.min.x + 14.0, body.min.y + 12.0)),
                (3, pos2(body.max.x - 16.0, body.max.y - 10.0)),
                (4, pos2(body.min.x + 14.0, body.max.y - 10.0)),
            ] {
                self.badge(c, Draw::in_digit(n), self.accent());
            }
        }
    }

    /// A dashed accent line from `a` to `b` (a page break).
    pub(super) fn pl_dash_line(&self, a: egui::Pos2, b: egui::Pos2) {
        let len = (b - a).length();
        if len < 1.0 {
            return;
        }
        let dir = (b - a) / len;
        let mut t = 0.0;
        while t < len {
            let t1 = (t + 4.0).min(len);
            self.line(a + dir * t, a + dir * t1, self.accent(), 1.5);
            t += 7.0;
        }
    }

    /// Two printed pages; the header row prints on the second one too.
    fn pl_print_titles(&self) {
        let p1 = self.page(8.0, 8.0, 70.0, 88.0);
        let p2 = self.page(98.0, 8.0, 70.0, 88.0);
        for (i, a) in [p1, p2].iter().enumerate() {
            let grid = Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), a.max);
            self.grid(grid, 6, 3, self.t.border);
            self.in_marks(grid, 6, 3, (0, 6, 0, 3), true);
            let head = Rect::from_min_size(a.min, vec2(a.width(), 10.0));
            let k = if i == 0 { 1.0 } else { self.reveal(0.2, 0.6) };
            if k > 0.0 {
                self.in_clip(Rect::from_min_max(pos2(head.max.x - head.width() * k, head.min.y), head.max), |d| {
                    d.p.rect_filled(head, 0.0, d.accent());
                    for j in 0..3 {
                        d.in_mark(d.in_cell(head, 1, 3, 0, j, true), true, 0.7, Color32::WHITE);
                    }
                });
            }
            self.badge(pos2(a.center().x, a.max.y - 2.0), Draw::in_digit(i + 1), self.grey());
        }
        let arrow = self.reveal(0.05, 0.25);
        if arrow > 0.0 {
            self.arrow_right(self.at(81.0, 52.0), self.at(81.0 + 14.0 * arrow, 52.0));
        }
    }

    /// A left-to-right sheet becomes a right-to-left one: column A and the row numbers move
    /// to the right.
    fn pl_sheet_rtl(&self) {
        let l = self.in_sheet(Rect::from_min_size(self.at(6.0, 14.0), vec2(72.0, 76.0)), 4, 3, false);
        self.in_marks(l, 4, 3, (0, 4, 0, 3), false);
        self.p.rect_filled(self.in_cell(l, 4, 3, 0, 0, false), 0.0, self.accent().linear_multiply(0.35));
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(82.0, 52.0), self.at(82.0 + 12.0 * arrow, 52.0));
        }
        let k = self.reveal(0.3, 0.7);
        if k <= 0.0 {
            return;
        }
        let outer = Rect::from_min_size(self.at(98.0, 14.0), vec2(72.0, 76.0));
        self.in_clip(Rect::from_min_max(pos2(outer.max.x - outer.width() * k, outer.min.y), outer.max), |d| {
            let r = d.in_sheet(outer, 4, 3, true);
            d.in_marks(r, 4, 3, (0, 4, 0, 3), true);
            d.p.rect_filled(d.in_cell(r, 4, 3, 0, 0, true), 0.0, d.accent().linear_multiply(0.35));
            d.p.rect_stroke(outer, 1.0, Stroke::new(1.5, d.accent()), egui::StrokeKind::Inside);
        });
    }

    /// One page, upright or sideways, with its grid; it settles in from a little smaller.
    fn pl_single_page(&self, landscape: bool) {
        let k = 0.85 + 0.15 * self.reveal(0.1, 0.5);
        let (w, h) = if landscape { (120.0 * k, 78.0 * k) } else { (70.0 * k, 92.0 * k) };
        let a = self.page(SIZE.x / 2.0 - w / 2.0, SIZE.y / 2.0 - h / 2.0, w, h);
        if landscape {
            self.grid(a, 4, 7, self.t.border);
        } else {
            self.grid(a, 7, 3, self.t.border);
        }
        self.p.rect_stroke(a.expand(8.0), 2.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        self.label(pos2(a.center().x, a.max.y - 4.0), if landscape { "أفقي" } else { "عمودي" }, 7.0, self.accent(), Align2::CENTER_CENTER);
    }

    /// A row is selected and a new dashed break draws in above it.
    fn pl_break_insert(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let row = self.in_range(body, 5, 5, (3, 3, 0, 4), true);
        self.in_select(row);
        let k = self.reveal(0.2, 0.6);
        self.pl_dash_line(pos2(body.max.x, row.min.y), pos2(body.max.x - body.width() * k, row.min.y));
        if k >= 1.0 {
            self.badge(pos2(body.min.x + 14.0, row.min.y - 10.0), "١", self.grey());
            self.badge(pos2(body.min.x + 14.0, row.min.y + 10.0), "٢", self.accent());
        }
    }

    /// The dashed break fades under a red cross and the rows run on.
    fn pl_break_remove(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let row = self.in_range(body, 5, 5, (3, 3, 0, 4), true);
        self.in_select(row);
        let k = self.reveal(0.2, 0.6);
        if k < 1.0 {
            let c = self.accent().linear_multiply(1.0 - k);
            let mut x = body.max.x;
            while x > body.min.x {
                self.line(pos2(x, row.min.y), pos2((x - 4.0).max(body.min.x), row.min.y), c, 1.5);
                x -= 7.0;
            }
        }
        if k > 0.3 {
            self.cross(pos2(body.center().x, row.min.y));
        }
    }
}
