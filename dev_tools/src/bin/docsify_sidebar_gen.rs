use std::collections::BTreeMap;
use std::path::Path;

const PAGES_ROOT: &str = "./docs/pages";
const SIDEBAR_PATH: &str = "./docs/_sidebar.md";

const SIDEBAR_HEAD: &str = "- [Welcome!](README)\n";

fn main() {
    println!("Refreshing _sidebar.md");
    gen_sidebar();
}

fn gen_sidebar() {
    let repo_root = find_git_repo().unwrap();
    let pages_root = repo_root.join(PAGES_ROOT);

    let mut lines = String::from(SIDEBAR_HEAD);

    // Collect and sort entries at root level first
    let mut root_files: Vec<SidebarEntry> = Vec::new();
    // Subdirectory name -> its files
    let mut sub_dirs: BTreeMap<String, Vec<SidebarEntry>> = BTreeMap::new();

    if let Ok(read_dir) = std::fs::read_dir(&pages_root) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                let entries = collect_markdown_files(&path);
                if !entries.is_empty() {
                    sub_dirs.insert(dir_name, entries);
                }
            } else if path.extension().map_or(false, |ext| ext == "md") {
                let title = extract_title(&path);
                let relative = path
                    .strip_prefix(&repo_root.join("docs"))
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                let link = relative
                    .strip_suffix(".md")
                    .unwrap_or(&relative)
                    .to_string();
                root_files.push(SidebarEntry { title, link });
            }
        }
    }

    // Sort root files by link for stable order
    root_files.sort_by(|a, b| a.link.cmp(&b.link));

    // Append root-level files
    for f in &root_files {
        lines.push_str(&format!("* [{}]({})\n", f.title, f.link));
    }

    // Append subdirectory groups
    for (dir_name, entries) in &sub_dirs {
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by(|a, b| a.link.cmp(&b.link));

        // Directory header with 2-space indent
        lines.push_str(&format!("* {}\n", dir_name));
        for f in &sorted_entries {
            lines.push_str(&format!("  * [{}]({})\n", f.title, f.link));
        }
    }

    let sidebar_path = repo_root.join(SIDEBAR_PATH);
    std::fs::write(&sidebar_path, lines).unwrap();
    println!("  Generated: {}", sidebar_path.display());
}

#[derive(Clone)]
struct SidebarEntry {
    title: String,
    link: String,
}

/// Collect all `.md` files directly under `dir` (non-recursive).
fn collect_markdown_files(dir: &Path) -> Vec<SidebarEntry> {
    let mut entries = Vec::new();
    let repo_root = find_git_repo().unwrap();
    let docs_root = repo_root.join("docs");

    if let Ok(read_dir) = std::fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "md") {
                let title = extract_title(&path);
                let relative = path
                    .strip_prefix(&docs_root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                let link = relative
                    .strip_suffix(".md")
                    .unwrap_or(&relative)
                    .to_string();
                entries.push(SidebarEntry { title, link });
            }
        }
    }

    entries
}

/// Extract title from the first line `<h1 align="center">TITLE</h1>`.
/// Fallback to filename stem.
fn extract_title(path: &Path) -> String {
    let content = std::fs::read_to_string(path).unwrap_or_default();
    if let Some(first_line) = content.lines().next() {
        let trimmed = first_line.trim();
        // Find `>TITLE<` between `<h1 align="center">` and `</h1>`
        if let Some(start) = trimmed.find('>') {
            let after_start = &trimmed[start + 1..];
            if let Some(end) = after_start.find('<') {
                return after_start[..end].to_string();
            }
        }
    }
    // Fallback: use file stem
    path.file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string())
}

fn find_git_repo() -> Option<std::path::PathBuf> {
    let mut current_dir = std::env::current_dir().ok()?;

    loop {
        let git_dir = current_dir.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            return Some(current_dir);
        }

        if !current_dir.pop() {
            break;
        }
    }

    None
}
