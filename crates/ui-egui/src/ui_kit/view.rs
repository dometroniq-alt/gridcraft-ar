//! View tab entries: zoom and frozen panes. Also home of the shared mini-worksheet helpers
//! (`vw_sheet`, `vw_two`, `vw_text`…) the Review and Misc files draw with.

#![allow(unused_imports)]

use egui::{Align2, Color32, Rect, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Zoom…",
        "Opens a box where you pick how large the sheet is drawn on the screen: 200% for big cells, 50% to see more of the sheet at once. The numbers themselves do not change. Example: choose 75% and twice as many rows fit on the screen.",
        |d| d.vw_zoom_dialog(),
    ),
    Entry::new(
        "Zoom to 100%",
        "Puts the sheet back at its normal size after you zoomed in or out. Example: a sheet shown at 50% returns to 100%, cells at their usual size.",
        |d| d.vw_zoom_100(),
    ),
    Entry::new(
        "Zoom to Selection",
        "Enlarges the sheet until the cells you selected fill the window, so a small table is easy to read. Example: select A1:D6 and those cells grow to fill the screen.",
        |d| d.vw_zoom_selection(),
    ),
    Entry::new(
        "Freeze Panes",
        "Opens the choices for keeping rows or columns in place while you scroll: the first row, the first column, or everything above and to the left of the selected cell. Example: freeze the title row and it stays visible while you scroll down a long list.",
        |d| d.vw_freeze_menu(),
    ),
    Entry::new(
        "view.freezePanes",
        "Keeps every row above the selected cell and every column to its left in place while the rest of the sheet scrolls. Example: select B2, and row 1 and column A stay visible as you scroll.",
        |d| d.vw_frozen(true, true, false),
    ),
    Entry::new(
        "view.freezeTopRow",
        "Keeps the first row in place while you scroll down, so the column titles are always visible. Example: scroll to row 500 and the titles in row 1 are still at the top.",
        |d| d.vw_frozen(true, false, false),
    ),
    Entry::new(
        "view.freezeFirstColumn",
        "Keeps the first column in place while you scroll to the right, so the names in column A are always visible. Example: scroll to column Z and the names in A are still at the side.",
        |d| d.vw_frozen(false, true, false),
    ),
    Entry::new(
        "view.unfreezePanes",
        "Lets every row and column scroll again; nothing stays fixed on the screen. Example: the frozen title row scrolls away with the rest of the sheet.",
        |d| d.vw_frozen(true, true, true),
    ),
];

/// Arabic-Indic digits for the numbers shown in a sample sheet. egui 0.36 draws a run of
/// Arabic-script characters right-to-left as a whole, with no bidi pass, and these digits are
/// Arabic script: the string is reversed here so ١٢٠ comes out as ١٢٠ and not ٠٢١ (also when
/// it follows Arabic words). Drop the reversal once the renderer orders digits.
pub(super) fn vw_digits(n: u32) -> String {
    n.to_string().chars().rev().map(|c| c.to_digit(10).and_then(|d| char::from_u32(0x0660 + d)).unwrap_or(c)).collect()
}

impl Draw<'_> {
    /// A mini worksheet in `r`: column letters across the top, row numbers down the left,
    /// `rows × cols` cells; returns the cell area.
    pub(super) fn vw_sheet(&self, r: Rect, rows: usize, cols: usize) -> Rect {
        const HW: f32 = 12.0;
        const HH: f32 = 9.0;
        let (rows, cols) = (rows.max(1), cols.max(1));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        let top = Rect::from_min_max(r.min, pos2(r.max.x, r.min.y + HH));
        let left = Rect::from_min_max(r.min, pos2(r.min.x + HW, r.max.y));
        self.p.rect_filled(top, 0.0, self.t.hover);
        self.p.rect_filled(left, 0.0, self.t.hover);
        let area = Rect::from_min_max(pos2(r.min.x + HW, r.min.y + HH), r.max);
        self.grid(area, rows, cols, self.t.border);
        for j in 0..cols {
            let c = self.cell(area, rows, cols, 0, j);
            let letter = char::from(b'A' + (j % 26) as u8).to_string();
            self.label(pos2(c.center().x, top.center().y), &letter, 6.0, self.grey(), Align2::CENTER_CENTER);
        }
        for i in 0..rows {
            let c = self.cell(area, rows, cols, i, 0);
            self.label(pos2(left.center().x, c.center().y), &(i + 1).to_string(), 6.0, self.grey(), Align2::CENTER_CENTER);
        }
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        area
    }
    /// A short text in cell `c`: right-aligned (Arabic words) unless `ltr`.
    pub(super) fn vw_text(&self, c: Rect, s: &str, color: Color32, ltr: bool) {
        if ltr {
            self.label(pos2(c.min.x + 2.0, c.center().y), s, 6.5, color, Align2::LEFT_CENTER);
        } else {
            self.label(pos2(c.max.x - 2.0, c.center().y), s, 6.5, color, Align2::RIGHT_CENTER);
        }
    }
    /// A number in cell `c`, in Arabic-Indic digits, right-aligned.
    pub(super) fn vw_num(&self, c: Rect, n: u32, color: Color32) {
        self.vw_text(c, &vw_digits(n), color, false);
    }
    /// A grey "text" dash in cell `c` (content that does not matter).
    pub(super) fn vw_dash(&self, c: Rect, color: Color32) {
        let y = c.center().y;
        self.line(pos2(c.min.x + 3.0, y), pos2(c.min.x + c.width() * 0.6, y), color, 1.5);
    }
    /// The frame of a selected range: the selection fill and a 1.5 pt accent border.
    pub(super) fn vw_select(&self, r: Rect) {
        self.selection(r);
        self.p.rect_stroke(r, 0.0, Stroke::new(1.5, self.t.accent), egui::StrokeKind::Outside);
    }
    /// Two panels with an arrow between them, `before(rect)` then `after(rect)`; the after
    /// panel is wiped in from the left as the film plays (the sheet-flavoured `before_after`).
    pub(super) fn vw_two(&self, before: impl FnOnce(&Draw<'_>, Rect), after: impl FnOnce(&Draw<'_>, Rect)) {
        let b = Rect::from_min_size(self.at(4.0, 14.0), vec2(72.0, 76.0));
        let a = Rect::from_min_size(self.at(100.0, 14.0), vec2(72.0, 76.0));
        before(self, b);
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * arrow, 52.0));
        }
        let wipe = self.reveal(0.3, 0.75);
        if wipe <= 0.0 {
            self.p.rect_filled(a, 1.0, Color32::WHITE);
            self.p.rect_stroke(a, 1.0, self.stroke(self.t.border), egui::StrokeKind::Inside);
            return;
        }
        let clip = Rect::from_min_max(a.min - vec2(2.0, 2.0), pos2(a.min.x + a.width() * wipe + 2.0, a.max.y + 2.0));
        let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clipped);
        after(&d, a);
        if wipe < 1.0 {
            let rest = self.p.with_clip_rect(Rect::from_min_max(pos2(clip.max.x, clip.min.y), a.max + vec2(2.0, 2.0)).intersect(self.p.clip_rect()));
            rest.rect_filled(a, 1.0, Color32::WHITE);
            rest.rect_stroke(a, 1.0, self.stroke(self.t.border), egui::StrokeKind::Inside);
        }
    }
    /// A sample sheet with a title row and a few names and numbers (the one most films start
    /// from); returns the cell area.
    pub(super) fn vw_sample(&self, r: Rect, rows: usize, cols: usize) -> Rect {
        let area = self.vw_sheet(r, rows, cols);
        let names = ["أحمد", "سارة", "علي", "نور", "ليث", "هدى", "زيد"];
        let values = [120, 85, 300, 42, 210, 95, 160];
        for i in 0..rows {
            for j in 0..cols {
                let c = self.cell(area, rows, cols, i, j);
                if i == 0 {
                    self.vw_dash(c, self.ink());
                } else if j == 0 {
                    if let Some(n) = names.get((i - 1) % names.len()) {
                        self.vw_text(c, n, self.grey(), false);
                    }
                } else if let Some(v) = values.get((i - 1 + j) % values.len()) {
                    self.vw_num(c, *v, self.grey());
                }
            }
        }
        area
    }

    // ---- zoom ----

    fn vw_zoom_dialog(&self) {
        let inner = self.dialog("Zoom", 0);
        let options = ["200%", "100%", "75%", "50%"];
        for (i, o) in options.iter().enumerate() {
            let y = inner.min.y + 6.0 + i as f32 * 12.0;
            let picked = i == 2;
            let c = pos2(inner.min.x + 5.0, y);
            self.p.circle_stroke(c, 3.5, self.stroke(self.grey()));
            if picked && self.reveal(0.3, 0.5) > 0.0 {
                self.p.circle_filled(c, 2.0, self.accent());
            }
            self.label(pos2(inner.min.x + 13.0, y), o, 7.0, if picked { self.ink() } else { self.grey() }, Align2::LEFT_CENTER);
        }
        // A sheet preview at the right of the box, cells shrinking to the picked size.
        let grow = 1.0 - 0.25 * self.reveal(0.5, 0.8);
        let pr = Rect::from_min_size(pos2(inner.min.x + 52.0, inner.min.y + 2.0), vec2(48.0, 44.0));
        let cells = (4.0 / grow).round().max(1.0) as usize;
        self.grid(pr, cells, cells, self.t.border);
        self.magnifier(pos2(pr.max.x - 4.0, pr.max.y - 4.0));
    }

    fn vw_zoom_100(&self) {
        self.vw_two(
            |d, r| {
                // Zoomed far out: many tiny cells.
                d.vw_sheet(r, 10, 7);
                d.badge(pos2(r.center().x, r.max.y - 8.0), "50%", d.grey());
            },
            |d, r| {
                d.vw_sample(r, 4, 3);
                d.badge(pos2(r.center().x, r.max.y - 8.0), "100%", d.accent());
            },
        );
    }

    fn vw_zoom_selection(&self) {
        self.vw_two(
            |d, r| {
                let area = d.vw_sheet(r, 8, 6);
                let sel = Rect::from_min_max(d.cell(area, 8, 6, 1, 1).min, d.cell(area, 8, 6, 3, 2).max);
                d.vw_select(sel);
            },
            |d, r| {
                // The selected block alone, filling the panel.
                let area = d.vw_sheet(r, 3, 2);
                d.vw_select(area);
                d.magnifier(pos2(r.max.x - 10.0, r.max.y - 10.0));
            },
        );
    }

    // ---- frozen panes ----

    /// The sheet with its frozen lines, the lower part scrolling up as the film plays. With
    /// `unfreeze`, the lines fade and everything scrolls together.
    fn vw_frozen(&self, top: bool, left: bool, unfreeze: bool) {
        let r = Rect::from_min_size(self.at(16.0, 8.0), vec2(144.0, 88.0));
        let (rows, cols) = (5, 4);
        let area = self.vw_sheet(r, rows, cols);
        let rh = area.height() / rows as f32;
        let cw = area.width() / cols as f32;
        let scroll = self.reveal(0.3, 0.8) * rh * 0.6;
        let frozen_rows = if top && !unfreeze { 1 } else { 0 };
        let frozen_cols = if left && !unfreeze { 1 } else { 0 };
        // Scrolling content: everything outside the frozen band moves up; the frozen band stays.
        let clip = Rect::from_min_max(pos2(area.min.x + frozen_cols as f32 * cw, area.min.y + frozen_rows as f32 * rh), area.max);
        let moving = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        let m = self.with_painter(&moving);
        for i in 0..=rows {
            for j in 0..cols {
                let c = self.cell(area, rows, cols, i, j).translate(vec2(0.0, -scroll));
                if i < frozen_rows || j < frozen_cols {
                    continue;
                }
                if (i + j) % 3 == 0 {
                    m.vw_num(c, 100 + (i * 7 + j * 13) as u32, self.grey());
                } else {
                    m.vw_dash(c, self.grey());
                }
                if scroll > 0.0 {
                    m.line(pos2(c.min.x, c.max.y), pos2(c.max.x, c.max.y), self.t.border, 1.0);
                }
            }
        }
        // The frozen band keeps its titles.
        for j in 0..cols {
            if frozen_rows > 0 {
                let c = self.cell(area, rows, cols, 0, j);
                self.p.rect_filled(c, 0.0, Color32::WHITE);
                self.vw_dash(c, self.ink());
            }
        }
        for i in 0..rows {
            if frozen_cols > 0 && i >= frozen_rows {
                let c = self.cell(area, rows, cols, i, 0);
                self.p.rect_filled(c, 0.0, Color32::WHITE);
                match ["", "أحمد", "سارة", "علي", "نور"].get(i).copied() {
                    Some("") | None => self.vw_dash(c, self.ink()),
                    Some(name) => self.vw_text(c, name, self.grey(), false),
                }
            }
        }
        self.grid(area, rows, cols, self.t.border);
        if unfreeze {
            // The old lines, fading out.
            let fade = 1.0 - self.reveal(0.0, 0.4);
            if fade > 0.0 {
                let c = self.accent().linear_multiply(fade);
                self.line(pos2(area.min.x, area.min.y + rh), pos2(area.max.x, area.min.y + rh), c, 2.0);
                self.line(pos2(area.min.x + cw, area.min.y), pos2(area.min.x + cw, area.max.y), c, 2.0);
            }
            self.arrow_up(pos2(r.max.x + 8.0, area.center().y + 14.0), pos2(r.max.x + 8.0, area.center().y - 14.0));
            return;
        }
        if top {
            self.line(pos2(area.min.x, area.min.y + rh), pos2(area.max.x, area.min.y + rh), self.accent(), 2.0);
        }
        if left {
            self.line(pos2(area.min.x + cw, area.min.y), pos2(area.min.x + cw, area.max.y), self.accent(), 2.0);
        }
        if top && left {
            self.vw_select(self.cell(area, rows, cols, 1, 1));
        }
        self.arrow_up(pos2(r.max.x + 8.0, area.center().y + 14.0), pos2(r.max.x + 8.0, area.center().y - 14.0));
    }

    /// The Freeze Panes button: the sheet with its menu of three choices open beside it.
    fn vw_freeze_menu(&self) {
        let r = Rect::from_min_size(self.at(6.0, 8.0), vec2(96.0, 88.0));
        let (rows, cols) = (5, 3);
        let area = self.vw_sample(r, rows, cols);
        let rh = area.height() / rows as f32;
        let cw = area.width() / cols as f32;
        self.line(pos2(area.min.x, area.min.y + rh), pos2(area.max.x, area.min.y + rh), self.accent(), 2.0);
        self.line(pos2(area.min.x + cw, area.min.y), pos2(area.min.x + cw, area.max.y), self.accent(), 2.0);
        // The menu: three rows, each with a tiny picture of the line it freezes.
        let menu = Rect::from_min_size(self.at(108.0, 20.0), vec2(62.0, 64.0));
        self.p.rect_filled(menu, 2.0, self.t.menu);
        self.p.rect_stroke(menu, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        for (i, (h, v)) in [(true, true), (true, false), (false, true)].into_iter().enumerate() {
            let show = self.reveal(0.2 + i as f32 * 0.15, 0.3 + i as f32 * 0.15);
            if show <= 0.0 {
                continue;
            }
            let y = menu.min.y + 6.0 + i as f32 * 19.0;
            let ic = Rect::from_min_size(pos2(menu.min.x + 5.0, y), vec2(16.0, 12.0));
            self.grid(ic, 3, 3, self.t.border);
            if h {
                self.line(pos2(ic.min.x, ic.min.y + 4.0), pos2(ic.max.x, ic.min.y + 4.0), self.accent(), 1.5);
            }
            if v {
                self.line(pos2(ic.min.x + 5.3, ic.min.y), pos2(ic.min.x + 5.3, ic.max.y), self.accent(), 1.5);
            }
            self.line(pos2(ic.max.x + 5.0, y + 6.0), pos2(menu.max.x - 5.0, y + 6.0), self.grey(), 1.5);
        }
    }
}
