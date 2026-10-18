//! Craft Exchange (`.craftx`): the JSON hand-off format shared by WordCraft AR, DeckCraft AR and
//! GridCraft AR (spec: `docs/ar-exchange.md`). A sheet goes out as one `table` block with its
//! Arabic typography — direction, alignment, kashida level — and a document comes in as one
//! sheet per `table` block plus a "Text" sheet for everything else.
//!
//! Everything read from a file is hostile: sizes are capped, every field is optional, unknown
//! kinds and fields are ignored, and nothing here panics on odd input.

use gridcraft_core::{CellRef, RangeRef, Value};
use gridcraft_model::style::{Color, HAlign, KashidaLevel, PatternType, Underline};
use gridcraft_model::{NumFmt, Sheet, Style, StyleId, Workbook};
use serde_json::{Map, Value as Json, json};

use crate::{EngineError, Result};

/// A sheet index into `Workbook::sheets`.
pub type SheetIdx = usize;
/// A cell range of a sheet.
pub type Range = RangeRef;

/// The file extension (without the dot).
pub const EXTENSION: &str = "craftx";
/// The `from` field GridCraft AR writes.
pub const FROM: &str = "gridcraft-ar";
/// The format version written, and the one read (any other number is read all the same: a
/// reader ignores what it doesn't know).
pub const VERSION: u64 = 1;
/// Most rows one `table` block puts on a sheet, and most columns; the rest of a hostile file is
/// dropped rather than allocated.
pub const MAX_TABLE_ROWS: usize = 10_000;
pub const MAX_TABLE_COLS: usize = 256;
/// Most rows the "Text" sheet takes.
pub const MAX_TEXT_ROWS: usize = 100_000;
/// Longest text one cell takes (Excel's cell limit).
const MAX_CELL_CHARS: usize = 32_767;
/// Most `table` sheets one file makes.
const MAX_SHEETS: usize = 255;
/// The sheet every non-table block lands on.
pub const TEXT_SHEET: &str = "Text";
/// Column widths (points) a table's `widths` are spread over when read.
const TABLE_WIDTH_PT: f32 = 640.0;

// ---------------------------------------------------------------- writing

/// One `table` block of `range` (the used range when `None`) of sheet `sheet`, preceded by a
/// level-1 heading with the sheet's name, as a `.craftx` document titled `title`.
pub fn write(wb: &Workbook, sheet: SheetIdx, range: Option<Range>, title: &str) -> Result<String> {
    let sh = wb.sheet(sheet).ok_or_else(|| EngineError::Other("no such sheet".into()))?;
    let rtl = sh.right_to_left;
    let heading = json!({
        "kind": "heading",
        "level": 1,
        "text": sh.name,
        "runs": [{"text": sh.name, "bold": true}],
        "para": {"bidi": rtl},
    });
    let table = table_block(wb, sh, range.or_else(|| sh.used_range()));
    let normal = wb.styles.get(StyleId::DEFAULT);
    let doc = json!({
        "craftx": VERSION,
        "from": FROM,
        "title": title,
        "rtl": rtl,
        "font": normal.font.name,
        "blocks": [heading, table],
    });
    serde_json::to_string_pretty(&doc).map_err(|e| EngineError::Other(e.to_string()))
}

/// The `table` block of `range`: visible rows and columns, cells as displayed.
fn table_block(wb: &Workbook, sh: &Sheet, range: Option<RangeRef>) -> Json {
    let rtl = sh.right_to_left;
    let Some(r) = range else {
        return json!({"kind": "table", "rtl": rtl, "header": false, "widths": [], "rows": []});
    };
    // A range larger than the caps is cut, never allocated whole.
    let end_row = r.start.row.saturating_add(MAX_TABLE_ROWS as u32 - 1).min(r.end.row);
    let end_col = r.start.col.saturating_add(MAX_TABLE_COLS as u32 - 1).min(r.end.col);
    let cols: Vec<u32> = (r.start.col..=end_col).filter(|&c| !sh.is_col_hidden(c)).collect();
    let mut rows = Vec::new();
    for row in r.start.row..=end_row {
        if sh.is_row_hidden(row) {
            continue;
        }
        let cells: Vec<Json> = cols.iter().map(|&col| cell_json(wb, sh, CellRef::new(row, col))).collect();
        rows.push(Json::Array(cells));
    }
    // The first visible row is a header when any of its cells is bold or filled.
    let first_row = (r.start.row..=end_row).find(|&row| !sh.is_row_hidden(row));
    let header = first_row.is_some_and(|row| {
        cols.iter().any(|&col| {
            let st = wb.styles.get(sh.style_id(CellRef::new(row, col)));
            st.font.bold || st.fill.pattern != PatternType::None
        })
    });
    let raw: Vec<f32> = cols.iter().map(|&c| sh.col_width(c).max(0.0)).collect();
    let sum: f32 = raw.iter().sum();
    let widths: Vec<Json> = raw
        .iter()
        .map(|w| {
            let share = if sum > 0.0 { w / sum } else { 1.0 / raw.len().max(1) as f32 };
            json!(((share as f64) * 10_000.0).round() / 10_000.0)
        })
        .collect();
    json!({"kind": "table", "rtl": rtl, "header": header, "widths": widths, "rows": rows})
}

/// A `Cell`: the displayed text, one run carrying the cell's font, its alignment and paragraph.
fn cell_json(wb: &Workbook, sh: &Sheet, c: CellRef) -> Json {
    let st = wb.styles.get(sh.style_id(c));
    let text = crate::display::value_text(wb, sh, c);
    let mut cell = Map::new();
    cell.insert("text".into(), json!(text));
    if !text.is_empty() {
        cell.insert("runs".into(), json!([run_json(wb, st, &text)]));
    }
    if let Some(a) = align_name(st.align.h, sh.right_to_left) {
        cell.insert("align".into(), json!(a));
    }
    let mut para = Map::new();
    para.insert("bidi".into(), json!(sh.right_to_left));
    if let Some(a) = align_name(st.align.h, sh.right_to_left) {
        para.insert("align".into(), json!(a));
    }
    if st.align.kashida != KashidaLevel::Off {
        para.insert("kashida".into(), json!(kashida_name(st.align.kashida)));
    }
    if st.align.indent > 0 {
        para.insert("indent_pt".into(), json!(st.align.indent as f64 * 9.0));
    }
    cell.insert("para".into(), Json::Object(para));
    Json::Object(cell)
}

fn run_json(wb: &Workbook, st: &Style, text: &str) -> Json {
    let mut run = Map::new();
    run.insert("text".into(), json!(text));
    if st.font.bold {
        run.insert("bold".into(), json!(true));
    }
    if st.font.italic {
        run.insert("italic".into(), json!(true));
    }
    if st.font.underline != Underline::None {
        run.insert("underline".into(), json!(true));
    }
    run.insert("size_pt".into(), json!(st.font.size));
    if let Some(hex) = st.font.color.hex(&wb.theme) {
        run.insert("color".into(), json!(hex));
    }
    run.insert("font".into(), json!(st.font.name));
    Json::Object(run)
}

/// The spec's logical alignment of a horizontal alignment: `start` is the right edge of a
/// right-to-left sheet. General and Fill have no name (the reader's default).
fn align_name(h: HAlign, rtl: bool) -> Option<&'static str> {
    Some(match h {
        HAlign::General | HAlign::Fill => return None,
        HAlign::Left => {
            if rtl {
                "end"
            } else {
                "start"
            }
        }
        HAlign::Right => {
            if rtl {
                "start"
            } else {
                "end"
            }
        }
        HAlign::Center | HAlign::CenterAcross => "center",
        HAlign::Justify => "justify",
        HAlign::Distributed => "distribute",
    })
}

fn kashida_name(k: KashidaLevel) -> &'static str {
    match k {
        KashidaLevel::Off => "none",
        KashidaLevel::Low => "low",
        KashidaLevel::Medium => "medium",
        KashidaLevel::High => "high",
    }
}

// ---------------------------------------------------------------- reading

/// A workbook from a `.craftx` document: one sheet per `table` block (named by the heading just
/// before it, else "Table n"), and the other text blocks as rows of column A of a "Text" sheet.
pub fn read(json: &str) -> Result<Workbook> {
    let doc: Json = serde_json::from_str(json).map_err(|e| EngineError::Other(format!("not a Craft exchange file: {e}")))?;
    let Some(obj) = doc.as_object() else {
        return Err(EngineError::Other("not a Craft exchange file: the document is not a JSON object".into()));
    };
    let Some(blocks) = obj.get("blocks").and_then(Json::as_array) else {
        return Err(EngineError::Other("not a Craft exchange file: no `blocks`".into()));
    };
    let rtl = obj.get("rtl").and_then(Json::as_bool).unwrap_or(false);
    let mut wb = Workbook::new();
    wb.sheets.clear();
    if let Some(font) = obj.get("font").and_then(Json::as_str).map(str::trim).filter(|f| !f.is_empty() && f.chars().count() <= 64) {
        let mut normal = wb.styles.get(StyleId::DEFAULT).clone();
        normal.font.name = font.to_string();
        wb.styles = gridcraft_model::StyleTable::with_default(normal);
    }
    let mut text = TextSheet { sheet: None, row: 0, rtl };
    let mut tables = 0usize;
    let mut heading_for_table: Option<String> = None;
    for (i, block) in blocks.iter().enumerate() {
        let kind = block.get("kind").and_then(Json::as_str).unwrap_or("");
        let next_is_table = blocks.get(i + 1).and_then(|b| b.get("kind")).and_then(Json::as_str) == Some("table");
        match kind {
            "heading" => {
                let t = block_text(block);
                if next_is_table {
                    // The heading names the table's sheet; it is not repeated on the Text sheet.
                    heading_for_table = Some(t);
                } else {
                    text.push(&mut wb, &t, para_of(block), true, false);
                }
            }
            "paragraph" => text.push(&mut wb, &block_text(block), para_of(block), false, false),
            "list" => {
                let ordered = block.get("ordered").and_then(Json::as_bool).unwrap_or(false);
                for (n, item) in block.get("items").and_then(Json::as_array).into_iter().flatten().take(MAX_TEXT_ROWS).enumerate() {
                    let prefix = if ordered { format!("{}. ", n + 1) } else { "• ".to_string() };
                    text.push(&mut wb, &format!("{prefix}{}", block_text(item)), para_of(item), false, false);
                }
            }
            "slide" => {
                let title = block.get("title").and_then(Json::as_str).unwrap_or("");
                text.push(&mut wb, title, None, true, false);
                if let Some(notes) = block.get("notes").and_then(Json::as_str).filter(|n| !n.trim().is_empty()) {
                    text.push(&mut wb, notes, None, false, true);
                }
            }
            "table" => {
                if wb.sheets.len() >= MAX_SHEETS {
                    continue;
                }
                tables += 1;
                let name = heading_for_table.take().filter(|h| !h.trim().is_empty()).unwrap_or_else(|| format!("Table {tables}"));
                let sheet = table_sheet(&mut wb, block, &name, rtl);
                wb.sheets.push(std::sync::Arc::new(sheet));
            }
            // Images, page breaks and kinds from a later version: nothing for a grid.
            _ => {}
        }
    }
    if let Some(mut sheet) = text.sheet.take() {
        // A table may have taken the name "Text" already.
        sheet.name = unique_sheet_name(&wb, TEXT_SHEET);
        wb.sheets.push(std::sync::Arc::new(sheet));
    }
    if wb.sheets.is_empty() {
        let mut sh = Sheet::new("Sheet1");
        sh.right_to_left = rtl;
        wb.sheets.push(std::sync::Arc::new(sh));
    }
    wb.active_sheet = 0;
    Ok(wb)
}

/// The "Text" sheet while it is being filled: created by the first block that needs it.
struct TextSheet {
    sheet: Option<Sheet>,
    row: usize,
    rtl: bool,
}

impl TextSheet {
    fn push(&mut self, wb: &mut Workbook, text: &str, para: Option<&Json>, bold: bool, italic: bool) {
        if self.row >= MAX_TEXT_ROWS {
            return;
        }
        let rtl = self.rtl;
        let sheet = self.sheet.get_or_insert_with(|| {
            let mut sh = Sheet::new(TEXT_SHEET);
            sh.right_to_left = rtl;
            sh.cols.insert(0, gridcraft_model::LineInfo { size: Some(TABLE_WIDTH_PT), ..Default::default() });
            sh
        });
        let text: String = text.chars().take(MAX_CELL_CHARS).collect();
        let mut st = wb.styles.get(StyleId::DEFAULT).clone();
        st.font.bold = bold;
        st.font.italic = italic;
        apply_para(&mut st, para, None, rtl);
        let style = wb.styles.intern(st);
        let c = CellRef::new(self.row as u32, 0);
        sheet.set_cell(c, gridcraft_model::Cell { value: Value::text(text), formula: None, style });
        self.row += 1;
    }
}

/// A sheet named `name` (made valid and unique in `wb`) from a `table` block.
fn table_sheet(wb: &mut Workbook, block: &Json, name: &str, doc_rtl: bool) -> Sheet {
    let rtl = block.get("rtl").and_then(Json::as_bool).unwrap_or(doc_rtl);
    let header = block.get("header").and_then(Json::as_bool).unwrap_or(false);
    let mut sh = Sheet::new(unique_sheet_name(wb, name));
    sh.right_to_left = rtl;
    let rows = block.get("rows").and_then(Json::as_array);
    let mut ncols = 0usize;
    for (ri, row) in rows.into_iter().flatten().take(MAX_TABLE_ROWS).enumerate() {
        let Some(cells) = row.as_array() else { continue };
        for (ci, cell) in cells.iter().take(MAX_TABLE_COLS).enumerate() {
            ncols = ncols.max(ci + 1);
            let text: String = block_text(cell).chars().take(MAX_CELL_CHARS).collect();
            let mut st = wb.styles.get(StyleId::DEFAULT).clone();
            if let Some(run) = first_run(cell) {
                apply_run(&mut st, run);
            }
            if header && ri == 0 {
                st.font.bold = true;
            }
            let align = cell.get("align").and_then(Json::as_str).or_else(|| cell.get("para").and_then(|p| p.get("align")).and_then(Json::as_str));
            apply_para(&mut st, cell.get("para"), align, rtl);
            let parsed = gridcraft_core::parse::parse_input(&text, wb.date_system);
            if let Some(code) = parsed.format {
                st.num_fmt = NumFmt::new(code);
            }
            let style = wb.styles.intern(st);
            let value = if text.is_empty() { Value::Empty } else { parsed.value };
            sh.set_cell(CellRef::new(ri as u32, ci as u32), gridcraft_model::Cell { value, formula: None, style });
        }
    }
    // Column widths from the table's shares (finite, positive; the rest keep the default).
    let widths: Vec<f64> =
        block.get("widths").and_then(Json::as_array).into_iter().flatten().take(MAX_TABLE_COLS).map(|w| w.as_f64().unwrap_or(0.0)).collect();
    let sum: f64 = widths.iter().filter(|w| w.is_finite() && **w > 0.0).sum();
    if sum > 0.0 && widths.len() >= ncols.max(1) {
        for (ci, w) in widths.iter().enumerate() {
            if w.is_finite() && *w > 0.0 {
                let pt = ((w / sum) * TABLE_WIDTH_PT as f64).clamp(12.0, 1200.0) as f32;
                sh.cols.insert(ci as u32, gridcraft_model::LineInfo { size: Some(pt), ..Default::default() });
            }
        }
    }
    sh
}

/// The plain text of a block, cell or list item: its runs joined, else its `text`.
fn block_text(b: &Json) -> String {
    if let Some(runs) = b.get("runs").and_then(Json::as_array) {
        let joined: String = runs.iter().filter_map(|r| r.get("text").and_then(Json::as_str)).collect();
        if !joined.is_empty() || b.get("text").is_none() {
            return joined;
        }
    }
    b.get("text").and_then(Json::as_str).unwrap_or("").to_string()
}

fn para_of(b: &Json) -> Option<&Json> {
    b.get("para")
}

/// The run that sets a cell's font: the first one with text, else the first.
fn first_run(cell: &Json) -> Option<&Json> {
    let runs = cell.get("runs")?.as_array()?;
    runs.iter().find(|r| r.get("text").and_then(Json::as_str).is_some_and(|t| !t.is_empty())).or_else(|| runs.first())
}

fn apply_run(st: &mut Style, run: &Json) {
    if run.get("bold").and_then(Json::as_bool) == Some(true) {
        st.font.bold = true;
    }
    if run.get("italic").and_then(Json::as_bool) == Some(true) {
        st.font.italic = true;
    }
    if run.get("underline").and_then(Json::as_bool) == Some(true) {
        st.font.underline = Underline::Single;
    }
    if let Some(size) = run.get("size_pt").and_then(Json::as_f64).filter(|s| s.is_finite()) {
        st.font.size = (size as f32).clamp(1.0, 409.0);
    }
    if let Some(color) = run.get("color").and_then(Json::as_str).and_then(Color::from_hex) {
        st.font.color = color;
    }
    if let Some(font) = run.get("font").and_then(Json::as_str).map(str::trim).filter(|f| !f.is_empty() && f.chars().count() <= 64) {
        st.font.name = font.to_string();
    }
}

/// Alignment, kashida and indent of a paragraph; `align` overrides the paragraph's own.
fn apply_para(st: &mut Style, para: Option<&Json>, align: Option<&str>, rtl: bool) {
    let bidi = para.and_then(|p| p.get("bidi")).and_then(Json::as_bool).unwrap_or(rtl);
    let align = align.or_else(|| para.and_then(|p| p.get("align")).and_then(Json::as_str));
    if let Some(a) = align {
        st.align.h = match a.trim().to_ascii_lowercase().as_str() {
            "start" => {
                if bidi {
                    HAlign::Right
                } else {
                    HAlign::Left
                }
            }
            "end" => {
                if bidi {
                    HAlign::Left
                } else {
                    HAlign::Right
                }
            }
            "left" => HAlign::Left,
            "right" => HAlign::Right,
            "center" => HAlign::Center,
            "justify" => HAlign::Justify,
            "distribute" => HAlign::Distributed,
            _ => st.align.h,
        };
    }
    if let Some(k) = para.and_then(|p| p.get("kashida")).and_then(Json::as_str).and_then(KashidaLevel::parse) {
        st.align.kashida = k;
    }
    if let Some(indent) = para.and_then(|p| p.get("indent_pt")).and_then(Json::as_f64).filter(|v| v.is_finite() && *v > 0.0) {
        st.align.indent = (indent / 9.0).round().clamp(0.0, 15.0) as u8;
    }
}

/// `name` as Excel allows a sheet name (31 characters, none of `: \ / ? * [ ]`, no edge
/// apostrophes, not "History"), unique among `wb`'s sheets (`name (2)`, `name (3)`…).
fn unique_sheet_name(wb: &Workbook, name: &str) -> String {
    let clean: String = name.trim().chars().filter(|c| !matches!(c, ':' | '\\' | '/' | '?' | '*' | '[' | ']') && !c.is_control()).collect();
    let mut base: String = clean.trim_matches('\'').chars().take(31).collect();
    base = base.trim().to_string();
    if base.is_empty() || base.eq_ignore_ascii_case("History") {
        base = "Table".into();
    }
    if wb.check_sheet_name(&base, None).is_ok() {
        return base;
    }
    for n in 2..10_000u32 {
        let suffix = format!(" ({n})");
        let keep = 31usize.saturating_sub(suffix.chars().count());
        let candidate = format!("{}{suffix}", base.chars().take(keep).collect::<String>().trim_end());
        if wb.check_sheet_name(&candidate, None).is_ok() {
            return candidate;
        }
    }
    wb.next_sheet_name()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gridcraft_model::LineInfo;

    fn arabic_sheet() -> Workbook {
        let mut wb = Workbook::new();
        let bold = wb.styles.derive(StyleId::DEFAULT, |s| s.font.bold = true);
        let high = wb.styles.derive(StyleId::DEFAULT, |s| {
            s.align.h = HAlign::Justify;
            s.align.kashida = KashidaLevel::High;
        });
        let centered = wb.styles.derive(StyleId::DEFAULT, |s| s.align.h = HAlign::Center);
        let sh = wb.sheet_mut(0).unwrap();
        sh.name = "الرواتب".into();
        sh.right_to_left = true;
        let put = |sh: &mut Sheet, a: &str, v: Value, st: StyleId| {
            sh.set_cell(CellRef::parse(a).unwrap(), gridcraft_model::Cell { value: v, formula: None, style: st });
        };
        put(sh, "A1", Value::text("الاسم"), bold);
        put(sh, "B1", Value::text("الراتب"), bold);
        put(sh, "A2", Value::text("على الطبي مستندات"), high);
        put(sh, "B2", Value::Number(1250.0), centered);
        put(sh, "A3", Value::text("أحمد"), StyleId::DEFAULT);
        put(sh, "B3", Value::Number(900.0), StyleId::DEFAULT);
        sh.cols.insert(0, LineInfo { size: Some(120.0), ..Default::default() });
        sh.cols.insert(1, LineInfo { size: Some(40.0), ..Default::default() });
        wb
    }

    #[test]
    fn arabic_sheet_round_trips_texts_order_rtl_kashida_and_alignment() {
        let wb = arabic_sheet();
        let json = write(&wb, 0, None, "الرواتب").unwrap();
        let doc: Json = serde_json::from_str(&json).unwrap();
        assert_eq!(doc["craftx"], 1);
        assert_eq!(doc["from"], FROM);
        assert_eq!(doc["rtl"], true);
        assert_eq!(doc["blocks"][0]["kind"], "heading");
        assert_eq!(doc["blocks"][0]["text"], "الرواتب");
        let table = &doc["blocks"][1];
        assert_eq!(table["kind"], "table");
        assert_eq!(table["rtl"], true);
        assert_eq!(table["header"], true, "a bold first row is a header");
        assert_eq!(table["widths"], json!([0.75, 0.25]));
        assert_eq!(table["rows"][0][0]["runs"][0]["bold"], true);
        assert_eq!(table["rows"][1][0]["para"]["kashida"], "high");
        assert_eq!(table["rows"][1][0]["align"], "justify");
        assert_eq!(table["rows"][1][1]["align"], "center");
        assert_eq!(table["rows"][1][1]["text"], "1250");

        let back = read(&json).unwrap();
        assert_eq!(back.sheets.len(), 1, "the heading names the sheet instead of filling a Text sheet");
        let sh = back.sheet(0).unwrap();
        assert_eq!(sh.name, "الرواتب");
        assert!(sh.right_to_left);
        let text = |a: &str| crate::display::value_text(&back, sh, CellRef::parse(a).unwrap());
        for a in ["A1", "B1", "A2", "B2", "A3", "B3"] {
            assert_eq!(text(a), crate::display::value_text(&wb, wb.sheet(0).unwrap(), CellRef::parse(a).unwrap()), "{a}");
        }
        assert_eq!(sh.value(CellRef::parse("B2").unwrap()), Value::Number(1250.0), "numbers come back as numbers");
        let style = |a: &str| back.styles.get(sh.style_id(CellRef::parse(a).unwrap())).clone();
        assert!(style("A1").font.bold && style("B1").font.bold);
        assert!(!style("A3").font.bold);
        assert_eq!((style("A2").align.h, style("A2").align.kashida), (HAlign::Justify, KashidaLevel::High));
        assert_eq!(style("B2").align.h, HAlign::Center);
        assert_eq!(style("A3").align.kashida, KashidaLevel::Off);
        assert!(sh.col_width(0) > sh.col_width(1), "column shares are kept");
    }

    #[test]
    fn start_and_end_follow_the_sheet_direction() {
        let mut wb = Workbook::new();
        let left = wb.styles.derive(StyleId::DEFAULT, |s| s.align.h = HAlign::Left);
        let right = wb.styles.derive(StyleId::DEFAULT, |s| s.align.h = HAlign::Right);
        let sh = wb.sheet_mut(0).unwrap();
        sh.set_cell(CellRef::new(0, 0), gridcraft_model::Cell { value: Value::text("a"), formula: None, style: left });
        sh.set_cell(CellRef::new(0, 1), gridcraft_model::Cell { value: Value::text("b"), formula: None, style: right });
        let ltr: Json = serde_json::from_str(&write(&wb, 0, None, "t").unwrap()).unwrap();
        assert_eq!(
            (ltr["blocks"][1]["rows"][0][0]["align"].as_str(), ltr["blocks"][1]["rows"][0][1]["align"].as_str()),
            (Some("start"), Some("end"))
        );
        wb.sheet_mut(0).unwrap().right_to_left = true;
        let rtl: Json = serde_json::from_str(&write(&wb, 0, None, "t").unwrap()).unwrap();
        assert_eq!(
            (rtl["blocks"][1]["rows"][0][0]["align"].as_str(), rtl["blocks"][1]["rows"][0][1]["align"].as_str()),
            (Some("end"), Some("start"))
        );
        // Read back, a bidi `start` is the right edge.
        let back = read(&rtl.to_string()).unwrap();
        let sh = back.sheet(0).unwrap();
        assert_eq!(back.styles.get(sh.style_id(CellRef::new(0, 0))).align.h, HAlign::Left);
        assert_eq!(back.styles.get(sh.style_id(CellRef::new(0, 1))).align.h, HAlign::Right);
    }

    #[test]
    fn a_document_becomes_a_text_sheet_and_one_sheet_per_table() {
        let doc = json!({
            "craftx": 1, "from": "wordcraft-ar", "rtl": true,
            "blocks": [
                {"kind": "heading", "level": 1, "text": "واجبات المستوصف", "para": {"bidi": true}},
                {"kind": "paragraph", "runs": [{"text": "الفقرة "}, {"text": "الأولى", "bold": true}], "para": {"align": "justify", "kashida": "medium"}},
                {"kind": "list", "ordered": true, "items": [{"text": "أ"}, {"text": "ب", "level": 1}]},
                {"kind": "list", "items": [{"text": "ج"}]},
                {"kind": "slide", "title": "الشريحة", "notes": "ملاحظة"},
                {"kind": "heading", "level": 2, "text": "الجدول: الأول"},
                {"kind": "table", "header": true, "widths": [0.5, 0.5], "rows": [[{"text": "أ"}, {"text": "ب"}], [{"text": "1"}, {"text": "2"}]]},
                {"kind": "table", "rtl": false, "rows": [[{"text": "x"}]]},
                {"kind": "image", "name": "p.png", "png_base64": "AAAA"},
                {"kind": "page_break"},
                {"kind": "mystery", "text": "ignored"}
            ]
        });
        let wb = read(&doc.to_string()).unwrap();
        let names: Vec<&str> = wb.sheets.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["الجدول الأول", "Table 2", TEXT_SHEET]);
        let t1 = wb.sheet(0).unwrap();
        assert!(t1.right_to_left, "the document's direction");
        assert!(wb.styles.get(t1.style_id(CellRef::new(0, 0))).font.bold, "header row");
        assert_eq!(t1.value(CellRef::new(1, 1)), Value::Number(2.0));
        assert!(!wb.sheet(1).unwrap().right_to_left, "the table's own rtl wins");
        let text = wb.sheet(2).unwrap();
        assert!(text.right_to_left);
        let col_a: Vec<String> = (0..7).map(|r| crate::display::value_text(&wb, text, CellRef::new(r, 0))).collect();
        assert_eq!(col_a, ["واجبات المستوصف", "الفقرة الأولى", "1. أ", "2. ب", "• ج", "الشريحة", "ملاحظة"]);
        assert!(wb.styles.get(text.style_id(CellRef::new(0, 0))).font.bold, "headings are bold");
        let para = wb.styles.get(text.style_id(CellRef::new(1, 0)));
        assert_eq!((para.align.h, para.align.kashida), (HAlign::Justify, KashidaLevel::Medium));
        assert!(wb.styles.get(text.style_id(CellRef::new(5, 0))).font.bold, "a slide title is bold");
    }

    #[test]
    fn hostile_json_never_panics() {
        let hostile = r#"{"craftx":1,"blocks":[{"kind":"table","rows":[[]]},{"kind":"table","rows":[[{"text":"x"}],[]]},{"kind":"heading"}]}"#;
        let wb = read(hostile).unwrap();
        assert_eq!(wb.sheets.len(), 3);
        assert_eq!(wb.sheets[1].value(CellRef::new(0, 0)), Value::text("x"));
        for bad in [
            "",
            "nope",
            "[]",
            "{}",
            r#"{"blocks": 5}"#,
            r#"{"blocks": [5, null, "x", [], {"kind": 7}]}"#,
            r#"{"blocks": [{"kind":"table","rows":5}]}"#,
            r#"{"blocks": [{"kind":"table","rows":[5, [5], [{"runs": 5}], [{"runs": [5, {"text": 5}]}]]}]}"#,
            r#"{"blocks": [{"kind":"table","widths":[1e308, -1, "x", null],"rows":[[{"text":"a","align":9,"para":{"kashida":"lots","indent_pt":1e308}}]]}]}"#,
            r#"{"blocks": [{"kind":"list","items":5},{"kind":"list","items":[5]},{"kind":"slide"},{"kind":"heading","text":"History"},{"kind":"table"}]}"#,
            r#"{"blocks": [{"kind":"heading","text":"a:b/c\\d?e*f[g]'"},{"kind":"table"},{"kind":"heading","text":"a:b/c\\d?e*f[g]'"},{"kind":"table"}]}"#,
            r##"{"rtl": "yes", "font": 5, "blocks": [{"kind":"paragraph","runs":[{"text":"a","size_pt":1e9,"color":"#zz","font":""}]}]}"##,
        ] {
            let _ = read(bad);
        }
        // Rows and columns past the caps are dropped, not allocated.
        let wide: Vec<Json> = (0..300).map(|i| json!({"text": i.to_string()})).collect();
        let doc = json!({"blocks": [{"kind": "table", "rows": [wide]}]});
        let wb = read(&doc.to_string()).unwrap();
        assert_eq!(wb.sheets[0].used_range().map(|r| r.width()), Some(MAX_TABLE_COLS as u32));
        // A sheet name is unique and valid.
        let doc = json!({"blocks": [
            {"kind":"heading","text":"x"},{"kind":"table"},
            {"kind":"heading","text":"X"},{"kind":"table"},
            {"kind":"heading","text":"Text"},{"kind":"table"},
            {"kind":"paragraph","text":"p"}
        ]});
        let wb = read(&doc.to_string()).unwrap();
        let names: Vec<&str> = wb.sheets.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["x", "X (2)", "Text", "Text (2)"]);
        for (i, n) in names.iter().enumerate() {
            assert!(wb.check_sheet_name(n, Some(i)).is_ok(), "{n}");
        }
    }

    #[test]
    fn an_empty_sheet_writes_an_empty_table() {
        let wb = Workbook::new();
        let doc: Json = serde_json::from_str(&write(&wb, 0, None, "Book1").unwrap()).unwrap();
        assert_eq!(doc["blocks"][1]["rows"], json!([]));
        assert!(write(&wb, 3, None, "Book1").is_err());
        let back = read(&doc.to_string()).unwrap();
        assert_eq!(back.sheets.len(), 1);
        assert_eq!(back.sheets[0].name, "Sheet1");
    }
}
