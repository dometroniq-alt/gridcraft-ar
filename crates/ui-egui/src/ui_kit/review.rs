//! Review tab entries: spelling, statistics, accessibility, comments and notes, protection.

#![allow(unused_imports)]

use egui::{Align2, Color32, Rect, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Dir, Draw, SIZE};
use super::view::vw_digits;

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Spelling (F7)",
        "Checks the spelling of the words in your sheet and suggests a correction for each word it does not know; you accept it or skip it. Example: «recieved» is flagged and «received» offered.",
        |d| d.rv_spelling(),
    ),
    Entry::new(
        "Workbook Statistics",
        "Shows a count of what is in the workbook: how many sheets, filled cells, formulas, tables and charts. Example: 3 sheets, 120 cells with data, 45 formulas.",
        |d| d.rv_statistics(),
    ),
    Entry::new(
        "Check Accessibility",
        "Looks for things that make the sheet hard to use for people with a screen reader or weak eyesight, and lists them with a fix. Example: a picture with no description, or grey text on a grey fill.",
        |d| d.rv_accessibility(),
    ),
    Entry::new(
        "Delete Comment",
        "Removes the comment attached to the selected cell, with all its replies; the cell's value stays. Example: the comment mark in the corner of B3 disappears.",
        |d| d.rv_delete_comment(),
    ),
    Entry::new(
        "Comments pane",
        "Opens a list of every comment in the sheet at the side of the window, so you can read them, reply and resolve them without hunting for the cells. Example: three comments on three different cells, listed together.",
        |d| d.rv_comments_pane(),
    ),
    Entry::new(
        "Notes",
        "Opens the choices for notes: a note is a yellow box of text attached to one cell, like a sticky note, shown when you point at the cell. Example: a note on B3 saying «check this figure».",
        |d| d.rv_notes_button(),
    ),
    Entry::new(
        "Protect Workbook structure",
        "Locks the list of sheets: nobody can add, delete, rename, hide or move a sheet until the workbook is unprotected (with the password, if you set one). Example: the «+» for a new sheet stops working.",
        |d| d.rv_protect_workbook(),
    ),
    Entry::new(
        "Allow Edit Ranges",
        "Chooses a few ranges that people may still fill in once the sheet is protected, each with its own password if you like. Example: the whole sheet is locked except B2:C6, where the clerk types the figures.",
        |d| d.rv_allow_edit_ranges(),
    ),
    Entry::new(
        "dialog:comment",
        "Attaches a yellow note to the selected cell and lets you type in it; a small red corner on the cell shows the note is there. Example: a note on B3 saying «check this figure».",
        |d| d.rv_new_note(),
    ),
    Entry::new(
        "review.showNote",
        "Shows the selected cell's note all the time, or hides it again so it appears only when you point at the cell. Example: the note on B3 stays open while you work.",
        |d| d.rv_show_note(),
    ),
    Entry::new(
        "review.showAllNotes",
        "Shows every note in the sheet at once, open beside its cell, so you can read them all in one look. Example: the notes on B3, C5 and D2 all open together.",
        |d| d.rv_show_all_notes(),
    ),
    Entry::new(
        "review.convertToComments",
        "Turns the sheet's notes into comments: the text stays, and people can now reply under it with their name and the time. Example: the yellow note on B3 becomes a comment thread.",
        |d| d.rv_convert_to_comments(),
    ),
];

impl Draw<'_> {
    // ---- shared pieces ----

    /// A red wavy line under a misspelt word, from `x0` to `x1`, growing with `k` (0 → 1).
    pub(super) fn rv_wavy(&self, x0: f32, x1: f32, y: f32, k: f32) {
        let end = x0 + (x1 - x0) * k;
        let mut x = x0;
        let mut up = true;
        while x + 2.0 <= end {
            let a = pos2(x, if up { y } else { y + 2.0 });
            let b = pos2(x + 2.0, if up { y + 2.0 } else { y });
            self.line(a, b, self.red(), 1.0);
            x += 2.0;
            up = !up;
        }
    }
    /// The note / comment mark in the top-right corner of cell `c`: red for a note, purple for
    /// a comment.
    pub(super) fn rv_mark(&self, c: Rect, comment: bool) {
        let color = if comment { Color32::from_rgb(120, 70, 190) } else { self.red() };
        let pts = vec![pos2(c.max.x - 5.0, c.min.y + 0.5), pos2(c.max.x - 0.5, c.min.y + 0.5), pos2(c.max.x - 0.5, c.min.y + 5.0)];
        self.p.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
    }
    /// A yellow sticky note in `r` with `lines` text lines (or an empty one with a caret), tied
    /// to `from` by a thin line.
    pub(super) fn rv_note(&self, r: Rect, from: egui::Pos2, lines: usize) {
        self.line(from, r.left_center(), self.grey(), 1.0);
        self.p.rect_filled(r, 1.0, Color32::from_rgb(255, 242, 170));
        self.p.rect_stroke(r, 1.0, self.stroke(Color32::from_rgb(200, 170, 60)), egui::StrokeKind::Inside);
        let inner = r.shrink(4.0);
        if lines == 0 {
            self.caret(pos2(inner.max.x - 1.0, inner.min.y), 8.0);
            return;
        }
        let widths: Vec<f32> = (0..lines).map(|i| inner.width() * if i + 1 == lines { 0.5 } else { 0.9 }).collect();
        self.text_lines(Rect::from_min_max(pos2(inner.min.x, inner.min.y - 2.0), inner.max), true, &widths, 7.0, self.ink());
    }
    /// A comment card in `r`: an author circle, a name, text lines; `reply` adds an indented reply.
    pub(super) fn rv_comment_card(&self, r: Rect, reply: bool) {
        self.p.rect_filled(r, 2.0, Color32::WHITE);
        self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let author = Color32::from_rgb(120, 70, 190);
        self.p.circle_filled(pos2(r.min.x + 8.0, r.min.y + 8.0), 4.0, author);
        self.line(pos2(r.min.x + 15.0, r.min.y + 8.0), pos2(r.min.x + 15.0 + r.width() * 0.4, r.min.y + 8.0), self.ink(), 2.0);
        let body = Rect::from_min_max(pos2(r.min.x + 5.0, r.min.y + 12.0), pos2(r.max.x - 5.0, r.max.y));
        self.text_lines(body, true, &[body.width(), body.width() * 0.6], 7.0, self.grey());
        if reply && r.height() > 36.0 {
            let y = r.min.y + 32.0;
            self.p.circle_filled(pos2(r.min.x + 14.0, y), 3.0, self.accent());
            self.line(pos2(r.min.x + 20.0, y), pos2(r.max.x - 5.0, y), self.grey(), 1.5);
        }
    }

    // ---- the entries ----

    fn rv_spelling(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(96.0, 88.0));
        let (rows, cols) = (5, 2);
        let area = self.vw_sheet(r, rows, cols);
        for i in 0..rows {
            let c = self.cell(area, rows, cols, i, 0);
            self.vw_dash(c, if i == 0 { self.ink() } else { self.grey() });
            let c = self.cell(area, rows, cols, i, 1);
            if i == 2 {
                self.vw_text(c, "recieved", self.ink(), true);
                self.rv_wavy(c.min.x + 2.0, c.min.x + 30.0, c.center().y + 4.0, self.reveal(0.0, 0.25));
            } else {
                self.vw_dash(c, self.grey());
            }
        }
        let show = self.reveal(0.3, 0.55);
        if show <= 0.0 {
            return;
        }
        // The spelling box slides in from the right.
        let dx = (1.0 - show) * 40.0;
        let b = Rect::from_min_size(self.at(84.0 + dx, 24.0), vec2(86.0, 56.0));
        self.p.rect_filled(b, 3.0, self.t.menu);
        self.p.rect_stroke(b, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(b.min, vec2(b.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(b.min.x + 5.0, b.min.y + 6.0), "Spelling", 7.0, self.ink(), Align2::LEFT_CENTER);
        self.label(pos2(b.min.x + 6.0, b.min.y + 22.0), "recieved", 7.5, self.red(), Align2::LEFT_CENTER);
        self.rv_wavy(b.min.x + 6.0, b.min.x + 36.0, b.min.y + 26.0, 1.0);
        self.label(pos2(b.min.x + 6.0, b.min.y + 36.0), "received", 7.5, self.ink(), Align2::LEFT_CENTER);
        if self.reveal(0.7, 0.9) > 0.0 {
            self.check(pos2(b.max.x - 14.0, b.min.y + 36.0));
        }
        self.chip(pos2(b.min.x + 24.0, b.max.y - 8.0), "Change", 38.0);
    }

    fn rv_statistics(&self) {
        let inner = self.dialog("Statistics", 0);
        let rows: [(&str, u32); 4] = [("Sheets", 3), ("Cells", 120), ("Formulas", 45), ("Charts", 2)];
        for (i, (name, n)) in rows.into_iter().enumerate() {
            let y = inner.min.y + 5.0 + i as f32 * 12.0;
            self.label(pos2(inner.min.x, y), name, 7.0, self.grey(), Align2::LEFT_CENTER);
            // The counts tick up one row after the other.
            let k = self.reveal(0.1 + i as f32 * 0.15, 0.35 + i as f32 * 0.15);
            if k > 0.0 {
                let shown = (n as f32 * k).round() as u32;
                self.label(pos2(inner.max.x, y), &vw_digits(shown), 8.0, self.accent(), Align2::RIGHT_CENTER);
            }
        }
    }

    fn rv_accessibility(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(92.0, 88.0));
        let area = self.vw_sample(r, 5, 2);
        let pic = Rect::from_min_size(pos2(area.min.x + 30.0, area.min.y + 28.0), vec2(40.0, 26.0));
        self.picture(pic);
        let pane = self.pane(true, 72.0, 0);
        let items: [(bool, f32); 3] = [(false, 0.9), (false, 0.7), (true, 0.8)];
        for (i, (ok, w)) in items.into_iter().enumerate() {
            let show = self.reveal(0.15 + i as f32 * 0.2, 0.3 + i as f32 * 0.2);
            if show <= 0.0 {
                continue;
            }
            let y = pane.min.y + 26.0 + i as f32 * 20.0;
            if ok {
                self.p.circle_filled(pos2(pane.min.x + 11.0, y), 5.5, self.green());
                self.line(pos2(pane.min.x + 8.5, y), pos2(pane.min.x + 10.5, y + 2.0), Color32::WHITE, 1.5);
                self.line(pos2(pane.min.x + 10.5, y + 2.0), pos2(pane.min.x + 14.0, y - 2.5), Color32::WHITE, 1.5);
            } else {
                self.warning(pos2(pane.min.x + 11.0, y));
            }
            self.line(pos2(pane.min.x + 20.0, y - 2.0), pos2(pane.min.x + 20.0 + (pane.width() - 26.0) * w, y - 2.0), self.ink(), 1.5);
            self.line(pos2(pane.min.x + 20.0, y + 3.0), pos2(pane.min.x + 20.0 + (pane.width() - 26.0) * 0.5, y + 3.0), self.grey(), 1.0);
            if i == 0 {
                // The first finding points at the picture without a description.
                self.warning(pos2(pic.max.x - 2.0, pic.min.y + 2.0));
            }
        }
    }

    fn rv_delete_comment(&self) {
        self.vw_two(
            |d, r| {
                let area = d.vw_sample(r, 4, 2);
                let c = d.cell(area, 4, 2, 2, 1);
                d.vw_select(c);
                d.rv_mark(c, true);
                d.rv_comment_card(Rect::from_min_size(pos2(c.min.x - 6.0, c.max.y + 3.0), vec2(44.0, 22.0)), false);
            },
            |d, r| {
                let area = d.vw_sample(r, 4, 2);
                let c = d.cell(area, 4, 2, 2, 1);
                d.vw_select(c);
                d.cross(pos2(r.max.x - 12.0, r.max.y - 12.0));
            },
        );
    }

    fn rv_comments_pane(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(90.0, 88.0));
        let area = self.vw_sample(r, 5, 2);
        for (row, col) in [(1usize, 1usize), (3, 0), (4, 1)] {
            self.rv_mark(self.cell(area, 5, 2, row, col), true);
        }
        let pane = self.pane(true, 76.0, 0);
        for i in 0..3 {
            let show = self.reveal(0.1 + i as f32 * 0.2, 0.25 + i as f32 * 0.2);
            if show <= 0.0 {
                continue;
            }
            let card = Rect::from_min_size(pos2(pane.min.x + 5.0, pane.min.y + 17.0 + i as f32 * 26.0), vec2(pane.width() - 10.0, 23.0));
            self.rv_comment_card(card, false);
        }
    }

    fn rv_notes_button(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(100.0, 88.0));
        let area = self.vw_sample(r, 5, 2);
        let c = self.cell(area, 5, 2, 2, 1);
        self.rv_mark(c, false);
        let pop = self.reveal(0.2, 0.5);
        if pop > 0.0 {
            let note = Rect::from_min_size(self.at(110.0, 22.0), vec2(58.0 * (0.6 + 0.4 * pop), 40.0 * (0.6 + 0.4 * pop)));
            self.rv_note(note, c.right_center(), 3);
        }
        // The menu's little list under it.
        let menu = Rect::from_min_size(self.at(110.0, 70.0), vec2(58.0, 28.0));
        self.p.rect_filled(menu, 2.0, self.t.menu);
        self.p.rect_stroke(menu, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        for i in 0..3 {
            let y = menu.min.y + 6.0 + i as f32 * 8.0;
            self.line(pos2(menu.min.x + 5.0, y), pos2(menu.min.x + 5.0 + 30.0 - i as f32 * 6.0, y), self.grey(), 1.5);
        }
    }

    fn rv_protect_workbook(&self) {
        let r = Rect::from_min_size(self.at(14.0, 6.0), vec2(148.0, 66.0));
        self.vw_sample(r, 4, 3);
        // The sheet tabs under the sheet.
        let tabs = ["ورقة١", "ورقة٢", "ورقة٣"];
        let mut x = r.min.x;
        for (i, t) in tabs.iter().enumerate() {
            let tr = Rect::from_min_size(pos2(x, r.max.y + 2.0), vec2(34.0, 13.0));
            self.p.rect_filled(tr, 2.0, if i == 0 { Color32::WHITE } else { self.t.hover });
            self.p.rect_stroke(tr, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            self.label(tr.center(), t, 6.5, self.grey(), Align2::CENTER_CENTER);
            x += 37.0;
        }
        let plus = Rect::from_min_size(pos2(x, r.max.y + 2.0), vec2(13.0, 13.0));
        self.p.rect_stroke(plus, 2.0, self.stroke(self.t.border), egui::StrokeKind::Inside);
        self.label(plus.center(), "+", 8.0, self.t.text_dim.linear_multiply(0.5), Align2::CENTER_CENTER);
        // The lock comes down over the tabs and closes.
        let k = self.reveal(0.2, 0.6);
        let lc = pos2(r.max.x - 6.0, r.max.y + 9.0 - (1.0 - k) * 20.0);
        let body = Rect::from_center_size(lc + vec2(0.0, 4.0), vec2(16.0, 12.0));
        self.p.rect_filled(body, 2.0, self.accent());
        let open = 1.0 - self.reveal(0.6, 0.85);
        let shackle = lc + vec2(open * 5.0, -4.0 - open * 3.0);
        self.p.add(egui::Shape::line(
            vec![
                shackle + vec2(-5.0, 2.0),
                shackle + vec2(-5.0, -3.0),
                shackle + vec2(0.0, -6.0),
                shackle + vec2(5.0, -3.0),
                shackle + vec2(5.0, 2.0),
            ],
            Stroke::new(2.0, self.accent()),
        ));
        self.p.circle_filled(body.center(), 1.8, Color32::WHITE);
    }

    fn rv_allow_edit_ranges(&self) {
        let r = Rect::from_min_size(self.at(14.0, 8.0), vec2(148.0, 88.0));
        let (rows, cols) = (5, 4);
        let area = self.vw_sheet(r, rows, cols);
        // The protected sheet: every cell greyed.
        self.p.rect_filled(area, 0.0, self.t.hover.linear_multiply(0.7));
        self.grid(area, rows, cols, self.t.border);
        for i in 0..rows {
            for j in 0..cols {
                self.vw_dash(self.cell(area, rows, cols, i, j), if i == 0 { self.ink() } else { self.grey() });
            }
        }
        let body = Rect::from_center_size(pos2(area.max.x - 12.0, area.min.y + 12.0), vec2(10.0, 8.0));
        self.p.rect_filled(body, 1.0, self.grey());
        self.p.add(egui::Shape::line(
            vec![
                body.min + vec2(2.0, 0.0),
                body.min + vec2(2.0, -4.0),
                body.center_top() + vec2(0.0, -6.0),
                body.right_top() + vec2(-2.0, -4.0),
                body.right_top() + vec2(-2.0, 0.0),
            ],
            Stroke::new(1.5, self.grey()),
        ));
        // The one range people may still edit: white, accent frame, a pencil.
        let k = self.reveal(0.2, 0.55);
        if k > 0.0 {
            let range = Rect::from_min_max(self.cell(area, rows, cols, 1, 1).min, self.cell(area, rows, cols, 3, 2).max);
            let shown = Rect::from_min_max(range.min, pos2(range.min.x + range.width() * k, range.max.y));
            self.p.rect_filled(shown, 0.0, Color32::WHITE);
            self.p.rect_stroke(shown, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Outside);
            if k >= 1.0 {
                for i in 1..=3 {
                    for j in 1..=2 {
                        self.vw_num(self.cell(area, rows, cols, i, j), 100 + (i * 17 + j * 5) as u32, self.ink());
                    }
                }
                self.pencil(range.max - vec2(2.0, 2.0), range.max + vec2(10.0, 10.0));
            }
        }
    }

    fn rv_new_note(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(100.0, 88.0));
        let area = self.vw_sample(r, 5, 2);
        let c = self.cell(area, 5, 2, 2, 1);
        self.vw_select(c);
        let pop = self.reveal(0.15, 0.45);
        if pop > 0.0 {
            self.rv_mark(c, false);
            let note = Rect::from_min_size(self.at(112.0, 26.0), vec2(56.0, 44.0));
            self.rv_note(note, c.right_center(), 0);
            // Words land in the note one line at a time.
            let inner = note.shrink(4.0);
            for i in 0..3 {
                let w = inner.width() * [0.9, 0.8, 0.45].get(i).copied().unwrap_or(0.5);
                let k = self.reveal(0.5 + i as f32 * 0.12, 0.6 + i as f32 * 0.12);
                if k > 0.0 {
                    let y = inner.min.y + 4.0 + i as f32 * 8.0;
                    self.line(pos2(inner.max.x - w * k, y), pos2(inner.max.x, y), self.ink(), 1.5);
                }
            }
        }
    }

    fn rv_show_note(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(100.0, 88.0));
        let area = self.vw_sample(r, 5, 2);
        let c = self.cell(area, 5, 2, 2, 1);
        self.rv_mark(c, false);
        let k = self.reveal(0.3, 0.55);
        if k > 0.0 {
            let note = Rect::from_min_size(self.at(112.0, 24.0), vec2(56.0, 40.0));
            self.rv_note(note, c.right_center(), 3);
        }
        // A switch under the note: off → on.
        let sw = Rect::from_min_size(self.at(124.0, 78.0), vec2(30.0, 14.0));
        self.p.rect_filled(sw, 7.0, if k >= 1.0 { self.accent() } else { self.t.border_strong });
        let knob = pos2(sw.min.x + 7.0 + 16.0 * k, sw.center().y);
        self.p.circle_filled(knob, 5.5, Color32::WHITE);
    }

    fn rv_show_all_notes(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(96.0, 88.0));
        let (rows, cols) = (5, 2);
        let area = self.vw_sample(r, rows, cols);
        let cells = [(1usize, 1usize), (2, 0), (4, 1)];
        for (i, (row, col)) in cells.into_iter().enumerate() {
            let c = self.cell(area, rows, cols, row, col);
            self.rv_mark(c, false);
            let k = self.reveal(0.15 + i as f32 * 0.15, 0.3 + i as f32 * 0.15);
            if k > 0.0 {
                let note = Rect::from_min_size(self.at(108.0, 10.0 + i as f32 * 30.0), vec2(62.0, 26.0));
                self.rv_note(note, c.right_center(), 2);
            }
        }
    }

    fn rv_convert_to_comments(&self) {
        self.vw_two(
            |d, r| {
                let area = d.vw_sample(r, 4, 2);
                let c = d.cell(area, 4, 2, 1, 1);
                d.rv_mark(c, false);
                d.rv_note(Rect::from_min_size(pos2(r.max.x - 50.0, c.max.y + 4.0), vec2(46.0, 24.0)), c.center_bottom(), 2);
            },
            |d, r| {
                let area = d.vw_sample(r, 4, 2);
                let c = d.cell(area, 4, 2, 1, 1);
                d.rv_mark(c, true);
                d.rv_comment_card(Rect::from_min_size(pos2(r.max.x - 52.0, c.max.y + 4.0), vec2(48.0, 40.0)), true);
            },
        );
    }
}
