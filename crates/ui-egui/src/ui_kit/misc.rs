//! Entries outside the main tabs: Automate, the Table Design and Chart Design contextual tabs,
//! and the Conditional Formatting submenus (Top/Bottom Rules, Clear Rules).

#![allow(unused_imports)]

use egui::{Align2, Color32, Rect, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Dir, Draw, SIZE};
use super::view::vw_digits;

pub const ENTRIES: &[Entry] = &[
    // ---- Automate ----
    Entry::new(
        "Macros and scripts in this workbook (Alt+F8)",
        "Lists the scripts saved in this workbook — recorded sequences of commands — and runs the one you pick. Example: a script that sorts the list, adds totals and bolds the title, in one click.",
        |d| d.ms_macros(),
    ),
    Entry::new(
        "Search and run any command",
        "Opens a search box where you type a few letters of what you want to do and pick the command from the list, instead of looking for its button. Example: type «freeze» and choose Freeze Top Row.",
        |d| d.ms_command_palette(),
    ),
    Entry::new(
        "How agents drive GridCraft (MCP / control channel)",
        "Explains how a program or an AI assistant can work the sheet for you: every button is also a command it can send, and it sees the same workbook. Example: an assistant fills B2:B9 and makes the chart while you watch.",
        |d| d.ms_agents(),
    ),
    Entry::new(
        "Every command run in this session (replayable)",
        "Shows the list of everything done since the workbook was opened, step by step, so you can see what happened or run the same steps again. Example: «set B2», «bold», «sort A:C» in order.",
        |d| d.ms_journal(),
    ),
    Entry::new(
        "Version, contributors and the AI models that helped",
        "Shows which version of GridCraft this is, who made it, and the AI models that helped build it. Example: the version number to quote when you report a problem.",
        |d| d.ms_about(),
    ),
    // ---- Table Design ----
    Entry::new(
        "Convert to Range",
        "Turns the table back into ordinary cells: the values and colours stay, but the filter arrows, the automatic growth and the table name are gone. Example: the table «Sales» becomes plain cells A1:C8.",
        |d| d.ms_convert_to_range(),
    ),
    Entry::new(
        "Insert Slicer",
        "Adds a box of buttons, one for each value in a column, so the table is filtered by clicking a button instead of opening the filter menu. Example: buttons «Baghdad», «Basra», «Erbil»; click one and only its rows show.",
        |d| d.ms_slicer(),
    ),
    Entry::new(
        "Table Styles",
        "Opens a gallery of ready colour schemes for the table: the header colour, the stripes, the borders. Example: pick the blue style and the header turns blue with light blue stripes.",
        |d| d.ms_table_styles(),
    ),
    Entry::new(
        "table.headerRow",
        "Shows or hides the table's title row, the one with the column names and filter arrows. Example: hide it and the table starts with the first data row.",
        |d| d.ms_table_toggle(TableLook { header: true, ..TableLook::PLAIN }),
    ),
    Entry::new(
        "table.totalRow",
        "Adds a row at the bottom of the table that sums (or counts, averages…) each column, and keeps it up to date. Example: a Total row showing the sum of the Amount column.",
        |d| d.ms_table_toggle(TableLook { header: true, total: true, ..TableLook::PLAIN }),
    ),
    Entry::new(
        "table.bandedRows",
        "Shades every other row of the table so a long row is easy to follow with the eye. Example: rows 2, 4 and 6 light blue, rows 3, 5 and 7 white.",
        |d| d.ms_table_toggle(TableLook { header: true, banded_rows: true, ..TableLook::PLAIN }),
    ),
    Entry::new(
        "table.firstColumn",
        "Makes the first column of the table stand out, in bold, since it usually holds the names. Example: the Name column in bold.",
        |d| d.ms_table_toggle(TableLook { header: true, first: true, ..TableLook::PLAIN }),
    ),
    Entry::new(
        "table.lastColumn",
        "Makes the last column of the table stand out, in bold, since it often holds the total. Example: the Total column in bold.",
        |d| d.ms_table_toggle(TableLook { header: true, last: true, ..TableLook::PLAIN }),
    ),
    Entry::new(
        "table.bandedColumns",
        "Shades every other column of the table so the columns are easy to tell apart. Example: columns B and D light blue, A and C white.",
        |d| d.ms_table_toggle(TableLook { header: true, banded_cols: true, ..TableLook::PLAIN }),
    ),
    // ---- Chart Design ----
    Entry::new(
        "Add Chart Element",
        "Opens the list of parts you can add to or take off the chart: the legend and where it sits, the value labels on the bars, the gridlines. Example: add a legend under the chart and labels above the bars.",
        |d| d.ms_add_element(),
    ),
    Entry::new(
        "Switch Row/Column",
        "Swaps what the chart groups by: the rows become the series and the columns the categories. Example: bars grouped by month with one colour per branch become bars grouped by branch with one colour per month.",
        |d| d.ms_switch_row_column(),
    ),
    Entry::new(
        "Select Data",
        "Opens a box where you choose which cells the chart draws, which series to keep and which cells hold the category names. Example: widen the range from B2:B7 to B2:C7 and a second series appears.",
        |d| d.ms_select_data(),
    ),
    Entry::new(
        "Change Chart Type",
        "Picks another kind of chart for the same numbers: columns, bars, a line, a pie, an area, a scatter… Example: the column chart becomes a line chart.",
        |d| d.ms_change_type(),
    ),
    Entry::new(
        "Format Chart pane",
        "Opens a pane at the side with the chart's settings: colours of the bars, the axis range, the fonts, the borders. Example: change the first series from blue to green.",
        |d| d.ms_format_pane(),
    ),
    Entry::new(
        "Edit chart title",
        "Opens a box to write the chart's title, or to clear it. Example: the title «Sales 2026» appears above the bars.",
        |d| d.ms_chart_title(),
    ),
    Entry::new(
        "Delete chart",
        "Removes the selected chart from the sheet; the numbers it drew stay in their cells. Example: the chart is gone, B2:B7 is untouched.",
        |d| d.ms_delete_chart(),
    ),
    Entry::new(
        "chart.set",
        "Turns a part of the chart on or off, or moves it: the legend (bottom, right, top or none), the value labels on the bars, the gridlines behind them. Example: Legend: Bottom puts the series names under the chart.",
        |d| d.ms_chart_set(),
    ),
    // ---- Conditional Formatting submenus ----
    Entry::new(
        "home.conditionalFormat",
        "Colours the cells that rank highest or lowest in the selection — the top 10 items, the top 10%, the bottom 10, or those above or below the average — and keeps the colours right when the numbers change. Example: the ten biggest sales turn red.",
        |d| d.ms_top_bottom(),
    ),
    Entry::new(
        "home.clearRules",
        "Removes the conditional formatting rules — from the selected cells only, or from the whole sheet — so the cells show their ordinary colours again. Example: the red top-ten cells go back to white.",
        |d| d.ms_clear_rules(),
    ),
];

/// Which parts of a table are styled.
#[derive(Clone, Copy)]
pub(super) struct TableLook {
    pub header: bool,
    pub total: bool,
    pub banded_rows: bool,
    pub banded_cols: bool,
    pub first: bool,
    pub last: bool,
    pub filter: bool,
}

impl TableLook {
    pub const PLAIN: TableLook =
        TableLook { header: false, total: false, banded_rows: false, banded_cols: false, first: false, last: false, filter: false };
    pub const STYLED: TableLook =
        TableLook { header: true, total: false, banded_rows: true, banded_cols: false, first: false, last: false, filter: true };
}

impl Draw<'_> {
    // ---- tables ----

    /// A styled table in `r`: `rows × cols` with the parts `look` names in the accent colour,
    /// and sample content (names, numbers). The `k` (0 → 1) wipes the styled parts in.
    pub(super) fn ms_table(&self, r: Rect, rows: usize, cols: usize, look: TableLook, k: f32) {
        let (rows, cols) = (rows.max(2), cols.max(1));
        self.p.rect_filled(r, 0.0, Color32::WHITE);
        let soft = self.accent().linear_multiply(0.18 * k.max(0.0));
        let header_fill = self.accent().linear_multiply(0.35 + 0.65 * k.max(0.0));
        let last_row = rows - 1;
        for i in 0..rows {
            for j in 0..cols {
                let c = self.cell(r, rows, cols, i, j);
                let header = look.header && i == 0;
                let total = look.total && i == last_row;
                let stripe_r = look.banded_rows && i > 0 && i % 2 == 0;
                let stripe_c = look.banded_cols && j % 2 == 1;
                let edge = (look.first && j == 0) || (look.last && j + 1 == cols);
                if header {
                    self.p.rect_filled(c, 0.0, header_fill);
                } else if total || stripe_r || stripe_c || edge {
                    self.p.rect_filled(c, 0.0, soft);
                }
                let strong = header || total || edge;
                let color = if header {
                    Color32::WHITE
                } else if strong {
                    self.ink()
                } else {
                    self.grey()
                };
                if header {
                    self.vw_dash(c, color);
                    if look.filter {
                        let a = pos2(c.max.x - 4.0, c.center().y);
                        self.p.add(egui::Shape::convex_polygon(
                            vec![a + vec2(-2.5, -1.5), a + vec2(2.5, -1.5), a + vec2(0.0, 1.5)],
                            Color32::WHITE,
                            Stroke::NONE,
                        ));
                    }
                } else if total {
                    if j == 0 {
                        self.vw_text(c, "المجموع", color, false);
                    } else {
                        self.vw_num(c, 400 + (j * 37) as u32, color);
                    }
                } else if j == 0 {
                    let names = ["أحمد", "سارة", "علي", "نور", "ليث", "هدى"];
                    self.vw_text(c, names.get(i.saturating_sub(1) % names.len()).copied().unwrap_or("—"), color, false);
                } else {
                    self.vw_num(c, 40 + ((i * 31 + j * 17) % 160) as u32, color);
                }
            }
        }
        self.grid(r, rows, cols, self.t.border);
        if look.total {
            let y = self.cell(r, rows, cols, last_row, 0).min.y;
            self.line(pos2(r.min.x, y), pos2(r.max.x, y), self.accent().linear_multiply(k.max(0.0)), 2.0);
        }
        self.p.rect_stroke(r, 0.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
    }

    fn ms_table_toggle(&self, look: TableLook) {
        let r = Rect::from_min_size(self.at(18.0, 12.0), vec2(140.0, 80.0));
        let k = self.reveal(0.2, 0.6);
        // Before the wipe the table is shown without the toggled part (a bare table when the
        // part is the header row itself).
        let only_header = !(look.total || look.banded_rows || look.banded_cols || look.first || look.last);
        let base = if only_header { TableLook::PLAIN } else { TableLook { header: true, ..TableLook::PLAIN } };
        self.ms_table(r, 6, 3, base, 1.0);
        if k <= 0.0 {
            return;
        }
        let clip = Rect::from_min_max(r.min, pos2(r.min.x + r.width() * k, r.max.y));
        let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        self.with_painter(&clipped).ms_table(r, 6, 3, look, 1.0);
    }

    fn ms_convert_to_range(&self) {
        self.vw_two(
            |d, r| {
                let area = d.vw_sheet(r, 5, 2);
                d.ms_table(area, 5, 2, TableLook::STYLED, 1.0);
            },
            |d, r| {
                let area = d.vw_sheet(r, 5, 2);
                d.ms_table(area, 5, 2, TableLook::PLAIN, 0.0);
            },
        );
    }

    fn ms_slicer(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(100.0, 88.0));
        let area = self.vw_sheet(r, 6, 2);
        self.ms_table(area, 6, 2, TableLook::STYLED, 1.0);
        let k = self.reveal(0.2, 0.5);
        if k <= 0.0 {
            return;
        }
        let s = Rect::from_min_size(self.at(110.0, 14.0), vec2(60.0, 74.0));
        self.p.rect_filled(s, 2.0, self.t.menu);
        self.p.rect_stroke(s, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(s.min, vec2(s.width(), 12.0)), 2.0, self.t.hover);
        self.label(pos2(s.max.x - 4.0, s.min.y + 6.0), "المدينة", 6.5, self.ink(), Align2::RIGHT_CENTER);
        let pressed = self.reveal(0.55, 0.75);
        for (i, city) in ["بغداد", "البصرة", "أربيل"].into_iter().enumerate() {
            let b = Rect::from_min_size(pos2(s.min.x + 5.0, s.min.y + 17.0 + i as f32 * 18.0), vec2(s.width() - 10.0, 14.0));
            let on = i == 0 && pressed > 0.0;
            self.p.rect_filled(b, 2.0, if on { self.accent() } else { Color32::WHITE });
            self.p.rect_stroke(b, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            self.label(b.center(), city, 6.5, if on { Color32::WHITE } else { self.ink() }, Align2::CENTER_CENTER);
        }
        if pressed > 0.0 {
            // The rows of the other cities are hidden.
            for i in [2usize, 4] {
                let c = Rect::from_min_max(self.cell(area, 6, 2, i, 0).min, self.cell(area, 6, 2, i, 1).max);
                self.p.rect_filled(c, 0.0, Color32::WHITE.linear_multiply(0.75 * pressed));
                self.line(pos2(c.min.x, c.center().y), pos2(c.max.x, c.center().y), self.t.border_strong, 1.0);
            }
        }
    }

    fn ms_table_styles(&self) {
        let colours = [
            Color32::from_rgb(0x2B, 0x7C, 0xD3),
            Color32::from_rgb(0x10, 0x7C, 0x41),
            Color32::from_rgb(0xE8, 0x7D, 0x2B),
            Color32::from_rgb(0x88, 0x55, 0xC8),
            Color32::from_rgb(0x60, 0x60, 0x60),
            Color32::from_rgb(0xD1, 0x3B, 0x2F),
        ];
        let slide = self.reveal(0.3, 0.6);
        for (i, c) in colours.into_iter().enumerate() {
            let col = i % 3;
            let row = i / 3;
            let r = Rect::from_min_size(self.at(10.0 + col as f32 * 54.0, 10.0 + row as f32 * 46.0), vec2(46.0, 36.0));
            self.p.rect_filled(r, 0.0, Color32::WHITE);
            let rows = 4;
            for k in 0..rows {
                let band = self.cell(r, rows, 1, k, 0);
                if k == 0 {
                    self.p.rect_filled(band, 0.0, c);
                } else if k % 2 == 0 {
                    self.p.rect_filled(band, 0.0, c.linear_multiply(0.18));
                }
            }
            self.grid(r, rows, 3, self.t.border);
        }
        // The pick frame slides from the first style to the second.
        let x = 10.0 + 54.0 * slide;
        let frame = Rect::from_min_size(self.at(x, 10.0), vec2(46.0, 36.0));
        self.p.rect_stroke(frame.expand(2.0), 2.0, Stroke::new(2.0, self.accent()), egui::StrokeKind::Outside);
    }

    // ---- charts ----

    /// A column chart in `r`: `groups` groups of `series` bars, with optional value labels and
    /// gridlines; `k` grows the bars from the base line.
    pub(super) fn ms_bars(&self, r: Rect, groups: usize, series: usize, labels: bool, gridlines: bool, k: f32) {
        let (groups, series) = (groups.clamp(1, 6), series.clamp(1, 4));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let plot = Rect::from_min_max(pos2(r.min.x + 6.0, r.min.y + 6.0), pos2(r.max.x - 6.0, r.max.y - 8.0));
        if gridlines {
            for i in 1..4 {
                let y = plot.min.y + plot.height() * i as f32 / 4.0;
                self.line(pos2(plot.min.x, y), pos2(plot.max.x, y), self.t.border, 1.0);
            }
        }
        self.line(plot.left_bottom(), plot.right_bottom(), self.grey(), 1.0);
        let heights = [0.55, 0.85, 0.4, 0.7, 0.95, 0.6, 0.3, 0.8, 0.5, 0.65, 0.45, 0.9];
        let gw = plot.width() / groups as f32;
        let bw = (gw - 4.0) / series as f32;
        for g in 0..groups {
            for s in 0..series {
                let h = heights.get((g * series + s) % heights.len()).copied().unwrap_or(0.5) * k.max(0.0);
                let x = plot.min.x + g as f32 * gw + 2.0 + s as f32 * bw;
                let top = plot.max.y - (plot.height() - 6.0) * h;
                let bar = Rect::from_min_max(pos2(x + 0.5, top), pos2(x + bw - 0.5, plot.max.y));
                self.p.rect_filled(bar, 0.5, self.ms_series_colour(s));
                if labels && k >= 1.0 {
                    self.label(pos2(bar.center().x, top - 3.0), &vw_digits((h * 100.0) as u32), 5.0, self.grey(), Align2::CENTER_CENTER);
                }
            }
        }
    }
    /// The colour of series `s`.
    pub(super) fn ms_series_colour(&self, s: usize) -> Color32 {
        match s % 4 {
            0 => self.accent(),
            1 => Color32::from_rgb(0xE8, 0x7D, 0x2B),
            2 => Color32::from_rgb(0x10, 0x7C, 0x41),
            _ => Color32::from_rgb(0x88, 0x55, 0xC8),
        }
    }
    /// A legend of `n` series centred at `c`, laid out in a row (or a column when `vertical`).
    pub(super) fn ms_legend(&self, c: egui::Pos2, n: usize, vertical: bool) {
        let n = n.clamp(1, 4);
        let step = 22.0;
        for s in 0..n {
            let off = (s as f32 - (n as f32 - 1.0) / 2.0) * step;
            let p = if vertical { c + vec2(0.0, off * 0.5) } else { c + vec2(off, 0.0) };
            self.p.rect_filled(Rect::from_center_size(p - vec2(6.0, 0.0), vec2(5.0, 5.0)), 0.5, self.ms_series_colour(s));
            self.line(p - vec2(2.0, 0.0), p + vec2(8.0, 0.0), self.grey(), 1.5);
        }
    }
    /// A line chart in `r` (one series with markers).
    pub(super) fn ms_line_chart(&self, r: Rect) {
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let plot = Rect::from_min_max(pos2(r.min.x + 6.0, r.min.y + 6.0), pos2(r.max.x - 6.0, r.max.y - 8.0));
        self.line(plot.left_bottom(), plot.right_bottom(), self.grey(), 1.0);
        let ys = [0.55, 0.85, 0.4, 0.7, 0.95];
        let pts: Vec<egui::Pos2> = ys
            .iter()
            .enumerate()
            .map(|(i, h)| pos2(plot.min.x + plot.width() * (i as f32 + 0.5) / ys.len() as f32, plot.max.y - (plot.height() - 6.0) * h))
            .collect();
        self.p.add(egui::Shape::line(pts.clone(), Stroke::new(1.5, self.accent())));
        for p in pts {
            self.p.circle_filled(p, 2.0, self.accent());
        }
    }
    /// A chart title at the top of `r`: `n` of the word's letters, with a caret when unfinished.
    fn ms_title_text(&self, r: Rect, text: &str, k: f32) {
        let shown: String = text.chars().take((text.chars().count() as f32 * k).round() as usize).collect();
        let c = pos2(r.center().x, r.min.y + 7.0);
        self.label(c, &shown, 7.0, self.ink(), Align2::CENTER_CENTER);
        if k < 1.0 {
            self.caret(pos2(c.x - 14.0, c.y - 4.0), 8.0);
        }
    }

    fn ms_add_element(&self) {
        let r = Rect::from_min_size(self.at(24.0, 8.0), vec2(128.0, 88.0));
        let title = self.reveal(0.1, 0.3);
        let labels = self.reveal(0.35, 0.55);
        let legend = self.reveal(0.6, 0.8);
        let plot = Rect::from_min_max(pos2(r.min.x, r.min.y + 14.0), pos2(r.max.x, r.max.y - 12.0));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.ms_bars(plot, 3, 2, labels > 0.0, true, 1.0);
        if title > 0.0 {
            self.ms_title_text(r, "المبيعات", title);
        }
        if legend > 0.0 {
            self.ms_legend(pos2(r.center().x, r.max.y - 6.0), 2, false);
        }
        // A "+" badge marks each part as it is added.
        for (k, at) in [
            (title, pos2(r.max.x - 8.0, r.min.y + 7.0)),
            (labels, pos2(r.max.x - 8.0, plot.min.y + 12.0)),
            (legend, pos2(r.max.x - 8.0, r.max.y - 6.0)),
        ] {
            if k > 0.0 && k < 1.0 {
                self.p.circle_filled(at, 6.0, self.accent());
                self.label(at, "+", 8.0, Color32::WHITE, Align2::CENTER_CENTER);
            }
        }
    }

    fn ms_switch_row_column(&self) {
        self.vw_two(
            |d, r| {
                d.ms_bars(Rect::from_min_max(r.min, pos2(r.max.x, r.max.y - 12.0)), 3, 2, false, false, 1.0);
                d.ms_legend(pos2(r.center().x, r.max.y - 6.0), 2, false);
            },
            |d, r| {
                d.ms_bars(Rect::from_min_max(r.min, pos2(r.max.x, r.max.y - 12.0)), 2, 3, false, false, 1.0);
                d.ms_legend(pos2(r.center().x, r.max.y - 6.0), 3, false);
            },
        );
    }

    fn ms_select_data(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(84.0, 88.0));
        let (rows, cols) = (6, 2);
        let area = self.vw_sample(r, rows, cols);
        // The marquee grows down the number column.
        let k = self.reveal(0.1, 0.5);
        let first = self.cell(area, rows, cols, 1, 1);
        let last = self.cell(area, rows, cols, 5, 1);
        let sel = Rect::from_min_max(first.min, pos2(last.max.x, first.min.y + (last.max.y - first.min.y) * k.max(0.15)));
        self.selection(sel);
        self.dashed_rect(sel, self.accent());
        let arrow = self.reveal(0.5, 0.7);
        if arrow > 0.0 {
            self.arrow_right(self.at(92.0, 52.0), self.at(92.0 + 14.0 * arrow, 52.0));
        }
        let grow = self.reveal(0.6, 0.95);
        self.ms_bars(Rect::from_min_size(self.at(110.0, 20.0), vec2(62.0, 64.0)), 5, 1, false, false, grow);
    }

    fn ms_change_type(&self) {
        self.vw_two(|d, r| d.ms_bars(r, 5, 1, false, false, 1.0), |d, r| d.ms_line_chart(r));
    }

    fn ms_format_pane(&self) {
        let r = Rect::from_min_size(self.at(4.0, 8.0), vec2(96.0, 88.0));
        self.ms_bars(r, 3, 2, false, true, 1.0);
        self.handles(r.shrink(1.0));
        let pane = self.pane(true, 70.0, 0);
        let swatches = [self.accent(), Color32::from_rgb(0x10, 0x7C, 0x41), Color32::from_rgb(0xE8, 0x7D, 0x2B), Color32::from_rgb(0x88, 0x55, 0xC8)];
        for (i, c) in swatches.into_iter().enumerate() {
            let s = Rect::from_min_size(pos2(pane.min.x + 6.0 + i as f32 * 15.0, pane.min.y + 18.0), vec2(12.0, 12.0));
            self.p.rect_filled(s, 2.0, c);
            if i == 1 && self.reveal(0.3, 0.5) > 0.0 {
                self.p.rect_stroke(s.expand(1.5), 2.0, Stroke::new(1.5, self.ink()), egui::StrokeKind::Outside);
            }
        }
        // Two sliders; the first one moves.
        let k = self.reveal(0.5, 0.8);
        for (i, v) in [(0.3 + 0.4 * k), 0.6].into_iter().enumerate() {
            let y = pane.min.y + 44.0 + i as f32 * 18.0;
            self.line(pos2(pane.min.x + 8.0, y), pos2(pane.max.x - 8.0, y), self.t.border_strong, 2.0);
            self.p.circle_filled(pos2(pane.min.x + 8.0 + (pane.width() - 16.0) * v, y), 3.5, self.accent());
        }
    }

    fn ms_chart_title(&self) {
        let r = Rect::from_min_size(self.at(24.0, 8.0), vec2(128.0, 88.0));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.ms_bars(Rect::from_min_max(pos2(r.min.x, r.min.y + 14.0), r.max), 4, 1, false, false, 1.0);
        let k = self.reveal(0.15, 0.8);
        self.ms_title_text(r, &format!("مبيعات {}", vw_digits(2026)), k);
        self.p.rect_stroke(
            Rect::from_center_size(pos2(r.center().x, r.min.y + 7.0), vec2(56.0, 12.0)),
            1.0,
            self.stroke(self.accent()),
            egui::StrokeKind::Outside,
        );
    }

    fn ms_delete_chart(&self) {
        self.vw_two(
            |d, r| {
                let area = d.vw_sample(r, 6, 2);
                let chart = Rect::from_min_size(pos2(area.min.x + 12.0, area.min.y + 10.0), vec2(44.0, 36.0));
                d.ms_bars(chart, 4, 1, false, false, 1.0);
                d.handles(chart);
            },
            |d, r| {
                d.vw_sample(r, 6, 2);
                d.cross(pos2(r.max.x - 12.0, r.max.y - 12.0));
            },
        );
    }

    fn ms_chart_set(&self) {
        let r = Rect::from_min_size(self.at(24.0, 8.0), vec2(128.0, 88.0));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let grid = self.reveal(0.1, 0.3);
        let labels = self.reveal(0.4, 0.6);
        let legend = self.reveal(0.7, 0.9);
        let plot = Rect::from_min_max(r.min, pos2(r.max.x, r.max.y - 12.0));
        self.ms_bars(plot, 3, 2, labels > 0.0, grid > 0.0, 1.0);
        if legend > 0.0 {
            self.ms_legend(pos2(r.center().x, r.max.y - 6.0), 2, false);
        }
        // Three small switches at the right edge, turning on one after the other.
        for (i, k) in [grid, labels, legend].into_iter().enumerate() {
            let sw = Rect::from_min_size(self.at(156.0, 14.0 + i as f32 * 30.0), vec2(16.0, 9.0));
            self.p.rect_filled(sw, 4.5, if k >= 1.0 { self.accent() } else { self.t.border_strong });
            self.p.circle_filled(pos2(sw.min.x + 4.5 + 7.0 * k, sw.center().y), 3.5, Color32::WHITE);
        }
    }

    // ---- Automate ----

    fn ms_macros(&self) {
        let inner = self.dialog("Macros", 0);
        let names = ["ترتيب القائمة", "إضافة المجاميع", "تنسيق العنوان"];
        let pick = self.reveal(0.2, 0.5);
        for (i, n) in names.iter().enumerate() {
            let row = Rect::from_min_size(pos2(inner.min.x, inner.min.y + i as f32 * 12.0), vec2(inner.width() - 30.0, 11.0));
            let picked = i == 1 && pick > 0.0;
            if picked {
                self.p.rect_filled(row, 1.0, self.t.selection);
            }
            self.label(pos2(row.max.x - 3.0, row.center().y), n, 6.5, if picked { self.ink() } else { self.grey() }, Align2::RIGHT_CENTER);
        }
        // The Run button, pressed at the end.
        let run = self.reveal(0.6, 0.8);
        let b = Rect::from_min_size(pos2(inner.max.x - 26.0, inner.min.y), vec2(26.0, 16.0));
        self.p.rect_filled(b, 3.0, if run >= 1.0 { self.accent() } else { self.t.menu });
        self.p.rect_stroke(b, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let c = b.center();
        self.p.add(egui::Shape::convex_polygon(
            vec![c + vec2(-3.0, -4.0), c + vec2(4.0, 0.0), c + vec2(-3.0, 4.0)],
            if run >= 1.0 { Color32::WHITE } else { self.ink() },
            Stroke::NONE,
        ));
    }

    fn ms_command_palette(&self) {
        let box_r = Rect::from_min_size(self.at(20.0, 10.0), vec2(136.0, 16.0));
        self.p.rect_filled(box_r, 3.0, self.t.input);
        self.p.rect_stroke(box_r, 3.0, self.stroke(self.accent()), egui::StrokeKind::Inside);
        self.magnifier(pos2(box_r.min.x + 9.0, box_r.center().y - 2.0));
        let typed = "freeze";
        let k = self.reveal(0.1, 0.6);
        let n = (typed.len() as f32 * k).round() as usize;
        let shown: String = typed.chars().take(n).collect();
        self.label(pos2(box_r.min.x + 22.0, box_r.center().y), &shown, 8.0, self.ink(), Align2::LEFT_CENTER);
        self.caret(pos2(box_r.min.x + 23.0 + n as f32 * 4.6, box_r.center().y - 5.0), 10.0);
        // The list shrinks to what matches as the letters come.
        let all = ["Freeze Panes", "Freeze Top Row", "Freeze First Column", "Format Cells", "Fill Down"];
        let keep = if n >= 2 { 3 } else { 5 };
        for (i, item) in all.iter().take(keep).enumerate() {
            let y = box_r.max.y + 10.0 + i as f32 * 13.0;
            let row = Rect::from_min_size(pos2(box_r.min.x, y - 6.0), vec2(box_r.width(), 12.0));
            if i == 1 && k >= 1.0 {
                self.p.rect_filled(row, 2.0, self.t.selection);
            }
            self.label(pos2(box_r.min.x + 6.0, y), item, 7.0, if i < 3 { self.ink() } else { self.grey() }, Align2::LEFT_CENTER);
        }
    }

    fn ms_agents(&self) {
        // A message from a program on the left, the sheet it fills on the right.
        let term = Rect::from_min_size(self.at(4.0, 18.0), vec2(72.0, 68.0));
        self.p.rect_filled(term, 3.0, Color32::from_rgb(40, 44, 52));
        for (i, (w, c)) in
            [(0.5, self.accent()), (0.8, Color32::from_rgb(200, 200, 200)), (0.65, Color32::from_rgb(200, 200, 200)), (0.4, self.green())]
                .into_iter()
                .enumerate()
        {
            let k = self.reveal(0.05 + i as f32 * 0.1, 0.1 + i as f32 * 0.1);
            if k > 0.0 {
                let y = term.min.y + 10.0 + i as f32 * 11.0;
                self.line(pos2(term.min.x + 6.0, y), pos2(term.min.x + 6.0 + (term.width() - 12.0) * w * k, y), c, 2.0);
            }
        }
        self.label(pos2(term.min.x + 6.0, term.min.y + 58.0), "{ }", 8.0, Color32::from_rgb(200, 200, 200), Align2::LEFT_CENTER);
        let arrow = self.reveal(0.45, 0.6);
        if arrow > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * arrow, 52.0));
        }
        let r = Rect::from_min_size(self.at(100.0, 10.0), vec2(72.0, 84.0));
        let (rows, cols) = (6, 2);
        let area = self.vw_sheet(r, rows, cols);
        let fill = self.reveal(0.6, 0.95);
        for i in 0..rows {
            for j in 0..cols {
                let c = self.cell(area, rows, cols, i, j);
                if i == 0 {
                    self.vw_dash(c, self.ink());
                } else if j == 0 {
                    self.vw_dash(c, self.grey());
                } else if (i as f32) <= 1.0 + fill * (rows - 1) as f32 {
                    self.vw_num(c, 50 + (i * 23) as u32, self.accent());
                }
            }
        }
    }

    fn ms_journal(&self) {
        let r = Rect::from_min_size(self.at(16.0, 8.0), vec2(144.0, 88.0));
        self.p.rect_filled(r, 2.0, Color32::WHITE);
        self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 12.0)), 2.0, self.t.hover);
        self.label(pos2(r.min.x + 6.0, r.min.y + 6.0), "Journal", 7.0, self.ink(), Align2::LEFT_CENTER);
        let steps = ["cell.set  B2", "home.bold", "data.sort  A:C", "insert.chart", "view.freezeTopRow"];
        for (i, s) in steps.iter().enumerate() {
            let k = self.reveal(0.05 + i as f32 * 0.15, 0.15 + i as f32 * 0.15);
            if k <= 0.0 {
                continue;
            }
            let y = r.min.y + 22.0 + i as f32 * 13.0;
            self.label(pos2(r.min.x + 6.0, y), &(i + 1).to_string(), 6.5, self.grey(), Align2::LEFT_CENTER);
            self.label(pos2(r.min.x + 16.0, y), s, 7.0, self.ink(), Align2::LEFT_CENTER);
            self.p.circle_filled(pos2(r.max.x - 10.0, y), 3.0, self.green());
        }
        self.label(pos2(r.max.x - 12.0, r.max.y - 8.0), "↻", 11.0, self.accent(), Align2::CENTER_CENTER);
    }

    fn ms_about(&self) {
        let inner = self.dialog("About GridCraft", 0);
        // The app's mark: a little grid tile.
        let tile = Rect::from_min_size(inner.min, vec2(30.0, 30.0));
        self.p.rect_filled(tile, 4.0, self.accent());
        self.grid(tile.shrink(6.0), 3, 3, Color32::WHITE);
        let k = self.reveal(0.2, 0.5);
        let text = Rect::from_min_max(pos2(tile.max.x + 8.0, inner.min.y), inner.max);
        self.line(pos2(text.min.x, text.min.y + 5.0), pos2(text.min.x + text.width() * 0.7 * k.max(0.2), text.min.y + 5.0), self.ink(), 2.5);
        self.text_lines(
            Rect::from_min_max(pos2(text.min.x, text.min.y + 10.0), text.max),
            false,
            &[text.width() * 0.9, text.width() * 0.6],
            8.0,
            self.grey(),
        );
        if self.reveal(0.5, 0.7) > 0.0 {
            self.badge(pos2(text.min.x + 20.0, inner.max.y - 10.0), "1.0", self.accent());
        }
        // The contributors: a row of little circles.
        for i in 0..4 {
            if self.reveal(0.6 + i as f32 * 0.08, 0.68 + i as f32 * 0.08) > 0.0 {
                self.p.circle_filled(pos2(text.min.x + 48.0 + i as f32 * 10.0, inner.max.y - 10.0), 4.0, self.ms_series_colour(i));
            }
        }
    }

    // ---- Conditional Formatting ----

    fn ms_top_bottom(&self) {
        let r = Rect::from_min_size(self.at(30.0, 6.0), vec2(116.0, 92.0));
        let (rows, cols) = (7, 2);
        let area = self.vw_sheet(r, rows, cols);
        let values: [u32; 6] = [120, 340, 85, 290, 60, 410];
        let names = ["أحمد", "سارة", "علي", "نور", "ليث", "هدى"];
        // The three biggest light up, biggest first.
        let mut order: Vec<usize> = (0..values.len()).collect();
        order.sort_by_key(|i| std::cmp::Reverse(values.get(*i).copied().unwrap_or(0)));
        for (rank, idx) in order.iter().take(3).enumerate() {
            let k = self.reveal(0.15 + rank as f32 * 0.2, 0.3 + rank as f32 * 0.2);
            if k > 0.0 {
                let c = self.cell(area, rows, cols, idx + 1, 1);
                self.p.rect_filled(c, 0.0, Color32::from_rgb(255, 199, 206).linear_multiply(k));
            }
        }
        for i in 0..rows {
            let name_c = self.cell(area, rows, cols, i, 0);
            let val_c = self.cell(area, rows, cols, i, 1);
            if i == 0 {
                self.vw_dash(name_c, self.ink());
                self.vw_dash(val_c, self.ink());
                continue;
            }
            if let (Some(n), Some(v)) = (names.get(i - 1), values.get(i - 1)) {
                self.vw_text(name_c, n, self.grey(), false);
                let top = order.iter().take(3).any(|x| x + 1 == i);
                self.vw_num(val_c, *v, if top { Color32::from_rgb(156, 0, 6) } else { self.grey() });
            }
        }
        self.grid(area, rows, cols, self.t.border);
        self.badge(pos2(r.max.x + 12.0, r.min.y + 14.0), &vw_digits(10), self.red());
    }

    fn ms_clear_rules(&self) {
        self.vw_two(
            |d, r| {
                let (rows, cols) = (5, 2);
                let area = d.vw_sample(r, rows, cols);
                let fills = [
                    Color32::from_rgb(255, 199, 206),
                    Color32::from_rgb(198, 239, 206),
                    Color32::from_rgb(255, 235, 156),
                    Color32::from_rgb(255, 199, 206),
                ];
                for (i, f) in fills.into_iter().enumerate() {
                    let c = d.cell(area, rows, cols, i + 1, 1);
                    d.p.rect_filled(c, 0.0, f);
                    d.vw_num(c, 60 + (i * 47) as u32, d.ink());
                }
                d.grid(area, rows, cols, d.t.border);
            },
            |d, r| {
                d.vw_sample(r, 5, 2);
                d.check(pos2(r.max.x - 12.0, r.max.y - 12.0));
            },
        );
    }
}
