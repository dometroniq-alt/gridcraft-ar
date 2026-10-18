//! Home › Send (GridCraft AR): hand the sheet to WordCraft AR or DeckCraft AR as a Craft
//! Exchange file (`.craftx`, see `docs/ar-exchange.md`), and receive one from them.

use serde_json::{Value as Json, json};

use super::*;
use crate::craft_launch::{Launched, Sibling, launch};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(noundo "send.toWordCraft", "Send to WordCraft AR", ["Home", "Send"], None, "{scope?: sheet|selection} → {path, launched: bool}: writes <temp>/craft-exchange/<title>-<HHMMSS>.craftx and starts WordCraft AR on it", has_doc, |s, p| send(s, p, Sibling::WordCraft)),
        cmd!(noundo "send.toDeckCraft", "Send to DeckCraft AR", ["Home", "Send"], None, "{scope?: sheet|selection} → {path, launched: bool}: writes <temp>/craft-exchange/<title>-<HHMMSS>.craftx and starts DeckCraft AR on it", has_doc, |s, p| send(s, p, Sibling::DeckCraft)),
        cmd!(noundo "send.importCraftx", "Receive from WordCraft / DeckCraft", ["Home", "Send"], None, "{path} (a .craftx file; opens it as a new workbook, one sheet per table plus a Text sheet)", always, import),
    ]
}

/// Writes the active sheet (or the selection) as a `.craftx` in the temp folder and starts the
/// sibling on it. When no sibling is installed the file is kept and the message says where.
fn send(s: &mut Session, p: &Json, app: Sibling) -> Result<Json> {
    let d = s.doc()?;
    let wb = &d.wb;
    let sheet = wb.active_sheet;
    let scope = str_param(p, "scope").unwrap_or("sheet");
    let range = match scope {
        "selection" => {
            // One selected cell means the whole used range, as a copy of a single cell would.
            let r = d.selection.ranges.first().copied().unwrap_or_default();
            if r.count() <= 1 { None } else { Some(r) }
        }
        "sheet" => None,
        other => return Err(bad("send", format!("`scope` is sheet or selection, not `{other}`"))),
    };
    let title = file_title(&d.display_title());
    let json = crate::craftx::write(wb, sheet, range, &title)?;
    let dir = std::env::temp_dir().join("craft-exchange");
    std::fs::create_dir_all(&dir).map_err(|e| EngineError::Other(format!("{}: {e}", dir.display())))?;
    let path = fresh_path(&dir, &title, &clock_stamp());
    let path_str = path.to_string_lossy().to_string();
    crate::io::write_file(&path_str, json.as_bytes())?;
    let launched = launch(app, &path);
    let message = match &launched {
        Launched::Started(_) => format!("Sent to {}", app.title()),
        Launched::NotFound => format!("Saved {path_str} — open it in {} (File › Open)", app.title()),
    };
    s.ui_requests.push(crate::UiRequest::Message(message));
    Ok(json!({"path": path_str, "launched": matches!(launched, Launched::Started(_)), "app": app.title()}))
}

/// Opens a `.craftx` as a new workbook (one sheet per table, a "Text" sheet for the rest).
/// Without `path` the interface asks for the file; programmatic calls get an error.
fn import(s: &mut Session, p: &Json) -> Result<Json> {
    let Some(path) = str_param(p, "path") else {
        s.ui_requests.push(crate::UiRequest::Dialog("open".into(), json!({"filter": crate::craftx::EXTENSION})));
        return ok();
    };
    let is_craftx = std::path::Path::new(path).extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case(crate::craftx::EXTENSION));
    if !is_craftx {
        return Err(bad("send.importCraftx", format!("`{path}` is not a .craftx file")));
    }
    super::file::open(s, &json!({"path": path}))
}

/// `<dir>/<title>-<stamp>.craftx`, or `…-<stamp>-2.craftx`, `-3`… when a send a moment ago
/// already took the name: two sends in one second never overwrite each other.
fn fresh_path(dir: &std::path::Path, title: &str, stamp: &str) -> std::path::PathBuf {
    let first = dir.join(format!("{title}-{stamp}.craftx"));
    if !first.exists() {
        return first;
    }
    (2..1000u32).map(|n| dir.join(format!("{title}-{stamp}-{n}.craftx"))).find(|p| !p.exists()).unwrap_or(first)
}

/// The document's name as a file stem: no extension, none of the characters a file name can't
/// hold, never empty.
fn file_title(display_title: &str) -> String {
    let stem = std::path::Path::new(display_title).file_stem().and_then(|s| s.to_str()).unwrap_or(display_title);
    let clean: String =
        stem.chars().filter(|c| !matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') && !c.is_control()).take(80).collect();
    let clean = clean.trim().trim_matches('.').to_string();
    if clean.is_empty() { "Book".into() } else { clean }
}

/// `HHMMSS` of the moment (UTC), so successive sends get distinct files. The browser has no
/// clock here (`SystemTime::now` traps on wasm), and no file system either.
fn clock_stamp() -> String {
    if cfg!(target_arch = "wasm32") {
        return "000000".into();
    }
    #[allow(clippy::disallowed_methods)] // guarded: never reached on wasm
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) % 86_400;
    format!("{:02}{:02}{:02}", secs / 3600, (secs / 60) % 60, secs % 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    fn arabic_session() -> Session {
        let mut s = Session::new();
        s.execute("file.new", json!({"rightToLeft": true})).unwrap();
        s.execute("sheet.rename", json!({"name": "الرواتب"})).unwrap();
        for (c, v) in [("A1", "الاسم"), ("B1", "الراتب"), ("A2", "على الطبي مستندات"), ("B2", "1250"), ("A3", "أحمد"), ("B3", "900")]
        {
            s.execute("cell.set", json!({"cell": c, "input": v})).unwrap();
        }
        s.execute("home.bold", json!({"range": "A1:B1"})).unwrap();
        s.execute("home.justify", json!({"range": "A2"})).unwrap();
        s.execute("home.kashida", json!({"range": "A2", "level": "high"})).unwrap();
        s.execute("home.alignCenter", json!({"range": "B2"})).unwrap();
        s
    }

    #[test]
    fn send_writes_a_craftx_to_temp_and_import_reads_it_back() {
        let mut s = arabic_session();
        assert!(s.doc().unwrap().wb.active().unwrap().right_to_left);
        let r = s.execute("send.toWordCraft", json!({})).unwrap();
        let path = r["path"].as_str().unwrap().to_string();
        assert!(path.ends_with(".craftx") && path.contains("craft-exchange"), "{path}");
        assert_eq!(r["launched"], false, "never launched under test");
        let msgs = s.take_ui_requests();
        assert!(matches!(msgs.first(), Some(crate::UiRequest::Message(m)) if m.starts_with("Saved ") && m.contains("WordCraft AR")), "{msgs:?}");
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("\"from\": \"gridcraft-ar\""));
        // A selection scope sends only the selection.
        s.execute("selection.set", json!({"range": "A1:B2"})).unwrap();
        let r2 = s.execute("send.toDeckCraft", json!({"scope": "selection"})).unwrap();
        let doc: Json = serde_json::from_str(&std::fs::read_to_string(r2["path"].as_str().unwrap()).unwrap()).unwrap();
        assert_eq!(doc["blocks"][1]["rows"].as_array().unwrap().len(), 2);
        assert!(s.execute("send.toDeckCraft", json!({"scope": "page"})).is_err());
        // Receive: a new workbook with the same sheet.
        let before = s.documents().len();
        let r3 = s.execute("send.importCraftx", json!({"path": path})).unwrap();
        assert_eq!(s.documents().len(), before + 1, "{r3}");
        let d = s.doc().unwrap();
        assert!(d.path.is_none(), "a received file is not the workbook's save target");
        let sh = d.wb.active().unwrap();
        assert_eq!(sh.name, "الرواتب");
        assert!(sh.right_to_left);
        let text = |a: &str| crate::display::value_text(&d.wb, sh, gridcraft_core::CellRef::parse(a).unwrap());
        assert_eq!((text("A1"), text("B2"), text("A3")), ("الاسم".into(), "1250".into(), "أحمد".into()));
        let st = |a: &str| d.wb.styles.get(sh.style_id(gridcraft_core::CellRef::parse(a).unwrap())).clone();
        assert!(st("A1").font.bold);
        assert_eq!(st("A2").align.kashida, gridcraft_model::style::KashidaLevel::High);
        assert_eq!(st("B2").align.h, gridcraft_model::HAlign::Center);
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(r2["path"].as_str().unwrap());
    }

    #[test]
    fn import_without_a_path_asks_the_interface_and_refuses_other_files() {
        let mut s = Session::new();
        s.execute("send.importCraftx", json!({})).unwrap();
        assert!(matches!(s.take_ui_requests().first(), Some(crate::UiRequest::Dialog(n, p)) if n == "open" && p["filter"] == "craftx"));
        assert!(s.execute_headless("send.importCraftx", json!({})).is_err(), "programmatic calls never open dialogs");
        assert!(s.execute("send.importCraftx", json!({"path": "book.xlsx"})).is_err());
        assert!(s.execute("send.importCraftx", json!({"path": "/nowhere/none.craftx"})).is_err());
        assert!(s.execute("send.toWordCraft", json!({})).is_err(), "no workbook: disabled");
    }

    #[test]
    fn file_titles_are_safe_file_stems() {
        assert_eq!(file_title("Book1"), "Book1");
        assert_eq!(file_title("الرواتب.xlsx"), "الرواتب");
        assert_eq!(file_title("b:c*?.xlsx"), "bc");
        assert_eq!(file_title("..."), "Book");
        assert_eq!(file_title(""), "Book");
        assert_eq!(clock_stamp().len(), 6);
    }
}
