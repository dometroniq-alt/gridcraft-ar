//! Formulas tab thumbnails: functions and names, the auditing arrows, showing and checking
//! formulas, the watch window and calculation — plus the Calculation Options menu. Most show a
//! small Arabic sheet with the formula cell in the accent colour and the formula bar above it.

use egui::{Align2, Color32, Pos2, Rect, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Insert Function (⇧F3)",
        "Opens a list of every function with a short description of each; pick one and it is written into the cell with its brackets ready. Example: choose SUM and the cell shows =SUM( waiting for the cells to add.",
        |d| d.fm_insert_function(),
    ),
    Entry::new(
        "AutoSum",
        "Writes a SUM formula in the empty cell under (or beside) your numbers and adds them up at once; the arrow next to it offers Average, Count, Max and Min too. Example: under ١٢، ٣٤ and ٢١ it writes =SUM(B1:B3) and shows ٦٧.",
        |d| d.fm_autosum(),
    ),
    Entry::new(
        "Name Manager",
        "Lists every name you gave to cells and ranges, with what each one points to, so you can rename, fix or delete them. Example: see that «المبيعات» means B2:B9 and change it to B2:B12.",
        |d| d.fm_name_manager(),
    ),
    Entry::new(
        "Define Name",
        "Gives the selected cells a word to call them by, so formulas can say =SUM(المبيعات) instead of =SUM(B2:B9). Example: select the sales column and name it «المبيعات».",
        |d| d.fm_define_name(),
    ),
    Entry::new(
        "Use in Formula",
        "While you type a formula, drops down the list of your names so you can put one in without remembering the cells. Example: in =SUM( pick «المبيعات» from the list.",
        |d| d.fm_use_in_formula(),
    ),
    Entry::new(
        "Create from Selection",
        "Names several ranges at once from their headings: select the table with its header row and each column is named after its heading. Example: columns «الاسم», «الرتبة», «الراتب» each get that name.",
        |d| d.fm_create_from_selection(),
    ),
    Entry::new(
        "Trace Precedents",
        "Draws arrows from the cells a formula reads into the formula cell, so you see where its number comes from. Example: the total cell gets arrows from the three cells it adds.",
        |d| d.fm_trace(true),
    ),
    Entry::new(
        "Trace Dependents",
        "Draws arrows from the selected cell to every formula that uses it, so you see what changes if you change it. Example: the price cell points to the three totals built on it.",
        |d| d.fm_trace(false),
    ),
    Entry::new(
        "Remove Arrows",
        "Clears all the tracing arrows from the sheet; the cells and formulas are untouched. Example: after checking the totals, the blue arrows disappear.",
        |d| d.fm_remove_arrows(),
    ),
    Entry::new(
        "Show Formulas (⌃`)",
        "Shows the formula text in every cell instead of its result, so you can read and compare them all at once; press again to go back to the numbers. Example: ٦٧ turns into =SUM(B1:B3) in its cell.",
        |d| d.fm_show_formulas(),
    ),
    Entry::new(
        "Error Checking",
        "Looks through the sheet for formulas with errors (#DIV/0!, #REF!, #NAME?) and takes you to each one with a hint about the cause. Example: it stops on a cell dividing by an empty cell.",
        |d| d.fm_error_checking(),
    ),
    Entry::new(
        "Evaluate Formula",
        "Works out the selected formula one step at a time, showing each part turn into its value, so you can find where it goes wrong. Example: =A1*2+B1 becomes =٣*2+B1, then =٦+B1, then ١٠.",
        |d| d.fm_evaluate(),
    ),
    Entry::new(
        "Watch Window",
        "Opens a small panel that keeps showing the values of cells you choose, even when they are far away or on another sheet. Example: watch the grand total while you edit rows at the bottom.",
        |d| d.fm_watch_window(),
    ),
    Entry::new(
        "Calculation Options",
        "Chooses whether formulas recalculate by themselves after every change (Automatic) or only when you ask (Manual). Example: on a huge slow workbook, switch to Manual and press F9 when ready.",
        |d| d.fm_calc_options(),
    ),
    Entry::new(
        "Calculate Now (F9)",
        "Recalculates every formula in every open workbook right now; needed when calculation is set to Manual. Example: after many edits, press it and all the totals update together.",
        |d| d.fm_calculate(false),
    ),
    Entry::new(
        "Calculate Sheet (⇧F9)",
        "Recalculates the formulas on the current sheet only, leaving other sheets as they are; quicker than the whole workbook. Example: refresh the totals on the «آذار» sheet without touching the rest.",
        |d| d.fm_calculate(true),
    ),
    // ---- Calculation Options menu ----
    Entry::new(
        "formulas.calculationOptions:automatic",
        "Every formula recalculates the moment any cell changes; what you see is always up to date. Example: type a new price and the total changes at once.",
        |d| d.fm_mode(0),
    ),
    Entry::new(
        "formulas.calculationOptions:automaticExceptTables",
        "Formulas recalculate at once, except what-if data tables, which wait until you press F9; data tables are slow on big sheets. Example: totals update instantly while the big scenario table refreshes only on request.",
        |d| d.fm_mode(1),
    ),
    Entry::new(
        "formulas.calculationOptions:manual",
        "Nothing recalculates until you press F9 (Calculate Now); edits are fast but totals may be stale until then. Example: change twenty prices, then press F9 once to update everything.",
        |d| d.fm_mode(2),
    ),
];

impl Draw<'_> {
    /// The formula bar: a name box, an «fx» tile and the input holding `text`.
    pub(super) fn fm_formula_bar(&self, text: &str, c: Color32) -> Rect {
        let bar = Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, 16.0));
        self.p.rect_filled(bar, 2.0, self.t.menu);
        let name = Rect::from_min_size(bar.min + vec2(2.0, 2.0), vec2(26.0, 12.0));
        self.p.rect_filled(name, 1.0, self.t.input);
        self.p.rect_stroke(name, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.label(name.center(), "B4", 6.5, self.grey(), Align2::CENTER_CENTER);
        self.label(pos2(name.max.x + 9.0, bar.center().y), "fx", 7.5, self.accent(), Align2::CENTER_CENTER);
        let input = Rect::from_min_max(pos2(name.max.x + 18.0, bar.min.y + 2.0), bar.max - vec2(2.0, 2.0));
        self.p.rect_filled(input, 1.0, self.t.input);
        self.p.rect_stroke(input, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.label(pos2(input.min.x + 4.0, input.center().y), text, 7.0, c, Align2::LEFT_CENTER);
        input
    }

    /// The sheet under a formula bar: 4 rows × 4 columns; returns its cell area.
    pub(super) fn fm_sheet(&self) -> Rect {
        self.in_sheet(Rect::from_min_size(self.at(6.0, 26.0), vec2(SIZE.x - 12.0, SIZE.y - 32.0)), 4, 4, true)
    }

    /// The column of numbers ١٢، ٣٤، ٢١ in column B (col index 1) of a 4 × 4 sheet.
    fn fm_numbers(&self, body: Rect) {
        for (i, n) in ["١٢", "٣٤", "٢١"].iter().enumerate() {
            self.in_mark(self.in_cell(body, 4, 4, i, 0, true), true, 0.7, self.grey());
            self.in_text(self.in_cell(body, 4, 4, i, 1, true), n, self.ink());
        }
    }

    /// The function list dialog; SUM is picked and the cell gets =SUM( in the bar.
    fn fm_insert_function(&self) {
        let pick = self.reveal(0.2, 0.5);
        self.fm_formula_bar(if pick > 0.5 { "=SUM(" } else { "" }, self.accent());
        let body = self.fm_sheet();
        self.fm_numbers(body);
        let r = Rect::from_center_size(self.at(SIZE.x / 2.0, 66.0), vec2(110.0, 62.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(r.max.x - 5.0, r.min.y + 6.0), "إدراج دالة", 7.0, self.ink(), Align2::RIGHT_CENTER);
        for (i, (f, desc)) in [("SUM", "يجمع الأرقام"), ("AVERAGE", "المعدل"), ("IF", "شرط")].iter().enumerate() {
            let y = r.min.y + 20.0 + i as f32 * 14.0;
            let row = Rect::from_min_size(pos2(r.min.x + 4.0, y - 6.0), vec2(r.width() - 8.0, 13.0));
            if i == 0 && pick > 0.0 {
                self.p.rect_filled(row, 2.0, self.t.selection);
            }
            self.label(pos2(row.min.x + 4.0, y), f, 7.0, self.accent(), Align2::LEFT_CENTER);
            self.label(pos2(row.max.x - 4.0, y), desc, 6.5, self.grey(), Align2::RIGHT_CENTER);
        }
    }

    /// The selection runs down the numbers, the formula appears, then the total.
    fn fm_autosum(&self) {
        let band = self.reveal(0.1, 0.4);
        let formula = self.reveal(0.4, 0.6);
        let total = self.reveal(0.65, 0.85);
        self.fm_formula_bar(if formula > 0.0 { "=SUM(B1:B3)" } else { "" }, self.accent());
        let body = self.fm_sheet();
        let col = self.in_range(body, 4, 4, (0, 2, 1, 1), true);
        if band > 0.0 {
            let r = Rect::from_min_size(col.min, vec2(col.width(), col.height() * band));
            self.selection(r);
            self.p.rect_stroke(r, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        }
        self.fm_numbers(body);
        let cell = self.in_cell(body, 4, 4, 3, 1, true);
        self.p.rect_stroke(cell, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        if total > 0.5 {
            self.in_text(cell, "٦٧", self.accent());
        } else if formula > 0.0 {
            self.label(cell.center(), "=SUM", 6.5, self.accent(), Align2::CENTER_CENTER);
        }
        self.label(pos2(body.min.x + 22.0, cell.center().y), "Σ", 12.0, self.accent(), Align2::CENTER_CENTER);
    }

    /// The names dialog: a name chip and the range it means, three rows.
    fn fm_name_manager(&self) {
        let body = self.fm_sheet();
        self.in_marks(body, 4, 4, (0, 4, 0, 4), true);
        self.fm_formula_bar("", self.grey());
        let r = Rect::from_center_size(self.at(SIZE.x / 2.0, 62.0), vec2(124.0, 66.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(r.max.x - 5.0, r.min.y + 6.0), "إدارة الأسماء", 7.0, self.ink(), Align2::RIGHT_CENTER);
        for (i, (name, range)) in [("المبيعات", "=B2:B9"), ("الضريبة", "=D1"), ("الأسماء", "=A2:A9")].iter().enumerate() {
            let k = self.reveal(0.1 + i as f32 * 0.15, 0.3 + i as f32 * 0.15);
            if k <= 0.0 {
                continue;
            }
            let y = r.min.y + 21.0 + i as f32 * 15.0;
            self.chip(pos2(r.max.x - 28.0, y), name, 42.0);
            self.label(pos2(r.min.x + 8.0, y), range, 7.0, self.grey().linear_multiply(k), Align2::LEFT_CENTER);
        }
    }

    /// A selected column; the name box fills with the new name and a chip floats over the range.
    fn fm_define_name(&self) {
        let k = self.reveal(0.2, 0.6);
        let bar = Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, 16.0));
        self.p.rect_filled(bar, 2.0, self.t.menu);
        let name = Rect::from_min_size(bar.min + vec2(2.0, 2.0), vec2(44.0, 12.0));
        self.p.rect_filled(name, 1.0, self.t.input);
        self.p.rect_stroke(name, 1.0, self.stroke(if k > 0.0 { self.accent() } else { self.t.input_border }), egui::StrokeKind::Inside);
        self.label(
            name.center(),
            if k > 0.5 { "المبيعات" } else { "B1:B3" },
            6.5,
            if k > 0.5 { self.accent() } else { self.grey() },
            Align2::CENTER_CENTER,
        );
        let body = self.fm_sheet();
        self.in_select(self.in_range(body, 4, 4, (0, 2, 1, 1), true));
        self.fm_numbers(body);
        if k > 0.0 {
            let range = self.in_range(body, 4, 4, (0, 2, 1, 1), true);
            let c = pos2(range.min.x - 24.0, range.center().y);
            let chip = Draw::in_grow(Rect::from_center_size(c, vec2(44.0, 14.0)), k);
            self.p.rect_filled(chip, 3.0, self.accent());
            self.label(chip.center(), "المبيعات", 7.0 * k.max(0.3), Color32::WHITE, Align2::CENTER_CENTER);
            self.arrow_right(pos2(chip.max.x, c.y), pos2(range.min.x - 2.0, c.y));
        }
    }

    /// Typing =SUM( drops the names list; the picked one lands in the formula.
    fn fm_use_in_formula(&self) {
        let pick = self.reveal(0.4, 0.7);
        let input = self.fm_formula_bar(if pick > 0.5 { "=SUM(المبيعات" } else { "=SUM(" }, self.accent());
        let body = self.fm_sheet();
        self.fm_numbers(body);
        let list = self.reveal(0.1, 0.35);
        if list <= 0.0 || pick > 0.5 {
            return;
        }
        let menu = Rect::from_min_size(pos2(input.min.x + 20.0, input.max.y + 2.0), vec2(70.0, 44.0 * list));
        self.in_clip(menu, |d| {
            let full = Rect::from_min_size(menu.min, vec2(70.0, 44.0));
            d.p.rect_filled(full, 3.0, d.t.menu);
            d.p.rect_stroke(full, 3.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
            for (i, name) in ["المبيعات", "الضريبة", "الأسماء"].iter().enumerate() {
                let y = full.min.y + 8.0 + i as f32 * 14.0;
                if i == 0 && pick > 0.0 {
                    d.p.rect_filled(Rect::from_min_size(pos2(full.min.x + 3.0, y - 6.0), vec2(full.width() - 6.0, 12.0)), 2.0, d.t.selection);
                }
                d.label(pos2(full.max.x - 5.0, y), name, 7.0, d.ink(), Align2::RIGHT_CENTER);
            }
        });
    }

    /// A table with headings; a name chip lifts off each heading in turn.
    fn fm_create_from_selection(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 30.0), vec2(SIZE.x - 12.0, SIZE.y - 36.0)), 4, 3, true);
        self.in_select(self.in_range(body, 4, 3, (0, 3, 0, 2), true));
        let heads = ["الاسم", "الرتبة", "الراتب"];
        for (j, h) in heads.iter().enumerate() {
            let cell = self.in_cell(body, 4, 3, 0, j, true);
            self.p.rect_filled(cell, 0.0, self.t.hover);
            self.in_text(cell, h, self.ink());
            for i in 1..4 {
                self.in_mark(self.in_cell(body, 4, 3, i, j, true), true, 0.6, self.grey());
            }
            let k = self.reveal(0.1 + j as f32 * 0.2, 0.3 + j as f32 * 0.2);
            if k > 0.0 {
                let c = pos2(cell.center().x, body.min.y - 14.0 + 8.0 * (1.0 - k));
                let chip = Rect::from_center_size(c, vec2(cell.width() - 8.0, 13.0));
                self.p.rect_filled(chip, 3.0, self.accent().linear_multiply(k));
                self.label(c, h, 6.5, Color32::WHITE.linear_multiply(k), Align2::CENTER_CENTER);
            }
        }
    }

    /// An auditing arrow with a dot at its start.
    fn fm_audit_arrow(&self, a: Pos2, b: Pos2, k: f32) {
        if k <= 0.0 {
            return;
        }
        let end = a + (b - a) * k;
        self.p.circle_filled(a, 2.5, self.accent());
        self.line(a, end, self.accent(), 1.5);
        let dir = (b - a).normalized();
        let n = vec2(-dir.y, dir.x);
        self.line(end, end - dir * 5.0 + n * 4.0, self.accent(), 1.5);
        self.line(end, end - dir * 5.0 - n * 4.0, self.accent(), 1.5);
    }

    /// Arrows into the total from its sources (`precedents`) or out of a cell to the formulas
    /// that use it.
    fn fm_trace(&self, precedents: bool) {
        self.fm_formula_bar(if precedents { "=SUM(B1:B3)" } else { "٣٤" }, self.accent());
        let body = self.fm_sheet();
        self.fm_numbers(body);
        let total = self.in_cell(body, 4, 4, 3, 1, true);
        self.in_text(total, "٦٧", self.ink());
        let k = self.reveal(0.15, 0.6);
        if precedents {
            self.p.rect_stroke(total, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
            for i in 0..3 {
                let src = self.in_cell(body, 4, 4, i, 1, true);
                self.fm_audit_arrow(src.center() + vec2(-8.0, 0.0), total.center() + vec2(-8.0, -4.0), k);
            }
        } else {
            let src = self.in_cell(body, 4, 4, 1, 1, true);
            self.p.rect_stroke(src, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
            let d1 = self.in_cell(body, 4, 4, 1, 3, true);
            let d2 = self.in_cell(body, 4, 4, 3, 3, true);
            self.in_text(d1, "٦٨", self.ink());
            self.in_text(d2, "٦٧", self.ink());
            self.in_text(total, "٦٧", self.ink());
            self.fm_audit_arrow(src.center(), d1.center() + vec2(10.0, 0.0), k);
            self.fm_audit_arrow(src.center(), d2.center() + vec2(10.0, -2.0), k);
            self.fm_audit_arrow(src.center(), total.center() + vec2(0.0, -6.0), k);
        }
    }

    /// The arrows fade out under a red cross.
    fn fm_remove_arrows(&self) {
        self.fm_formula_bar("=SUM(B1:B3)", self.grey());
        let body = self.fm_sheet();
        self.fm_numbers(body);
        let total = self.in_cell(body, 4, 4, 3, 1, true);
        self.in_text(total, "٦٧", self.ink());
        let k = self.reveal(0.2, 0.6);
        if k < 1.0 {
            let fade = self.accent().linear_multiply(1.0 - k);
            for i in 0..3 {
                let src = self.in_cell(body, 4, 4, i, 1, true).center() + vec2(-8.0, 0.0);
                self.p.circle_filled(src, 2.5, fade);
                self.line(src, total.center() + vec2(-8.0, -4.0), fade, 1.5);
            }
        }
        if k > 0.3 {
            self.cross(pos2(body.min.x + 30.0, body.center().y));
        }
    }

    /// Values on the left, the same cells showing their formulas on the right.
    fn fm_show_formulas(&self) {
        let l = self.in_sheet(Rect::from_min_size(self.at(6.0, 14.0), vec2(72.0, 76.0)), 4, 2, true);
        for (i, n) in ["١٢", "٣٤", "٢١", "٦٧"].iter().enumerate() {
            self.in_mark(self.in_cell(l, 4, 2, i, 0, true), true, 0.7, self.grey());
            self.in_text(self.in_cell(l, 4, 2, i, 1, true), n, if i == 3 { self.accent() } else { self.ink() });
        }
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(82.0, 52.0), self.at(82.0 + 12.0 * arrow, 52.0));
        }
        let k = self.reveal(0.3, 0.7);
        if k <= 0.0 {
            return;
        }
        let outer = Rect::from_min_size(self.at(98.0, 14.0), vec2(72.0, 76.0));
        self.in_clip(Rect::from_min_max(outer.min, pos2(outer.min.x + outer.width() * k, outer.max.y)), |d| {
            let r = d.in_sheet(outer, 4, 2, true);
            for (i, f) in ["12", "34", "21", "=SUM(B1:B3)"].iter().enumerate() {
                d.in_mark(d.in_cell(r, 4, 2, i, 0, true), true, 0.7, d.grey());
                let cell = d.in_cell(r, 4, 2, i, 1, true);
                d.label(cell.center(), f, if i == 3 { 5.5 } else { 7.0 }, if i == 3 { d.accent() } else { d.ink() }, Align2::CENTER_CENTER);
            }
            d.p.rect_stroke(outer, 1.0, Stroke::new(1.5, d.accent()), egui::StrokeKind::Inside);
        });
    }

    /// A red error cell, the warning mark, and the hint card beside it.
    fn fm_error_checking(&self) {
        self.fm_formula_bar("=B1/C1", self.red());
        let body = self.fm_sheet();
        self.in_marks(body, 4, 4, (0, 4, 0, 1), true);
        self.in_text(self.in_cell(body, 4, 4, 0, 1, true), "١٢", self.ink());
        let bad = self.in_cell(body, 4, 4, 1, 1, true);
        self.p.rect_filled(bad, 0.0, Color32::from_rgb(255, 230, 230));
        self.label(bad.center(), "#DIV/0!", 6.0, self.red(), Align2::CENTER_CENTER);
        self.p.rect_stroke(bad, 0.0, Stroke::new(1.5, self.red()), egui::StrokeKind::Inside);
        let k = self.reveal(0.2, 0.5);
        if k > 0.0 {
            self.warning(pos2(bad.max.x + 10.0, bad.center().y));
        }
        let card = self.reveal(0.45, 0.8);
        if card > 0.0 {
            let r = Rect::from_min_size(pos2(body.min.x + 4.0, bad.max.y + 6.0), vec2(80.0 * card.max(0.05), 24.0));
            self.p.rect_filled(r, 2.0, self.t.menu);
            self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            self.in_clip(r, |d| {
                d.label(pos2(r.min.x + 80.0 - 4.0, r.min.y + 7.0), "قسمة على خلية فارغة", 6.0, d.ink(), Align2::RIGHT_CENTER);
                d.label(pos2(r.min.x + 80.0 - 4.0, r.min.y + 17.0), "C1 فارغة", 6.0, d.grey(), Align2::RIGHT_CENTER);
            });
        }
    }

    /// The formula resolves one part at a time inside the evaluate dialog.
    fn fm_evaluate(&self) {
        let body = self.fm_sheet();
        self.in_text(self.in_cell(body, 4, 4, 0, 0, true), "٣", self.ink());
        self.in_text(self.in_cell(body, 4, 4, 0, 1, true), "٤", self.ink());
        self.fm_formula_bar("=A1*2+B1", self.grey());
        let r = Rect::from_center_size(self.at(SIZE.x / 2.0, 66.0), vec2(118.0, 56.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(r.max.x - 5.0, r.min.y + 6.0), "تقييم الصيغة", 7.0, self.ink(), Align2::RIGHT_CENTER);
        let field = Rect::from_min_size(pos2(r.min.x + 8.0, r.min.y + 18.0), vec2(r.width() - 16.0, 14.0));
        self.p.rect_filled(field, 1.0, self.t.input);
        self.p.rect_stroke(field, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        let step = if self.phase >= 1.0 { 3 } else { (self.phase * 4.0) as usize };
        let (text, under) = match step {
            0 => ("=A1*2+B1", Some((1.0, 3.0))),
            1 => ("=3*2+B1", Some((1.0, 4.0))),
            2 => ("=6+B1", Some((1.0, 5.0))),
            _ => ("=10", None),
        };
        self.label(pos2(field.min.x + 4.0, field.center().y), text, 8.0, self.ink(), Align2::LEFT_CENTER);
        if let Some((a, b)) = under {
            let x0 = field.min.x + 4.0 + a * 5.0;
            let x1 = field.min.x + 4.0 + b * 5.0 + 4.0;
            self.line(pos2(x0, field.max.y - 2.5), pos2(x1, field.max.y - 2.5), self.accent(), 1.5);
        }
        let y = r.max.y - 9.0;
        self.button(pos2(r.max.x - 26.0, y), "تقييم", step < 3);
        self.button(pos2(r.max.x - 70.0, y), "خطوة", false);
    }

    /// The sheet with a small panel at the bottom listing watched cells and their values.
    fn fm_watch_window(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, 54.0)), 3, 5, true);
        self.in_marks(body, 3, 5, (0, 3, 0, 5), true);
        let total = self.in_cell(body, 3, 5, 1, 2, true);
        self.p.rect_filled(total, 0.0, Color32::WHITE);
        self.in_text(total, "٦٧", self.accent());
        self.p.rect_stroke(total, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        let k = self.reveal(0.1, 0.5);
        if k <= 0.0 {
            return;
        }
        let pane = Rect::from_min_size(self.at(6.0, 98.0 - 34.0 * k), vec2(SIZE.x - 12.0, 34.0 * k));
        self.in_clip(pane, |d| {
            let full = Rect::from_min_size(pane.min, vec2(SIZE.x - 12.0, 34.0));
            d.p.rect_filled(full, 2.0, d.t.panel);
            d.p.rect_stroke(full, 2.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
            d.p.rect_filled(Rect::from_min_size(full.min, vec2(full.width(), 10.0)), 2.0, d.t.hover);
            d.label(pos2(full.max.x - 4.0, full.min.y + 5.0), "نافذة المراقبة", 6.0, d.ink(), Align2::RIGHT_CENTER);
            for (i, (cell, name, val)) in [("C2", "المجموع", "٦٧"), ("E9", "الضريبة", "١٥٪")].iter().enumerate() {
                let y = full.min.y + 16.0 + i as f32 * 10.0;
                d.label(pos2(full.max.x - 6.0, y), name, 6.0, d.grey(), Align2::RIGHT_CENTER);
                d.label(pos2(full.center().x, y), cell, 6.0, d.grey(), Align2::CENTER_CENTER);
                d.in_label(pos2(full.min.x + 8.0, y), val, 6.5, d.accent(), Align2::LEFT_CENTER);
            }
        });
        self.arrow_down(total.center() + vec2(0.0, 7.0), pos2(total.center().x, pane.min.y - 2.0));
    }

    /// A gear and the two choices; the pick moves from Automatic to Manual.
    fn fm_calc_options(&self) {
        self.gear(self.at(34.0, 52.0), 12.0);
        let k = self.reveal(0.3, 0.6);
        let manual = k > 0.5;
        self.button(self.at(112.0, 34.0), "تلقائي", !manual);
        self.button(self.at(112.0, 70.0), "يدوي", manual);
        if k > 0.0 && k < 1.0 {
            self.pointer(self.at(126.0, 40.0 + 24.0 * k));
        }
    }

    /// Stale totals refresh: a circular arrow turns and the numbers switch to new values; the
    /// sheet tabs show which sheet when `sheet_only`.
    fn fm_calculate(&self, sheet_only: bool) {
        let h = if sheet_only { 74.0 } else { 92.0 };
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, h)), 4, 4, true);
        self.fm_numbers(body);
        let k = self.reveal(0.3, 0.6);
        let total = self.in_cell(body, 4, 4, 3, 1, true);
        self.in_text(total, if k > 0.5 { "٦٧" } else { "٦١" }, if k > 0.5 { self.accent() } else { self.grey() });
        let spin = self.reveal(0.05, 0.65);
        let c = pos2(body.min.x + 34.0, body.center().y);
        let n = 20;
        let pts: Vec<Pos2> = (0..=n)
            .map(|i| {
                let a = -std::f32::consts::FRAC_PI_2 + i as f32 / n as f32 * std::f32::consts::TAU * 0.8 * spin.max(0.05);
                c + vec2(a.cos() * 12.0, a.sin() * 12.0)
            })
            .collect();
        self.p.add(egui::Shape::line(pts.clone(), Stroke::new(2.0, self.accent())));
        if let (Some(a), Some(b)) = (pts.get(n - 1), pts.get(n)) {
            let dir = (*b - *a).normalized();
            let nrm = vec2(-dir.y, dir.x);
            self.line(*b, *b - dir * 5.0 + nrm * 4.0, self.accent(), 2.0);
            self.line(*b, *b - dir * 5.0 - nrm * 4.0, self.accent(), 2.0);
        }
        if sheet_only {
            for (i, name) in ["شباط", "آذار", "نيسان"].iter().enumerate() {
                let r = Rect::from_min_size(self.at(SIZE.x - 6.0 - 44.0 * (i as f32 + 1.0), 82.0), vec2(42.0, 14.0));
                let on = i == 1;
                self.p.rect_filled(r, 2.0, if on { Color32::WHITE } else { self.t.hover });
                self.p.rect_stroke(r, 2.0, self.stroke(if on { self.accent() } else { self.t.border_strong }), egui::StrokeKind::Inside);
                self.label(r.center(), name, 6.5, if on { self.accent() } else { self.grey() }, Align2::CENTER_CENTER);
            }
        }
    }

    /// One calculation mode: a price is typed and the total reacts — at once (0), at once
    /// but with the data table waiting (1), or not until F9 (2).
    fn fm_mode(&self, mode: usize) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0)), 4, 4, true);
        let typed = self.reveal(0.1, 0.35);
        let react = self.reveal(0.4, 0.65);
        let price = self.in_cell(body, 4, 4, 0, 1, true);
        self.in_mark(self.in_cell(body, 4, 4, 0, 0, true), true, 0.7, self.grey());
        self.in_mark(self.in_cell(body, 4, 4, 3, 0, true), true, 0.7, self.grey());
        self.in_text(price, if typed > 0.5 { "٢٠" } else { "١٠" }, if typed > 0.5 { self.ink() } else { self.grey() });
        self.p.rect_stroke(price, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        let total = self.in_cell(body, 4, 4, 3, 1, true);
        let updated = mode < 2 && react > 0.5;
        self.in_text(total, if updated { "٦٠" } else { "٣٠" }, if updated { self.accent() } else { self.grey() });
        let arrow = pos2(price.center().x - 14.0, price.max.y + 2.0);
        if typed > 0.5 {
            self.arrow_down(arrow, pos2(arrow.x, total.min.y - 2.0));
        }
        match mode {
            0 => {
                if updated {
                    self.check(pos2(body.min.x + 30.0, body.center().y));
                }
            }
            1 => {
                if updated {
                    self.check(pos2(body.min.x + 30.0, body.min.y + 22.0));
                }
                let dt = Rect::from_min_size(pos2(body.min.x + 8.0, body.max.y - 34.0), vec2(44.0, 28.0));
                self.grid(dt, 3, 3, self.t.border);
                self.p.rect_filled(self.cell(dt, 3, 3, 0, 0).union(self.cell(dt, 3, 3, 0, 2)), 0.0, self.t.hover);
                self.p.rect_filled(self.cell(dt, 3, 3, 0, 0).union(self.cell(dt, 3, 3, 2, 0)), 0.0, self.t.hover);
                let c = dt.center() + vec2(6.0, 4.0);
                self.p.rect_filled(Rect::from_center_size(c - vec2(3.0, 0.0), vec2(3.0, 10.0)), 0.5, self.grey());
                self.p.rect_filled(Rect::from_center_size(c + vec2(3.0, 0.0), vec2(3.0, 10.0)), 0.5, self.grey());
            }
            _ => {
                if react > 0.5 {
                    self.badge(pos2(body.min.x + 30.0, body.center().y), "F9", self.accent());
                }
            }
        }
    }
}
