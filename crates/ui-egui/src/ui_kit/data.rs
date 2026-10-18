//! Data tab thumbnails: getting data in, sorting and filtering, splitting and filling columns,
//! duplicates and validation, what-if tools, outlines and subtotals, analysis and Solver —
//! plus the Data Validation and What-If menus. Rows of an Arabic list move, vanish or split so
//! the eye sees what each button does to the data.

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Import a text or CSV file",
        "Reads a plain text file whose values are separated by commas or tabs and puts them into cells, one line per row. Example: the export from the attendance machine becomes a sheet with a column for each field.",
        |d| d.da_import_csv(),
    ),
    Entry::new(
        "Sort A to Z",
        "Reorders the rows so the selected column runs from smallest to largest: numbers ascending, words in alphabetical order, dates oldest first. Example: salaries from the lowest to the highest.",
        |d| d.da_sort(true),
    ),
    Entry::new(
        "Sort Z to A",
        "Reorders the rows so the selected column runs from largest to smallest: numbers descending, words from the last letter to the first, dates newest first. Example: salaries from the highest to the lowest.",
        |d| d.da_sort(false),
    ),
    Entry::new(
        "Custom Sort",
        "Opens a dialog to sort by several columns in turn, or by colour, each ascending or descending. Example: first by department, then by name within each department.",
        |d| d.da_custom_sort(),
    ),
    Entry::new(
        "Filter (⌘⇧F)",
        "Puts a small arrow on every heading; click one to show only the rows that match what you pick, hiding the rest without deleting them. Example: show only the rows where the city is «البصرة».",
        |d| d.da_filter(),
    ),
    Entry::new(
        "Clear Filter",
        "Removes every filter so all the hidden rows come back, keeping the filter arrows in place. Example: after looking at one city, all cities show again.",
        |d| d.da_clear_filter(),
    ),
    Entry::new(
        "Reapply",
        "Runs the current filter and sort again after you changed or added data, so rows that no longer match disappear and new ones that match appear. Example: a new «البصرة» row joins the filtered list.",
        |d| d.da_reapply(),
    ),
    Entry::new(
        "Text to Columns",
        "Splits one column into several at a separator you choose (a comma, a space, a dash) or at fixed widths. Example: «علي، بغداد» becomes «علي» in one cell and «بغداد» in the next.",
        |d| d.da_text_to_columns(),
    ),
    Entry::new(
        "Flash Fill (⌘E)",
        "Looks at the example you typed beside the first row, works out the pattern and fills the rest of the column the same way. Example: type the first name taken from «علي حسن» and the other first names fill in.",
        |d| d.da_flash_fill(),
    ),
    Entry::new(
        "Remove Duplicates",
        "Deletes rows that repeat an earlier row in the chosen columns, keeping the first of each; the sheet tells you how many went. Example: a name entered twice stays once.",
        |d| d.da_remove_duplicates(),
    ),
    Entry::new(
        "Data Validation",
        "Limits what can be typed in the selected cells: a list of allowed words, a number range, a date; anything else is refused with a message. Example: the «الرتبة» column accepts only ranks from your list.",
        |d| d.da_validation(),
    ),
    Entry::new(
        "What-If Analysis",
        "Tools for trying out numbers: Goal Seek finds the input that gives a result you want, Scenario Manager saves sets of inputs to compare, Data Table shows a formula for many inputs at once. Example: find the price that gives a profit of exactly ١٠٠٠.",
        |d| d.da_what_if(),
    ),
    Entry::new(
        "Forecast Sheet",
        "Makes a new sheet with a chart and a table that continue your dated figures into the future, with a band showing how sure the guess is. Example: twelve months of sales become a forecast for the next six.",
        |d| d.da_forecast(),
    ),
    Entry::new(
        "Group",
        "Joins the selected rows or columns into an outline level you can fold shut with a minus sign and open again with a plus. Example: fold the twenty detail rows of a department into one line.",
        |d| d.da_group(),
    ),
    Entry::new(
        "Ungroup",
        "Removes the outline level from the selected rows or columns; they stay where they are but can no longer be folded. Example: the bracket beside the department rows disappears.",
        |d| d.da_ungroup(),
    ),
    Entry::new(
        "Subtotal",
        "Inserts a total row under each group of a sorted column and a grand total at the end, with outline buttons to fold the groups. Example: a sum under each department's rows and the overall sum at the bottom.",
        |d| d.da_subtotal(),
    ),
    Entry::new(
        "Show Detail",
        "Opens the folded group so its rows (or columns) are visible again. Example: the department's twenty rows reappear under its total.",
        |d| d.da_detail(true),
    ),
    Entry::new(
        "Hide Detail",
        "Folds the group shut so only its summary row (or column) stays visible. Example: the twenty detail rows vanish and only the department total remains.",
        |d| d.da_detail(false),
    ),
    Entry::new(
        "Data Analysis",
        "Opens the statistics tools: descriptive statistics, histogram, correlation, regression, moving average and more, each writing its results to a new range. Example: a histogram of ages in ten-year bands.",
        |d| d.da_data_analysis(),
    ),
    Entry::new(
        "Solver",
        "Finds the input values that make a formula as large, as small or exactly as you want, while keeping the limits you set. Example: how many of each item to buy for the most profit within the budget.",
        |d| d.da_solver(),
    ),
    // ---- Data Validation menu ----
    Entry::new(
        "dialog:dataValidation",
        "Opens the dialog where you set the rule for the selected cells (a list, a number range, a date, a text length), the message shown while typing and the error shown when a value is refused. Example: allow whole numbers from ١ to ١٠٠.",
        |d| {
            let body = d.in_sheet_full(true);
            d.in_select(d.in_range(body, 5, 5, (0, 4, 1, 1), true));
            d.dialog("التحقق من البيانات", 3);
            let k = d.reveal(0.2, 0.5);
            if k > 0.0 {
                d.chip(d.at(SIZE.x / 2.0 + 14.0, 36.0), "قائمة", 34.0);
            }
        },
    ),
    Entry::new(
        "data.circleInvalid",
        "Draws a red ring round every cell whose value breaks its validation rule, so you can find mistakes typed before the rule existed. Example: two salaries above the allowed maximum get red rings.",
        |d| d.da_circle_invalid(),
    ),
    Entry::new(
        "data.validation",
        "Removes the validation rule from the selected cells so anything may be typed in them again, and clears the red rings. Example: the «الرتبة» column stops checking against the list.",
        |d| d.da_clear_validation(),
    ),
    // ---- What-If menu ----
    Entry::new(
        "dialog:goalSeek",
        "Finds the input value that makes a formula come out at the number you want: tell it the formula cell, the target, and which cell to change. Example: which price makes the profit cell exactly ١٠٠٠.",
        |d| d.da_goal_seek(),
    ),
    Entry::new(
        "data.scenarioManager",
        "Saves several sets of input values under names so you can switch between them and compare their results in a summary. Example: «متفائل», «متوقع» and «متشائم» each with its own sales figures.",
        |d| d.da_scenarios(),
    ),
    Entry::new(
        "data.dataTable",
        "Fills a block with the result of one formula for every input value along a row or column (or both), so you see all the outcomes side by side. Example: the monthly payment for each interest rate and each loan length.",
        |d| d.da_data_table(),
    ),
];

impl Draw<'_> {
    /// A 6-row Arabic list (name, city, number) on a sheet; returns its cell area.
    fn da_list(&self, r: Rect, rows: usize) -> Rect {
        self.in_sheet(r, rows, 3, true)
    }

    /// A white disc with a border, a backing for a mark drawn over the cells.
    fn da_disc(&self, c: Pos2) {
        self.p.circle_filled(c, 15.0, Color32::WHITE);
        self.p.circle_stroke(c, 15.0, self.stroke(self.t.border_strong));
    }

    /// The CSV file icon, an arrow, and the rows land on the sheet one after the other.
    fn da_import_csv(&self) {
        let file = Rect::from_min_size(self.at(14.0, 24.0), vec2(40.0, 52.0));
        self.in_file(file, "CSV", self.accent());
        for i in 0..3 {
            let y = file.min.y + 10.0 + i as f32 * 7.0;
            self.line(pos2(file.min.x + 5.0, y), pos2(file.max.x - 5.0, y), self.grey(), 1.2);
        }
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(62.0, 52.0), self.at(62.0 + 14.0 * arrow, 52.0));
        }
        let body = self.da_list(Rect::from_min_size(self.at(84.0, 8.0), vec2(86.0, 88.0)), 6);
        for i in 0..6 {
            let k = self.reveal(0.3 + i as f32 * 0.08, 0.4 + i as f32 * 0.08);
            if k <= 0.0 {
                continue;
            }
            for j in 0..3 {
                let frac = [0.7, 0.5, 0.4].get(j).copied().unwrap_or(0.5);
                self.in_mark(self.in_cell(body, 6, 3, i, j, true), true, frac, self.accent().linear_multiply(k));
            }
        }
    }

    /// Rows of bars shuffle into order: `asc` from short to long.
    fn da_sort(&self, asc: bool) {
        let body = self.in_sheet_full(true);
        let header = self.in_range(body, 5, 5, (0, 0, 0, 2), true);
        self.p.rect_filled(header, 0.0, self.t.hover);
        for j in 0..3 {
            self.in_mark(self.in_cell(body, 5, 5, 0, j, true), true, 0.6, self.ink());
        }
        let col = self.in_range(body, 5, 5, (0, 4, 1, 1), true);
        self.p.rect_stroke(col, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        // Values in rows 1..5 before sorting, and each row's place after.
        let before: [usize; 4] = [3, 1, 4, 2];
        let k = self.reveal(0.2, 0.7);
        let rh = body.height() / 5.0;
        for (i, v) in before.iter().enumerate() {
            let rank = if asc { *v - 1 } else { 4 - *v };
            let y_from = body.min.y + (i as f32 + 1.5) * rh;
            let y_to = body.min.y + (rank as f32 + 1.5) * rh;
            let y = y_from + (y_to - y_from) * k;
            let name = self.in_cell(body, 5, 5, 1, 0, true);
            let num = self.in_cell(body, 5, 5, 1, 1, true);
            let bar = self.in_cell(body, 5, 5, 1, 2, true);
            self.in_mark(
                Rect::from_center_size(pos2(name.center().x, y), name.size()),
                true,
                [0.7, 0.5, 0.8, 0.6].get(i).copied().unwrap_or(0.6),
                self.grey(),
            );
            self.label(pos2(num.center().x, y), Draw::in_digit(*v * 2), 7.0, self.accent(), Align2::CENTER_CENTER);
            let w = (bar.width() - 6.0) * *v as f32 / 4.0;
            self.p.rect_filled(Rect::from_min_size(pos2(bar.max.x - 3.0 - w, y - 3.0), vec2(w, 6.0)), 1.0, self.accent().linear_multiply(0.5));
        }
        let c = pos2(body.min.x + 16.0, body.min.y + 14.0);
        if asc {
            self.arrow_down(c, c + vec2(0.0, 22.0));
        } else {
            self.arrow_up(c + vec2(0.0, 22.0), c);
        }
    }

    /// The sort dialog with two levels and a third one being added.
    fn da_custom_sort(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let inner = self.dialog("فرز", 0);
        let rows = [("القسم", "أ → ي"), ("الاسم", "أ → ي"), ("الراتب", "ي → أ")];
        for (i, (by, order)) in rows.iter().enumerate() {
            let k = if i < 2 { 1.0 } else { self.reveal(0.3, 0.6) };
            if k <= 0.0 {
                continue;
            }
            let y = inner.min.y + 4.0 + i as f32 * 15.0;
            self.label(pos2(inner.max.x, y + 4.0), if i == 0 { "حسب" } else { "ثم" }, 6.5, self.grey().linear_multiply(k), Align2::RIGHT_CENTER);
            let f1 = Rect::from_min_size(pos2(inner.max.x - 68.0, y), vec2(38.0, 10.0));
            let f2 = Rect::from_min_size(pos2(inner.min.x, y), vec2(30.0, 10.0));
            for (f, t) in [(f1, by), (f2, order)] {
                self.p.rect_filled(f, 1.0, self.t.input.linear_multiply(k));
                self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border.linear_multiply(k)), egui::StrokeKind::Inside);
                self.label(f.center(), t, 6.0, self.ink().linear_multiply(k), Align2::CENTER_CENTER);
            }
        }
    }

    /// The list with a header; rows that do not match collapse away.
    fn da_filter_rows(&self, body: Rect, hide: f32, pressed: Option<usize>) {
        let cities = ["بغداد", "البصرة", "بغداد", "البصرة", "الموصل"];
        let mut y = body.min.y + body.height() / 6.0;
        let rh = body.height() / 6.0;
        for (i, city) in cities.iter().enumerate() {
            let keep = *city == "البصرة";
            let h = if keep { rh } else { rh * (1.0 - hide) };
            if h > 0.5 {
                let row = Rect::from_min_size(pos2(body.min.x, y), vec2(body.width(), h));
                self.in_clip(row, |d| {
                    let full = Rect::from_min_size(pos2(body.min.x, y), vec2(body.width(), rh));
                    let a = if keep { 1.0 } else { 1.0 - hide };
                    d.in_mark(
                        d.in_cell(full, 1, 3, 0, 0, true),
                        true,
                        [0.7, 0.5, 0.8, 0.6, 0.7].get(i).copied().unwrap_or(0.6),
                        d.grey().linear_multiply(a),
                    );
                    d.in_text(d.in_cell(full, 1, 3, 0, 1, true), city, if keep && hide > 0.5 { d.accent() } else { d.ink().linear_multiply(a) });
                    d.in_text(d.in_cell(full, 1, 3, 0, 2, true), Draw::in_digit(i + 2), d.grey().linear_multiply(a));
                });
                self.line(pos2(body.min.x, y + h), pos2(body.max.x, y + h), self.t.border, 1.0);
            }
            y += h;
        }
        let head = self.in_range(body, 6, 3, (0, 0, 0, 2), true);
        self.p.rect_filled(head, 0.0, self.t.hover);
        for j in 0..3 {
            let c = self.in_cell(body, 6, 3, 0, j, true);
            self.in_mark(c, true, 0.55, self.ink());
            let on = pressed == Some(j);
            if on {
                self.in_funnel(pos2(c.min.x + 6.0, c.center().y), self.accent());
            } else {
                self.in_triangle(pos2(c.min.x + 6.0, c.center().y), self.grey());
            }
        }
    }

    /// A bare sheet frame with a 6 × 3 header-and-rows area; returns the cell area.
    fn da_filter_sheet(&self) -> Rect {
        let r = Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        let side = Rect::from_min_max(pos2(r.max.x - 11.0, r.min.y), r.max);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 9.0)), 0.0, self.t.hover);
        self.p.rect_filled(side, 0.0, self.t.hover);
        let body = Rect::from_min_max(pos2(r.min.x, r.min.y + 9.0), pos2(side.min.x, r.max.y));
        for j in 1..3 {
            let x = body.min.x + body.width() * j as f32 / 3.0;
            self.line(pos2(x, body.min.y), pos2(x, body.max.y), self.t.border, 1.0);
        }
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        body
    }

    /// Filter arrows appear on the headings, one is pressed, the other cities' rows fold away.
    fn da_filter(&self) {
        let body = self.da_filter_sheet();
        let press = self.reveal(0.2, 0.4);
        let hide = self.reveal(0.45, 0.8);
        self.da_filter_rows(body, hide, (press > 0.5).then_some(1));
        if press > 0.0 && press < 1.0 {
            let c = self.in_cell(body, 6, 3, 0, 1, true);
            self.pointer(pos2(c.min.x + 8.0, c.center().y));
        }
    }

    /// The folded rows unfold again; the funnel gets a cross.
    fn da_clear_filter(&self) {
        let body = self.da_filter_sheet();
        let k = self.reveal(0.2, 0.7);
        self.da_filter_rows(body, 1.0 - k, (k < 0.5).then_some(1));
        let c = self.at(30.0, 60.0);
        self.da_disc(c);
        self.in_funnel(c, self.accent());
        self.line(c + vec2(4.0, -8.0), c + vec2(12.0, 0.0), self.red(), 2.0);
        self.line(c + vec2(12.0, -8.0), c + vec2(4.0, 0.0), self.red(), 2.0);
    }

    /// A filtered list; a new row typed at the bottom gets checked against the filter.
    fn da_reapply(&self) {
        let body = self.da_filter_sheet();
        self.da_filter_rows(body, 1.0, Some(1));
        let rh = body.height() / 6.0;
        let new = Rect::from_min_size(pos2(body.min.x, body.min.y + rh * 3.0), vec2(body.width(), rh));
        let typed = self.reveal(0.1, 0.3);
        let judged = self.reveal(0.5, 0.75);
        if typed > 0.0 {
            self.in_mark(self.in_cell(new, 1, 3, 0, 0, true), true, 0.6, self.grey());
            self.in_text(self.in_cell(new, 1, 3, 0, 1, true), "بغداد", if judged > 0.5 { self.grey().linear_multiply(0.4) } else { self.ink() });
            self.in_text(self.in_cell(new, 1, 3, 0, 2, true), "٩", self.grey());
            self.p.rect_stroke(new, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        }
        if judged > 0.5 {
            self.p.rect_filled(new, 0.0, Color32::from_rgba_unmultiplied(255, 255, 255, 170));
            self.line(pos2(new.min.x + 10.0, new.center().y), pos2(new.max.x - 10.0, new.center().y), self.red(), 1.5);
        }
        let c = self.at(30.0, 72.0);
        self.da_disc(c);
        let n = 20;
        let spin = self.reveal(0.3, 0.6);
        let pts: Vec<Pos2> = (0..=n)
            .map(|i| {
                let a = -std::f32::consts::FRAC_PI_2 + i as f32 / n as f32 * std::f32::consts::TAU * 0.8 * spin.max(0.05);
                c + vec2(a.cos() * 10.0, a.sin() * 10.0)
            })
            .collect();
        self.p.add(Shape::line(pts, Stroke::new(2.0, self.accent())));
        self.in_funnel(c, self.accent().linear_multiply(0.7));
    }

    /// One cell «علي، بغداد» becomes two cells side by side.
    fn da_text_to_columns(&self) {
        let l = self.in_sheet(Rect::from_min_size(self.at(6.0, 14.0), vec2(72.0, 76.0)), 4, 2, true);
        let pairs = [("علي", "بغداد"), ("سارة", "البصرة"), ("حسن", "الموصل")];
        for (i, (a, b)) in pairs.iter().enumerate() {
            let cell = self.in_range(l, 4, 2, (i, i, 0, 1), true);
            self.label(pos2(cell.max.x - 3.0, cell.center().y), &format!("{a}، {b}"), 6.5, self.ink(), Align2::RIGHT_CENTER);
        }
        self.in_select(self.in_range(l, 4, 2, (0, 2, 0, 1), true));
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
            let r = d.in_sheet(outer, 4, 2, true);
            for (i, (a, b)) in pairs.iter().enumerate() {
                d.in_text(d.in_cell(r, 4, 2, i, 0, true), a, d.ink());
                d.in_text(d.in_cell(r, 4, 2, i, 1, true), b, d.accent());
            }
            d.p.rect_stroke(outer, 1.0, Stroke::new(1.5, d.accent()), egui::StrokeKind::Inside);
        });
    }

    /// Full names on the right; the first name typed once, the rest fill in by themselves.
    fn da_flash_fill(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0)), 5, 3, true);
        let names = [("علي حسن", "علي"), ("سارة كريم", "سارة"), ("حسن علي", "حسن"), ("نور سعد", "نور"), ("زيد عمر", "زيد")];
        for (i, (full, first)) in names.iter().enumerate() {
            self.label(self.in_cell(body, 5, 3, i, 0, true).center(), full, 6.5, self.ink(), Align2::CENTER_CENTER);
            let cell = self.in_cell(body, 5, 3, i, 1, true);
            if i == 0 {
                self.in_text(cell, first, self.ink());
                self.p.rect_stroke(cell, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
            } else {
                let k = self.reveal(0.2 + i as f32 * 0.12, 0.3 + i as f32 * 0.12);
                if k > 0.0 {
                    self.in_text(cell, first, self.accent().linear_multiply(k));
                }
            }
        }
        let k = self.reveal(0.2, 0.8);
        let col = self.in_range(body, 5, 3, (1, 4, 1, 1), true);
        if k > 0.0 {
            self.p.rect_filled(Rect::from_min_size(col.min, vec2(col.width(), col.height() * k)), 0.0, self.accent().linear_multiply(0.12));
        }
        self.label(pos2(body.min.x + 20.0, body.min.y + 14.0), "⚡", 10.0, self.accent(), Align2::CENTER_CENTER);
    }

    /// Two repeated rows turn red, then fold away and the list closes up.
    fn da_remove_duplicates(&self) {
        let body = self.da_filter_sheet();
        let rows = [("علي", "بغداد"), ("سارة", "البصرة"), ("علي", "بغداد"), ("حسن", "الموصل"), ("سارة", "البصرة")];
        let mark = self.reveal(0.1, 0.35);
        let fold = self.reveal(0.45, 0.8);
        let rh = body.height() / 6.0;
        let mut y = body.min.y + rh;
        for (i, (name, city)) in rows.iter().enumerate() {
            let dup = i == 2 || i == 4;
            let h = if dup { rh * (1.0 - fold) } else { rh };
            if h > 0.5 {
                let row = Rect::from_min_size(pos2(body.min.x, y), vec2(body.width(), h));
                self.in_clip(row, |d| {
                    let full = Rect::from_min_size(pos2(body.min.x, y), vec2(body.width(), rh));
                    if dup && mark > 0.0 {
                        d.p.rect_filled(full, 0.0, Color32::from_rgb(255, 225, 225).linear_multiply(mark));
                    }
                    let c = if dup && mark > 0.5 { d.red() } else { d.ink() };
                    d.in_text(d.in_cell(full, 1, 3, 0, 0, true), name, c);
                    d.in_text(d.in_cell(full, 1, 3, 0, 1, true), city, c);
                    d.in_text(d.in_cell(full, 1, 3, 0, 2, true), Draw::in_digit(i + 1), d.grey());
                });
                self.line(pos2(body.min.x, y + h), pos2(body.max.x, y + h), self.t.border, 1.0);
            }
            y += h;
        }
        let head = self.in_range(body, 6, 3, (0, 0, 0, 2), true);
        self.p.rect_filled(head, 0.0, self.t.hover);
        for j in 0..3 {
            self.in_mark(self.in_cell(body, 6, 3, 0, j, true), true, 0.55, self.ink());
        }
        if fold >= 1.0 {
            self.badge(pos2(body.min.x + 30.0, body.max.y - 12.0), &Draw::in_num("−٢"), self.red());
        }
    }

    /// A cell with its dropdown of allowed words; a wrong word gets the red cross.
    fn da_validation(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 1), true);
        let cell = self.in_cell(body, 5, 5, 1, 1, true);
        self.p.rect_stroke(cell, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        self.in_text(cell, "نقيب", self.ink());
        let btn = Rect::from_min_size(pos2(cell.min.x - 9.0, cell.min.y), vec2(9.0, cell.height()));
        self.p.rect_filled(btn, 1.0, self.t.menu);
        self.p.rect_stroke(btn, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.in_triangle(btn.center(), self.ink());
        let open = self.reveal(0.1, 0.4);
        if open > 0.0 {
            let menu = Rect::from_min_size(pos2(cell.min.x - 9.0, cell.max.y + 1.0), vec2(cell.width() + 9.0, 42.0 * open));
            self.in_clip(menu, |d| {
                let full = Rect::from_min_size(menu.min, vec2(menu.width(), 42.0));
                d.p.rect_filled(full, 2.0, d.t.menu);
                d.p.rect_stroke(full, 2.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
                for (i, rank) in ["ملازم", "نقيب", "رائد"].iter().enumerate() {
                    let y = full.min.y + 7.0 + i as f32 * 14.0;
                    if i == 1 {
                        d.p.rect_filled(Rect::from_min_size(pos2(full.min.x + 2.0, y - 6.0), vec2(full.width() - 4.0, 12.0)), 2.0, d.t.selection);
                    }
                    d.label(pos2(full.max.x - 4.0, y), rank, 6.5, d.ink(), Align2::RIGHT_CENTER);
                }
            });
        }
        let bad = self.in_cell(body, 5, 5, 1, 3, true);
        let wrong = self.reveal(0.5, 0.75);
        if wrong > 0.0 {
            self.in_text(bad, "جندي", self.red());
            self.cross(pos2(bad.min.x + 4.0, bad.max.y + 4.0));
        }
    }

    /// A result cell with a question mark; three candidate inputs flip through.
    fn da_what_if(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0)), 4, 4, true);
        self.in_mark(self.in_cell(body, 4, 4, 0, 0, true), true, 0.7, self.grey());
        self.in_mark(self.in_cell(body, 4, 4, 3, 0, true), true, 0.7, self.grey());
        let input = self.in_cell(body, 4, 4, 0, 1, true);
        let result = self.in_cell(body, 4, 4, 3, 1, true);
        let step = if self.phase >= 1.0 { 2 } else { ((self.phase * 3.0) as usize).min(2) };
        let (i, r) = match step {
            0 => ("٨", "٨٠٠"),
            1 => ("٩", "٩٠٠"),
            _ => ("١٠", "١٠٠٠"),
        };
        self.in_text(input, i, self.accent());
        self.p.rect_stroke(input, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        self.in_text(result, r, if step == 2 { self.green() } else { self.ink() });
        self.arrow_down(pos2(input.center().x - 14.0, input.max.y + 2.0), pos2(input.center().x - 14.0, result.min.y - 2.0));
        self.badge(pos2(body.min.x + 30.0, body.center().y), "؟", self.accent());
        if step == 2 {
            self.check(pos2(result.min.x - 14.0, result.center().y));
        }
    }

    /// Actual figures as a solid line, the forecast dashed with its confidence band widening.
    fn da_forecast(&self) {
        let r = Rect::from_min_size(self.at(10.0, 10.0), vec2(SIZE.x - 20.0, SIZE.y - 20.0));
        self.p.rect_filled(r, 2.0, Color32::WHITE);
        self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let a = r.shrink(8.0);
        self.line(a.left_bottom(), a.right_bottom(), self.grey(), 1.0);
        self.line(a.left_top(), a.left_bottom(), self.grey(), 1.0);
        let ys = [0.7, 0.6, 0.65, 0.5, 0.45, 0.5, 0.35];
        let n = ys.len();
        let xs = |i: usize| a.min.x + a.width() * 0.55 * i as f32 / (n - 1) as f32;
        let actual: Vec<Pos2> = ys.iter().enumerate().map(|(i, y)| pos2(xs(i), a.min.y + a.height() * y)).collect();
        self.p.add(Shape::line(actual.clone(), Stroke::new(2.0, self.ink())));
        let k = self.reveal(0.2, 0.75);
        let Some(last) = actual.last().copied() else { return };
        let end = pos2(a.max.x, a.min.y + a.height() * 0.15);
        let split = pos2(last.x, a.min.y);
        self.pl_dash_line(split, pos2(last.x, a.max.y));
        if k <= 0.0 {
            return;
        }
        let tip = last + (end - last) * k;
        let band = vec![last, pos2(tip.x, tip.y - 14.0 * k), pos2(tip.x, tip.y + 14.0 * k)];
        self.p.add(Shape::convex_polygon(band, self.accent().linear_multiply(0.2), Stroke::NONE));
        let mut t = 0.0;
        let len = (tip - last).length();
        let dir = if len > 0.0 { (tip - last) / len } else { vec2(1.0, 0.0) };
        while t < len {
            self.line(last + dir * t, last + dir * (t + 4.0).min(len), self.accent(), 2.0);
            t += 7.0;
        }
    }

    /// The outline bracket draws down beside the selected rows and a minus box appears.
    fn da_group(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 30.0, SIZE.y - 12.0)), 5, 5, true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let rows = self.in_range(body, 5, 5, (1, 3, 0, 4), true);
        self.in_select(rows);
        let k = self.reveal(0.2, 0.6);
        let x = body.max.x + 23.0;
        self.line(pos2(x, rows.min.y + 2.0), pos2(x, rows.min.y + 2.0 + (rows.height() - 4.0) * k), self.accent(), 2.0);
        if k >= 1.0 {
            self.line(pos2(x, rows.max.y - 2.0), pos2(x - 4.0, rows.max.y - 2.0), self.accent(), 2.0);
            let b = Rect::from_center_size(pos2(x, rows.max.y + 8.0), vec2(9.0, 9.0));
            self.p.rect_filled(b, 1.0, Color32::WHITE);
            self.p.rect_stroke(b, 1.0, self.stroke(self.accent()), egui::StrokeKind::Inside);
            self.line(b.center() + vec2(-2.5, 0.0), b.center() + vec2(2.5, 0.0), self.accent(), 1.5);
        }
    }

    /// The bracket and its box fade away; the rows stay.
    fn da_ungroup(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 30.0, SIZE.y - 12.0)), 5, 5, true);
        self.in_marks(body, 5, 5, (0, 5, 0, 5), true);
        let rows = self.in_range(body, 5, 5, (1, 3, 0, 4), true);
        self.in_select(rows);
        let k = self.reveal(0.2, 0.6);
        if k < 1.0 {
            let c = self.accent().linear_multiply(1.0 - k);
            let x = body.max.x + 23.0;
            self.line(pos2(x, rows.min.y + 2.0), pos2(x, rows.max.y - 2.0), c, 2.0);
            self.line(pos2(x, rows.max.y - 2.0), pos2(x - 4.0, rows.max.y - 2.0), c, 2.0);
            let b = Rect::from_center_size(pos2(x, rows.max.y + 8.0), vec2(9.0, 9.0));
            self.p.rect_stroke(b, 1.0, self.stroke(c), egui::StrokeKind::Inside);
        }
        if k > 0.3 {
            self.cross(pos2(body.min.x + 24.0, body.center().y));
        }
    }

    /// Total rows slide in under each group, and a grand total at the bottom.
    fn da_subtotal(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 30.0, SIZE.y - 12.0)), 7, 3, true);
        let k = self.reveal(0.2, 0.7);
        let kinds = [0u8, 0, 1, 0, 0, 1, 2];
        for (i, kind) in kinds.iter().enumerate() {
            let cell0 = self.in_cell(body, 7, 3, i, 0, true);
            let cell1 = self.in_cell(body, 7, 3, i, 1, true);
            let cell2 = self.in_cell(body, 7, 3, i, 2, true);
            match kind {
                0 => {
                    self.in_mark(cell0, true, 0.7, self.grey());
                    self.in_mark(cell1, true, 0.5, self.grey());
                    self.in_text(cell2, Draw::in_digit(i + 1), self.ink());
                }
                1 => {
                    if k > 0.0 {
                        let row = cell0.union(cell2);
                        self.p.rect_filled(row, 0.0, self.accent().linear_multiply(0.15 * k));
                        self.label(pos2(cell0.max.x - 3.0, cell0.center().y), "المجموع", 6.5, self.accent().linear_multiply(k), Align2::RIGHT_CENTER);
                        self.in_text(cell2, if i == 2 { "٣" } else { "٩" }, self.accent().linear_multiply(k));
                    }
                }
                _ => {
                    if k > 0.5 {
                        let row = cell0.union(cell2);
                        self.p.rect_filled(row, 0.0, self.accent().linear_multiply(0.3));
                        self.label(pos2(cell0.max.x - 3.0, cell0.center().y), "الإجمالي", 6.5, self.ink(), Align2::RIGHT_CENTER);
                        self.in_text(cell2, "١٢", self.ink());
                    }
                }
            }
        }
        let x = body.max.x + 23.0;
        for (top, bottom) in [(0usize, 1usize), (3, 4)] {
            let a = self.in_cell(body, 7, 3, top, 0, true).min.y + 2.0;
            let b = self.in_cell(body, 7, 3, bottom, 0, true).max.y - 2.0;
            self.line(pos2(x, a), pos2(x, b), self.accent().linear_multiply(k), 1.5);
        }
    }

    /// A folded group opens (`show`) or an open one folds shut: the rows wipe in or out
    /// under the plus/minus box.
    fn da_detail(&self, show: bool) {
        let r = Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 30.0, SIZE.y - 12.0));
        let k = self.reveal(0.2, 0.65);
        let open = if show { k } else { 1.0 - k };
        let rh = (r.height() - 9.0) / 5.0;
        let detail_h = rh * 3.0 * open;
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 9.0)), 0.0, self.t.hover);
        let side = Rect::from_min_max(pos2(r.max.x - 11.0, r.min.y), r.max);
        self.p.rect_filled(side, 0.0, self.t.hover);
        let body = Rect::from_min_max(pos2(r.min.x, r.min.y + 9.0), pos2(side.min.x, r.max.y));
        let head = Rect::from_min_size(body.min, vec2(body.width(), rh));
        self.in_mark(self.in_cell(head, 1, 3, 0, 0, true), true, 0.7, self.ink());
        self.in_mark(self.in_cell(head, 1, 3, 0, 1, true), true, 0.5, self.ink());
        let detail = Rect::from_min_size(pos2(body.min.x, head.max.y), vec2(body.width(), detail_h));
        if detail_h > 0.5 {
            self.in_clip(detail, |d| {
                for i in 0..3 {
                    let row = Rect::from_min_size(pos2(body.min.x, head.max.y + i as f32 * rh), vec2(body.width(), rh));
                    d.in_mark(d.in_cell(row, 1, 3, 0, 0, true), true, 0.6, d.grey());
                    d.in_mark(d.in_cell(row, 1, 3, 0, 1, true), true, 0.4, d.grey());
                    d.in_text(d.in_cell(row, 1, 3, 0, 2, true), Draw::in_digit(i + 2), d.grey());
                    d.line(row.left_bottom(), row.right_bottom(), d.t.border, 1.0);
                }
            });
        }
        let total = Rect::from_min_size(pos2(body.min.x, detail.max.y), vec2(body.width(), rh));
        self.p.rect_filled(total, 0.0, self.accent().linear_multiply(0.15));
        self.label(pos2(total.max.x - 3.0, total.center().y), "المجموع", 6.5, self.accent(), Align2::RIGHT_CENTER);
        self.in_text(self.in_cell(total, 1, 3, 0, 2, true), "٩", self.accent());
        for j in 1..3 {
            let x = body.min.x + body.width() * j as f32 / 3.0;
            self.line(pos2(x, body.min.y), pos2(x, total.max.y), self.t.border, 1.0);
        }
        self.line(head.left_bottom(), head.right_bottom(), self.t.border, 1.0);
        self.line(total.left_bottom(), total.right_bottom(), self.t.border, 1.0);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let x = r.max.x + 12.0;
        if detail_h > 0.5 {
            self.line(pos2(x, detail.min.y + 2.0), pos2(x, detail.max.y - 2.0), self.accent(), 2.0);
        }
        let b = Rect::from_center_size(pos2(x, total.center().y), vec2(11.0, 11.0));
        self.p.rect_filled(b, 1.0, Color32::WHITE);
        self.p.rect_stroke(b, 1.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        self.line(b.center() + vec2(-3.0, 0.0), b.center() + vec2(3.0, 0.0), self.accent(), 1.5);
        if open < 0.5 {
            self.line(b.center() + vec2(0.0, -3.0), b.center() + vec2(0.0, 3.0), self.accent(), 1.5);
        }
    }

    /// The analysis tools list with a small histogram beside it.
    fn da_data_analysis(&self) {
        let r = Rect::from_min_size(self.at(8.0, 8.0), vec2(SIZE.x - 16.0, SIZE.y - 16.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(r.max.x - 5.0, r.min.y + 6.0), "تحليل البيانات", 7.0, self.ink(), Align2::RIGHT_CENTER);
        let pick = self.reveal(0.15, 0.4);
        for (i, tool) in ["إحصاء وصفي", "مدرج تكراري", "ارتباط", "انحدار"].iter().enumerate() {
            let y = r.min.y + 22.0 + i as f32 * 15.0;
            let row = Rect::from_min_size(pos2(r.max.x - 76.0, y - 6.0), vec2(72.0, 13.0));
            if i == 1 && pick > 0.0 {
                self.p.rect_filled(row, 2.0, self.t.selection);
            }
            self.label(pos2(row.max.x - 4.0, y), tool, 6.5, self.ink(), Align2::RIGHT_CENTER);
        }
        let k = self.reveal(0.45, 0.85);
        let chart = Rect::from_min_size(pos2(r.min.x + 8.0, r.min.y + 20.0), vec2(72.0, 60.0));
        self.p.rect_filled(chart, 1.0, Color32::WHITE);
        self.p.rect_stroke(chart, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let bw = (chart.width() - 8.0) / 6.0;
        for (i, h) in [0.2, 0.5, 0.9, 0.7, 0.4, 0.15].iter().enumerate() {
            let x = chart.min.x + 4.0 + i as f32 * bw;
            let top = chart.max.y - 4.0 - (chart.height() - 8.0) * h * k;
            self.p.rect_filled(Rect::from_min_max(pos2(x + 1.0, top), pos2(x + bw - 1.0, chart.max.y - 4.0)), 0.0, self.accent());
        }
    }

    /// Variable cells change while the target climbs to its best; a tick when done.
    fn da_solver(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0)), 4, 4, true);
        let k = self.reveal(0.15, 0.7);
        let step = (k * 3.0).floor().min(3.0) as usize;
        let vars = [("٢", "٥"), ("٣", "٤"), ("٤", "٣"), ("٥", "٣")];
        let (a, b) = vars.get(step).copied().unwrap_or(("٥", "٣"));
        let targets = ["٤٠٠", "٥٥٠", "٦٨٠", "٧٥٠"];
        self.in_mark(self.in_cell(body, 4, 4, 0, 0, true), true, 0.7, self.grey());
        self.in_mark(self.in_cell(body, 4, 4, 1, 0, true), true, 0.7, self.grey());
        self.label(self.in_cell(body, 4, 4, 3, 0, true).center(), "الربح", 6.5, self.ink(), Align2::CENTER_CENTER);
        let c1 = self.in_cell(body, 4, 4, 0, 1, true);
        let c2 = self.in_cell(body, 4, 4, 1, 1, true);
        self.in_text(c1, a, self.accent());
        self.in_text(c2, b, self.accent());
        self.p.rect_stroke(c1.union(c2), 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        let target = self.in_cell(body, 4, 4, 3, 1, true);
        self.in_text(target, targets.get(step).copied().unwrap_or("٧٥٠"), self.ink());
        self.badge(pos2(target.min.x - 18.0, target.center().y), "max", self.accent());
        self.label(pos2(body.min.x + 40.0, body.min.y + 22.0), "≤ الميزانية", 6.5, self.grey(), Align2::CENTER_CENTER);
        if k >= 1.0 {
            self.check(pos2(body.min.x + 28.0, body.max.y - 18.0));
        }
    }

    /// Red rings pop round the cells that break the rule.
    fn da_circle_invalid(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 1), true);
        let vals = ["٤٥", "١٢٠", "٨٠", "٩٩", "١٥٠"];
        for (i, v) in vals.iter().enumerate() {
            let cell = self.in_cell(body, 5, 5, i, 1, true);
            let bad = i == 1 || i == 4;
            self.in_text(cell, v, if bad { self.red() } else { self.ink() });
            if bad {
                let k = self.reveal(0.15 + i as f32 * 0.08, 0.4 + i as f32 * 0.08);
                if k > 0.0 {
                    let n = 24;
                    let pts: Vec<Pos2> = (0..=n)
                        .map(|j| {
                            let a = j as f32 / n as f32 * std::f32::consts::TAU * k;
                            cell.center() + vec2(a.cos() * (cell.width() / 2.0 - 1.0), a.sin() * (cell.height() / 2.0 + 1.0))
                        })
                        .collect();
                    self.p.add(Shape::line(pts, Stroke::new(1.8, self.red())));
                }
            }
        }
        self.in_label(pos2(body.min.x + 34.0, body.min.y + 12.0), "≤ ١٠٠", 7.0, self.grey(), Align2::CENTER_CENTER);
    }

    /// The dropdown button and the red rings fade from the column; a cross marks the rule gone.
    fn da_clear_validation(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 1), true);
        let k = self.reveal(0.2, 0.6);
        let fade = 1.0 - k;
        let col = self.in_range(body, 5, 5, (0, 4, 1, 1), true);
        self.in_select(col);
        for i in 0..5 {
            let cell = self.in_cell(body, 5, 5, i, 1, true);
            self.in_text(cell, ["ملازم", "نقيب", "رائد", "نقيب", "ملازم"].get(i).copied().unwrap_or(""), self.ink());
        }
        if fade > 0.0 {
            let first = self.in_cell(body, 5, 5, 1, 1, true);
            let btn = Rect::from_min_size(pos2(first.min.x - 9.0, first.min.y), vec2(9.0, first.height()));
            self.p.rect_filled(btn, 1.0, self.t.menu.linear_multiply(fade));
            self.p.rect_stroke(btn, 1.0, self.stroke(self.t.border_strong.linear_multiply(fade)), egui::StrokeKind::Inside);
            self.in_triangle(btn.center(), self.ink().linear_multiply(fade));
            let ring = self.in_cell(body, 5, 5, 3, 1, true);
            let n = 24;
            let pts: Vec<Pos2> = (0..=n)
                .map(|j| {
                    let a = j as f32 / n as f32 * std::f32::consts::TAU;
                    ring.center() + vec2(a.cos() * (ring.width() / 2.0 - 1.0), a.sin() * (ring.height() / 2.0 + 1.0))
                })
                .collect();
            self.p.add(Shape::closed_line(pts, Stroke::new(1.8, self.red().linear_multiply(fade))));
        }
        if k > 0.3 {
            self.cross(pos2(body.min.x + 30.0, body.center().y));
        }
    }

    /// The Goal Seek dialog: the formula cell, the target, the cell to change.
    fn da_goal_seek(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 1), true);
        let inner = self.dialog("الاستهداف", 0);
        let rows = [("الخلية", "B4"), ("القيمة", "١٠٠٠"), ("بتغيير", "B1")];
        for (i, (label, val)) in rows.iter().enumerate() {
            let k = self.reveal(0.1 + i as f32 * 0.15, 0.3 + i as f32 * 0.15);
            let y = inner.min.y + i as f32 * 14.0;
            self.label(pos2(inner.max.x, y + 5.0), label, 6.5, self.grey(), Align2::RIGHT_CENTER);
            let f = Rect::from_min_size(pos2(inner.min.x, y), vec2(inner.width() - 34.0, 10.0));
            self.p.rect_filled(f, 1.0, self.t.input);
            self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
            if k > 0.0 {
                self.label(pos2(f.min.x + 4.0, f.center().y), val, 6.5, self.accent().linear_multiply(k), Align2::LEFT_CENTER);
            }
        }
    }

    /// The scenarios dialog: three named sets, one highlighted, and the sheet's numbers switch.
    fn da_scenarios(&self) {
        let body = self.in_sheet_full(true);
        self.in_marks(body, 5, 5, (0, 5, 0, 1), true);
        let k = self.reveal(0.3, 0.6);
        let picked = if k > 0.5 { 2 } else { 1 };
        let vals = if picked == 2 { ["٦٠", "٧٠", "٥٥"] } else { ["٨٠", "٩٠", "٨٥"] };
        for (i, v) in vals.iter().enumerate() {
            self.in_text(self.in_cell(body, 5, 5, i, 1, true), v, self.accent());
        }
        let r = Rect::from_min_size(self.at(8.0, 14.0), vec2(74.0, 76.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(r.max.x - 5.0, r.min.y + 6.0), "السيناريوهات", 6.5, self.ink(), Align2::RIGHT_CENTER);
        for (i, name) in ["متفائل", "متوقع", "متشائم"].iter().enumerate() {
            let y = r.min.y + 24.0 + i as f32 * 16.0;
            let row = Rect::from_min_size(pos2(r.min.x + 4.0, y - 7.0), vec2(r.width() - 8.0, 14.0));
            if i == picked {
                self.p.rect_filled(row, 2.0, self.t.selection);
            }
            self.label(pos2(row.max.x - 4.0, y), name, 7.0, self.ink(), Align2::RIGHT_CENTER);
        }
        if k > 0.0 && k < 1.0 {
            self.pointer(pos2(r.min.x + 16.0, r.min.y + 44.0 + 12.0 * k));
        }
    }

    /// Input values along the top and side; the body fills with the formula's results.
    fn da_data_table(&self) {
        let body = self.in_sheet(Rect::from_min_size(self.at(6.0, 6.0), vec2(SIZE.x - 12.0, SIZE.y - 12.0)), 5, 5, true);
        let corner = self.in_cell(body, 5, 5, 0, 0, true);
        self.p.rect_filled(corner, 0.0, self.accent().linear_multiply(0.35));
        self.label(corner.center(), "=B1", 6.0, self.ink(), Align2::CENTER_CENTER);
        let rates = ["٥٪", "٦٪", "٧٪", "٨٪"];
        let years = ["٥", "١٠", "١٥", "٢٠"];
        for (j, r) in rates.iter().enumerate() {
            let c = self.in_cell(body, 5, 5, 0, j + 1, true);
            self.p.rect_filled(c, 0.0, self.t.hover);
            self.in_text(c, r, self.ink());
        }
        for (i, y) in years.iter().enumerate() {
            let c = self.in_cell(body, 5, 5, i + 1, 0, true);
            self.p.rect_filled(c, 0.0, self.t.hover);
            self.in_text(c, y, self.ink());
        }
        for i in 0..4 {
            for j in 0..4 {
                let k = self.reveal(0.1 + (i * 4 + j) as f32 * 0.035, 0.2 + (i * 4 + j) as f32 * 0.035);
                if k <= 0.0 {
                    continue;
                }
                let c = self.in_cell(body, 5, 5, i + 1, j + 1, true);
                let n = (i + 1) * (j + 2);
                let text = format!("{}{}", Draw::in_digit(n / 2 + 1), Draw::in_digit(n % 9 + 1));
                self.in_text(c, &text, self.accent().linear_multiply(k));
            }
        }
    }
}
