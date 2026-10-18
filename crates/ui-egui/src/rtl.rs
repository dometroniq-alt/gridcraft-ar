//! GridCraft AR — the single switch for a right-to-left interface: when the interface language
//! is Arabic, rows run from the right, as Excel's Arabic interface does — the ribbon's tabs start
//! at the right edge, its groups run from the right (Clipboard at the far right), a small
//! button's icon stands at its right end, the formula bar's name box sits at the right, the sheet
//! tabs start at the right and the status bar's zoom sits at the left. Everything that mirrors
//! asks here; nothing else checks the language for layout. (The sheet's own direction is Excel's
//! per-sheet "right-to-left" setting, handled by the grid, and is not this.)
//!
//! When the interface is not Arabic every helper is exactly egui's left-to-right default (the
//! `Ui` helpers reproduce `ui.horizontal`, `ui.horizontal_top`, `ui.horizontal_centered` and
//! `ui.vertical` to the pixel), so every other language is laid out as before.

use egui::{Align, Align2, InnerResponse, Layout, Pos2, Rect, Ui, pos2, vec2};

use crate::i18n::{self, Language};

/// Whether the interface is laid out right-to-left this frame.
pub fn on(ctx: &egui::Context) -> bool {
    i18n::current(ctx) == Language::Ar
}

/// A row layout: right-to-left when [`on`], else left-to-right.
pub fn row(ctx: &egui::Context, valign: Align) -> Layout {
    if on(ctx) { Layout::right_to_left(valign) } else { Layout::left_to_right(valign) }
}

/// The opposite row (for a cluster that sits at the end of a bar): from the right in a
/// left-to-right interface, from the left when [`on`].
pub fn row_from_end(ctx: &egui::Context, valign: Align) -> Layout {
    if on(ctx) { Layout::left_to_right(valign) } else { Layout::right_to_left(valign) }
}

/// A column whose items are aligned to the start edge (`ui.vertical`'s layout mirrored).
pub fn column(ctx: &egui::Context) -> Layout {
    Layout::top_down(align_start(ctx))
}

/// The start-edge alignment: RIGHT when [`on`], else LEFT.
pub fn align_start(ctx: &egui::Context) -> Align {
    if on(ctx) { Align::RIGHT } else { Align::LEFT }
}

/// The end-edge alignment: LEFT when [`on`], else RIGHT.
pub fn align_end(ctx: &egui::Context) -> Align {
    if on(ctx) { Align::LEFT } else { Align::RIGHT }
}

/// The x of a point `inset` in from the start edge of `r`.
pub fn start_x(ctx: &egui::Context, r: Rect, inset: f32) -> f32 {
    if on(ctx) { r.max.x - inset } else { r.min.x + inset }
}

/// The x of a point `inset` in from the end edge of `r`.
pub fn end_x(ctx: &egui::Context, r: Rect, inset: f32) -> f32 {
    if on(ctx) { r.min.x + inset } else { r.max.x - inset }
}

/// Text anchor at the start edge, vertically centred.
pub fn start_center(ctx: &egui::Context) -> Align2 {
    if on(ctx) { Align2::RIGHT_CENTER } else { Align2::LEFT_CENTER }
}

/// Text anchor at the end edge, vertically centred.
pub fn end_center(ctx: &egui::Context) -> Align2 {
    if on(ctx) { Align2::LEFT_CENTER } else { Align2::RIGHT_CENTER }
}

/// `rect` mirrored horizontally within `within` when [`on`]; `rect` itself otherwise.
pub fn mirror_x(ctx: &egui::Context, rect: Rect, within: Rect) -> Rect {
    if !on(ctx) {
        return rect;
    }
    let left = within.min.x + (within.max.x - rect.max.x);
    let right = within.max.x - (rect.min.x - within.min.x);
    Rect::from_min_max(pos2(left, rect.min.y), pos2(right, rect.max.y))
}

/// Where the next item of a row goes: its left edge in a left-to-right row, its right edge when
/// the row runs right-to-left (egui's cursor rect keeps that side as its `max.x`).
pub fn cursor_x(ui: &Ui) -> f32 {
    let c = ui.cursor();
    if ui.layout().main_dir() == egui::Direction::RightToLeft { c.max.x } else { c.min.x }
}

/// The top corner where the next item of a row goes ([`cursor_x`] at the cursor's top).
pub fn cursor_start(ui: &Ui) -> Pos2 {
    pos2(cursor_x(ui), ui.cursor().min.y)
}

/// The direction a row inside `ui` runs: right-to-left when [`on`], or when egui itself would
/// prefer it there (a column aligned to the right), else left-to-right — `ui.horizontal`'s rule
/// plus the Arabic interface.
fn row_in(ui: &Ui, valign: Align) -> Layout {
    if on(ui.ctx()) || ui.layout().prefer_right_to_left() { Layout::right_to_left(valign) } else { Layout::left_to_right(valign) }
}

/// `ui.horizontal`, mirrored when [`on`].
pub fn horizontal<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let size = vec2(ui.available_size_before_wrap().x, ui.spacing().interact_size.y);
    let layout = row_in(ui, Align::Center);
    ui.allocate_ui_with_layout(size, layout, add)
}

/// `ui.horizontal_top`, mirrored when [`on`].
pub fn horizontal_top<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let size = ui.available_size_before_wrap();
    let layout = row_in(ui, Align::Min);
    ui.allocate_ui_with_layout(size, layout, add)
}

/// `ui.horizontal_centered`, mirrored when [`on`].
pub fn horizontal_centered<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let size = ui.available_size_before_wrap();
    let layout = row_in(ui, Align::Center);
    ui.allocate_ui_with_layout(size, layout, add)
}

/// `ui.vertical`, its items aligned to the start edge when [`on`].
pub fn vertical<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let layout = column(ui.ctx());
    ui.with_layout(layout, add)
}

/// A menu item's button: its text at the start edge. Menu popups keep egui's left-to-right
/// column, so a right-to-left item puts its text at the right end of the button through the
/// button's own parts instead of the column's alignment.
pub fn menu_item<'a>(ctx: &egui::Context, text: &'a str) -> egui::Button<'a> {
    if on(ctx) { egui::Button::new("").right_text(text) } else { egui::Button::new(text) }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::SheetApp;
    use crate::i18n::{Language, set_current};

    /// Every text drawn in one frame of the whole app at 2560×900, with the top-left corner of each.
    fn drawn_texts(ctx: &egui::Context, app: &mut SheetApp) -> Vec<(String, egui::Pos2)> {
        fn texts(shape: &egui::Shape, out: &mut Vec<(String, egui::Pos2)>) {
            match shape {
                egui::Shape::Text(t) => out.push((t.galley.text().to_string(), t.pos)),
                egui::Shape::Vec(v) => v.iter().for_each(|s| texts(s, out)),
                _ => {}
            }
        }
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(2560.0, 900.0));
        let mut drawn = Vec::new();
        // Fonts install over the first frames; the ribbon's right-to-left strip settles in one.
        for n in 0..5 {
            let input = egui::RawInput { screen_rect: Some(screen), time: Some(f64::from(n) * 0.1), ..Default::default() };
            let mut out = ctx.run_ui(input, |ui| {
                app.logic(ui.ctx());
                app.ui(ui);
            });
            drawn.clear();
            for s in &out.shapes {
                texts(&s.shape, &mut drawn);
            }
            out.textures_delta.clear();
        }
        drawn
    }

    fn x_of(drawn: &[(String, egui::Pos2)], text: &str) -> f32 {
        let hits: Vec<f32> = drawn.iter().filter(|(t, _)| t == text).map(|(_, p)| p.x).collect();
        assert!(!hits.is_empty(), "{text:?} was not drawn; drawn: {:?}", drawn.iter().map(|(t, _)| t).collect::<Vec<_>>());
        hits.into_iter().fold(f32::NAN, f32::min)
    }

    fn app_in(code: &str) -> (egui::Context, SheetApp) {
        let ctx = egui::Context::default();
        let mut session = gridcraft_engine::Session::new();
        session.execute("file.new", json!({"sample": "sales"})).unwrap();
        let mut app = SheetApp::new(session, Default::default());
        app.run("app.language.set", json!({"language": code})).unwrap();
        app.ui.ribbon_tab = "Home".into();
        app.ui.ribbon_collapsed = false;
        (ctx, app)
    }

    /// Arabic interface: the tab strip starts at the right (Home right of Insert) and the ribbon's
    /// groups run right-to-left (Paste right of Bold), as in Excel's Arabic interface.
    #[test]
    fn arabic_interface_is_laid_out_right_to_left() {
        let (ctx, mut app) = app_in("ar");
        let drawn = drawn_texts(&ctx, &mut app);
        assert!(super::on(&ctx), "the interface language is Arabic while the app is drawn");
        let (home, insert, review) = (x_of(&drawn, "الشريط الرئيسي"), x_of(&drawn, "إدراج"), x_of(&drawn, "مراجعة"));
        assert!(home > insert, "Home tab at x={home} must be right of Insert at x={insert}");
        assert!(home > review, "Home tab at x={home} must be right of Review at x={review}");
        assert!(home > 2560.0 / 2.0, "Home tab at x={home} starts at the right edge");
        // The first group (Clipboard) starts at the ribbon's right edge: well into the right half.
        let paste = x_of(&drawn, "لصق");
        assert!(paste > 2560.0 / 2.0, "Paste at x={paste} sits at the far right");
        set_current(&ctx, Language::En);
        assert!(!super::on(&ctx));
    }

    /// English (and every other language): left-to-right, unchanged.
    #[test]
    fn english_interface_stays_left_to_right() {
        let (ctx, mut app) = app_in("en");
        let drawn = drawn_texts(&ctx, &mut app);
        assert!(!super::on(&ctx));
        let (home, insert, review) = (x_of(&drawn, "Home"), x_of(&drawn, "Insert"), x_of(&drawn, "Review"));
        assert!(home < insert, "Home tab at x={home} must be left of Insert at x={insert}");
        assert!(home < review, "Home tab at x={home} must be left of Review at x={review}");
        assert!(home < 2560.0 / 2.0, "Home tab at x={home} starts at the left edge");
        let paste = x_of(&drawn, "Paste");
        assert!(paste < 2560.0 / 2.0, "Paste at x={paste} sits at the far left");
    }

    /// The helpers are egui's own layouts in a left-to-right interface and their mirror images
    /// in Arabic.
    #[test]
    fn helpers_follow_the_interface_language() {
        let ctx = egui::Context::default();
        set_current(&ctx, Language::En);
        assert_eq!(super::row(&ctx, egui::Align::Min), egui::Layout::left_to_right(egui::Align::Min));
        assert_eq!(super::row_from_end(&ctx, egui::Align::Center), egui::Layout::right_to_left(egui::Align::Center));
        assert_eq!(super::column(&ctx), egui::Layout::top_down(egui::Align::Min));
        assert_eq!(super::align_start(&ctx), egui::Align::LEFT);
        assert_eq!(super::align_end(&ctx), egui::Align::RIGHT);
        let within = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(100.0, 10.0));
        let r = egui::Rect::from_min_max(egui::pos2(10.0, 0.0), egui::pos2(30.0, 10.0));
        assert_eq!(super::mirror_x(&ctx, r, within), r);
        assert_eq!(super::start_x(&ctx, r, 4.0), 14.0);
        assert_eq!(super::end_x(&ctx, r, 4.0), 26.0);
        assert_eq!(super::start_center(&ctx), egui::Align2::LEFT_CENTER);
        set_current(&ctx, Language::Ar);
        assert!(super::on(&ctx));
        assert_eq!(super::row(&ctx, egui::Align::Min), egui::Layout::right_to_left(egui::Align::Min));
        assert_eq!(super::row_from_end(&ctx, egui::Align::Center), egui::Layout::left_to_right(egui::Align::Center));
        assert_eq!(super::column(&ctx), egui::Layout::top_down(egui::Align::Max));
        assert_eq!(super::align_start(&ctx), egui::Align::RIGHT);
        assert_eq!(super::align_end(&ctx), egui::Align::LEFT);
        assert_eq!(super::start_center(&ctx), egui::Align2::RIGHT_CENTER);
        assert_eq!(super::end_center(&ctx), egui::Align2::LEFT_CENTER);
        assert_eq!(super::mirror_x(&ctx, r, within), egui::Rect::from_min_max(egui::pos2(70.0, 0.0), egui::pos2(90.0, 10.0)));
        assert_eq!(super::start_x(&ctx, r, 4.0), 26.0);
        assert_eq!(super::end_x(&ctx, r, 4.0), 14.0);
        set_current(&ctx, Language::En);
    }
}
