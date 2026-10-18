//! Home tab thumbnails: the clipboard, font and alignment, number formats, conditional
//! formatting and styles, cells (insert, delete, format) and editing (fill, clear, sort and
//! filter, find and select) — the buttons and every item of the Home menus. Mostly pairs of
//! little sheets: the left sheet as the cells are, the right one after the button, with what
//! changed in the accent colour. Single-sheet drawings are short films: a selection grows, a row
//! slides in, a mark lands.

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::draw::Draw;

pub const ENTRIES: &[Entry] = &[
    // ---- Clipboard ----
    Entry::new(
        "Paste (⌘V)",
        "Puts the cells you cut or copied into the sheet, starting at the selected cell. Example: copy A1:A3, click C1 and paste; the three values appear in C1:C3.",
        |d| d.hm_paste(0),
    ),
    Entry::new(
        "Cut (⌘X)",
        "Takes the selected cells out of the sheet and keeps them on the clipboard, so you can paste them somewhere else. Example: cut a column of names and paste it beside another table.",
        |d| d.hm_clip(true),
    ),
    Entry::new(
        "Copy (⌘C)",
        "Keeps a copy of the selected cells on the clipboard to paste elsewhere; the originals stay where they are. Example: copy this month's header row and paste it into next month's sheet.",
        |d| d.hm_clip(false),
    ),
    Entry::new(
        "Format Painter (double-click to keep it on)",
        "Copies the look of the selected cell (fill, font, borders, number format) and paints it onto the cells you click next; double-click the button to paint many places. Example: click a yellow bold title, then click another title to make it look the same.",
        |d| d.hm_format_painter(),
    ),
    // ---- Font ----
    Entry::new("Increase Font Size", "Makes the text in the selected cells one step bigger. Example: a title in 11 points becomes 12 points.", |d| {
        d.hm_font_size(true)
    }),
    Entry::new(
        "Decrease Font Size",
        "Makes the text in the selected cells one step smaller. Example: a title in 12 points becomes 11 points.",
        |d| d.hm_font_size(false),
    ),
    Entry::new(
        "Bold (⌘B)",
        "Makes the text in the selected cells bold: thicker, darker letters that stand out. Example: the header row of a table is made bold.",
        |d| d.hm_bold(),
    ),
    Entry::new(
        "Italic (⌘I)",
        "Slants the text in the selected cells to the side, like handwriting. Example: a note under a table is written slanted.",
        |d| d.hm_italic(),
    ),
    Entry::new(
        "Strikethrough",
        "Draws a line through the middle of the text in the selected cells, as if crossed out. Example: a cancelled order is crossed out but still readable.",
        |d| d.hm_strike(),
    ),
    Entry::new("Fill Color", "Colours the background of the selected cells. Example: the total row is filled light yellow so it stands out.", |d| {
        d.hm_fill_color()
    }),
    Entry::new(
        "Font Color",
        "Colours the text in the selected cells; the background stays as it is. Example: the negative numbers are written in red.",
        |d| d.hm_font_color(),
    ),
    // ---- Alignment ----
    Entry::new(
        "Orientation",
        "Turns the text in the selected cells: slanted, vertical or rotated, so long headers fit over narrow columns. Example: the column titles are written at an angle.",
        |d| d.hm_orient(1),
    ),
    Entry::new(
        "Wrap Text",
        "Breaks long text onto several lines inside its cell, making the row taller, instead of letting it spill over the next cells. Example: a long address shows on three lines in one cell.",
        |d| d.hm_wrap(),
    ),
    Entry::new(
        "Decrease Indent",
        "Moves the text in the selected cells one step back towards the cell's edge. Example: an indented item moves back in line with the others.",
        |d| d.hm_indent(false),
    ),
    Entry::new(
        "Increase Indent",
        "Moves the text in the selected cells one step away from the cell's edge. Example: the items under a heading are pushed in to show they belong to it.",
        |d| d.hm_indent(true),
    ),
    // ---- Number ----
    Entry::new("Percent Style", "Shows the selected numbers as percentages: 0.25 becomes 25%. Example: a share of 0.5 shows as 50%.", |d| {
        d.hm_numfmt(&["٠٫٢٥", "٠٫٥", "٠٫٨"], &["٢٥٪", "٥٠٪", "٨٠٪"])
    }),
    Entry::new("Comma Style", "Shows the selected numbers with a thousands separator and two decimals. Example: 1250 becomes 1,250.00.", |d| {
        d.hm_numfmt(&["١٢٥٠", "٣٠٠٠", "٨٧٥"], &["١٬٢٥٠٫٠٠", "٣٬٠٠٠٫٠٠", "٨٧٥٫٠٠"])
    }),
    Entry::new("Increase Decimal", "Shows one more digit after the decimal point in the selected cells. Example: 3.1 becomes 3.14.", |d| {
        d.hm_numfmt(&["٣٫١", "٢٫٥", "٧٫٠"], &["٣٫١٤", "٢٫٥٠", "٧٫٠٠"])
    }),
    Entry::new(
        "Decrease Decimal",
        "Shows one digit fewer after the decimal point in the selected cells, rounding what you see. Example: 3.14 becomes 3.1.",
        |d| d.hm_numfmt(&["٣٫١٤", "٢٫٥٠", "٧٫٠٠"], &["٣٫١", "٢٫٥", "٧٫٠"]),
    ),
    // ---- Styles ----
    Entry::new(
        "Conditional Formatting",
        "Colours cells automatically by their values, with rules that keep working as the numbers change: highlight the large ones, add bars or colour scales. Example: every sale above 1000 turns green by itself.",
        |d| d.hm_cond_format(),
    ),
    Entry::new(
        "Format as Table",
        "Turns the selected cells into a table with a styled header, banded rows and filter buttons. Example: a plain list of orders becomes a striped table with filter arrows.",
        |d| d.hm_format_table(),
    ),
    Entry::new(
        "Cell Styles",
        "Applies a ready-made look (title, heading, total, good, bad…) to the selected cells in one click. Example: the last row gets the Total style: bold with a line above it.",
        |d| d.hm_cell_styles(),
    ),
    // ---- Cells ----
    Entry::new(
        "Insert",
        "Adds empty cells, rows, columns or a new sheet at the selection, pushing the existing ones aside. Example: insert a row above row 3 to add a forgotten item.",
        |d| d.hm_rowcol(true, true),
    ),
    Entry::new(
        "Delete",
        "Removes the selected cells, rows, columns or sheet; the others move up or left to close the gap. Example: delete the empty row between two tables.",
        |d| d.hm_rowcol(false, true),
    ),
    Entry::new(
        "Format",
        "Changes the size of rows and columns, hides or shows them, and manages the sheet: rename, protect, move. Example: make column A wide enough for the names.",
        |d| d.hm_format_menu(),
    ),
    // ---- Editing ----
    Entry::new(
        "Fill",
        "Copies the selected cell's value or formula into the cells next to it (down, right, up or left), or continues a series. Example: a formula in B2 is filled down to B10.",
        |d| d.hm_fill_dir(0),
    ),
    Entry::new(
        "Clear",
        "Empties the selected cells: everything, only the formatting, only the contents, or only comments and links. Example: clear the contents but keep the yellow fill.",
        |d| d.hm_clear(0),
    ),
    Entry::new(
        "Sort & Filter",
        "Puts the rows in order (A to Z, smallest to largest) or shows only the rows that match a condition. Example: sort a list of names alphabetically, or show only the unpaid invoices.",
        |d| d.hm_sort_filter(),
    ),
    Entry::new(
        "Find & Select",
        "Searches the sheet for a word or number, replaces it, jumps to a cell, or selects all cells of one kind. Example: find every «بغداد» and replace it with «البصرة».",
        |d| d.hm_find(false),
    ),
    // ---- Paste menu ----
    Entry::new(
        "edit.paste",
        "Pastes the copied cells as they are: values, formulas and formatting together. Example: a formatted total with its formula comes in complete.",
        |d| d.hm_paste(0),
    ),
    Entry::new(
        "edit.pasteSpecial:values",
        "Pastes only the results: the numbers and text, without formulas or formatting. Example: a cell showing 150 from =A1*3 is pasted as the plain number 150.",
        |d| d.hm_paste(1),
    ),
    Entry::new(
        "edit.pasteSpecial:formulas",
        "Pastes only the formulas, without the formatting; they recalculate for their new place. Example: =SUM(A1:A3) pasted into column B becomes =SUM(B1:B3).",
        |d| d.hm_paste(2),
    ),
    Entry::new(
        "edit.pasteSpecial:formats",
        "Pastes only the look (fill, font, borders, number format), leaving the destination's values as they are. Example: the yellow fill and bold of one header are copied onto another header.",
        |d| d.hm_paste(3),
    ),
    Entry::new(
        "edit.pasteSpecial:all",
        "Pastes everything with a twist: Transpose turns the copied rows into columns and columns into rows; Paste Link pastes formulas that point back to the copied cells, so they follow every change. Example: a header row A1:D1 becomes a column A1:A4.",
        |d| d.hm_transpose(),
    ),
    Entry::new(
        "dialog:pasteSpecial",
        "Opens a box with every paste choice: values, formulas, formats, comments, column widths, transpose, link, or add and multiply with the pasted numbers. Example: paste only the column widths of another table.",
        |d| d.hm_paste_special_dialog(),
    ),
    // ---- Underline menu ----
    Entry::new(
        "home.underline:single",
        "Draws one line under the text in the selected cells. Example: a column title with a single line under it.",
        |d| d.hm_underline(false),
    ),
    Entry::new(
        "home.underline:double",
        "Draws two lines under the text in the selected cells, as accountants mark a final total. Example: the grand total has a double underline.",
        |d| d.hm_underline(true),
    ),
    // ---- Orientation menu ----
    Entry::new(
        "home.orientation",
        "Turns the text in the selected cells to the chosen angle: slanted up or down, vertical letters, rotated up or down, or back to horizontal. Example: a long header is written at an angle over a narrow column.",
        |d| d.hm_orient(0),
    ),
    Entry::new(
        "home.orientation:counterclockwise",
        "Slants the text upwards, from bottom-left to top-right, at 45 degrees. Example: the month names rise over their narrow columns.",
        |d| d.hm_orient(1),
    ),
    Entry::new(
        "home.orientation:clockwise",
        "Slants the text downwards, from top-left to bottom-right, at 45 degrees. Example: the month names lean down over their columns.",
        |d| d.hm_orient(2),
    ),
    Entry::new(
        "home.orientation:vertical",
        "Stacks the letters one under the other, upright, in a tall cell. Example: a one-word header reads from top to bottom.",
        |d| d.hm_orient(3),
    ),
    Entry::new(
        "home.orientation:up",
        "Rotates the text 90 degrees so it reads from bottom to top. Example: a label beside a tall merged cell is read looking up.",
        |d| d.hm_orient(4),
    ),
    Entry::new(
        "home.orientation:down",
        "Rotates the text 90 degrees so it reads from top to bottom. Example: a label on the right edge of a table is read looking down.",
        |d| d.hm_orient(5),
    ),
    // ---- Format Cells dialog ----
    Entry::new(
        "dialog:formatCells",
        "Opens the Format Cells box with every setting in one place: number format, alignment, font, borders, fill and protection. Example: give a cell a date format, centre it and add a border, all at once.",
        |d| d.hm_format_cells(0),
    ),
    Entry::new(
        "dialog:formatCells:Alignment",
        "Opens the Format Cells box on its Alignment tab: horizontal and vertical alignment, indent, wrapping, shrink to fit, merging and text angle. Example: centre a title vertically in a tall cell.",
        |d| d.hm_format_cells(1),
    ),
    Entry::new(
        "dialog:formatCells:Number",
        "Opens the Format Cells box on its Number tab, where you choose among all the number, currency, accounting, date, percentage and custom formats. Example: show amounts in Iraqi dinars with no decimals.",
        |d| d.hm_format_cells(2),
    ),
    // ---- Merge menu ----
    Entry::new(
        "home.mergeCenter",
        "Joins the selected cells into one big cell and centres the text in it. Example: a title over four columns is merged and centred above them.",
        |d| d.hm_merge(0),
    ),
    Entry::new(
        "home.mergeAcross",
        "Joins the selected cells row by row: each row becomes one wide cell, the rows stay separate. Example: two title rows each become a wide cell of their own.",
        |d| d.hm_merge(1),
    ),
    Entry::new(
        "home.mergeCells",
        "Joins the selected cells into one cell without changing the alignment; only the top-left value is kept. Example: four cells become one, and the text stays at its edge.",
        |d| d.hm_merge(2),
    ),
    Entry::new(
        "home.unmergeCells",
        "Splits a merged cell back into its original cells; the value stays in the first one. Example: a merged title cell becomes four cells again.",
        |d| d.hm_merge(3),
    ),
    // ---- Accounting number formats ----
    Entry::new(
        "home.numberFormat",
        "Shows the selected numbers as money in the chosen currency ($, £, €, ¥), with the symbol at the edge of the cell, thousands separators and two decimals. Example: 1250 becomes $ 1,250.00.",
        |d| d.hm_currency(),
    ),
    // ---- Insert menu ----
    Entry::new(
        "dialog:insertCells",
        "Opens a box asking how to make room: shift the existing cells right or down, or insert whole rows or columns. Example: shift cells down to add one item in the middle of a list.",
        |d| d.hm_cells_dialog(true),
    ),
    Entry::new(
        "home.insertRows",
        "Adds an empty row above each selected row; the rows below move down. Example: a new row appears above row 3 for the missing item.",
        |d| d.hm_rowcol(true, true),
    ),
    Entry::new(
        "home.insertColumns",
        "Adds an empty column beside each selected column; the other columns move over. Example: a new column appears before C for the quantity.",
        |d| d.hm_rowcol(true, false),
    ),
    Entry::new(
        "home.insertSheet",
        "Adds a new blank sheet to the workbook, with its own tab at the bottom. Example: a tab «ورقة٢» appears beside «ورقة١».",
        |d| d.hm_sheet_tabs(0),
    ),
    // ---- Delete menu ----
    Entry::new(
        "dialog:deleteCells",
        "Opens a box asking how to close the gap: shift the remaining cells left or up, or delete whole rows or columns. Example: delete and shift up to drop one item from a list.",
        |d| d.hm_cells_dialog(false),
    ),
    Entry::new(
        "home.deleteRows",
        "Removes the selected rows entirely; the rows below move up to fill the gap. Example: row 3 is deleted and row 4 becomes row 3.",
        |d| d.hm_rowcol(false, true),
    ),
    Entry::new(
        "home.deleteColumns",
        "Removes the selected columns entirely; the other columns move over to fill the gap. Example: column B is deleted and column C becomes B.",
        |d| d.hm_rowcol(false, false),
    ),
    Entry::new(
        "home.deleteSheet",
        "Removes the current sheet and everything on it from the workbook; this cannot be undone. Example: an empty «ورقة٣» is deleted.",
        |d| d.hm_sheet_tabs(1),
    ),
    // ---- Format menu ----
    Entry::new(
        "dialog:rowHeight",
        "Opens a box where you type an exact height for the selected rows. Example: set the title row to 30 points.",
        |d| d.hm_size_dialog(true),
    ),
    Entry::new(
        "home.autofitRowHeight",
        "Makes the selected rows exactly as tall as their tallest text needs. Example: a row with wrapped text grows to show all its lines.",
        |d| d.hm_autofit(true),
    ),
    Entry::new(
        "dialog:columnWidth",
        "Opens a box where you type an exact width for the selected columns. Example: set column A to 20 characters wide.",
        |d| d.hm_size_dialog(false),
    ),
    Entry::new(
        "home.autofitColumnWidth",
        "Makes the selected columns exactly as wide as their longest entry. Example: a column of names widens so no name is cut off.",
        |d| d.hm_autofit(false),
    ),
    Entry::new(
        "dialog:defaultWidth",
        "Sets the standard width for every column of the sheet that you have not resized by hand. Example: all the columns become 12 characters wide.",
        |d| d.hm_default_width(),
    ),
    Entry::new(
        "home.hideRows",
        "Hides the selected rows; their numbers are skipped and their data stays. Example: rows 2 and 3 disappear, and row 4 follows row 1.",
        |d| d.hm_hide(true, false),
    ),
    Entry::new(
        "home.hideColumns",
        "Hides the selected columns; their letters are skipped and their data stays. Example: column B disappears, and C follows A.",
        |d| d.hm_hide(false, false),
    ),
    Entry::new(
        "home.unhideRows",
        "Shows again the rows hidden inside the selection. Example: select rows 1 to 4 and the hidden rows 2 and 3 come back.",
        |d| d.hm_hide(true, true),
    ),
    Entry::new(
        "home.unhideColumns",
        "Shows again the columns hidden inside the selection. Example: select columns A to C and the hidden column B comes back.",
        |d| d.hm_hide(false, true),
    ),
    Entry::new(
        "sheet.hide",
        "Hides the current sheet: its tab disappears, but its data and formulas keep working. Example: a sheet of exchange rates is hidden from view.",
        |d| d.hm_sheet_tabs(2),
    ),
    Entry::new(
        "sheet.unhide",
        "Opens a list of the hidden sheets so you can bring one back. Example: pick the hidden rates sheet and its tab returns.",
        |d| d.hm_unhide_sheet(),
    ),
    Entry::new(
        "dialog:renameSheet",
        "Lets you type a new name for the current sheet's tab. Example: «ورقة١» becomes «المبيعات».",
        |d| d.hm_rename_sheet(),
    ),
    Entry::new(
        "dialog:moveSheet",
        "Moves the current sheet to another place in the tab row or into another workbook, or makes a copy of it. Example: copy January's sheet to start February.",
        |d| d.hm_sheet_tabs(3),
    ),
    Entry::new(
        "dialog:protectSheet",
        "Locks the sheet so nobody can change its cells, optionally with a password; cells you unlocked first stay editable. Example: protect an invoice so only the quantity cells can be changed.",
        |d| d.hm_protect(),
    ),
    Entry::new(
        "home.lockCell",
        "Marks the selected cells as locked or unlocked; the lock takes effect once the sheet is protected. Example: unlock the input cells, then protect the sheet.",
        |d| d.hm_lock_cell(),
    ),
    // ---- Fill menu ----
    Entry::new(
        "edit.fillDown",
        "Copies the top cell of the selection into every cell below it. Example: a formula in B2 is copied down to B6.",
        |d| d.hm_fill_dir(0),
    ),
    Entry::new(
        "edit.fillRight",
        "Copies the leftmost cell of the selection into every cell to its right. Example: the formula in B2 is copied across to E2.",
        |d| d.hm_fill_dir(1),
    ),
    Entry::new(
        "edit.fillUp",
        "Copies the bottom cell of the selection into every cell above it. Example: the value in B6 is copied up to B2.",
        |d| d.hm_fill_dir(2),
    ),
    Entry::new(
        "edit.fillLeft",
        "Copies the rightmost cell of the selection into every cell to its left. Example: the value in E2 is copied back to B2.",
        |d| d.hm_fill_dir(3),
    ),
    Entry::new(
        "dialog:series",
        "Fills the selection with a series: numbers growing by a step, dates by day, month or year, or a growth factor. Example: 1, 2, 3, 4 down a column, or the first day of every month.",
        |d| d.hm_series(),
    ),
    Entry::new(
        "edit.flashFill",
        "Looks at the examples you typed and fills the rest of the column the same way. Example: type the first name from «أحمد علي» into B2, and Flash Fill writes the first name for every row.",
        |d| d.hm_flash_fill(),
    ),
    // ---- Clear menu ----
    Entry::new(
        "edit.clearAll",
        "Empties the selected cells completely: contents, formatting, comments and links. Example: a yellow bold 150 becomes a plain empty cell.",
        |d| d.hm_clear(0),
    ),
    Entry::new(
        "edit.clearFormats",
        "Removes the formatting of the selected cells but keeps what is in them. Example: a yellow bold 150 becomes a plain 150.",
        |d| d.hm_clear(1),
    ),
    Entry::new(
        "edit.clearContents",
        "Deletes what is in the selected cells but keeps their formatting. Example: the yellow fill stays and the number goes.",
        |d| d.hm_clear(2),
    ),
    Entry::new(
        "edit.clearComments",
        "Removes the comments and notes attached to the selected cells; the values stay. Example: the little comment marks in the corners disappear.",
        |d| d.hm_clear(3),
    ),
    Entry::new(
        "edit.clearHyperlinks",
        "Removes the links from the selected cells; the text stays but is no longer blue or clickable. Example: a web address becomes ordinary text.",
        |d| d.hm_clear(4),
    ),
    // ---- Sort & Filter menu ----
    Entry::new(
        "data.sortAscending",
        "Orders the rows by the selected column from smallest to largest: numbers first, then A to Z, oldest dates first. Example: 9, 3, 5 becomes 3, 5, 9.",
        |d| d.hm_sort(true),
    ),
    Entry::new(
        "data.sortDescending",
        "Orders the rows by the selected column from largest to smallest: Z to A, newest dates first. Example: 3, 5, 9 becomes 9, 5, 3.",
        |d| d.hm_sort(false),
    ),
    Entry::new(
        "dialog:sort",
        "Opens a box to sort by several columns at once, each ascending or descending, or by colour. Example: sort by city, then by name within each city.",
        |d| d.hm_sort_dialog(),
    ),
    Entry::new(
        "data.filter",
        "Adds filter buttons to the header row so you can show only the rows that match. Example: show only the rows where the city is «بغداد».",
        |d| d.hm_filter(0),
    ),
    Entry::new(
        "data.clearFilter",
        "Shows all the rows again by removing every filter condition; the filter buttons stay. Example: the hidden rows come back.",
        |d| d.hm_filter(1),
    ),
    Entry::new(
        "data.reapply",
        "Runs the current filter and sort again after the data changed. Example: a row you edited to match the filter now appears.",
        |d| d.hm_filter(2),
    ),
    // ---- Find & Select menu ----
    Entry::new(
        "dialog:find",
        "Searches the sheet or the whole workbook for a word or number and jumps to each match. Example: find every cell containing 2024.",
        |d| d.hm_find(false),
    ),
    Entry::new(
        "dialog:find:replace",
        "Finds a word or number and replaces it with another, one match at a time or all at once. Example: replace every «بغداد» with «البصرة».",
        |d| d.hm_find(true),
    ),
    Entry::new("dialog:goTo", "Jumps to the cell or range you type, or to a named range. Example: type D20 and the selection moves there.", |d| {
        d.hm_goto()
    }),
    Entry::new(
        "dialog:goToSpecial",
        "Selects every cell of one kind in the sheet at once: formulas, typed values, blanks, notes, conditional formats or validation rules. Example: select all the formula cells to check them.",
        |d| d.hm_goto_special(0),
    ),
    Entry::new("edit.goToSpecial:formulas", "Selects every cell that contains a formula. Example: all the =SUM cells are selected at once.", |d| {
        d.hm_goto_special(1)
    }),
    Entry::new(
        "edit.goToSpecial:notes",
        "Selects every cell that has a note or comment attached. Example: the three cells with remarks are selected.",
        |d| d.hm_goto_special(2),
    ),
    Entry::new(
        "edit.goToSpecial:conditionalFormats",
        "Selects every cell that has conditional formatting. Example: the cells that turn green above 1000 are selected.",
        |d| d.hm_goto_special(3),
    ),
    Entry::new(
        "edit.goToSpecial:constants",
        "Selects every cell that holds a typed value (number or text), not a formula. Example: the entered amounts are selected, the totals are not.",
        |d| d.hm_goto_special(4),
    ),
    Entry::new(
        "edit.goToSpecial:dataValidation",
        "Selects every cell that has a data-validation rule (a list or a limit on what can be typed). Example: the cells with a drop-down list are selected.",
        |d| d.hm_goto_special(5),
    ),
];

/// Height of the column-letter strip and width of the row-number strip of a mini sheet.
const HDR_H: f32 = 7.0;
const HDR_W: f32 = 9.0;
/// The font size of text written into a cell.
const CELL_FONT: f32 = 7.0;

impl Draw<'_> {
    // ---- the sheet kit ----

    /// A mini sheet at `x, y` of `w × h`: a column-letter strip on top, a row-number strip on
    /// the left and a `rows × cols` grid of cells; returns the grid's rect.
    pub(super) fn hm_sheet(&self, x: f32, y: f32, w: f32, h: f32, rows: usize, cols: usize) -> Rect {
        let outer = Rect::from_min_size(self.at(x, y), vec2(w, h));
        self.p.rect_filled(outer, 1.0, Color32::WHITE);
        self.p.rect_filled(Rect::from_min_size(outer.min, vec2(w, HDR_H)), 0.0, self.t.hover);
        self.p.rect_filled(Rect::from_min_size(outer.min, vec2(HDR_W, h)), 0.0, self.t.hover);
        let grid = Rect::from_min_max(pos2(outer.min.x + HDR_W, outer.min.y + HDR_H), outer.max);
        self.grid(grid, rows, cols, self.t.border);
        for j in 1..cols.max(1) {
            let gx = grid.min.x + j as f32 * grid.width() / cols as f32;
            self.line(pos2(gx, outer.min.y), pos2(gx, grid.min.y), self.t.border, 1.0);
        }
        for i in 1..rows.max(1) {
            let gy = grid.min.y + i as f32 * grid.height() / rows as f32;
            self.line(pos2(outer.min.x, gy), pos2(grid.min.x, gy), self.t.border, 1.0);
        }
        self.p.rect_stroke(outer, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        grid
    }
    /// One centred sheet of `rows × cols` cells.
    pub(super) fn hm_sheet_center(&self, rows: usize, cols: usize) -> Rect {
        self.hm_sheet(28.0, 12.0, 120.0, 80.0, rows, cols)
    }
    /// Two sheets with an arrow between them: `before(grid)` then `after(grid)`. In motion the
    /// arrow grows first and the "after" sheet is wiped in from the left.
    pub(super) fn hm_pair(&self, rows: usize, cols: usize, before: impl FnOnce(&Draw<'_>, Rect), after: impl FnOnce(&Draw<'_>, Rect)) {
        let b = self.hm_sheet(6.0, 12.0, 70.0, 80.0, rows, cols);
        before(self, b);
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * arrow, 52.0));
        }
        let outer = Rect::from_min_size(self.at(100.0, 12.0), vec2(70.0, 80.0));
        let wipe = self.reveal(0.3, 0.75);
        if wipe <= 0.0 {
            self.hm_sheet(100.0, 12.0, 70.0, 80.0, rows, cols);
            return;
        }
        let clip = Rect::from_min_max(outer.min - vec2(2.0, 2.0), pos2(outer.min.x + outer.width() * wipe + 2.0, outer.max.y + 2.0));
        let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clipped);
        let a = d.hm_sheet(100.0, 12.0, 70.0, 80.0, rows, cols);
        after(&d, a);
        if wipe < 1.0 {
            let rest =
                self.p.with_clip_rect(Rect::from_min_max(pos2(clip.max.x, clip.min.y), outer.max + vec2(2.0, 2.0)).intersect(self.p.clip_rect()));
            self.with_painter(&rest).hm_sheet(100.0, 12.0, 70.0, 80.0, rows, cols);
        }
    }
    /// A "text" line in a cell, right-aligned (Arabic runs right-to-left), `frac` of its width.
    pub(super) fn hm_tline(&self, cell: Rect, frac: f32, c: Color32, w: f32) {
        let y = cell.center().y;
        let x1 = cell.max.x - 3.0;
        self.line(pos2(x1 - (cell.width() - 6.0) * frac.clamp(0.0, 1.0), y), pos2(x1, y), c, w);
    }
    /// Fills a `rows × cols` grid with text lines: a dark header row when `header`, grey below.
    pub(super) fn hm_rows(&self, g: Rect, rows: usize, cols: usize, header: bool) {
        let fr = [0.7, 0.5, 0.62, 0.45, 0.68, 0.55];
        for r in 0..rows {
            for c in 0..cols {
                let f = fr.get((r * 3 + c) % fr.len()).copied().unwrap_or(0.6);
                let cell = self.cell(g, rows, cols, r, c);
                if header && r == 0 {
                    self.hm_tline(cell, f, self.ink(), 2.5);
                } else {
                    self.hm_tline(cell, f, self.grey(), 1.5);
                }
            }
        }
    }
    /// A sample string as the text renderer needs it. egui 0.36 shapes a run of Arabic-script
    /// characters right-to-left as a whole, with no bidi pass, and Arabic-Indic digits are
    /// Arabic script: a number like ١٢٥٠ would come out as ٠٥٢١. Reversing a string that holds
    /// digits but no letters makes it read right; drop this once the renderer orders digits.
    pub(super) fn hm_shape(&self, s: &str) -> String {
        let letters = s.chars().any(|c| ('\u{0621}'..='\u{064A}').contains(&c));
        let digits = s.chars().any(|c| ('\u{0660}'..='\u{0669}').contains(&c));
        if digits && !letters { s.chars().rev().collect() } else { s.to_string() }
    }
    /// Text in a cell, right-aligned.
    pub(super) fn hm_txt(&self, cell: Rect, s: &str, c: Color32) {
        self.hm_txt_sz(cell, s, c, CELL_FONT);
    }
    /// Text in a cell, right-aligned, at `size`.
    pub(super) fn hm_txt_sz(&self, cell: Rect, s: &str, c: Color32, size: f32) {
        self.label(pos2(cell.max.x - 2.0, cell.center().y), &self.hm_shape(s), size, c, Align2::RIGHT_CENTER);
    }
    /// Text in a cell, centred.
    pub(super) fn hm_txt_c(&self, cell: Rect, s: &str, c: Color32) {
        self.label(cell.center(), &self.hm_shape(s), CELL_FONT, c, Align2::CENTER_CENTER);
    }
    /// Text in a cell, left-aligned (a currency symbol, a formula).
    pub(super) fn hm_txt_l(&self, cell: Rect, s: &str, c: Color32) {
        self.label(pos2(cell.min.x + 2.0, cell.center().y), &self.hm_shape(s), CELL_FONT, c, Align2::LEFT_CENTER);
    }
    /// A cell's background.
    pub(super) fn hm_fill(&self, cell: Rect, c: Color32) {
        self.p.rect_filled(cell.shrink(0.5), 0.0, c);
    }
    /// A selected range: tinted, with an accent frame.
    pub(super) fn hm_sel(&self, r: Rect) {
        self.p.rect_filled(r, 0.0, self.t.selection);
        self.p.rect_stroke(r, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
    }
    /// A clipboard (a board with a clip) centred at `c`, holding a 2 × 2 patch of cells drawn in
    /// `lines`, faded by `alpha`.
    pub(super) fn hm_clipboard(&self, c: Pos2, lines: Color32, alpha: f32) {
        let board = Rect::from_center_size(c, vec2(30.0, 38.0));
        self.p.rect_filled(board, 2.0, self.t.hover);
        self.p.rect_stroke(board, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let clip = Rect::from_center_size(pos2(c.x, board.min.y), vec2(12.0, 6.0));
        self.p.rect_filled(clip, 1.0, self.t.text);
        let patch = Rect::from_center_size(c + vec2(0.0, 3.0), vec2(20.0, 20.0));
        self.p.rect_filled(patch, 0.0, Color32::WHITE.linear_multiply(alpha));
        self.grid(patch, 2, 2, self.t.border.linear_multiply(alpha));
        for r in 0..2 {
            for col in 0..2 {
                self.hm_tline(self.cell(patch, 2, 2, r, col), 0.7, lines.linear_multiply(alpha), 1.5);
            }
        }
    }
    /// The row of sheet tabs under a sheet: `names`, the `active` one lit; returns each tab.
    pub(super) fn hm_tabs(&self, y: f32, names: &[&str], active: usize) -> Vec<Rect> {
        let mut x = 30.0;
        let mut out = Vec::with_capacity(names.len());
        for (i, n) in names.iter().enumerate() {
            let w = n.chars().count() as f32 * 5.0 + 12.0;
            let r = Rect::from_min_size(self.at(x, y), vec2(w, 12.0));
            let on = i == active;
            self.p.rect_filled(r, 2.0, if on { Color32::WHITE } else { self.t.hover });
            self.p.rect_stroke(r, 2.0, self.stroke(if on { self.accent() } else { self.t.border_strong }), egui::StrokeKind::Inside);
            self.label(r.center(), n, 6.5, if on { self.accent() } else { self.t.text }, Align2::CENTER_CENTER);
            out.push(r);
            x += w + 3.0;
        }
        out
    }
    /// A filter funnel centred at `c`.
    pub(super) fn hm_funnel(&self, c: Pos2, s: f32, col: Color32) {
        let pts = vec![
            c + vec2(-s, -s),
            c + vec2(s, -s),
            c + vec2(s * 0.2, s * 0.2),
            c + vec2(s * 0.2, s),
            c + vec2(-s * 0.2, s * 0.7),
            c + vec2(-s * 0.2, s * 0.2),
        ];
        self.p.add(Shape::convex_polygon(pts, col, Stroke::NONE));
    }
    /// A small filter button in the corner of a header cell.
    pub(super) fn hm_filter_button(&self, cell: Rect, on: bool) {
        let r = Rect::from_min_size(pos2(cell.min.x + 1.5, cell.center().y - 4.0), vec2(8.0, 8.0));
        self.p.rect_filled(r, 1.0, if on { self.accent() } else { Color32::WHITE });
        self.p.rect_stroke(r, 1.0, self.stroke(if on { self.accent() } else { self.t.border_strong }), egui::StrokeKind::Inside);
        let c = r.center();
        let tri = vec![c + vec2(-2.5, -1.5), c + vec2(2.5, -1.5), c + vec2(0.0, 2.0)];
        self.p.add(Shape::convex_polygon(tri, if on { Color32::WHITE } else { self.ink() }, Stroke::NONE));
    }
    /// A padlock centred at `c`, open (shackle lifted) or closed.
    pub(super) fn hm_lock(&self, c: Pos2, open: bool) {
        let body = Rect::from_center_size(c + vec2(0.0, 4.0), vec2(14.0, 11.0));
        self.p.rect_filled(body, 2.0, self.accent());
        let lift = if open { -4.0 } else { 0.0 };
        let top = c.y - 3.0 + lift;
        let pts: Vec<Pos2> = (0..=8)
            .map(|i| {
                let a = std::f32::consts::PI * (1.0 + i as f32 / 8.0);
                pos2(c.x + 5.0 * a.cos(), top + 5.0 * a.sin())
            })
            .collect();
        self.p.add(Shape::line(pts, Stroke::new(2.0, self.ink())));
        self.line(pos2(c.x - 5.0, top), pos2(c.x - 5.0, body.min.y), self.ink(), 2.0);
        if !open {
            self.line(pos2(c.x + 5.0, top), pos2(c.x + 5.0, body.min.y), self.ink(), 2.0);
        }
        self.p.circle_filled(body.center(), 1.6, Color32::WHITE);
    }
    /// Text turned by `angle` (egui radians, clockwise), its top-left at `pos`.
    pub(super) fn hm_rot_text(&self, pos: Pos2, s: &str, angle: f32, c: Color32) {
        let galley = self.p.layout_no_wrap(super::artwork(s).to_string(), self.font(CELL_FONT), c);
        let mut shape = egui::epaint::TextShape::new(pos, galley, c);
        shape.angle = angle;
        self.p.add(shape);
    }
    /// A brush: a handle from `a` to the tip near `b`, with a wide accent tip.
    pub(super) fn hm_brush(&self, a: Pos2, b: Pos2) {
        let dir = (b - a).normalized();
        let mid = b - dir * 8.0;
        self.line(a, mid, self.ink(), 2.5);
        self.line(mid, b, self.accent(), 5.0);
    }
    /// An eraser (a slanted block) centred at `c`.
    pub(super) fn hm_eraser(&self, c: Pos2) {
        let pts = vec![c + vec2(-12.0, 2.0), c + vec2(-2.0, -8.0), c + vec2(10.0, 4.0), c + vec2(0.0, 14.0)];
        self.p.add(Shape::convex_polygon(pts.clone(), Color32::from_rgb(240, 160, 170), Stroke::NONE));
        self.p.add(Shape::closed_line(pts, Stroke::new(1.2, self.ink())));
        self.line(c + vec2(-7.0, -3.0), c + vec2(5.0, 9.0), self.ink(), 1.2);
    }
    /// A small comment mark in the top-right corner of a cell.
    pub(super) fn hm_note_mark(&self, cell: Rect, c: Color32) {
        let pts = vec![cell.right_top() + vec2(-6.0, 1.0), cell.right_top() + vec2(-1.0, 1.0), cell.right_top() + vec2(-1.0, 6.0)];
        self.p.add(Shape::convex_polygon(pts, c, Stroke::NONE));
    }
    /// A slanted (italic) text line in a cell.
    pub(super) fn hm_italic_line(&self, cell: Rect, frac: f32, c: Color32) {
        let y = cell.center().y;
        let x1 = cell.max.x - 3.0;
        let x0 = x1 - (cell.width() - 6.0) * frac;
        let pts = vec![pos2(x0 + 1.5, y - 1.5), pos2(x1 + 1.5, y - 1.5), pos2(x1 - 1.5, y + 1.5), pos2(x0 - 1.5, y + 1.5)];
        self.p.add(Shape::convex_polygon(pts, c, Stroke::NONE));
    }

    // ---- clipboard ----

    /// The clipboard's cells land on the sheet. 0: everything (accent cells with a formula);
    /// 1: values only (plain numbers); 2: formulas only (the formula, no fill); 3: formats only
    /// (the fill lands, the numbers underneath stay).
    pub(super) fn hm_paste(&self, kind: u8) {
        let coloured = matches!(kind, 0 | 3);
        self.hm_clipboard(self.at(30.0, 52.0), if coloured { self.red() } else { self.ink() }, 1.0);
        self.label(
            self.at(30.0, 84.0),
            &self.hm_shape(if kind == 1 { "١٥٠" } else { "=SUM" }),
            6.5,
            if coloured { self.red() } else { self.ink() },
            Align2::CENTER_CENTER,
        );
        let go = self.reveal(0.0, 0.25);
        if go > 0.0 {
            self.arrow_right(self.at(54.0, 52.0), self.at(54.0 + 20.0 * go, 52.0));
        }
        let g = self.hm_sheet(84.0, 14.0, 84.0, 76.0, 4, 3);
        self.hm_rows(g, 4, 3, true);
        let k = self.reveal(0.25, 0.7);
        let target = Rect::from_min_max(self.cell(g, 4, 3, 1, 1).min, self.cell(g, 4, 3, 2, 2).max);
        self.p.rect_filled(target, 0.0, Color32::WHITE);
        if k > 0.0 {
            let a = k;
            for r in 1..3 {
                for c in 1..3 {
                    let cell = self.cell(g, 4, 3, r, c);
                    if coloured {
                        self.hm_fill(cell, self.yellow().linear_multiply(a));
                    }
                    match kind {
                        1 => self.hm_txt(cell, "١٥٠", self.ink().linear_multiply(a)),
                        3 => self.hm_tline(cell, 0.5, self.grey().linear_multiply(a), 1.5),
                        _ => self.hm_txt_l(cell, "=A1", self.accent().linear_multiply(a)),
                    }
                }
            }
        }
        self.hm_sel(target);
    }
    /// Cut (`remove`) or Copy: a selected block on the left sheet; on the right it is gone
    /// (cut) or still there (copy), and a clipboard holding it fades in.
    pub(super) fn hm_clip(&self, remove: bool) {
        let b = self.hm_sheet(6.0, 12.0, 66.0, 80.0, 4, 3);
        self.hm_rows(b, 4, 3, true);
        let block = Rect::from_min_max(self.cell(b, 4, 3, 1, 1).min, self.cell(b, 4, 3, 2, 2).max);
        self.hm_sel(block);
        let go = self.reveal(0.0, 0.2);
        if go > 0.0 {
            self.arrow_right(self.at(76.0, 52.0), self.at(76.0 + 16.0 * go, 52.0));
        }
        let k = self.reveal(0.2, 0.6);
        let a = self.hm_sheet(96.0, 12.0, 66.0, 80.0, 4, 3);
        self.hm_rows(a, 4, 3, true);
        let block2 = Rect::from_min_max(self.cell(a, 4, 3, 1, 1).min, self.cell(a, 4, 3, 2, 2).max);
        if remove {
            self.p.rect_filled(block2.shrink(0.5), 0.0, Color32::WHITE.linear_multiply(k));
            self.dashed_rect(block2, self.accent().linear_multiply(k));
        } else {
            self.dashed_rect(block2, self.accent());
        }
        self.hm_clipboard(self.at(150.0, 80.0), self.accent(), k.max(0.15));
    }
    /// A brush carries the yellow bold look of one header onto another.
    pub(super) fn hm_format_painter(&self) {
        self.hm_pair(
            4,
            3,
            |d, g| {
                d.hm_rows(g, 4, 3, false);
                let c = d.cell(g, 4, 3, 0, 0);
                d.hm_fill(c, d.yellow());
                d.hm_tline(c, 0.7, d.ink(), 2.5);
                d.hm_sel(d.cell(g, 4, 3, 2, 0));
            },
            |d, g| {
                d.hm_rows(g, 4, 3, false);
                for r in [0usize, 2] {
                    let c = d.cell(g, 4, 3, r, 0);
                    d.hm_fill(c, d.yellow());
                    d.hm_tline(c, 0.7, d.ink(), 2.5);
                }
            },
        );
        self.hm_brush(self.at(80.0, 20.0), self.at(96.0, 40.0));
    }

    // ---- font ----

    /// A word in a cell grows (or shrinks) one step; the row follows.
    pub(super) fn hm_font_size(&self, bigger: bool) {
        let (s0, s1) = if bigger { (7.0, 10.0) } else { (10.0, 7.0) };
        self.hm_pair(
            3,
            2,
            |d, g| {
                d.hm_rows(g, 3, 2, false);
                let c = d.cell(g, 3, 2, 0, 0);
                d.hm_fill(c, Color32::WHITE);
                d.p.rect_filled(d.cell(g, 3, 2, 0, 1).shrink(0.5), 0.0, Color32::WHITE);
                d.label(pos2(c.max.x - 2.0, c.center().y), "عنوان", s0, d.ink(), Align2::RIGHT_CENTER);
                d.hm_sel(c);
            },
            |d, g| {
                d.hm_rows(g, 3, 2, false);
                let c = d.cell(g, 3, 2, 0, 0);
                d.hm_fill(c, Color32::WHITE);
                d.p.rect_filled(d.cell(g, 3, 2, 0, 1).shrink(0.5), 0.0, Color32::WHITE);
                d.label(pos2(c.max.x - 2.0, c.center().y), "عنوان", s1, d.accent(), Align2::RIGHT_CENTER);
            },
        );
        let (a, b) = if bigger { (self.at(84.0, 30.0), self.at(92.0, 22.0)) } else { (self.at(84.0, 22.0), self.at(92.0, 30.0)) };
        self.label(a, "A", 7.0, self.grey(), Align2::CENTER_CENTER);
        self.label(b, "A", 11.0, self.accent(), Align2::CENTER_CENTER);
    }
    /// The header row's thin lines become thick.
    pub(super) fn hm_bold(&self) {
        self.hm_pair(
            4,
            3,
            |d, g| {
                d.hm_rows(g, 4, 3, false);
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 0, 0).min, d.cell(g, 4, 3, 0, 2).max));
            },
            |d, g| {
                d.hm_rows(g, 4, 3, false);
                for c in 0..3 {
                    let cell = d.cell(g, 4, 3, 0, c);
                    d.hm_fill(cell, Color32::WHITE);
                    d.hm_tline(cell, [0.7, 0.5, 0.62].get(c).copied().unwrap_or(0.6), d.accent(), 3.0);
                }
            },
        );
    }
    /// The lines of one column lean over.
    pub(super) fn hm_italic(&self) {
        self.hm_pair(
            4,
            3,
            |d, g| {
                d.hm_rows(g, 4, 3, true);
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 1, 1).min, d.cell(g, 4, 3, 3, 1).max));
            },
            |d, g| {
                d.hm_rows(g, 4, 3, true);
                for r in 1..4 {
                    let cell = d.cell(g, 4, 3, r, 1);
                    d.hm_fill(cell, Color32::WHITE);
                    d.hm_italic_line(cell, 0.55, d.accent());
                }
            },
        );
    }
    /// A line through the middle of one row's text.
    pub(super) fn hm_strike(&self) {
        self.hm_pair(
            4,
            3,
            |d, g| {
                d.hm_rows(g, 4, 3, true);
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 2, 0).min, d.cell(g, 4, 3, 2, 2).max));
            },
            |d, g| {
                d.hm_rows(g, 4, 3, true);
                for c in 0..3 {
                    let cell = d.cell(g, 4, 3, 2, c);
                    d.hm_fill(cell, Color32::WHITE);
                    let f = [0.7, 0.5, 0.62].get(c).copied().unwrap_or(0.6);
                    d.hm_tline(cell, f, d.grey(), 3.5);
                    let y = cell.center().y;
                    d.line(pos2(cell.max.x - 3.0 - (cell.width() - 6.0) * f - 2.0, y), pos2(cell.max.x - 1.0, y), d.accent(), 1.2);
                }
            },
        );
    }
    /// One or two lines under the text of a cell.
    pub(super) fn hm_underline(&self, double: bool) {
        self.hm_pair(
            4,
            2,
            |d, g| {
                d.hm_rows(g, 4, 2, true);
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 2, 3, 0).min, d.cell(g, 4, 2, 3, 1).max));
            },
            |d, g| {
                d.hm_rows(g, 4, 2, true);
                for c in 0..2 {
                    let cell = d.cell(g, 4, 2, 3, c);
                    let y = cell.center().y + 4.0;
                    let w = (cell.width() - 6.0) * if c == 0 { 0.45 } else { 0.7 };
                    d.line(pos2(cell.max.x - 3.0 - w, y), pos2(cell.max.x - 3.0, y), d.accent(), 1.2);
                    if double {
                        d.line(pos2(cell.max.x - 3.0 - w, y + 2.5), pos2(cell.max.x - 3.0, y + 2.5), d.accent(), 1.2);
                    }
                }
            },
        );
    }
    /// The total row turns yellow.
    pub(super) fn hm_fill_color(&self) {
        self.hm_pair(
            4,
            3,
            |d, g| {
                d.hm_rows(g, 4, 3, true);
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 3, 0).min, d.cell(g, 4, 3, 3, 2).max));
            },
            |d, g| {
                for c in 0..3 {
                    d.hm_fill(d.cell(g, 4, 3, 3, c), d.yellow());
                }
                d.hm_rows(g, 4, 3, true);
            },
        );
        // A paint bucket's drop.
        self.p.circle_filled(self.at(88.0, 30.0), 4.0, self.yellow());
        self.p.circle_stroke(self.at(88.0, 30.0), 4.0, Stroke::new(1.0, self.ink()));
    }
    /// The numbers of one column turn red.
    pub(super) fn hm_font_color(&self) {
        let nums = ["١٢٠", "-٤٥", "٨٠", "-٣٠"];
        self.hm_pair(
            4,
            2,
            |d, g| {
                d.hm_rows(g, 4, 2, false);
                for (r, n) in nums.iter().enumerate() {
                    let cell = d.cell(g, 4, 2, r, 1);
                    d.hm_fill(cell, Color32::WHITE);
                    d.hm_txt(cell, n, d.ink());
                }
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 2, 0, 1).min, d.cell(g, 4, 2, 3, 1).max));
            },
            |d, g| {
                d.hm_rows(g, 4, 2, false);
                for (r, n) in nums.iter().enumerate() {
                    let cell = d.cell(g, 4, 2, r, 1);
                    d.hm_fill(cell, Color32::WHITE);
                    d.hm_txt(cell, n, if n.starts_with('-') { d.red() } else { d.ink() });
                }
            },
        );
        self.label(self.at(88.0, 30.0), "A", 11.0, self.red(), Align2::CENTER_CENTER);
        self.line(self.at(83.0, 37.0), self.at(93.0, 37.0), self.red(), 2.5);
    }

    // ---- alignment ----

    /// Header text at an angle. 0: the fan of choices; 1: counterclockwise; 2: clockwise;
    /// 3: vertical (stacked letters); 4: rotated up; 5: rotated down.
    pub(super) fn hm_orient(&self, which: u8) {
        let g = self.hm_sheet(28.0, 12.0, 120.0, 80.0, 3, 4);
        // A tall header row: paint over the first two grid rows as one.
        let hdr = Rect::from_min_max(self.cell(g, 3, 4, 0, 0).min, self.cell(g, 3, 4, 1, 3).max);
        self.p.rect_filled(hdr.shrink(0.5), 0.0, Color32::WHITE);
        for j in 1..4 {
            let x = g.min.x + j as f32 * g.width() / 4.0;
            self.line(pos2(x, hdr.min.y), pos2(x, hdr.max.y), self.t.border, 1.0);
        }
        for c in 0..4 {
            self.hm_tline(self.cell(g, 3, 4, 2, c), 0.5, self.grey(), 1.5);
        }
        let k = self.reveal(0.1, 0.6);
        let months = ["كانون", "شباط", "آذار", "نيسان"];
        for (c, m) in months.iter().enumerate() {
            let cell = Rect::from_min_max(self.cell(g, 3, 4, 0, c).min, self.cell(g, 3, 4, 1, c).max);
            let kind = if which == 0 { [1u8, 2, 3, 4].get(c).copied().unwrap_or(1) } else { which };
            let col = self.accent();
            match kind {
                1 => self.hm_rot_text(pos2(cell.min.x + 4.0, cell.max.y - 6.0), m, -std::f32::consts::FRAC_PI_4 * k, col),
                2 => self.hm_rot_text(pos2(cell.min.x + 8.0, cell.min.y + 6.0), m, std::f32::consts::FRAC_PI_4 * k, col),
                3 => {
                    let n = m.chars().count().max(1) as f32;
                    let step = (cell.height() - 6.0) / n;
                    for (i, ch) in m.chars().enumerate() {
                        let y = cell.min.y + 3.0 + step * (i as f32 + 0.5);
                        self.label(pos2(cell.center().x, y), &ch.to_string(), 6.0, col, Align2::CENTER_CENTER);
                    }
                }
                4 => self.hm_rot_text(pos2(cell.center().x - 4.0, cell.max.y - 4.0), m, -std::f32::consts::FRAC_PI_2 * k, col),
                5 => self.hm_rot_text(pos2(cell.center().x + 4.0, cell.min.y + 4.0), m, std::f32::consts::FRAC_PI_2 * k, col),
                _ => self.hm_txt_c(cell, m, col),
            }
        }
    }
    /// Text spilling over the next cells is wrapped into a taller cell.
    pub(super) fn hm_wrap(&self) {
        self.hm_pair(
            4,
            3,
            |d, g| {
                d.hm_rows(g, 4, 3, true);
                let c = d.cell(g, 4, 3, 1, 1);
                d.p.rect_filled(Rect::from_min_max(d.cell(g, 4, 3, 1, 0).min, c.max).shrink(0.5), 0.0, Color32::WHITE);
                let y = c.center().y;
                d.line(pos2(c.max.x - 3.0, y), pos2(c.min.x - 16.0, y), d.accent(), 1.5);
                d.hm_sel(c);
            },
            |d, g| {
                d.hm_rows(g, 4, 3, true);
                let c = d.cell(g, 4, 3, 1, 1);
                let tall = Rect::from_min_max(c.min, pos2(c.max.x, c.max.y + c.height()));
                d.p.rect_filled(tall.shrink(0.5), 0.0, Color32::WHITE);
                d.p.rect_filled(d.cell(g, 4, 3, 2, 1).shrink(0.5), 0.0, Color32::WHITE);
                for i in 0..3 {
                    let y = tall.min.y + 6.0 + i as f32 * 8.0;
                    let w = [0.8, 0.7, 0.4].get(i).copied().unwrap_or(0.5) * (c.width() - 6.0);
                    d.line(pos2(c.max.x - 3.0 - w, y), pos2(c.max.x - 3.0, y), d.accent(), 1.5);
                }
                d.p.rect_stroke(tall, 0.0, Stroke::new(1.0, d.accent()), egui::StrokeKind::Inside);
            },
        );
    }
    /// The text of three cells steps away from (or back to) the cell's edge.
    pub(super) fn hm_indent(&self, inc: bool) {
        let (a, b) = if inc { (0.0, 6.0) } else { (6.0, 0.0) };
        self.hm_pair(
            4,
            2,
            |d, g| {
                d.hm_rows(g, 4, 2, true);
                for r in 1..4 {
                    let c = d.cell(g, 4, 2, r, 1);
                    d.hm_fill(c, Color32::WHITE);
                    let y = c.center().y;
                    d.line(pos2(c.max.x - 3.0 - a - 20.0, y), pos2(c.max.x - 3.0 - a, y), d.grey(), 1.5);
                }
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 2, 1, 1).min, d.cell(g, 4, 2, 3, 1).max));
            },
            |d, g| {
                d.hm_rows(g, 4, 2, true);
                for r in 1..4 {
                    let c = d.cell(g, 4, 2, r, 1);
                    d.hm_fill(c, Color32::WHITE);
                    let y = c.center().y;
                    d.line(pos2(c.max.x - 3.0 - b - 20.0, y), pos2(c.max.x - 3.0 - b, y), d.accent(), 1.5);
                }
            },
        );
        if inc {
            self.arrow_left(self.at(94.0, 30.0), self.at(82.0, 30.0));
        } else {
            self.arrow_right(self.at(82.0, 30.0), self.at(94.0, 30.0));
        }
    }
    /// Three numbers in one column, before and after a number format.
    pub(super) fn hm_numfmt(&self, before: &[&str], after: &[&str]) {
        self.hm_pair(
            4,
            1,
            |d, g| {
                d.hm_tline(d.cell(g, 4, 1, 0, 0), 0.5, d.ink(), 2.5);
                for (i, s) in before.iter().enumerate() {
                    d.hm_txt(d.cell(g, 4, 1, i + 1, 0), s, d.ink());
                }
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 1, 1, 0).min, d.cell(g, 4, 1, 3, 0).max));
            },
            |d, g| {
                d.hm_tline(d.cell(g, 4, 1, 0, 0), 0.5, d.ink(), 2.5);
                for (i, s) in after.iter().enumerate() {
                    d.hm_txt(d.cell(g, 4, 1, i + 1, 0), s, d.accent());
                }
            },
        );
    }
    /// Accounting money: the symbol at the left edge, the amount at the right.
    pub(super) fn hm_currency(&self) {
        let before = ["١٢٥٠", "٣٠٠٠", "٨٧٥"];
        let after = ["١٬٢٥٠٫٠٠", "٣٬٠٠٠٫٠٠", "٨٧٥٫٠٠"];
        self.hm_pair(
            4,
            1,
            |d, g| {
                d.hm_tline(d.cell(g, 4, 1, 0, 0), 0.5, d.ink(), 2.5);
                for (i, s) in before.iter().enumerate() {
                    d.hm_txt(d.cell(g, 4, 1, i + 1, 0), s, d.ink());
                }
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 1, 1, 0).min, d.cell(g, 4, 1, 3, 0).max));
            },
            |d, g| {
                d.hm_tline(d.cell(g, 4, 1, 0, 0), 0.5, d.ink(), 2.5);
                for (i, s) in after.iter().enumerate() {
                    let c = d.cell(g, 4, 1, i + 1, 0);
                    d.hm_txt_l(c, "$", d.accent());
                    d.hm_txt(c, s, d.accent());
                }
            },
        );
        for (i, s) in ["$", "£", "€", "¥"].iter().enumerate() {
            self.label(self.at(78.0 + 7.0 * i as f32, 24.0), s, 7.0, self.grey(), Align2::CENTER_CENTER);
        }
    }

    // ---- styles ----

    /// Numbers above a threshold turn green and get data bars, one after another.
    pub(super) fn hm_cond_format(&self) {
        let g = self.hm_sheet_center(4, 3);
        self.hm_rows(g, 4, 3, true);
        let nums = [("٤٢٠", 0.3), ("١٢٥٠", 0.9), ("٩٨٠", 0.7), ("١٦٠٠", 1.0), ("٣٠٠", 0.2), ("١١٠٠", 0.8)];
        for (i, (n, f)) in nums.iter().enumerate() {
            let (r, c) = (1 + i / 2, 1 + i % 2);
            let cell = self.cell(g, 4, 3, r, c);
            self.hm_fill(cell, Color32::WHITE);
            let k = self.reveal(0.1 + i as f32 * 0.08, 0.3 + i as f32 * 0.08);
            if c == 1 {
                let bar = Rect::from_min_max(
                    pos2(cell.min.x + 1.0, cell.min.y + 3.0),
                    pos2(cell.min.x + 1.0 + (cell.width() - 2.0) * f * k, cell.max.y - 3.0),
                );
                self.p.rect_filled(bar, 0.0, self.accent().linear_multiply(0.35));
                self.hm_txt(cell, n, self.ink());
            } else {
                if *f > 0.6 {
                    self.hm_fill(cell, self.green().linear_multiply(0.3 * k));
                }
                self.hm_txt(cell, n, if *f > 0.6 { self.green() } else { self.ink() });
            }
        }
    }
    /// A plain list becomes a banded table with filter buttons.
    pub(super) fn hm_format_table(&self) {
        self.hm_pair(
            5,
            3,
            |d, g| {
                d.hm_rows(g, 5, 3, false);
                d.hm_sel(g);
            },
            |d, g| {
                for c in 0..3 {
                    d.hm_fill(d.cell(g, 5, 3, 0, c), d.accent());
                }
                for r in [2usize, 4] {
                    for c in 0..3 {
                        d.hm_fill(d.cell(g, 5, 3, r, c), d.accent().linear_multiply(0.15));
                    }
                }
                for c in 0..3 {
                    let cell = d.cell(g, 5, 3, 0, c);
                    d.hm_tline(cell, 0.35, Color32::WHITE, 2.0);
                    d.hm_filter_button(cell, false);
                }
                for r in 1..5 {
                    for c in 0..3 {
                        d.hm_tline(d.cell(g, 5, 3, r, c), [0.6, 0.45, 0.7].get((r + c) % 3).copied().unwrap_or(0.5), d.grey(), 1.5);
                    }
                }
            },
        );
    }
    /// A gallery of style chips, and the total row taking one of them.
    pub(super) fn hm_cell_styles(&self) {
        let styles: [(&str, Color32, Color32); 4] = [
            ("عنوان", Color32::WHITE, self.ink()),
            ("جيد", Color32::from_rgb(200, 240, 200), self.green()),
            ("سيئ", Color32::from_rgb(250, 200, 200), self.red()),
            ("مجموع", self.t.hover, self.ink()),
        ];
        for (i, (name, fill, ink)) in styles.iter().enumerate() {
            let r = Rect::from_min_size(self.at(8.0, 12.0 + i as f32 * 21.0), vec2(56.0, 17.0));
            self.p.rect_filled(r, 2.0, *fill);
            self.p.rect_stroke(r, 2.0, self.stroke(if i == 3 { self.accent() } else { self.t.border_strong }), egui::StrokeKind::Inside);
            if i == 3 {
                self.line(pos2(r.min.x + 3.0, r.min.y + 2.0), pos2(r.max.x - 3.0, r.min.y + 2.0), self.ink(), 1.5);
            }
            self.label(r.center(), name, 7.5, *ink, Align2::CENTER_CENTER);
        }
        let k = self.reveal(0.2, 0.6);
        self.arrow_right(self.at(70.0, 82.0), self.at(70.0 + 14.0 * k, 82.0));
        let g = self.hm_sheet(90.0, 14.0, 78.0, 76.0, 4, 2);
        self.hm_rows(g, 4, 2, true);
        if k > 0.0 {
            for c in 0..2 {
                let cell = self.cell(g, 4, 2, 3, c);
                self.hm_fill(cell, self.t.hover.linear_multiply(k));
                self.line(pos2(cell.min.x, cell.min.y + 0.5), pos2(cell.max.x, cell.min.y + 0.5), self.ink().linear_multiply(k), 1.5);
                self.hm_tline(cell, if c == 0 { 0.6 } else { 0.45 }, self.ink(), 2.5);
            }
        }
    }

    // ---- cells ----

    /// A row (or column) is inserted into — or deleted from — a sheet; the others slide.
    pub(super) fn hm_rowcol(&self, insert: bool, rows: bool) {
        let (nr, nc) = if rows { (4usize, 2usize) } else { (3, 4) };
        let (nr2, nc2) = if rows { (5, 2) } else { (3, 5) };
        self.hm_pair(
            nr,
            nc,
            |d, g| {
                d.hm_rows(g, nr, nc, true);
                let sel = if rows {
                    Rect::from_min_max(d.cell(g, nr, nc, 1, 0).min, d.cell(g, nr, nc, 1, nc - 1).max)
                } else {
                    Rect::from_min_max(d.cell(g, nr, nc, 0, 1).min, d.cell(g, nr, nc, nr - 1, 1).max)
                };
                d.hm_sel(sel);
            },
            |d, g| {
                if insert {
                    d.hm_rows(g, nr2, nc2, true);
                    let new = if rows {
                        Rect::from_min_max(d.cell(g, nr2, nc2, 1, 0).min, d.cell(g, nr2, nc2, 1, nc2 - 1).max)
                    } else {
                        Rect::from_min_max(d.cell(g, nr2, nc2, 0, 1).min, d.cell(g, nr2, nc2, nr2 - 1, 1).max)
                    };
                    d.p.rect_filled(new.shrink(0.5), 0.0, Color32::WHITE);
                    d.p.rect_filled(new.shrink(0.5), 0.0, d.accent().linear_multiply(0.2));
                    d.p.rect_stroke(new, 0.0, Stroke::new(1.5, d.accent()), egui::StrokeKind::Inside);
                } else {
                    // The same grid, the rows (columns) after the gap moved up (over): the last one empty.
                    d.hm_rows(g, nr, nc, true);
                    let last = if rows {
                        Rect::from_min_max(d.cell(g, nr, nc, nr - 1, 0).min, d.cell(g, nr, nc, nr - 1, nc - 1).max)
                    } else {
                        Rect::from_min_max(d.cell(g, nr, nc, 0, nc - 1).min, d.cell(g, nr, nc, nr - 1, nc - 1).max)
                    };
                    d.p.rect_filled(last.shrink(0.5), 0.0, Color32::WHITE);
                }
            },
        );
        if rows {
            if insert {
                self.arrow_down(self.at(88.0, 20.0), self.at(88.0, 36.0));
            } else {
                self.arrow_up(self.at(88.0, 36.0), self.at(88.0, 20.0));
            }
        } else if insert {
            self.arrow_right(self.at(80.0, 28.0), self.at(96.0, 28.0));
        } else {
            self.arrow_left(self.at(96.0, 28.0), self.at(80.0, 28.0));
        }
    }
    /// The Format menu: a column grows wider under a measuring arrow.
    pub(super) fn hm_format_menu(&self) {
        let g = self.hm_sheet_center(4, 3);
        self.hm_rows(g, 4, 3, true);
        let k = self.reveal(0.1, 0.6);
        let c0 = self.cell(g, 4, 3, 0, 0);
        let x = c0.max.x + 10.0 * k;
        self.p.rect_filled(Rect::from_min_max(pos2(c0.min.x, g.min.y), pos2(x, g.max.y)).shrink(0.5), 0.0, Color32::WHITE);
        for r in 0..4 {
            let y = g.min.y + r as f32 * g.height() / 4.0;
            self.line(pos2(c0.min.x, y), pos2(x, y), self.t.border, 1.0);
            let cell = Rect::from_min_max(pos2(c0.min.x, y), pos2(x, y + g.height() / 4.0));
            self.hm_tline(cell, if r == 0 { 0.8 } else { 0.7 }, if r == 0 { self.ink() } else { self.grey() }, if r == 0 { 2.5 } else { 1.5 });
        }
        self.line(pos2(x, g.min.y - HDR_H), pos2(x, g.max.y), self.accent(), 1.5);
        self.measure_h(pos2(c0.min.x, g.min.y - 10.0), pos2(x, g.min.y - 10.0));
    }
    /// A merge. 0: merge and centre a title; 1: merge across (row by row); 2: merge, text at the
    /// edge; 3: unmerge.
    pub(super) fn hm_merge(&self, kind: u8) {
        let title = |d: &Draw<'_>, g: Rect, rows: usize| {
            d.hm_rows(g, rows, 3, false);
            let tl = d.cell(g, rows, 3, 0, 0);
            d.hm_fill(tl, Color32::WHITE);
            d.hm_txt(tl, "اسم", d.ink());
            if kind == 1 {
                let c = d.cell(g, rows, 3, 1, 0);
                d.hm_fill(c, Color32::WHITE);
                d.hm_txt(c, "فرع", d.ink());
            }
        };
        let merged = |d: &Draw<'_>, g: Rect, rows: usize, r: usize, text: &str, centre: bool| {
            let wide = Rect::from_min_max(d.cell(g, rows, 3, r, 0).min, d.cell(g, rows, 3, r, 2).max);
            d.p.rect_filled(wide.shrink(0.5), 0.0, Color32::WHITE);
            d.p.rect_stroke(wide, 0.0, Stroke::new(1.2, d.accent()), egui::StrokeKind::Inside);
            if centre {
                d.hm_txt_c(wide, text, d.accent());
            } else {
                d.hm_txt(wide, text, d.accent());
            }
        };
        match kind {
            0 => self.hm_pair(
                4,
                3,
                |d, g| {
                    title(d, g, 4);
                    d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 0, 0).min, d.cell(g, 4, 3, 0, 2).max));
                },
                |d, g| {
                    d.hm_rows(g, 4, 3, false);
                    merged(d, g, 4, 0, "اسم", true);
                },
            ),
            1 => self.hm_pair(
                4,
                3,
                |d, g| {
                    title(d, g, 4);
                    d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 0, 0).min, d.cell(g, 4, 3, 1, 2).max));
                },
                |d, g| {
                    d.hm_rows(g, 4, 3, false);
                    merged(d, g, 4, 0, "اسم", false);
                    merged(d, g, 4, 1, "فرع", false);
                },
            ),
            2 => self.hm_pair(
                4,
                3,
                |d, g| {
                    title(d, g, 4);
                    d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 0, 0).min, d.cell(g, 4, 3, 1, 1).max));
                },
                |d, g| {
                    d.hm_rows(g, 4, 3, false);
                    let big = Rect::from_min_max(d.cell(g, 4, 3, 0, 0).min, d.cell(g, 4, 3, 1, 1).max);
                    d.p.rect_filled(big.shrink(0.5), 0.0, Color32::WHITE);
                    d.p.rect_stroke(big, 0.0, Stroke::new(1.2, d.accent()), egui::StrokeKind::Inside);
                    d.label(pos2(big.max.x - 2.0, big.min.y + 8.0), "اسم", CELL_FONT, d.accent(), Align2::RIGHT_CENTER);
                },
            ),
            _ => self.hm_pair(
                4,
                3,
                |d, g| {
                    d.hm_rows(g, 4, 3, false);
                    merged(d, g, 4, 0, "اسم", true);
                    d.hm_sel(Rect::from_min_max(d.cell(g, 4, 3, 0, 0).min, d.cell(g, 4, 3, 0, 2).max));
                },
                |d, g| {
                    title(d, g, 4);
                    for c in 1..3 {
                        d.hm_fill(d.cell(g, 4, 3, 0, c), Color32::WHITE);
                    }
                    for c in 0..3 {
                        d.p.rect_stroke(d.cell(g, 4, 3, 0, c), 0.0, Stroke::new(1.2, d.accent()), egui::StrokeKind::Inside);
                    }
                },
            ),
        }
    }
    /// The Format Cells dialog with its tab strip; `tab` 0 = all, 1 = Alignment lit, 2 = Number lit.
    pub(super) fn hm_format_cells(&self, tab: u8) {
        let inner = self.dialog("Format Cells", 0);
        let tabs = ["Number", "Alignment", "Font", "…"];
        let mut x = inner.min.x;
        let y = inner.min.y - 2.0;
        for (i, name) in tabs.iter().enumerate() {
            let w = name.chars().count() as f32 * 3.4 + 6.0;
            let r = Rect::from_min_size(pos2(x, y), vec2(w, 11.0));
            let lit = match tab {
                1 => i == 1,
                2 => i == 0,
                _ => false,
            };
            self.p.rect_filled(r, 1.0, if lit { Color32::WHITE } else { self.t.hover });
            self.p.rect_stroke(r, 1.0, self.stroke(if lit { self.accent() } else { self.t.border }), egui::StrokeKind::Inside);
            self.label(r.center(), name, 5.0, if lit { self.accent() } else { self.t.text }, Align2::CENTER_CENTER);
            x += w + 2.0;
        }
        let body = Rect::from_min_max(pos2(inner.min.x, y + 13.0), inner.max);
        self.p.rect_filled(body, 1.0, Color32::WHITE);
        self.p.rect_stroke(body, 1.0, self.stroke(self.t.border), egui::StrokeKind::Inside);
        match tab {
            1 => {
                // Alignment: a cell with its text sliding to the centre.
                let k = self.reveal(0.2, 0.6);
                let cell = Rect::from_center_size(body.center(), vec2(50.0, 20.0));
                self.p.rect_stroke(cell, 0.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
                let w = 22.0;
                let x1 = cell.max.x - 3.0 - (cell.width() - 6.0 - w) * 0.5 * k;
                self.line(pos2(x1 - w, cell.center().y), pos2(x1, cell.center().y), self.accent(), 2.0);
            }
            2 => {
                for (i, s) in ["١٢٣٤", "١٬٢٣٤٫٠٠", "١٢٣٤٪"].iter().enumerate() {
                    let yy = body.min.y + 8.0 + i as f32 * 11.0;
                    self.label(
                        pos2(body.max.x - 6.0, yy),
                        &self.hm_shape(s),
                        7.0,
                        if i == 1 { self.accent() } else { self.grey() },
                        Align2::RIGHT_CENTER,
                    );
                    self.line(pos2(body.min.x + 6.0, yy), pos2(body.min.x + 30.0, yy), self.grey(), 1.5);
                }
            }
            _ => {
                let rows = 3;
                for i in 0..rows {
                    let yy = body.min.y + 5.0 + i as f32 * 11.0;
                    self.line(pos2(body.min.x + 6.0, yy + 4.0), pos2(body.min.x + 28.0, yy + 4.0), self.grey(), 1.5);
                    let f = Rect::from_min_size(pos2(body.min.x + 34.0, yy), vec2(body.width() - 40.0, 8.0));
                    self.p.rect_filled(f, 1.0, self.t.input);
                    self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
                }
            }
        }
    }
    /// The Paste Special dialog: a list of choices, one ticked.
    pub(super) fn hm_paste_special_dialog(&self) {
        let inner = self.dialog("Paste Special", 0);
        let items = [("All", false), ("Values", true), ("Formats", false), ("Widths", false)];
        for (i, (name, on)) in items.iter().enumerate() {
            let y = inner.min.y + 4.0 + i as f32 * 11.0;
            let c = pos2(inner.min.x + 6.0, y);
            self.p.circle_stroke(c, 3.5, Stroke::new(1.0, self.t.border_strong));
            if *on {
                self.p.circle_filled(c, 2.0 * self.reveal(0.2, 0.5), self.accent());
            }
            self.label(pos2(inner.min.x + 14.0, y), name, 6.5, if *on { self.accent() } else { self.t.text }, Align2::LEFT_CENTER);
        }
        self.button(pos2(inner.max.x - 22.0, inner.max.y - 8.0), "OK", false);
    }
    /// The Insert / Delete cells dialog: the shift choices, with little arrows.
    pub(super) fn hm_cells_dialog(&self, insert: bool) {
        let inner = self.dialog(if insert { "Insert" } else { "Delete" }, 0);
        let items = if insert {
            ["Shift cells right", "Shift cells down", "Entire row", "Entire column"]
        } else {
            ["Shift cells left", "Shift cells up", "Entire row", "Entire column"]
        };
        for (i, name) in items.iter().enumerate() {
            let y = inner.min.y + 4.0 + i as f32 * 11.0;
            let c = pos2(inner.min.x + 6.0, y);
            self.p.circle_stroke(c, 3.5, Stroke::new(1.0, self.t.border_strong));
            if i == 1 {
                self.p.circle_filled(c, 2.0 * self.reveal(0.2, 0.5), self.accent());
            }
            self.label(pos2(inner.min.x + 14.0, y), name, 6.5, if i == 1 { self.accent() } else { self.t.text }, Align2::LEFT_CENTER);
        }
        let g = Rect::from_min_size(pos2(inner.max.x - 26.0, inner.min.y + 2.0), vec2(22.0, 30.0));
        self.grid(g, 3, 2, self.t.border);
        let k = self.reveal(0.2, 0.6);
        if insert {
            self.hm_fill(self.cell(g, 3, 2, 0, 0), self.accent().linear_multiply(0.3 * k));
            self.arrow_down(pos2(g.min.x + 5.5, g.min.y + 2.0), pos2(g.min.x + 5.5, g.min.y + 2.0 + 14.0 * k.max(0.2)));
        } else {
            self.hm_fill(self.cell(g, 3, 2, 1, 0), self.red().linear_multiply(0.3 * (1.0 - k)));
            self.arrow_up(pos2(g.min.x + 5.5, g.max.y - 2.0), pos2(g.min.x + 5.5, g.max.y - 2.0 - 14.0 * k.max(0.2)));
        }
    }
    /// The sheet-tab row. 0: a new tab appears; 1: the active tab is crossed out and goes;
    /// 2: the active tab fades (hidden); 3: a tab moves to another place.
    pub(super) fn hm_sheet_tabs(&self, kind: u8) {
        let g = self.hm_sheet(28.0, 10.0, 120.0, 66.0, 3, 3);
        self.hm_rows(g, 3, 3, true);
        let k = self.reveal(0.15, 0.6);
        match kind {
            0 => {
                let tabs = self.hm_tabs(80.0, &["ورقة١", "ورقة٢"], 1);
                if let Some(t) = tabs.get(1) {
                    self.p.rect_filled(t.expand(1.0), 2.0, self.t.panel.linear_multiply(1.0 - k));
                    self.label(pos2(t.max.x + 10.0, t.center().y), "+", 11.0, self.accent(), Align2::CENTER_CENTER);
                }
            }
            1 => {
                let tabs = self.hm_tabs(80.0, &["ورقة١", "ورقة٢", "ورقة٣"], 2);
                if let Some(t) = tabs.get(2) {
                    self.p.rect_filled(t.expand(1.0), 2.0, self.t.panel.linear_multiply(k * 0.6));
                    self.cross(pos2(t.center().x, t.min.y - 10.0));
                }
            }
            2 => {
                let tabs = self.hm_tabs(80.0, &["ورقة١", "أسعار", "ورقة٣"], 1);
                if let Some(t) = tabs.get(1) {
                    self.p.rect_filled(t.expand(1.0), 2.0, self.t.panel.linear_multiply(k * 0.85));
                    // An eye, struck through: hidden.
                    let e = pos2(t.max.x + 46.0, t.center().y);
                    self.p.circle_stroke(e, 4.0, Stroke::new(1.5, self.accent()));
                    self.p.circle_filled(e, 1.5, self.accent());
                    self.line(e + vec2(-6.0, 6.0), e + vec2(6.0, -6.0), self.accent(), 1.5);
                }
            }
            _ => {
                self.p.rect_filled(Rect::from_min_size(self.at(28.0, 62.0), vec2(120.0, 14.0)), 0.0, self.t.panel);
                let tabs = self.hm_tabs(80.0, &["ورقة١", "ورقة٢", "ورقة٣"], 0);
                if let (Some(a), Some(c)) = (tabs.first(), tabs.get(2)) {
                    let x = a.center().x + (c.max.x + 8.0 - a.center().x) * k;
                    self.p.rect_filled(Rect::from_center_size(pos2(x, a.center().y - 16.0), a.size()), 2.0, Color32::WHITE);
                    self.p.rect_stroke(
                        Rect::from_center_size(pos2(x, a.center().y - 16.0), a.size()),
                        2.0,
                        self.stroke(self.accent()),
                        egui::StrokeKind::Inside,
                    );
                    self.label(pos2(x, a.center().y - 16.0), "ورقة١", 6.5, self.accent(), Align2::CENTER_CENTER);
                    self.arrow_right(pos2(a.center().x, a.min.y - 4.0), pos2(x.max(a.center().x + 6.0), a.min.y - 4.0));
                }
            }
        }
    }
    /// The Unhide dialog: a list of hidden sheets, one chosen.
    pub(super) fn hm_unhide_sheet(&self) {
        let inner = self.dialog("Unhide", 0);
        let list = Rect::from_min_max(inner.min, pos2(inner.max.x, inner.max.y - 16.0));
        self.p.rect_filled(list, 1.0, Color32::WHITE);
        self.p.rect_stroke(list, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        for (i, n) in ["أسعار", "أرشيف"].iter().enumerate() {
            let y = list.min.y + 7.0 + i as f32 * 12.0;
            if i == 0 {
                self.p.rect_filled(Rect::from_min_size(pos2(list.min.x + 1.0, y - 5.5), vec2(list.width() - 2.0, 11.0)), 0.0, self.t.selection);
            }
            self.label(pos2(list.max.x - 4.0, y), n, 7.0, self.t.text, Align2::RIGHT_CENTER);
        }
        self.button(pos2(inner.max.x - 22.0, inner.max.y - 7.0), "OK", false);
        let k = self.reveal(0.2, 0.6);
        let tab = Rect::from_min_size(self.at(66.0, 92.0), vec2(34.0, 11.0));
        self.p.rect_filled(tab, 2.0, Color32::WHITE.linear_multiply(k));
        self.p.rect_stroke(tab, 2.0, self.stroke(self.accent().linear_multiply(k)), egui::StrokeKind::Inside);
        self.label(tab.center(), "أسعار", 6.5, self.accent().linear_multiply(k), Align2::CENTER_CENTER);
    }
    /// A tab being renamed: the old name is replaced by the new one with a caret.
    pub(super) fn hm_rename_sheet(&self) {
        let g = self.hm_sheet(28.0, 10.0, 120.0, 66.0, 3, 3);
        self.hm_rows(g, 3, 3, true);
        let tabs = self.hm_tabs(80.0, &["ورقة١", "ورقة٢"], 0);
        if let Some(t) = tabs.first() {
            let k = self.reveal(0.1, 0.6);
            let field = Rect::from_min_size(t.min, vec2(44.0, 12.0));
            self.p.rect_filled(field, 2.0, Color32::WHITE);
            self.p.rect_stroke(field, 2.0, self.stroke(self.accent()), egui::StrokeKind::Inside);
            let name = if k < 0.5 { "ورقة١" } else { "المبيعات" };
            self.label(pos2(field.max.x - 3.0, field.center().y), name, 6.5, self.accent(), Align2::RIGHT_CENTER);
            self.caret(pos2(field.min.x + 4.0, field.min.y + 2.0), 8.0);
            self.pencil(pos2(field.min.x - 2.0, field.min.y - 2.0), pos2(field.min.x - 14.0, field.min.y - 14.0));
        }
    }
    /// A sheet behind a padlock; a few unlocked input cells stay white.
    pub(super) fn hm_protect(&self) {
        let g = self.hm_sheet_center(4, 3);
        self.hm_rows(g, 4, 3, true);
        let k = self.reveal(0.1, 0.6);
        for r in 1..4 {
            for c in 0..3 {
                if c != 1 {
                    self.hm_fill(self.cell(g, 4, 3, r, c), self.t.hover.linear_multiply(k));
                }
            }
        }
        for c in 0..3 {
            self.hm_fill(self.cell(g, 4, 3, 0, c), self.t.hover.linear_multiply(k));
        }
        self.hm_rows(g, 4, 3, true);
        self.hm_lock(self.at(150.0, 78.0), k < 0.5);
    }
    /// One selected cell gets a small padlock.
    pub(super) fn hm_lock_cell(&self) {
        let g = self.hm_sheet_center(4, 3);
        self.hm_rows(g, 4, 3, true);
        let cell = self.cell(g, 4, 3, 2, 1);
        self.hm_sel(cell);
        self.hm_tline(cell, 0.5, self.grey(), 1.5);
        let k = self.reveal(0.15, 0.5);
        if k > 0.0 {
            self.hm_lock(pos2(cell.max.x - 8.0, cell.center().y - 2.0), k < 1.0);
        }
    }
    /// The Row Height / Column Width dialog, with the measured row or column beside it.
    pub(super) fn hm_size_dialog(&self, row: bool) {
        let g = self.hm_sheet(6.0, 12.0, 60.0, 80.0, 4, 2);
        self.hm_rows(g, 4, 2, true);
        let k = self.reveal(0.1, 0.6);
        if row {
            let c = self.cell(g, 4, 2, 1, 0);
            let r = Rect::from_min_max(c.min, pos2(g.max.x, c.max.y + 8.0 * k));
            self.p.rect_filled(r.shrink(0.5), 0.0, Color32::WHITE);
            self.p.rect_stroke(r, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
            self.hm_tline(Rect::from_min_max(r.min, pos2(c.max.x, r.max.y)), 0.6, self.grey(), 1.5);
            self.hm_tline(Rect::from_min_max(pos2(c.max.x, r.min.y), r.max), 0.5, self.grey(), 1.5);
            self.measure_v(pos2(g.max.x + 6.0, r.min.y), pos2(g.max.x + 6.0, r.max.y));
        } else {
            let c = self.cell(g, 4, 2, 0, 0);
            let r = Rect::from_min_max(pos2(c.min.x, g.min.y), pos2(c.max.x + 8.0 * k, g.max.y));
            self.p.rect_filled(r.shrink(0.5), 0.0, Color32::WHITE);
            self.p.rect_stroke(r, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
            for i in 0..4 {
                let cell = Rect::from_min_max(
                    pos2(r.min.x, g.min.y + i as f32 * g.height() / 4.0),
                    pos2(r.max.x, g.min.y + (i + 1) as f32 * g.height() / 4.0),
                );
                self.hm_tline(cell, 0.6, if i == 0 { self.ink() } else { self.grey() }, if i == 0 { 2.5 } else { 1.5 });
            }
            self.measure_h(pos2(r.min.x, g.min.y - 10.0), pos2(r.max.x, g.min.y - 10.0));
        }
        let dlg = Rect::from_min_size(self.at(78.0, 30.0), vec2(92.0, 44.0));
        self.p.rect_filled(dlg, 3.0, self.t.menu);
        self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(dlg.min, vec2(dlg.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(dlg.min.x + 5.0, dlg.min.y + 6.0), if row { "Row Height" } else { "Column Width" }, 7.0, self.t.text, Align2::LEFT_CENTER);
        let f = Rect::from_min_size(pos2(dlg.min.x + 8.0, dlg.min.y + 20.0), vec2(dlg.width() - 16.0, 12.0));
        self.p.rect_filled(f, 1.0, self.t.input);
        self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.label(pos2(f.max.x - 4.0, f.center().y), &self.hm_shape(if row { "٣٠" } else { "٢٠" }), 7.0, self.accent(), Align2::RIGHT_CENTER);
    }
    /// A row (or column) grows to fit its content.
    pub(super) fn hm_autofit(&self, row: bool) {
        if row {
            self.hm_pair(
                4,
                2,
                |d, g| {
                    d.hm_rows(g, 4, 2, true);
                    let c = d.cell(g, 4, 2, 1, 0);
                    d.hm_fill(c, Color32::WHITE);
                    let clip = d.p.with_clip_rect(c.intersect(d.p.clip_rect()));
                    let dd = d.with_painter(&clip);
                    for i in 0..3 {
                        let y = c.min.y + 5.0 + i as f32 * 7.0;
                        dd.line(pos2(c.max.x - 3.0 - (c.width() - 6.0) * 0.8, y), pos2(c.max.x - 3.0, y), d.accent(), 1.5);
                    }
                    d.hm_sel(Rect::from_min_max(c.min, pos2(g.max.x, c.max.y)));
                },
                |d, g| {
                    // Four rows again, the second one taller: heights 1 : 1.9 : 0.55 : 0.55.
                    let unit = g.height() / 4.0;
                    let heights = [unit, unit * 1.9, unit * 0.55, unit * 0.55];
                    d.p.rect_filled(Rect::from_min_max(pos2(g.min.x - HDR_W, g.min.y), g.max).shrink(0.5), 0.0, Color32::WHITE);
                    let mut y = g.min.y;
                    let cw = g.width() / 2.0;
                    for (i, h) in heights.iter().enumerate() {
                        let row = Rect::from_min_max(pos2(g.min.x, y), pos2(g.max.x, y + h));
                        if i > 0 {
                            d.line(pos2(g.min.x - HDR_W, y), pos2(g.max.x, y), d.t.border, 1.0);
                        }
                        d.line(pos2(g.min.x + cw, row.min.y), pos2(g.min.x + cw, row.max.y), d.t.border, 1.0);
                        let c0 = Rect::from_min_max(row.min, pos2(g.min.x + cw, row.max.y));
                        let c1 = Rect::from_min_max(pos2(g.min.x + cw, row.min.y), row.max);
                        if i == 1 {
                            for j in 0..3 {
                                let yy = row.min.y + 6.0 + j as f32 * 8.0;
                                d.line(pos2(c0.max.x - 3.0 - (c0.width() - 6.0) * 0.8, yy), pos2(c0.max.x - 3.0, yy), d.accent(), 1.5);
                            }
                            d.hm_tline(c1, 0.5, d.grey(), 1.5);
                            d.measure_v(pos2(g.min.x - 4.0, row.min.y), pos2(g.min.x - 4.0, row.max.y));
                        } else {
                            d.hm_tline(c0, 0.6, if i == 0 { d.ink() } else { d.grey() }, if i == 0 { 2.5 } else { 1.5 });
                            d.hm_tline(c1, 0.45, if i == 0 { d.ink() } else { d.grey() }, if i == 0 { 2.5 } else { 1.5 });
                        }
                        y += h;
                    }
                },
            );
        } else {
            self.hm_pair(
                4,
                3,
                |d, g| {
                    d.hm_rows(g, 4, 3, true);
                    for r in 1..4 {
                        let c = d.cell(g, 4, 3, r, 0);
                        d.hm_fill(c, Color32::WHITE);
                        let clip = d.p.with_clip_rect(c.intersect(d.p.clip_rect()));
                        d.with_painter(&clip).line(pos2(c.min.x - 10.0, c.center().y), pos2(c.max.x - 3.0, c.center().y), d.accent(), 1.5);
                    }
                    d.hm_sel(Rect::from_min_max(g.min, pos2(d.cell(g, 4, 3, 3, 0).max.x, g.max.y)));
                },
                |d, g| {
                    let wide = Rect::from_min_max(g.min, pos2(d.cell(g, 4, 3, 0, 0).max.x + 14.0, g.max.y));
                    d.hm_rows(g, 4, 3, true);
                    d.p.rect_filled(wide.shrink(0.5), 0.0, Color32::WHITE);
                    for r in 0..4 {
                        let y = g.min.y + r as f32 * g.height() / 4.0;
                        d.line(pos2(wide.min.x, y), pos2(wide.max.x, y), d.t.border, 1.0);
                        let cell = Rect::from_min_max(pos2(wide.min.x, y), pos2(wide.max.x, y + g.height() / 4.0));
                        if r == 0 {
                            d.hm_tline(cell, 0.6, d.ink(), 2.5);
                        } else {
                            d.line(pos2(cell.min.x + 3.0, cell.center().y), pos2(cell.max.x - 3.0, cell.center().y), d.accent(), 1.5);
                        }
                    }
                    d.line(pos2(wide.max.x, g.min.y - HDR_H), pos2(wide.max.x, g.max.y), d.t.border, 1.0);
                    d.measure_h(pos2(wide.min.x, g.max.y + 7.0), pos2(wide.max.x, g.max.y + 7.0));
                },
            );
        }
    }
    /// Every column of the sheet takes the same new width.
    pub(super) fn hm_default_width(&self) {
        let k = self.reveal(0.1, 0.6);
        let cols = 4;
        let g = self.hm_sheet(28.0, 22.0, 80.0 + 40.0 * k, 70.0, 4, cols);
        self.hm_rows(g, 4, cols, true);
        for j in 0..cols {
            let c = self.cell(g, 4, cols, 0, j);
            self.measure_h(pos2(c.min.x + 1.0, g.min.y - 12.0), pos2(c.max.x - 1.0, g.min.y - 12.0));
        }
    }
    /// Rows (or columns) hidden — or shown again.
    pub(super) fn hm_hide(&self, rows: bool, unhide: bool) {
        let full = |d: &Draw<'_>, g: Rect| {
            let (nr, nc) = if rows { (5, 2) } else { (3, 4) };
            d.hm_rows(g, nr, nc, true);
            if !unhide {
                let sel = if rows {
                    Rect::from_min_max(d.cell(g, nr, nc, 1, 0).min, d.cell(g, nr, nc, 2, nc - 1).max)
                } else {
                    Rect::from_min_max(d.cell(g, nr, nc, 0, 1).min, d.cell(g, nr, nc, nr - 1, 1).max)
                };
                d.hm_sel(sel);
            }
        };
        let hidden = |d: &Draw<'_>, g: Rect| {
            let (nr, nc) = if rows { (3, 2) } else { (3, 3) };
            d.hm_rows(g, nr, nc, true);
            if rows {
                let y = d.cell(g, nr, nc, 1, 0).min.y;
                d.line(pos2(g.min.x - HDR_W, y), pos2(g.max.x, y), d.accent(), 2.5);
            } else {
                let x = d.cell(g, nr, nc, 0, 1).min.x;
                d.line(pos2(x, g.min.y - HDR_H), pos2(x, g.max.y), d.accent(), 2.5);
            }
            if unhide {
                let sel = if rows {
                    Rect::from_min_max(g.min, d.cell(g, nr, nc, 1, nc - 1).max)
                } else {
                    Rect::from_min_max(g.min, d.cell(g, nr, nc, nr - 1, 1).max)
                };
                d.hm_sel(sel);
            }
        };
        if unhide {
            if rows {
                self.hm_pair(3, 2, hidden, full);
            } else {
                self.hm_pair(3, 3, hidden, full);
            }
        } else if rows {
            self.hm_pair(5, 2, full, hidden);
        } else {
            self.hm_pair(3, 4, full, hidden);
        }
    }

    // ---- editing ----

    /// A value flows from one cell into its neighbours: 0 down, 1 right, 2 up, 3 left.
    pub(super) fn hm_fill_dir(&self, dir: u8) {
        let g = self.hm_sheet_center(4, 4);
        self.hm_rows(g, 4, 4, true);
        let k = self.reveal(0.1, 0.7);
        let vertical = matches!(dir, 0 | 2);
        let n = if vertical { 3 } else { 4 };
        let cells: Vec<Rect> = (0..n)
            .map(|i| {
                let i = if matches!(dir, 2 | 3) { n - 1 - i } else { i };
                if vertical { self.cell(g, 4, 4, 1 + i, 1) } else { self.cell(g, 4, 4, 2, i) }
            })
            .collect();
        let whole = cells.iter().fold(Rect::NOTHING, |a, c| a.union(*c));
        self.p.rect_filled(whole.shrink(0.5), 0.0, Color32::WHITE);
        self.grid(whole, if vertical { n } else { 1 }, if vertical { 1 } else { n }, self.t.border);
        for (i, c) in cells.iter().enumerate() {
            let t = if i == 0 { 1.0 } else { ((k * (n - 1) as f32) - (i - 1) as f32).clamp(0.0, 1.0) };
            if t > 0.0 {
                self.hm_fill(*c, self.accent().linear_multiply(0.18 * t));
                self.hm_txt_l(*c, "=A1*٢", self.accent().linear_multiply(t));
            }
        }
        self.hm_sel(whole);
        if let (Some(first), Some(last)) = (cells.first(), cells.last()) {
            let a = first.center();
            let b = last.center();
            let end = a + (b - a) * k.max(0.25);
            match dir {
                0 => self.arrow_down(pos2(g.max.x + 8.0, a.y), pos2(g.max.x + 8.0, end.y)),
                2 => self.arrow_up(pos2(g.max.x + 8.0, a.y), pos2(g.max.x + 8.0, end.y)),
                1 => self.arrow_right(pos2(a.x, g.max.y + 8.0), pos2(end.x, g.max.y + 8.0)),
                _ => self.arrow_left(pos2(a.x, g.max.y + 8.0), pos2(end.x, g.max.y + 8.0)),
            }
        }
    }
    /// The Series dialog beside a column counting 1, 2, 3, 4.
    pub(super) fn hm_series(&self) {
        let g = self.hm_sheet(6.0, 12.0, 50.0, 80.0, 4, 1);
        let k = self.reveal(0.1, 0.7);
        for (i, s) in ["١", "٢", "٣", "٤"].iter().enumerate() {
            let t = if i == 0 { 1.0 } else { (k * 3.0 - (i - 1) as f32).clamp(0.0, 1.0) };
            self.hm_txt(self.cell(g, 4, 1, i, 0), s, if i == 0 { self.ink() } else { self.accent().linear_multiply(t) });
        }
        self.hm_sel(g);
        let dlg = Rect::from_min_size(self.at(68.0, 18.0), vec2(102.0, 68.0));
        self.p.rect_filled(dlg, 3.0, self.t.menu);
        self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(dlg.min, vec2(dlg.width(), 12.0)), 3.0, self.t.hover);
        self.label(pos2(dlg.min.x + 5.0, dlg.min.y + 6.0), "Series", 7.0, self.t.text, Align2::LEFT_CENTER);
        for (i, (name, on)) in [("Linear", true), ("Growth", false), ("Date", false)].iter().enumerate() {
            let y = dlg.min.y + 22.0 + i as f32 * 11.0;
            let c = pos2(dlg.min.x + 8.0, y);
            self.p.circle_stroke(c, 3.0, Stroke::new(1.0, self.t.border_strong));
            if *on {
                self.p.circle_filled(c, 1.8, self.accent());
            }
            self.label(pos2(dlg.min.x + 15.0, y), name, 6.5, self.t.text, Align2::LEFT_CENTER);
        }
        self.label(pos2(dlg.min.x + 58.0, dlg.min.y + 27.0), "Step", 6.5, self.grey(), Align2::LEFT_CENTER);
        let f = Rect::from_min_size(pos2(dlg.min.x + 58.0, dlg.min.y + 32.0), vec2(36.0, 11.0));
        self.p.rect_filled(f, 1.0, self.t.input);
        self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.label(pos2(f.max.x - 3.0, f.center().y), "١", 7.0, self.accent(), Align2::RIGHT_CENTER);
    }
    /// Flash Fill: one example in column B, and the rest of the column fills itself.
    pub(super) fn hm_flash_fill(&self) {
        let full = ["أحمد علي", "سارة حسن", "علي كريم", "نور سعيد"];
        let first = ["أحمد", "سارة", "علي", "نور"];
        self.hm_pair(
            5,
            2,
            |d, g| {
                d.hm_tline(d.cell(g, 5, 2, 0, 0), 0.6, d.ink(), 2.5);
                d.hm_tline(d.cell(g, 5, 2, 0, 1), 0.6, d.ink(), 2.5);
                for (i, n) in full.iter().enumerate() {
                    d.hm_txt_sz(d.cell(g, 5, 2, i + 1, 0), n, d.ink(), 5.5);
                }
                if let Some(f) = first.first() {
                    d.hm_txt(d.cell(g, 5, 2, 1, 1), f, d.ink());
                }
                d.hm_sel(d.cell(g, 5, 2, 2, 1));
            },
            |d, g| {
                d.hm_tline(d.cell(g, 5, 2, 0, 0), 0.6, d.ink(), 2.5);
                d.hm_tline(d.cell(g, 5, 2, 0, 1), 0.6, d.ink(), 2.5);
                for (i, n) in full.iter().enumerate() {
                    d.hm_txt_sz(d.cell(g, 5, 2, i + 1, 0), n, d.ink(), 5.5);
                }
                for (i, f) in first.iter().enumerate() {
                    d.hm_txt(d.cell(g, 5, 2, i + 1, 1), f, if i == 0 { d.ink() } else { d.accent() });
                }
            },
        );
        // A little lightning bolt over the arrow.
        let pts = vec![self.at(90.0, 22.0), self.at(84.0, 34.0), self.at(89.0, 34.0), self.at(86.0, 44.0), self.at(94.0, 30.0), self.at(89.0, 30.0)];
        self.p.add(Shape::closed_line(pts, Stroke::new(1.5, self.accent())));
    }
    /// Clearing a yellow bold 150 with a comment and a link. 0: all; 1: formats; 2: contents;
    /// 3: comments; 4: hyperlinks.
    pub(super) fn hm_clear(&self, kind: u8) {
        let decorated = |d: &Draw<'_>, g: Rect, fill: bool, text: Option<(Color32, f32)>, note: bool, link: bool| {
            d.hm_rows(g, 4, 2, true);
            for r in 1..4 {
                let c = d.cell(g, 4, 2, r, 1);
                d.hm_fill(c, if fill { d.yellow() } else { Color32::WHITE });
                if let Some((col, w)) = text {
                    if r == 3 && link {
                        d.hm_txt(c, "موقع", Color32::from_rgb(40, 90, 200));
                        d.line(
                            pos2(c.max.x - 20.0, c.center().y + 4.0),
                            pos2(c.max.x - 2.0, c.center().y + 4.0),
                            Color32::from_rgb(40, 90, 200),
                            1.0,
                        );
                    } else if r == 3 {
                        d.hm_txt(c, "موقع", col);
                    } else {
                        d.hm_tline(c, 0.5, col, w);
                    }
                }
                if note && r == 1 {
                    d.hm_note_mark(c, d.red());
                }
            }
        };
        self.hm_pair(
            4,
            2,
            |d, g| {
                decorated(d, g, true, Some((d.ink(), 2.5)), true, true);
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 2, 1, 1).min, d.cell(g, 4, 2, 3, 1).max));
            },
            |d, g| match kind {
                0 => decorated(d, g, false, None, false, false),
                1 => decorated(d, g, false, Some((d.ink(), 1.5)), true, false),
                2 => decorated(d, g, true, None, false, false),
                3 => decorated(d, g, true, Some((d.ink(), 2.5)), false, true),
                _ => decorated(d, g, true, Some((d.ink(), 2.5)), true, false),
            },
        );
        self.hm_eraser(self.at(88.0, 26.0));
    }
    /// The Sort & Filter button: numbers fall into order under a funnel.
    pub(super) fn hm_sort_filter(&self) {
        self.hm_sort(true);
        self.hm_funnel(self.at(88.0, 74.0), 7.0, self.accent());
    }
    /// One column's numbers sorted ascending or descending.
    pub(super) fn hm_sort(&self, asc: bool) {
        let before = ["٩", "٣", "٧", "٥"];
        let after = if asc { ["٣", "٥", "٧", "٩"] } else { ["٩", "٧", "٥", "٣"] };
        self.hm_pair(
            5,
            2,
            |d, g| {
                d.hm_rows(g, 5, 2, true);
                for (i, n) in before.iter().enumerate() {
                    let c = d.cell(g, 5, 2, i + 1, 1);
                    d.hm_fill(c, Color32::WHITE);
                    d.hm_txt(c, n, d.ink());
                }
                d.hm_sel(Rect::from_min_max(d.cell(g, 5, 2, 1, 1).min, d.cell(g, 5, 2, 4, 1).max));
            },
            |d, g| {
                d.hm_rows(g, 5, 2, true);
                for (i, n) in after.iter().enumerate() {
                    let c = d.cell(g, 5, 2, i + 1, 1);
                    d.hm_fill(c, Color32::WHITE);
                    d.hm_txt(c, n, d.accent());
                }
            },
        );
        let (a, b) = if asc { ("A", "Z") } else { ("Z", "A") };
        self.label(self.at(84.0, 26.0), a, 7.0, self.grey(), Align2::CENTER_CENTER);
        self.label(self.at(92.0, 36.0), b, 9.0, self.accent(), Align2::CENTER_CENTER);
    }
    /// The Sort dialog: two levels, each with a column and an order.
    pub(super) fn hm_sort_dialog(&self) {
        let inner = self.dialog("Sort", 0);
        for (i, (by, order)) in [("المدينة", "تصاعدي"), ("الاسم", "تصاعدي")].iter().enumerate() {
            let y = inner.min.y + 2.0 + i as f32 * 14.0;
            self.label(pos2(inner.min.x, y + 5.0), if i == 0 { "Sort by" } else { "Then by" }, 6.0, self.grey(), Align2::LEFT_CENTER);
            let f1 = Rect::from_min_size(pos2(inner.min.x + 32.0, y), vec2(34.0, 10.0));
            let f2 = Rect::from_min_size(pos2(inner.min.x + 70.0, y), vec2(inner.width() - 70.0, 10.0));
            for f in [f1, f2] {
                self.p.rect_filled(f, 1.0, self.t.input);
                self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
            }
            self.label(pos2(f1.max.x - 3.0, f1.center().y), by, 6.0, self.t.text, Align2::RIGHT_CENTER);
            self.label(pos2(f2.max.x - 3.0, f2.center().y), order, 6.0, self.accent(), Align2::RIGHT_CENTER);
        }
        let k = self.reveal(0.2, 0.6);
        self.button(pos2(inner.min.x + 26.0, inner.max.y - 8.0), "+ Level", k > 0.5);
        self.button(pos2(inner.max.x - 18.0, inner.max.y - 8.0), "OK", false);
    }
    /// Filtering. 0: filter buttons appear and rows that do not match go; 1: the rows come back;
    /// 2: reapply (a refresh arrow over the filter).
    pub(super) fn hm_filter(&self, kind: u8) {
        let cities = ["بغداد", "بابل", "بغداد", "نينوى"];
        let plain = |d: &Draw<'_>, g: Rect, buttons: bool| {
            d.hm_rows(g, 5, 2, true);
            for (i, c) in cities.iter().enumerate() {
                let cell = d.cell(g, 5, 2, i + 1, 1);
                d.hm_fill(cell, Color32::WHITE);
                d.hm_txt(cell, c, d.ink());
            }
            if buttons {
                for c in 0..2 {
                    d.hm_filter_button(d.cell(g, 5, 2, 0, c), false);
                }
            }
        };
        let filtered = |d: &Draw<'_>, g: Rect| {
            d.hm_rows(g, 3, 2, true);
            for (i, c) in cities.iter().filter(|c| **c == "بغداد").enumerate() {
                let cell = d.cell(g, 3, 2, i + 1, 1);
                d.hm_fill(cell, Color32::WHITE);
                d.hm_txt(cell, c, d.accent());
            }
            d.hm_filter_button(d.cell(g, 3, 2, 0, 0), false);
            d.hm_filter_button(d.cell(g, 3, 2, 0, 1), true);
            // Hidden rows: a double line where they were.
            let y = d.cell(g, 3, 2, 1, 0).max.y;
            d.line(pos2(g.min.x - HDR_W, y - 1.0), pos2(g.max.x, y - 1.0), d.accent(), 1.0);
            d.line(pos2(g.min.x - HDR_W, y + 1.0), pos2(g.max.x, y + 1.0), d.accent(), 1.0);
        };
        match kind {
            0 => self.hm_pair(5, 2, |d, g| plain(d, g, false), filtered),
            1 => self.hm_pair(3, 2, filtered, |d, g| plain(d, g, true)),
            _ => {
                let g = self.hm_sheet(28.0, 12.0, 96.0, 80.0, 3, 2);
                filtered(self, g);
                let c = self.at(150.0, 40.0);
                let k = self.reveal(0.1, 0.6);
                let n = 12;
                let pts: Vec<Pos2> = (0..=n)
                    .map(|i| {
                        let a = -1.2 + 4.5 * k * i as f32 / n as f32;
                        c + vec2(7.0 * a.cos(), 7.0 * a.sin())
                    })
                    .collect();
                self.p.add(Shape::line(pts.clone(), Stroke::new(1.5, self.accent())));
                if let (Some(last), Some(prev)) = (pts.last(), pts.get(n - 1)) {
                    let dir = (*last - *prev).normalized();
                    self.line(*last, *last - dir * 4.0 + dir.rot90() * 3.0, self.accent(), 1.5);
                    self.line(*last, *last - dir * 4.0 - dir.rot90() * 3.0, self.accent(), 1.5);
                }
            }
        }
        self.hm_funnel(
            self.at(if kind == 2 { 150.0 } else { 88.0 }, if kind == 2 { 72.0 } else { 30.0 }),
            6.0,
            if kind == 1 { self.grey() } else { self.accent() },
        );
        if kind == 1 {
            self.line(self.at(81.0, 37.0), self.at(95.0, 23.0), self.red(), 1.5);
        }
    }
    /// Find: matches light up one after another under a magnifier; Replace: the new word lands.
    pub(super) fn hm_find(&self, replace: bool) {
        let g = if replace { self.hm_sheet(60.0, 12.0, 108.0, 80.0, 4, 3) } else { self.hm_sheet_center(4, 3) };
        self.hm_rows(g, 4, 3, true);
        let hits = [(1usize, 0usize), (2, 2), (3, 1)];
        for (i, (r, c)) in hits.iter().enumerate() {
            let cell = self.cell(g, 4, 3, *r, *c);
            self.hm_fill(cell, Color32::WHITE);
            let k = self.reveal(0.1 + i as f32 * 0.2, 0.25 + i as f32 * 0.2);
            if k > 0.0 {
                self.highlight(cell.shrink(1.0));
                self.hm_txt(cell, if replace && k >= 1.0 { "البصرة" } else { "بغداد" }, if replace && k >= 1.0 { self.accent() } else { self.ink() });
            } else {
                self.hm_txt(cell, "بغداد", self.ink());
            }
        }
        if replace {
            let dlg = Rect::from_min_size(self.at(4.0, 30.0), vec2(50.0, 44.0));
            self.p.rect_filled(dlg, 3.0, self.t.menu);
            self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            self.p.rect_filled(Rect::from_min_size(dlg.min, vec2(dlg.width(), 10.0)), 3.0, self.t.hover);
            self.label(pos2(dlg.min.x + 4.0, dlg.min.y + 5.0), "Replace", 6.0, self.t.text, Align2::LEFT_CENTER);
            for (i, (w, col)) in [("بغداد", self.grey()), ("البصرة", self.accent())].iter().enumerate() {
                let f = Rect::from_min_size(pos2(dlg.min.x + 5.0, dlg.min.y + 14.0 + i as f32 * 14.0), vec2(dlg.width() - 10.0, 11.0));
                self.p.rect_filled(f, 1.0, self.t.input);
                self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
                self.label(pos2(f.max.x - 3.0, f.center().y), w, 6.5, *col, Align2::RIGHT_CENTER);
            }
            self.arrow_right(pos2(dlg.max.x + 1.0, dlg.center().y), pos2(dlg.max.x + 5.0, dlg.center().y));
        } else {
            self.magnifier(self.at(150.0, 78.0));
        }
    }
    /// Go To: a reference typed in a box, and the selection jumping to that cell.
    pub(super) fn hm_goto(&self) {
        let g = self.hm_sheet(60.0, 12.0, 108.0, 80.0, 4, 4);
        self.hm_rows(g, 4, 4, true);
        let dlg = Rect::from_min_size(self.at(4.0, 30.0), vec2(50.0, 36.0));
        self.p.rect_filled(dlg, 3.0, self.t.menu);
        self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(dlg.min, vec2(dlg.width(), 10.0)), 3.0, self.t.hover);
        self.label(pos2(dlg.min.x + 4.0, dlg.min.y + 5.0), "Go To", 6.0, self.t.text, Align2::LEFT_CENTER);
        let f = Rect::from_min_size(pos2(dlg.min.x + 5.0, dlg.min.y + 16.0), vec2(40.0, 12.0));
        self.p.rect_filled(f, 1.0, self.t.input);
        self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.label(f.center(), "D3", 7.5, self.accent(), Align2::CENTER_CENTER);
        let k = self.reveal(0.15, 0.6);
        let target = self.cell(g, 4, 4, 2, 3);
        let from = self.cell(g, 4, 4, 0, 0);
        let cur = Rect::from_center_size(from.center() + (target.center() - from.center()) * k, target.size());
        self.hm_sel(cur);
        self.arrow_right(pos2(f.max.x + 2.0, f.center().y), pos2(f.max.x + 2.0 + 14.0 * k.max(0.3), f.center().y));
    }
    /// Go To Special: cells of one kind light up across the sheet. 0: a mix (the dialog);
    /// 1: formulas; 2: notes; 3: conditional formats; 4: constants; 5: data validation.
    pub(super) fn hm_goto_special(&self, kind: u8) {
        let g = self.hm_sheet_center(4, 4);
        self.hm_rows(g, 4, 4, true);
        let k = self.reveal(0.1, 0.5);
        let formulas = [(3usize, 1usize), (3, 2), (3, 3)];
        let constants = [(1usize, 1usize), (1, 2), (1, 3), (2, 1), (2, 2), (2, 3)];
        let notes = [(1usize, 2usize), (2, 1)];
        let (cells, label): (&[(usize, usize)], &str) = match kind {
            1 => (&formulas, "=SUM"),
            2 => (&notes, ""),
            3 => (&[(1, 1), (2, 3)], ""),
            4 => (&constants, ""),
            5 => (&[(1, 1), (2, 1), (3, 1)], ""),
            _ => (&formulas, "=SUM"),
        };
        for (r, c) in cells {
            let cell = self.cell(g, 4, 4, *r, *c);
            self.hm_fill(cell, Color32::WHITE);
            match kind {
                2 => self.hm_note_mark(cell, self.red()),
                3 => self.hm_fill(cell, self.green().linear_multiply(0.3)),
                5 => {
                    let b = Rect::from_min_size(pos2(cell.min.x + 1.5, cell.center().y - 4.0), vec2(8.0, 8.0));
                    self.p.rect_stroke(b, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
                    let tri = vec![b.center() + vec2(-2.5, -1.5), b.center() + vec2(2.5, -1.5), b.center() + vec2(0.0, 2.0)];
                    self.p.add(Shape::convex_polygon(tri, self.ink(), Stroke::NONE));
                    self.hm_tline(cell, 0.4, self.grey(), 1.5);
                }
                _ => {}
            }
            if label.is_empty() {
                if matches!(kind, 2..=4) {
                    self.hm_txt(cell, "١٢٠", self.ink());
                }
            } else {
                self.label(pos2(cell.min.x + 2.0, cell.center().y), label, 6.0, self.ink(), Align2::LEFT_CENTER);
            }
            if k > 0.0 {
                self.p.rect_filled(cell, 0.0, self.t.selection.linear_multiply(k));
                self.p.rect_stroke(cell, 0.0, Stroke::new(1.5, self.accent().linear_multiply(k)), egui::StrokeKind::Inside);
            }
        }
        if kind == 0 {
            let dlg = Rect::from_min_size(self.at(4.0, 8.0), vec2(72.0, 44.0));
            self.p.rect_filled(dlg, 3.0, self.t.menu);
            self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            for (i, n) in ["Formulas", "Constants", "Blanks"].iter().enumerate() {
                let y = dlg.min.y + 8.0 + i as f32 * 12.0;
                let c = pos2(dlg.min.x + 8.0, y);
                self.p.circle_stroke(c, 3.0, Stroke::new(1.0, self.t.border_strong));
                if i == 0 {
                    self.p.circle_filled(c, 1.8, self.accent());
                }
                self.label(pos2(dlg.min.x + 15.0, y), n, 6.5, if i == 0 { self.accent() } else { self.t.text }, Align2::LEFT_CENTER);
            }
        }
    }
    /// Transpose: a copied row stands up as a column.
    pub(super) fn hm_transpose(&self) {
        let vals = ["أ", "ب", "ج", "د"];
        self.hm_pair(
            4,
            4,
            |d, g| {
                for (c, v) in vals.iter().enumerate() {
                    let cell = d.cell(g, 4, 4, 0, c);
                    d.hm_txt_c(cell, v, d.ink());
                }
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 4, 0, 0).min, d.cell(g, 4, 4, 0, 3).max));
            },
            |d, g| {
                for (r, v) in vals.iter().enumerate() {
                    let cell = d.cell(g, 4, 4, r, 0);
                    d.hm_txt_c(cell, v, d.accent());
                }
                d.hm_sel(Rect::from_min_max(d.cell(g, 4, 4, 0, 0).min, d.cell(g, 4, 4, 3, 0).max));
            },
        );
        // A quarter-turn arrow over the middle.
        let c = self.at(88.0, 32.0);
        let n = 10;
        let pts: Vec<Pos2> =
            (0..=n).map(|i| c + vec2(8.0 * (-1.57 + 1.57 * i as f32 / n as f32).cos(), 8.0 * (-1.57 + 1.57 * i as f32 / n as f32).sin())).collect();
        self.p.add(Shape::line(pts, Stroke::new(1.5, self.accent())));
        self.line(c + vec2(8.0, 0.0), c + vec2(5.0, -4.0), self.accent(), 1.5);
        self.line(c + vec2(8.0, 0.0), c + vec2(11.0, -4.0), self.accent(), 1.5);
    }
}
