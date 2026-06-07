use std::path::{Path, PathBuf};

/// Detect the project theme file for a given entry Frame file.
///
/// Looks for `app-theme.frame` in the same directory as the entry file.
/// Returns `None` if the file does not exist.
pub fn resolve_theme_file(entry_path: &Path) -> Option<PathBuf> {
    let parent = entry_path.parent()?;
    let theme = parent.join("app-theme.frame");
    if theme.exists() {
        Some(theme)
    } else {
        None
    }
}

#[allow(dead_code)]
/// Detect the project theme file for a source path that may not exist on disk.
/// Used when the entry is inferred from config rather than an existing path.
pub fn resolve_theme_for_source(source: &str) -> Option<PathBuf> {
    let parent = Path::new(source).parent()?;
    let theme = parent.join("app-theme.frame");
    if theme.exists() {
        Some(theme)
    } else {
        None
    }
}
