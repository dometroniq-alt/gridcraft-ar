//! GridCraft AR — built-in developer systems, reachable from View › GridCraft AR (ported from
//! WordCraft AR's `dev_tools.rs`):
//!
//! - **UI Kit**: every entry of the registry ([`crate::ui_kit`]) with its thumbnail (drawn
//!   still), its key, its explanation in English and in Arabic, filterable — the place to check
//!   that a new button looks right and reads right before it ships.
//! - **Icons**: the whole icon set ([`crate::icons::ALL`]) at the ribbon's two sizes, 16 and
//!   30 pt, so weights can be compared side by side.
//! - **Self-test**: runs every registered engine command with empty params on a scratch workbook
//!   (never the open one) and lists what each answered and how long it took — the
//!   `every_command_survives_empty_params` test, inside the app, on this machine.
//! - **Checklist**: what is still missing — ribbon controls without a UI-kit entry, explanations
//!   without an Arabic translation — the same checks the tests make, readable without a build.
//!
//! The dialog is `uiKit` (`app.open_dialog("uiKit", json!({"tab": "Self-test"}))` opens a tab).

use egui::{Rect, Sense, Stroke, Ui, vec2};
use serde_json::{Value as Json, json};

use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::i18n::Language;
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// The dialog's tabs, in order.
pub const TABS: [&str; 4] = ["UI Kit", "Icons", "Self-test", "Checklist"];

/// Every ribbon control (tab function › key) with its icon, generated from the inventory files
/// `grid_a.tsv`, `grid_b.tsv` and `grid_c.tsv` (one row per button or menu item; a button is
/// keyed by its English tip, a menu item by `ui_kit::key(id, params)`). Regenerate it when the
/// ribbon gains a control.
pub const INVENTORY: &[(&str, &str, Option<Icon>)] = &[
    ("title_bar", "Home", Some(Icon::Home)),
    ("title_bar", "Save (⌘S)", Some(Icon::Save)),
    ("title_bar", "Undo list", Some(Icon::Chevron)),
    ("title_bar", "More commands", Some(Icon::More)),
    ("title_bar", "Search commands (⌘⇧Space)", Some(Icon::Search)),
    ("title_bar", "Share: export or save a copy", Some(Icon::Share)),
    ("title_bar", "Comments", Some(Icon::Comment)),
    ("title_bar", "Dark Mode", Some(Icon::Appearance)),
    ("home", "Paste (⌘V)", Some(Icon::Paste)),
    ("home", "Cut (⌘X)", Some(Icon::Cut)),
    ("home", "Copy (⌘C)", Some(Icon::Copy)),
    ("home", "Format Painter (double-click to keep it on)", Some(Icon::Brush)),
    ("home", "Increase Font Size", Some(Icon::Plus)),
    ("home", "Decrease Font Size", Some(Icon::Minus)),
    ("home", "Bold (⌘B)", Some(Icon::Bold)),
    ("home", "Italic (⌘I)", Some(Icon::Italic)),
    ("home", "Strikethrough", Some(Icon::Strike)),
    ("home", "Fill Color", Some(Icon::Fill)),
    ("home", "Font Color", Some(Icon::FontColor)),
    ("home", "Orientation", Some(Icon::Orientation)),
    ("home", "Wrap Text", Some(Icon::Wrap)),
    ("home", "Decrease Indent", Some(Icon::IndentDec)),
    ("home", "Increase Indent", Some(Icon::IndentInc)),
    ("home", "Percent Style", Some(Icon::Percent)),
    ("home", "Comma Style", Some(Icon::Comma)),
    ("home", "Increase Decimal", Some(Icon::DecInc)),
    ("home", "Decrease Decimal", Some(Icon::DecDec)),
    ("home", "Conditional Formatting", Some(Icon::CondFormat)),
    ("home", "Format as Table", Some(Icon::FormatTable)),
    ("home", "Cell Styles", Some(Icon::CellStyles)),
    ("home", "Insert", Some(Icon::Insert)),
    ("home", "Delete", Some(Icon::Delete)),
    ("home", "Format", Some(Icon::Format)),
    ("home", "Fill", Some(Icon::FillDown)),
    ("home", "Clear", Some(Icon::Eraser)),
    ("home", "Sort & Filter", Some(Icon::SortFilter)),
    ("home", "Find & Select", Some(Icon::Find)),
    ("home", "Send to WordCraft AR", Some(Icon::SendWord)),
    ("home", "Send to DeckCraft AR", Some(Icon::SendDeck)),
    ("home", "Receive from WordCraft / DeckCraft", Some(Icon::ReceiveCraft)),
    ("insert", "PivotTable", Some(Icon::PivotTable)),
    ("insert", "Table (⌘T)", Some(Icon::Table)),
    ("insert", "Insert a picture", Some(Icon::Picture)),
    ("insert", "Shapes", Some(Icon::Shapes)),
    ("insert", "Insert an icon", Some(Icon::Theme)),
    ("insert", "Recommended Charts", Some(Icon::Chart)),
    ("insert", "Insert a slicer to filter a table", Some(Icon::Filter)),
    ("insert", "Sparklines", Some(Icon::Sparkline)),
    ("insert", "Insert checkboxes in the selected cells", Some(Icon::Checkbox)),
    ("insert", "Insert Link (⌘K)", Some(Icon::Link)),
    ("insert", "New Comment", Some(Icon::Comment)),
    ("insert", "Text Box", Some(Icon::TextBox)),
    ("insert", "Header & Footer", Some(Icon::PageLayout)),
    ("insert", "Symbol", Some(Icon::Symbol)),
    ("draw", "Select and move ink and shapes", Some(Icon::Shapes)),
    ("draw", "Draw with ink", Some(Icon::Pen)),
    ("draw", "Erase ink strokes", Some(Icon::Eraser)),
    ("draw", "Convert the last ink stroke to a shape", Some(Icon::Shapes)),
    ("page_layout", "Themes", Some(Icon::Theme)),
    ("page_layout", "Margins", Some(Icon::Margins)),
    ("page_layout", "Page Layout|Orientation", Some(Icon::Orient)),
    ("page_layout", "Paper Size", Some(Icon::Paper)),
    ("page_layout", "Print Area", Some(Icon::PrintArea)),
    ("page_layout", "Page Breaks", Some(Icon::PageBreak)),
    ("page_layout", "Print Titles", Some(Icon::Sheet)),
    ("page_layout", "Sheet Right-to-Left", Some(Icon::SheetRtl)),
    ("formulas", "Insert Function (⇧F3)", Some(Icon::Function)),
    ("formulas", "AutoSum", Some(Icon::Sum)),
    ("formulas", "Name Manager", Some(Icon::Name)),
    ("formulas", "Define Name", Some(Icon::Name)),
    ("formulas", "Use in Formula", Some(Icon::Function)),
    ("formulas", "Create from Selection", Some(Icon::Table)),
    ("formulas", "Trace Precedents", Some(Icon::Trace)),
    ("formulas", "Trace Dependents", Some(Icon::Trace)),
    ("formulas", "Remove Arrows", Some(Icon::Close)),
    ("formulas", "Show Formulas (⌃`)", Some(Icon::Fx)),
    ("formulas", "Error Checking", Some(Icon::Validation)),
    ("formulas", "Evaluate Formula", Some(Icon::Calc)),
    ("formulas", "Watch Window", Some(Icon::Search)),
    ("formulas", "Calculation Options", Some(Icon::Calc)),
    ("formulas", "Calculate Now (F9)", Some(Icon::Calc)),
    ("formulas", "Calculate Sheet (⇧F9)", Some(Icon::Sheet)),
    ("data", "Import a text or CSV file", Some(Icon::Folder)),
    ("data", "Sort A to Z", Some(Icon::SortAsc)),
    ("data", "Sort Z to A", Some(Icon::SortDesc)),
    ("data", "Custom Sort", Some(Icon::SortFilter)),
    ("data", "Filter (⌘⇧F)", Some(Icon::Filter)),
    ("data", "Clear Filter", Some(Icon::Eraser)),
    ("data", "Reapply", Some(Icon::Redo)),
    ("data", "Text to Columns", Some(Icon::TextColumns)),
    ("data", "Flash Fill (⌘E)", Some(Icon::FillDown)),
    ("data", "Remove Duplicates", Some(Icon::Duplicates)),
    ("data", "Data Validation", Some(Icon::Validation)),
    ("data", "What-If Analysis", Some(Icon::Calc)),
    ("data", "Forecast Sheet", Some(Icon::ChartLine)),
    ("data", "Group", Some(Icon::Group)),
    ("data", "Ungroup", Some(Icon::Ungroup)),
    ("data", "Subtotal", Some(Icon::Subtotal)),
    ("data", "Show Detail", Some(Icon::Plus)),
    ("data", "Hide Detail", Some(Icon::Minus)),
    ("data", "Data Analysis", Some(Icon::ChartBar)),
    ("data", "Solver", Some(Icon::Function)),
    ("review", "Spelling (F7)", Some(Icon::Spell)),
    ("review", "Workbook Statistics", Some(Icon::Calc)),
    ("review", "Check Accessibility", Some(Icon::Validation)),
    ("review", "Delete Comment", Some(Icon::Delete)),
    ("review", "Comments pane", Some(Icon::Comment)),
    ("review", "Notes", Some(Icon::Note)),
    ("review", "Protect Workbook structure", Some(Icon::Book)),
    ("review", "Allow Edit Ranges", Some(Icon::Validation)),
    ("view", "Zoom…", Some(Icon::Zoom)),
    ("view", "Zoom to 100%", Some(Icon::Search)),
    ("view", "Zoom to Selection", Some(Icon::Zoom)),
    ("view", "Freeze Panes", Some(Icon::Freeze)),
    ("automate", "Macros and scripts in this workbook (Alt+F8)", Some(Icon::Script)),
    ("automate", "Search and run any command", Some(Icon::Script)),
    ("automate", "How agents drive GridCraft (MCP / control channel)", Some(Icon::Script)),
    ("automate", "Every command run in this session (replayable)", Some(Icon::Script)),
    ("automate", "Version, contributors and the AI models that helped", Some(Icon::Script)),
    ("table_design", "Convert to Range", Some(Icon::Table)),
    ("table_design", "Insert Slicer", Some(Icon::Filter)),
    ("table_design", "Table Styles", Some(Icon::FormatTable)),
    ("chart_design", "Add Chart Element", Some(Icon::Chart)),
    ("chart_design", "Switch Row/Column", Some(Icon::Redo)),
    ("chart_design", "Select Data", Some(Icon::Table)),
    ("chart_design", "Change Chart Type", Some(Icon::ChartBar)),
    ("chart_design", "Format Chart pane", Some(Icon::Settings)),
    ("chart_design", "Edit chart title", Some(Icon::TextBox)),
    ("chart_design", "Delete chart", Some(Icon::Delete)),
    ("home", "edit.paste", None),
    ("home", "edit.pasteSpecial:values", None),
    ("home", "edit.pasteSpecial:formulas", None),
    ("home", "edit.pasteSpecial:formats", None),
    ("home", "edit.pasteSpecial:all", None),
    ("home", "dialog:pasteSpecial", None),
    ("home", "home.underline:single", None),
    ("home", "home.underline:double", None),
    ("home", "home.orientation", None),
    ("home", "dialog:formatCells", None),
    ("home", "home.mergeCenter", None),
    ("home", "home.mergeAcross", None),
    ("home", "home.mergeCells", None),
    ("home", "home.unmergeCells", None),
    ("home", "home.numberFormat", None),
    ("home", "dialog:insertCells", None),
    ("home", "home.insertRows", None),
    ("home", "home.insertColumns", None),
    ("home", "home.insertSheet", None),
    ("home", "dialog:deleteCells", None),
    ("home", "home.deleteRows", None),
    ("home", "home.deleteColumns", None),
    ("home", "home.deleteSheet", None),
    ("home", "dialog:rowHeight", None),
    ("home", "home.autofitRowHeight", None),
    ("home", "dialog:columnWidth", None),
    ("home", "home.autofitColumnWidth", None),
    ("home", "dialog:defaultWidth", None),
    ("home", "home.hideRows", None),
    ("home", "home.hideColumns", None),
    ("home", "home.unhideRows", None),
    ("home", "home.unhideColumns", None),
    ("home", "sheet.hide", None),
    ("home", "sheet.unhide", None),
    ("home", "dialog:renameSheet", None),
    ("home", "dialog:moveSheet", None),
    ("home", "dialog:protectSheet", None),
    ("home", "home.lockCell", None),
    ("home", "edit.fillDown", None),
    ("home", "edit.fillRight", None),
    ("home", "edit.fillUp", None),
    ("home", "edit.fillLeft", None),
    ("home", "dialog:series", None),
    ("home", "edit.flashFill", None),
    ("home", "edit.clearAll", None),
    ("home", "edit.clearFormats", None),
    ("home", "edit.clearContents", None),
    ("home", "edit.clearComments", None),
    ("home", "edit.clearHyperlinks", None),
    ("home", "data.sortAscending", None),
    ("home", "data.sortDescending", None),
    ("home", "dialog:sort", None),
    ("home", "data.filter", None),
    ("home", "data.clearFilter", None),
    ("home", "data.reapply", None),
    ("home", "dialog:find", None),
    ("home", "dialog:goTo", None),
    ("home", "dialog:goToSpecial", None),
    ("home", "edit.goToSpecial:formulas", None),
    ("home", "edit.goToSpecial:notes", None),
    ("home", "edit.goToSpecial:conditionalFormats", None),
    ("home", "edit.goToSpecial:constants", None),
    ("home", "edit.goToSpecial:dataValidation", None),
    ("cf_menu", "home.conditionalFormat", None),
    ("cf_menu", "home.clearRules", None),
    ("page_layout", "pageLayout.margins:normal", None),
    ("page_layout", "pageLayout.margins:wide", None),
    ("page_layout", "pageLayout.margins:narrow", None),
    ("page_layout", "dialog:pageSetup", None),
    ("page_layout", "pageLayout.orientation", None),
    ("page_layout", "pageLayout.printArea", None),
    ("page_layout", "pageLayout.breaks", None),
    ("page_layout", "pageLayout.breaks:remove", None),
    ("formulas", "formulas.calculationOptions:automatic", None),
    ("formulas", "formulas.calculationOptions:automaticExceptTables", None),
    ("formulas", "formulas.calculationOptions:manual", None),
    ("data", "dialog:dataValidation", None),
    ("data", "data.circleInvalid", None),
    ("data", "data.validation", None),
    ("data", "dialog:goalSeek", None),
    ("data", "data.scenarioManager", None),
    ("data", "data.dataTable", None),
    ("review", "dialog:comment", None),
    ("review", "review.showNote", None),
    ("review", "review.showAllNotes", None),
    ("review", "review.convertToComments", None),
    ("view", "view.freezePanes", None),
    ("view", "view.freezeTopRow", None),
    ("view", "view.freezeFirstColumn", None),
    ("view", "view.unfreezePanes", None),
    ("table_design", "table.headerRow", None),
    ("table_design", "table.totalRow", None),
    ("table_design", "table.bandedRows", None),
    ("table_design", "table.firstColumn", None),
    ("table_design", "table.lastColumn", None),
    ("table_design", "table.bandedColumns", None),
    ("chart_design", "chart.set", None),
];

/// One command's run in the self-test.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Run {
    pub id: String,
    /// What the command answered: a tick with a glimpse of its result, the dialog it asked
    /// for, or its error message.
    pub answer: String,
    pub ok: bool,
    pub millis: f64,
}

/// The self-test's result.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Report {
    pub runs: Vec<Run>,
    pub sheets: usize,
}

impl Report {
    pub fn failed(&self) -> usize {
        self.runs.iter().filter(|r| !r.ok).count()
    }
    pub fn total_ms(&self) -> f64 {
        self.runs.iter().map(|r| r.millis).sum()
    }
}

/// Commands the self-test leaves out: they touch files on disk, close the scratch workbook, or
/// (Home › Send) would start WordCraft AR / DeckCraft AR on this machine.
const SKIPPED: &[&str] = &[
    "file.close",
    "file.open",
    "file.save",
    "file.saveAs",
    "file.exportCsv",
    "file.exportHtml",
    "insert.picture",
    "send.toWordCraft",
    "send.toDeckCraft",
    "send.importCraftx",
];

/// The scratch workbook the self-test works on: the sales sample, in a session of its own.
pub fn scratch_session() -> gridcraft_engine::Session {
    let mut s = gridcraft_engine::Session::new();
    let _ = s.execute("file.new", json!({"sample": "sales"}));
    s
}

/// Runs every registered engine command with empty params on a scratch workbook and reports
/// what each answered and how long it took. Dialogs the commands ask for are noted, not opened.
pub fn self_test() -> Report {
    let mut s = scratch_session();
    let mut runs = Vec::new();
    for spec in gridcraft_engine::command_specs() {
        if SKIPPED.contains(&spec.id) {
            continue;
        }
        let start = crate::now_ms();
        let result = s.execute(spec.id, json!({}));
        let millis = crate::now_ms() - start;
        let requests = s.take_ui_requests();
        let dialog = requests.iter().find_map(|q| match q {
            gridcraft_engine::UiRequest::Dialog(name, _) => Some(name.clone()),
            _ => None,
        });
        let (ok, answer) = match (result, dialog) {
            (Ok(_), Some(name)) => (true, format!("opens the {name} dialog")),
            (Ok(r), None) => (true, glimpse(&r)),
            (Err(e), _) => (false, crate::clean_error(&e.to_string())),
        };
        runs.push(Run { id: spec.id.to_string(), answer, ok, millis });
    }
    let sheets = s.active().map(|d| d.wb.sheets.len()).unwrap_or(0);
    Report { runs, sheets }
}

/// A short look at a command's result for the table (`✓` alone for an empty one).
fn glimpse(r: &Json) -> String {
    let text = match r {
        Json::Null => String::new(),
        Json::Object(m) if m.is_empty() => String::new(),
        other => other.to_string(),
    };
    if text.is_empty() {
        return "✓".into();
    }
    let short: String = text.chars().take(72).collect();
    if short.chars().count() < text.chars().count() { format!("✓ {short}…") } else { format!("✓ {short}") }
}

/// Ribbon controls (tab › key) of the inventory that have no UI-kit entry yet.
pub fn unexplained() -> Vec<String> {
    INVENTORY.iter().filter(|(_, key, _)| crate::ui_kit::entry(key).is_none()).map(|(tab, key, _)| format!("{tab} › {key}")).collect()
}

/// UI-kit entries whose explanation the Arabic catalog doesn't translate.
pub fn untranslated() -> Vec<&'static str> {
    crate::ui_kit::entries().filter(|e| Language::Ar.tr(e.explain) == e.explain).map(|e| e.key).collect()
}

/// The icon the inventory gives a UI-kit key (a menu item has none).
fn icon_for(key: &str) -> Option<Icon> {
    INVENTORY.iter().find(|(_, k, _)| *k == key).and_then(|(_, _, i)| *i)
}

/// The dialog body: `d.tab` is the open tab, `d.search` the filter, `d.result` the last
/// self-test report.
pub fn body(app: &mut SheetApp, ui: &mut Ui, d: &mut Dialog, open: &mut bool) {
    let lang = app.ui.language;
    ui.set_width(760.0);
    ui.horizontal(|ui| {
        for name in TABS {
            if ui.selectable_label(d.tab == name, lang.tr(name)).clicked() {
                d.tab = name.to_string();
            }
        }
        ui.add_space(12.0);
        if d.tab == "UI Kit" || d.tab == "Icons" {
            ui.label(lang.tr("Filter:"));
            ui.add(egui::TextEdit::singleline(&mut d.search).desired_width(180.0));
        }
    });
    ui.separator();
    let t = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical().max_height(480.0).show(ui, |ui| match d.tab.as_str() {
        "Icons" => icon_sheet(ui, &t, lang, &d.search),
        "Self-test" => self_test_ui(ui, lang, &mut d.result),
        "Checklist" => checklist(app, ui, &t, lang),
        _ => kit_gallery(ui, &t, lang, &d.search),
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(lang.tr("Add a control: one Entry in ui_kit/<tab>.rs, one row in the inventory, one Arabic line in i18n/ar.rs."))
                .small()
                .weak(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(lang.tr("Close")).clicked() {
                *open = false;
            }
        });
    });
}

fn matches(filter: &str, hay: &[&str]) -> bool {
    let f = filter.trim().to_lowercase();
    f.is_empty() || hay.iter().any(|h| h.to_lowercase().contains(&f))
}

fn count_line(ui: &mut Ui, lang: Language, shown: usize, total: usize, what: &str) {
    ui.label(egui::RichText::new(format!("{shown} / {total} {}", lang.tr(what))).small().weak());
}

/// Every registry entry: thumbnail, icon, key, explanation, Arabic.
fn kit_gallery(ui: &mut Ui, t: &Tokens, lang: Language, filter: &str) {
    let palette = crate::ui_kit::Palette::from_tokens(t);
    let mut shown = 0;
    for e in crate::ui_kit::entries() {
        let arabic = crate::ui_kit::explain(Language::Ar, e.explain);
        if !matches(filter, &[e.key, e.explain, arabic]) {
            continue;
        }
        shown += 1;
        ui.horizontal_top(|ui| {
            let (r, _) = ui.allocate_exact_size(crate::ui_kit::SIZE, Sense::hover());
            ui.painter().rect_filled(r, 4.0, t.window);
            ui.painter().rect_stroke(r, 4.0, Stroke::new(1.0, t.separator), egui::StrokeKind::Inside);
            (e.draw)(&crate::ui_kit::Draw::new(ui.painter(), r, &palette));
            ui.add_space(8.0);
            let (ir, _) = ui.allocate_exact_size(vec2(32.0, 32.0), Sense::hover());
            if let Some(icon) = icon_for(e.key) {
                icons::paint(ui.painter(), Rect::from_center_size(ir.center(), vec2(30.0, 30.0)), icon, t.text);
            }
            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.set_max_width(440.0);
                ui.label(egui::RichText::new(crate::ui_kit::artwork(e.key)).font(theme::ui_bold(12.0)));
                crate::rtl_text::paragraph(ui, crate::ui_kit::explain(lang, e.explain), theme::ui_font(11.5), t.text, 440.0);
                if arabic != e.explain && lang != Language::Ar {
                    crate::rtl_text::paragraph(ui, arabic, theme::ui_font(11.5), t.text_dim, 440.0);
                } else if arabic == e.explain {
                    ui.label(egui::RichText::new(lang.tr("No Arabic translation yet")).small().color(t.danger));
                }
            });
        });
        ui.add_space(6.0);
    }
    count_line(ui, lang, shown, crate::ui_kit::entries().count(), "entries");
}

/// The icon set at 16 and 30 pt, six per row.
fn icon_sheet(ui: &mut Ui, t: &Tokens, lang: Language, filter: &str) {
    let named: Vec<(String, Icon)> = icons::ALL.iter().map(|i| (format!("{i:?}"), *i)).filter(|(n, _)| matches(filter, &[n])).collect();
    for row in named.chunks(6) {
        ui.horizontal(|ui| {
            for (name, icon) in row {
                let (r, resp) = ui.allocate_exact_size(vec2(122.0, 44.0), Sense::hover());
                if resp.hovered() {
                    ui.painter().rect_filled(r, 3.0, t.hover);
                }
                let small = Rect::from_center_size(r.min + vec2(14.0, 22.0), vec2(16.0, 16.0));
                let big = Rect::from_center_size(r.min + vec2(44.0, 22.0), vec2(30.0, 30.0));
                icons::paint(ui.painter(), small, *icon, t.text);
                icons::paint(ui.painter(), big, *icon, t.text);
                ui.painter().text(
                    r.min + vec2(66.0, 22.0),
                    egui::Align2::LEFT_CENTER,
                    crate::ui_kit::artwork(name),
                    egui::FontId::proportional(9.5),
                    t.text,
                );
            }
        });
    }
    count_line(ui, lang, named.len(), icons::ALL.len(), "icons");
}

fn self_test_ui(ui: &mut Ui, lang: Language, result: &mut Option<Json>) {
    ui.label(lang.tr("Runs every engine command with empty parameters on a scratch copy of the sales sample (your workbook is not touched) and lists what each one answered."));
    if ui.button(lang.tr("Run the self-test")).clicked() {
        *result = serde_json::to_value(self_test()).ok();
    }
    let Some(rep) = result.as_ref().and_then(|r| serde_json::from_value::<Report>(r.clone()).ok()) else { return };
    let failed = rep.failed();
    ui.label(
        egui::RichText::new(format!(
            "{} {}; {:.0} ms; {} {}; {} {}",
            rep.runs.len(),
            lang.tr("commands ran"),
            rep.total_ms(),
            failed,
            lang.tr("answered with an error (a feature that needs a selection or a parameter)"),
            rep.sheets,
            lang.tr("sheet(s) in the scratch workbook")
        ))
        .font(theme::ui_bold(12.0)),
    );
    egui::Grid::new("gridcraft_ar_self_test").striped(true).min_col_width(60.0).show(ui, |ui| {
        for h in ["Command", "ms", "Answer"] {
            ui.label(egui::RichText::new(lang.tr(h)).font(theme::ui_bold(11.5)));
        }
        ui.end_row();
        for r in &rep.runs {
            ui.label(crate::ui_kit::artwork(&r.id));
            ui.label(format!("{:.1}", r.millis));
            let color = if r.ok { icons::GREEN } else { icons::ORANGE };
            ui.add(egui::Label::new(egui::RichText::new(crate::ui_kit::artwork(&r.answer)).color(color)).truncate());
            ui.end_row();
        }
    });
}

fn checklist(app: &SheetApp, ui: &mut Ui, t: &Tokens, lang: Language) {
    let missing = unexplained();
    let untranslated = untranslated();
    let line = |ui: &mut Ui, ok: bool, text: String| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(if ok { "✓" } else { "✗" }).color(if ok { icons::GREEN } else { t.danger }).font(theme::ui_bold(13.0)));
            ui.label(text);
        });
    };
    line(ui, missing.is_empty(), format!("{} {}", missing.len(), lang.tr("ribbon controls without a UI-kit entry")));
    for m in &missing {
        ui.label(egui::RichText::new(crate::ui_kit::artwork(&format!("    {m}"))).small());
    }
    line(ui, untranslated.is_empty(), format!("{} {}", untranslated.len(), lang.tr("explanations without an Arabic translation")));
    for m in &untranslated {
        ui.label(egui::RichText::new(crate::ui_kit::artwork(&format!("    {m}"))).small());
    }
    line(ui, true, format!("{} {}", INVENTORY.len(), lang.tr("ribbon controls in the inventory")));
    let entries = crate::ui_kit::entries().count();
    line(ui, entries > 0, format!("{entries} {}", lang.tr("entries in the UI kit")));
    line(ui, true, format!("{} {}", icons::ALL.len(), lang.tr("icons in the set")));
    line(ui, true, format!("{} {}", gridcraft_engine::command_specs().len(), lang.tr("commands in the engine registry")));
    let _ = app;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_test_runs_every_command_without_a_panic() {
        let rep = self_test();
        let expected = gridcraft_engine::command_specs().iter().filter(|s| !SKIPPED.contains(&s.id)).count();
        assert_eq!(rep.runs.len(), expected);
        assert!(rep.runs.len() > 100, "{} ran", rep.runs.len());
        for r in &rep.runs {
            assert!(!r.answer.is_empty(), "{}: no answer", r.id);
            assert!(!r.answer.contains("panic"), "{}: {}", r.id, r.answer);
        }
    }

    #[test]
    fn inventory_keys_are_unique_per_tab_and_the_checklist_runs() {
        let mut seen = std::collections::HashSet::new();
        for (tab, key, _) in INVENTORY {
            assert!(seen.insert((tab, key)), "{tab} › {key}: listed twice");
        }
        let _ = unexplained();
        let _ = untranslated();
    }

    #[test]
    fn the_report_round_trips_through_json() {
        let rep = Report { runs: vec![Run { id: "cell.set".into(), answer: "✓".into(), ok: true, millis: 0.5 }], sheets: 3 };
        let json = serde_json::to_value(&rep).unwrap();
        assert_eq!(serde_json::from_value::<Report>(json).unwrap(), rep);
    }
}
