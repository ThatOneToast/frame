use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use frame_core::Document;
use frame_parser::parse;

use crate::diagnostics::print_diagnostics;

pub fn load_frame_document(
    file: &Path,
    include_paths: &[PathBuf],
    stack: &mut Vec<PathBuf>,
    seen: &mut HashSet<PathBuf>,
    theme_path: Option<&Path>,
) -> anyhow::Result<Document> {
    let file = fs::canonicalize(file).unwrap_or_else(|_| file.to_path_buf());

    if let Some(index) = stack.iter().position(|path| path == &file) {
        let mut cycle = stack[index..]
            .iter()
            .chain(std::iter::once(&file))
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>();
        cycle.dedup();
        anyhow::bail!(
            "Include cycle detected:\n\n{}\n\nRemove one include to break the cycle.",
            cycle.join(" -> ")
        );
    }

    if !seen.insert(file.clone()) {
        return Ok(Document {
            includes: Vec::new(),
            declarations: Vec::new(),
            components: Vec::new(),
        });
    }

    let source =
        fs::read_to_string(&file).with_context(|| format!("failed to read {}", file.display()))?;
    let document = match parse(&source) {
        Ok(document) => document,
        Err(error) => {
            print_diagnostics(&error.diagnostics);
            anyhow::bail!("Frame compile failed");
        }
    };

    stack.push(file.clone());
    let mut declarations = Vec::new();
    let mut components = Vec::new();
    for include in &document.includes {
        let candidates = include_candidates(&file, &include.target, include_paths);
        let Some(target) = candidates.iter().find(|candidate| candidate.exists()) else {
            let searched = candidates
                .iter()
                .map(|path| format!("- {}", path.display()))
                .collect::<Vec<_>>()
                .join("\n");
            anyhow::bail!(
                "Could not resolve include `{}`.\n\nSearched:\n{}",
                include.target,
                searched
            );
        };
        let included = load_frame_document(target, include_paths, stack, seen, theme_path)?;
        declarations.extend(included.declarations);
        components.extend(included.components);
    }
    // Load implicit theme file after explicit includes but before local declarations.
    // This gives local declarations precedence over the theme, and explicit includes
    // precedence over the theme as well (they are loaded earlier; duplicates will be
    // caught by the semantic validator).
    if let Some(theme) = theme_path {
        if theme != file && theme.exists() {
            let theme_doc = load_frame_document(theme, include_paths, stack, seen, None)?;
            declarations.extend(theme_doc.declarations);
            components.extend(theme_doc.components);
        }
    }

    stack.pop();

    declarations.extend(document.declarations);
    components.extend(document.components);
    Ok(Document {
        includes: document.includes,
        declarations,
        components,
    })
}

pub fn include_candidates(file: &Path, target: &str, include_paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let current_dir = file.parent().unwrap_or_else(|| Path::new("."));
    let target_path = Path::new(target);
    let with_extension = |path: PathBuf| {
        if path.extension().is_some() {
            path
        } else {
            path.with_extension("frame")
        }
    };

    if target.starts_with("./") || target.starts_with("../") || target_path.is_absolute() {
        candidates.push(with_extension(current_dir.join(target_path)));
    } else {
        candidates.push(with_extension(current_dir.join(target)));
        candidates.push(with_extension(PathBuf::from(target)));
        for include_path in include_paths {
            candidates.push(with_extension(include_path.join(target)));
        }
    }

    candidates
}
