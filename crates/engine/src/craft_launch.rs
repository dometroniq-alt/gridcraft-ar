//! Starting a sibling AR app (WordCraft AR, DeckCraft AR) on a `.craftx` file, the way the Craft
//! Exchange spec (`docs/ar-exchange.md`, "Launching the sibling app") lays it out: the executable
//! beside ours, then its own folder under the same Program Files parent, then the Windows App
//! Paths keys the AR installers register, then the name on PATH. Nothing here panics: a sibling
//! that isn't found is reported, and the file stays where it was written.

use std::path::{Path, PathBuf};

/// The two apps a sheet can be sent to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sibling {
    WordCraft,
    DeckCraft,
}

impl Sibling {
    /// The product name shown to the user.
    pub fn title(self) -> &'static str {
        match self {
            Sibling::WordCraft => "WordCraft AR",
            Sibling::DeckCraft => "DeckCraft AR",
        }
    }
    /// Executable file names, in the order to try (the AR build first).
    fn exe_names(self) -> &'static [&'static str] {
        match self {
            Sibling::WordCraft => &["WordCraft AR.exe", "wordcraft-ar.exe", "wordcraft.exe", "wordcraft-ar", "wordcraft"],
            Sibling::DeckCraft => &["deckcraft.exe", "deckcraft-ar.exe", "deckcraft-ar", "deckcraft"],
        }
    }
    /// The sibling's own folder under the Program Files parent of ours.
    fn program_dir(self) -> &'static str {
        self.title()
    }
    /// `HKLM\...\App Paths\<key>` keys the installers register, AR key first.
    fn app_paths(self) -> &'static [&'static str] {
        match self {
            Sibling::WordCraft => &["WordCraft AR.exe", "wordcraft-ar.exe", "wordcraft.exe"],
            Sibling::DeckCraft => &["deckcraft-ar.exe", "deckcraft.exe"],
        }
    }
    /// Names looked up on PATH.
    fn path_names(self) -> &'static [&'static str] {
        match self {
            Sibling::WordCraft => &["wordcraft-ar", "wordcraft"],
            Sibling::DeckCraft => &["deckcraft-ar", "deckcraft"],
        }
    }
}

/// What came of a launch attempt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Launched {
    /// The sibling started from this executable (or PATH name) with the file as its argument.
    Started(PathBuf),
    /// No sibling was found (or starting one failed): the file is still there to open by hand.
    NotFound,
}

/// Starts `app` with `path` as its only argument. Never starts a program under `cfg(test)`
/// (the command fuzzers run every command) or in the browser.
pub fn launch(app: Sibling, path: &Path) -> Launched {
    if cfg!(test) || cfg!(target_arch = "wasm32") {
        return Launched::NotFound;
    }
    for exe in candidates(app) {
        if start(&exe, path) {
            return Launched::Started(exe);
        }
    }
    Launched::NotFound
}

/// Where the sibling may be, in the spec's order; existing files first, PATH names last.
pub fn candidates(app: Sibling) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    let mut push = |p: PathBuf| {
        if !out.contains(&p) {
            out.push(p);
        }
    };
    // 1. Beside the current executable, then `..\<Sibling> AR\` under the same parent.
    if let Some(dir) = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)) {
        for name in app.exe_names() {
            let beside = dir.join(name);
            if beside.is_file() {
                push(beside);
            }
        }
        if let Some(parent) = dir.parent() {
            for name in app.exe_names() {
                let sibling = parent.join(app.program_dir()).join(name);
                if sibling.is_file() {
                    push(sibling);
                }
            }
        }
    }
    // 2. Windows: the App Paths keys.
    if cfg!(windows) {
        for key in app.app_paths() {
            if let Some(p) = app_path(key) {
                push(p);
            }
        }
    }
    // 3. The name on PATH (the OS resolves it when the process starts).
    for name in app.path_names() {
        push(PathBuf::from(name));
    }
    out
}

/// The default value of `HKLM\Software\Microsoft\Windows\CurrentVersion\App Paths\<key>`, read
/// through `reg query` (no registry crate), when it names an existing file.
fn app_path(key: &str) -> Option<PathBuf> {
    let full = format!("HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\App Paths\\{key}");
    let output = std::process::Command::new("reg").args(["query", &full, "/ve"]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let value = parse_reg_value(&text)?;
    let p = PathBuf::from(value);
    p.is_file().then_some(p)
}

/// The string value in `reg query` output: the text after the `REG_SZ` (or `REG_EXPAND_SZ`)
/// column of the first line that has one, quotes trimmed. `None` for anything else.
pub fn parse_reg_value(output: &str) -> Option<String> {
    for line in output.lines() {
        let Some(at) = line.find("REG_") else { continue };
        let rest = line.get(at..)?;
        let mut parts = rest.splitn(2, char::is_whitespace);
        let _kind = parts.next();
        let value = parts.next()?.trim().trim_matches('"').trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}

/// Starts `exe` with `path` as its only argument; false when it can't be started.
fn start(exe: &Path, path: &Path) -> bool {
    match std::process::Command::new(exe).arg(path).spawn() {
        Ok(_) => true,
        Err(e) => {
            log::info!("{}: {e}", exe.display());
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launching_is_skipped_under_test_and_reports_not_found() {
        let dir = std::env::temp_dir();
        assert_eq!(launch(Sibling::WordCraft, &dir.join("x.craftx")), Launched::NotFound);
        assert_eq!(launch(Sibling::DeckCraft, &dir.join("x.craftx")), Launched::NotFound);
        // The candidate list ends with the PATH names, AR build first; nothing panics.
        let c = candidates(Sibling::DeckCraft);
        assert!(c.ends_with(&[PathBuf::from("deckcraft-ar"), PathBuf::from("deckcraft")]), "{c:?}");
        assert_eq!(Sibling::WordCraft.title(), "WordCraft AR");
    }

    #[test]
    fn reg_query_output_is_parsed_without_panicking() {
        let out = "\r\nHKEY_LOCAL_MACHINE\\Software\\Microsoft\\Windows\\CurrentVersion\\App Paths\\deckcraft-ar.exe\r\n    (Default)    REG_SZ    C:\\Program Files\\DeckCraft AR\\deckcraft.exe\r\n\r\n";
        assert_eq!(parse_reg_value(out).as_deref(), Some("C:\\Program Files\\DeckCraft AR\\deckcraft.exe"));
        assert_eq!(parse_reg_value("    (Default)    REG_EXPAND_SZ    \"C:\\x y\\a.exe\"").as_deref(), Some("C:\\x y\\a.exe"));
        for odd in ["", "ERROR: The system was unable to find the specified registry key or value.", "REG_SZ", "REG_SZ   ", "   REG_\n", "ü REG_SZ ß"]
        {
            let _ = parse_reg_value(odd);
        }
        assert_eq!(parse_reg_value("ü REG_SZ ß").as_deref(), Some("ß"));
    }
}
