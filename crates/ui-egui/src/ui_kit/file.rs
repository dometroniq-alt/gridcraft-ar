//! Title-bar thumbnails: the quick buttons above the ribbon — Home, Save, the undo list, more
//! commands, search, share, comments and dark mode — each a little film of what the button
//! opens or does.

use egui::{Align2, Color32, Rect, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Home",
        "Takes you back to the start screen, where you open a recent workbook or begin a new one; the file you have open stays as it is. Example: press it to find yesterday's file in the recent list.",
        |d| d.fl_home_screen(),
    ),
    Entry::new(
        "Save (⌘S)",
        "Writes the workbook to its file on the computer so nothing is lost if the program closes; the first time, it asks where to put the file. Example: after typing the day's figures, press it and the «unsaved» mark on the title goes away.",
        |d| d.fl_save(),
    ),
    Entry::new(
        "Undo list",
        "Opens the list of your recent changes so you can take back several at once: pick how far back to go. Example: choose the third row to undo the last three edits together.",
        |d| d.fl_undo_list(),
    ),
    Entry::new(
        "More commands",
        "Shows the rest of the quick buttons that did not fit in the title bar, in a small menu. Example: in a narrow window, Print and Export are found here.",
        |d| d.fl_more(),
    ),
    Entry::new(
        "Search commands (⌘⇧Space)",
        "Finds any button by typing part of its name, then runs it from the list; quicker than hunting through the tabs. Example: type «تجميد» and pick Freeze Panes.",
        |d| d.fl_search(),
    ),
    Entry::new(
        "Share: export or save a copy",
        "Exports the workbook as a PDF or saves a copy in another format or place, ready to send to someone. Example: make a PDF of the monthly report to send to the commander.",
        |d| d.fl_share(),
    ),
    Entry::new(
        "Comments",
        "Opens the comments pane listing every note attached to cells in the workbook, so you can read and answer them in one place. Example: see the three notes a colleague left on the budget sheet.",
        |d| d.fl_comments_pane(),
    ),
    Entry::new(
        "Dark Mode",
        "Switches the interface between light and dark colours; your numbers and formatting do not change. Example: at night the white sheet turns dark grey with light text.",
        |d| d.fl_dark_mode(),
    ),
];

impl Draw<'_> {
    /// The start screen: a column of buttons and recent-file tiles appearing one by one.
    fn fl_home_screen(&self) {
        let r = Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let side = Rect::from_min_size(r.min, vec2(48.0, r.height()));
        self.p.rect_filled(side, 3.0, self.t.hover);
        for (i, text) in ["جديد", "فتح"].iter().enumerate() {
            let c = pos2(side.center().x, side.min.y + 18.0 + i as f32 * 22.0);
            self.button(c, text, i == 0);
        }
        self.label(pos2(r.max.x - 8.0, r.min.y + 12.0), "الأخيرة", 7.5, self.ink(), Align2::RIGHT_CENTER);
        for i in 0..3 {
            let k = self.reveal(0.1 + i as f32 * 0.2, 0.3 + i as f32 * 0.2);
            if k <= 0.0 {
                continue;
            }
            let tile = Rect::from_min_size(pos2(side.max.x + 10.0 + i as f32 * 36.0, r.min.y + 24.0), vec2(28.0, 36.0));
            let tile = Draw::in_grow(tile, k);
            self.p.rect_filled(tile, 1.0, Color32::WHITE);
            self.p.rect_stroke(tile, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            self.grid(tile.shrink(3.0), 4, 3, self.t.border);
            self.line(pos2(tile.min.x + 2.0, tile.max.y + 8.0), pos2(tile.max.x - 2.0, tile.max.y + 8.0), self.grey(), 1.5);
        }
    }

    /// An edited cell, then the file icon with a green tick: it is on disk.
    fn fl_save(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 8.0), vec2(96.0, 88.0)), 5, 3, true);
        self.in_marks(body, 5, 3, (0, 5, 0, 3), true);
        let edited = self.in_cell(body, 5, 3, 2, 1, true);
        self.p.rect_filled(edited, 0.0, Color32::WHITE);
        self.in_text(edited, "٤٢٠", self.accent());
        self.p.rect_stroke(edited, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        let arrow = self.reveal(0.1, 0.35);
        if arrow > 0.0 {
            self.arrow_right(self.at(108.0, 52.0), self.at(108.0 + 14.0 * arrow, 52.0));
        }
        let file = Rect::from_min_size(self.at(130.0, 24.0), vec2(36.0, 46.0));
        self.in_file(file, "xlsx", self.accent());
        let tick = self.reveal(0.5, 0.8);
        if tick > 0.0 {
            let c = file.right_top() + vec2(-2.0, 2.0);
            self.p.circle_filled(c, 11.0 * tick, self.green());
            if tick > 0.6 {
                self.line(c + vec2(-5.0, 0.0), c + vec2(-1.5, 4.0), Color32::WHITE, 2.5);
                self.line(c + vec2(-1.5, 4.0), c + vec2(6.0, -5.0), Color32::WHITE, 2.5);
            }
        }
    }

    /// The undo button with its dropdown: the highlight grows down over three past actions.
    fn fl_undo_list(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0)), 5, 5, true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let btn = Rect::from_min_size(self.at(16.0, 14.0), vec2(40.0, 16.0));
        self.p.rect_filled(btn, 3.0, self.t.menu);
        self.p.rect_stroke(btn, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.arrow_left(pos2(btn.min.x + 24.0, btn.center().y), pos2(btn.min.x + 8.0, btn.center().y));
        self.in_triangle(pos2(btn.max.x - 8.0, btn.center().y), self.ink());
        let menu = Rect::from_min_size(pos2(btn.min.x, btn.max.y + 2.0), vec2(88.0, 62.0));
        self.p.rect_filled(menu, 3.0, self.t.menu);
        self.p.rect_stroke(menu, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let k = self.reveal(0.15, 0.6);
        let rows = ["كتابة ٤٢٠", "تنسيق غامق", "لصق", "حذف صف"];
        let hl = (k * 3.0).ceil().clamp(0.0, 3.0) as usize;
        for (i, text) in rows.iter().enumerate() {
            let y = menu.min.y + 4.0 + i as f32 * 14.0;
            let row = Rect::from_min_size(pos2(menu.min.x + 3.0, y), vec2(menu.width() - 6.0, 13.0));
            if i < hl {
                self.p.rect_filled(row, 2.0, self.t.selection);
            }
            self.in_label(pos2(row.max.x - 4.0, row.center().y), text, 7.0, self.ink(), Align2::RIGHT_CENTER);
        }
        if k > 0.0 && k < 1.0 {
            self.pointer(pos2(menu.min.x + 20.0, menu.min.y + 6.0 + 42.0 * k));
        }
    }

    /// A short title bar with a «…» button; its menu drops down with the hidden buttons.
    fn fl_more(&self) {
        let bar = Rect::from_min_size(self.at(6.0, 8.0), vec2(SIZE.x - 12.0, 20.0));
        self.p.rect_filled(bar, 3.0, self.t.hover);
        for i in 0..4 {
            let c = pos2(bar.max.x - 14.0 - i as f32 * 20.0, bar.center().y);
            self.p.rect_filled(Rect::from_center_size(c, vec2(12.0, 12.0)), 2.0, self.t.menu);
            self.p.rect_stroke(Rect::from_center_size(c, vec2(12.0, 12.0)), 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        }
        let more = pos2(bar.min.x + 16.0, bar.center().y);
        self.p.rect_filled(Rect::from_center_size(more, vec2(20.0, 14.0)), 3.0, self.t.checked);
        for dx in [-5.0, 0.0, 5.0] {
            self.p.circle_filled(more + vec2(dx, 0.0), 1.5, self.ink());
        }
        let k = self.reveal(0.15, 0.55);
        if k <= 0.0 {
            return;
        }
        let menu = Rect::from_min_size(pos2(bar.min.x + 4.0, bar.max.y + 2.0), vec2(72.0, 62.0 * k));
        self.in_clip(menu, |d| {
            let full = Rect::from_min_size(menu.min, vec2(72.0, 62.0));
            d.p.rect_filled(full, 3.0, d.t.menu);
            d.p.rect_stroke(full, 3.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
            for (i, text) in ["طباعة", "تصدير", "خصائص"].iter().enumerate() {
                let y = full.min.y + 10.0 + i as f32 * 17.0;
                d.p.rect_filled(Rect::from_center_size(pos2(full.min.x + 11.0, y), vec2(10.0, 10.0)), 2.0, d.accent().linear_multiply(0.4));
                d.label(pos2(full.max.x - 6.0, y), text, 7.0, d.ink(), Align2::RIGHT_CENTER);
            }
        });
    }

    /// The search box: the word types itself, matching commands appear beneath.
    fn fl_search(&self) {
        let field = Rect::from_min_size(self.at(24.0, 12.0), vec2(128.0, 18.0));
        self.p.rect_filled(field, 3.0, self.t.input);
        self.p.rect_stroke(field, 3.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.magnifier(pos2(field.min.x + 10.0, field.center().y - 2.0));
        let typed = self.reveal(0.1, 0.4);
        let word = "تجميد";
        let n = (word.chars().count() as f32 * typed).round() as usize;
        let shown: String = word.chars().take(n).collect();
        self.label(pos2(field.max.x - 6.0, field.center().y), &shown, 8.0, self.ink(), Align2::RIGHT_CENTER);
        if typed < 1.0 {
            self.caret(pos2(field.max.x - 8.0 - n as f32 * 5.0, field.min.y + 4.0), 10.0);
        }
        let list = self.reveal(0.4, 0.75);
        if list <= 0.0 {
            return;
        }
        let menu = Rect::from_min_size(pos2(field.min.x, field.max.y + 3.0), vec2(field.width(), 56.0));
        self.p.rect_filled(menu, 3.0, self.t.menu);
        self.p.rect_stroke(menu, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        for (i, text) in ["تجميد الأجزاء", "تجميد الصف الأول", "تجميد العمود الأول"].iter().enumerate() {
            let k = self.reveal(0.4 + i as f32 * 0.1, 0.55 + i as f32 * 0.1);
            if k <= 0.0 {
                continue;
            }
            let y = menu.min.y + 10.0 + i as f32 * 17.0;
            let row = Rect::from_min_size(pos2(menu.min.x + 3.0, y - 7.0), vec2(menu.width() - 6.0, 14.0));
            if i == 0 {
                self.p.rect_filled(row, 2.0, self.t.selection);
            }
            self.label(pos2(row.max.x - 5.0, y), text, 7.0, self.ink().linear_multiply(k), Align2::RIGHT_CENTER);
        }
    }

    /// The sheet goes out as files: PDF, a copy, CSV — badges popping one after the other.
    fn fl_share(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 10.0), vec2(80.0, 84.0)), 5, 3, true);
        self.in_marks(body, 5, 3, (0, 5, 0, 3), true);
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(92.0, 52.0), self.at(92.0 + 14.0 * arrow, 52.0));
        }
        let items = [("PDF", self.red()), ("xlsx", self.green()), ("CSV", self.accent())];
        for (i, (tag, fill)) in items.iter().enumerate() {
            let k = self.reveal(0.3 + i as f32 * 0.15, 0.5 + i as f32 * 0.15);
            if k <= 0.0 {
                continue;
            }
            let file = Draw::in_grow(Rect::from_min_size(self.at(108.0 + i as f32 * 14.0, 12.0 + i as f32 * 14.0), vec2(34.0, 44.0)), k);
            self.in_file(file, tag, *fill);
        }
    }

    /// Cells with red corners on the sheet; the pane on the side lists their notes.
    fn fl_comments_pane(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(74.0, 6.0), vec2(96.0, 92.0)), 6, 3, true);
        self.in_marks(body, 6, 3, (0, 6, 0, 3), true);
        for (i, j) in [(1usize, 1usize), (3, 0), (4, 2)] {
            let c = self.in_cell(body, 6, 3, i, j, true);
            let tri = vec![c.right_top(), c.right_top() + vec2(-5.0, 0.0), c.right_top() + vec2(0.0, 5.0)];
            self.p.add(egui::Shape::convex_polygon(tri, self.red(), egui::Stroke::NONE));
        }
        let pane = Rect::from_min_size(self.at(6.0, 6.0), vec2(62.0, 92.0));
        self.p.rect_filled(pane, 2.0, self.t.panel);
        self.p.rect_stroke(pane, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(pane.min, vec2(pane.width(), 12.0)), 2.0, self.t.hover);
        self.label(pos2(pane.max.x - 4.0, pane.min.y + 6.0), "التعليقات", 6.5, self.ink(), Align2::RIGHT_CENTER);
        for i in 0..3 {
            let k = self.reveal(0.1 + i as f32 * 0.2, 0.3 + i as f32 * 0.2);
            if k <= 0.0 {
                continue;
            }
            let card = Rect::from_min_size(pos2(pane.min.x + 4.0, pane.min.y + 16.0 + i as f32 * 25.0), vec2(pane.width() - 8.0, 22.0));
            self.p.rect_filled(card, 2.0, Color32::WHITE.linear_multiply(k));
            self.p.rect_stroke(card, 2.0, self.stroke(self.t.border.linear_multiply(k)), egui::StrokeKind::Inside);
            self.p.circle_filled(pos2(card.max.x - 7.0, card.min.y + 6.0), 3.5, self.accent().linear_multiply(k));
            self.text_lines(
                Rect::from_min_max(pos2(card.min.x + 3.0, card.min.y + 8.0), pos2(card.max.x - 14.0, card.max.y)),
                true,
                &[30.0, 20.0],
                6.0,
                self.grey().linear_multiply(k),
            );
        }
    }

    /// Light sheet on the left, the dark one wipes in over the right half.
    fn fl_dark_mode(&self) {
        let r = Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0));
        let body = self.in_sheet(r, 5, 6, true);
        self.in_marks(body, 5, 6, (0, 5, 0, 6), true);
        let k = self.reveal(0.15, 0.7);
        if k <= 0.0 {
            return;
        }
        let dark = Rect::from_min_max(pos2(r.max.x - r.width() * 0.5 * k, r.min.y), r.max);
        self.in_clip(dark, |d| {
            let bg = Color32::from_rgb(38, 40, 46);
            d.p.rect_filled(r, 1.0, bg);
            let head = Color32::from_rgb(58, 60, 68);
            d.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 9.0)), 0.0, head);
            d.p.rect_filled(Rect::from_min_max(pos2(r.max.x - 11.0, r.min.y), r.max), 0.0, head);
            d.grid(body, 5, 6, Color32::from_rgb(70, 72, 80));
            for i in 0..5 {
                for j in 0..6 {
                    d.in_mark(d.in_cell(body, 5, 6, i, j, true), true, 0.6, Color32::from_rgb(200, 200, 205));
                }
            }
            d.p.rect_stroke(r, 1.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
        });
        self.line(pos2(dark.min.x, r.min.y), pos2(dark.min.x, r.max.y), self.accent(), 1.5);
    }
}
