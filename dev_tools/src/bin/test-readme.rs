use colored::Colorize;
use std::path::{Path, PathBuf};

use tools::{eprintln_cargo_style, println_cargo_style, run_cmd_and_capture_stderr};

/// Represents a parsed code block from README.md
struct CodeBlock {
    /// The line number in README.md where this block starts
    line: usize,
    /// The raw Rust source code
    code: String,
    /// Feature flags extracted from `// Features: [...]` comment
    features: Vec<String>,
    /// External dependencies extracted from `// Dependencies:` comments
    external_deps: Vec<(String, String)>,
    /// Whether this block has a `fn main` entry point
    has_main: bool,
    /// Whether this block has `gen_program!()` call
    has_gen_program: bool,
}

fn main() {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let readme_path = PathBuf::from("README.md");
    if !readme_path.exists() {
        eprintln_cargo_style!("README.md not found in current directory");
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(&readme_path).unwrap_or_else(|e| {
        eprintln_cargo_style!("Failed to read README.md: {}", e);
        std::process::exit(1);
    });

    let blocks = parse_code_blocks(&content);
    if blocks.is_empty() {
        eprintln_cargo_style!("No Rust code blocks found in README.md");
        std::process::exit(1);
    }

    println_cargo_style!("Test: found {} Rust code blocks in README.md", blocks.len());

    // Ensure temp directory exists
    let temp_dir = PathBuf::from(".temp/readme-test");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(temp_dir.join("src")).unwrap_or_else(|e| {
        eprintln_cargo_style!("Failed to create temp directory: {}", e);
        std::process::exit(1);
    });

    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut results: Vec<(usize, bool, String)> = Vec::new();

    for (i, block) in blocks.iter().enumerate() {
        let label = format!("Block {} (line {})", i + 1, block.line);
        print!("  {label} ... ");

        let (ok, err) = build_block(&temp_dir, block);
        if ok {
            println!("{}", "passed".bold().bright_green());
            passed += 1;
            results.push((block.line, true, String::new()));
        } else {
            println!("{}", "failed".bold().bright_red());
            failed += 1;
            results.push((block.line, false, err.clone()));
            eprintln_cargo_style!("  {} FAILED:\n{}", label, err);
        }
    }

    println_cargo_style!(
        "Result: {passed}/{total} blocks passed",
        total = blocks.len()
    );

    write_summary_report(
        Path::new(".temp/README-TEST-RESULT.md"),
        &results,
        blocks.len(),
        passed,
        failed,
    );

    if failed > 0 {
        eprintln_cargo_style!("{failed} block(s) failed to build");
        std::process::exit(1);
    }

    println_cargo_style!("Done: All README code blocks build successfully!");
}

/// Parse all ```rust code blocks from README content
fn parse_code_blocks(content: &str) -> Vec<CodeBlock> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].trim() == "```rust" {
            if let Some(block) = parse_single_block(&lines, i) {
                blocks.push(block);
            }
            i += 1;
            while i < lines.len() && lines[i].trim() != "```" {
                i += 1;
            }
        }
        i += 1;
    }

    blocks
}

/// Parse a single code block starting at the ```rust line
fn parse_single_block(lines: &[&str], start: usize) -> Option<CodeBlock> {
    let line_num = start + 1; // 1-based line number

    let mut code_lines: Vec<String> = Vec::new();
    let mut features: Vec<String> = Vec::new();
    let mut external_deps: Vec<(String, String)> = Vec::new();
    let mut has_main = false;
    let mut has_gen_program = false;

    let mut idx = start + 1;
    let mut in_header = true;

    while idx < lines.len() {
        let raw_line = lines[idx];
        let trimmed = raw_line.trim();

        if trimmed == "```" {
            break;
        }

        // Parse header comments
        if in_header && trimmed.starts_with("// ") {
            if trimmed.starts_with("// Features:") {
                let feat_str = trimmed.trim_start_matches("// Features:").trim();
                if feat_str.starts_with('[') && feat_str.ends_with(']') {
                    let inner = &feat_str[1..feat_str.len() - 1];
                    if !inner.is_empty() {
                        features = inner
                            .split(',')
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
                idx += 1;
                continue;
            }
            if trimmed == "// Dependencies:" {
                idx += 1;
                // Collect subsequent `// crate = "version"` lines
                while idx < lines.len() {
                    let next = lines[idx].trim();
                    if next == "```" {
                        break;
                    }
                    if next.starts_with("// ") {
                        let dep_line = next.trim_start_matches("// ").trim();
                        if let Some((name, ver)) = dep_line.split_once(" = ") {
                            external_deps.push((
                                name.trim().to_string(),
                                ver.trim().trim_matches('"').to_string(),
                            ));
                        }
                        idx += 1;
                    } else {
                        break;
                    }
                }
                continue;
            }
        }

        in_header = false;

        if raw_line.contains("fn main") {
            has_main = true;
        }
        if raw_line.contains("gen_program!") {
            has_gen_program = true;
        }

        code_lines.push(raw_line.to_string());
        idx += 1;
    }

    if code_lines.is_empty() {
        return None;
    }

    Some(CodeBlock {
        line: line_num,
        code: code_lines.join("\n"),
        features,
        external_deps,
        has_main,
        has_gen_program,
    })
}

/// Generate a Cargo.toml for a block
fn generate_cargo_toml(block: &CodeBlock) -> String {
    let features_str = if !block.features.is_empty() {
        let feats: Vec<String> = block.features.iter().map(|f| format!("\"{f}\"")).collect();
        format!("features = [{}]", feats.join(", "))
    } else {
        String::new()
    };

    let mut extra_deps = String::new();
    for (name, version) in &block.external_deps {
        if !version.starts_with('{') {
            // Plain version string, e.g. "1"
            if name == "serde" || name == "clap" {
                extra_deps.push_str(&format!(
                    "{name} = {{ version = \"{version}\", features = [\"derive\"] }}\n"
                ));
            } else {
                extra_deps.push_str(&format!("{name} = \"{version}\"\n"));
            }
        } else {
            // Already in TOML inline table format, e.g. { version = "1", features = [...] }
            extra_deps.push_str(&format!("{name} = {version}\n"));
        }
    }

    let deps_section = if features_str.is_empty() {
        format!("[dependencies]\nmingling = {{ path = \"../../mingling\" }}\n{extra_deps}")
    } else {
        format!(
            "[dependencies]\nmingling = {{ path = \"../../mingling\", {features_str} }}\n{extra_deps}"
        )
    };

    format!(
        r#"[package]
name = "readme-test-block"
version = "0.0.0"
edition = "2024"

{deps_section}
[workspace]
"#
    )
}

/// Generate main.rs for a block
fn generate_main_rs(block: &CodeBlock) -> String {
    let mut output = String::from("#![allow(dead_code)]\n\n");

    output.push_str(&block.code);
    output.push('\n');

    if !block.has_main {
        output.push_str("\nfn main() {}\n");
    }

    if !block.has_gen_program {
        output.push_str("\nmingling::macros::gen_program!();\n");
    }

    output
}

/// Build a single code block as a Cargo project.
/// Always writes to `{temp_dir}/Cargo.toml` and `{temp_dir}/src/main.rs`.
/// Returns (success, error_message).
fn build_block(temp_dir: &Path, block: &CodeBlock) -> (bool, String) {
    let src_dir = temp_dir.join("src");
    if let Err(e) = std::fs::create_dir_all(&src_dir) {
        return (false, format!("mkdir: {e}"));
    }

    // Write Cargo.toml
    let cargo_toml = generate_cargo_toml(block);
    if let Err(e) = std::fs::write(temp_dir.join("Cargo.toml"), &cargo_toml) {
        return (false, format!("write Cargo.toml: {e}"));
    }

    // Write main.rs
    let main_rs = generate_main_rs(block);
    if let Err(e) = std::fs::write(src_dir.join("main.rs"), &main_rs) {
        return (false, format!("write main.rs: {e}"));
    }

    // Build with release (single run via shared macro)
    let manifest_path = temp_dir.join("Cargo.toml");
    match run_cmd_and_capture_stderr!(
        "cargo build --release --manifest-path {}",
        manifest_path.to_string_lossy()
    ) {
        Ok(_) => (true, String::new()),
        Err((code, log)) => {
            let mut last_lines: Vec<&str> = log.lines().rev().take(20).collect();
            last_lines.reverse();
            let detail = last_lines.join("\n");
            (false, format!("exit code {code}\n{detail}"))
        }
    }
}

/// Write the .temp/README-TEST-RESULT.md summary report
fn write_summary_report(
    path: &Path,
    results: &[(usize, bool, String)],
    total: usize,
    passed: usize,
    failed: usize,
) {
    let mut content = String::new();
    content.push_str("# README Code Block Test Report\n\n");
    content.push_str(&format!(
        "Tested **{total}** code blocks: **{passed}** passed, **{failed}** failed.\n\n"
    ));
    content.push_str("## Results\n\n");
    content.push_str("| Block | Line | Status |\n");
    content.push_str("|-------|------|--------|\n");

    for (i, (line, ok, _)) in results.iter().enumerate() {
        let status = if *ok { "PASS" } else { "FAIL" };
        content.push_str(&format!("| {} | {} | {status} |\n", i + 1, line));
    }

    let has_failures = results.iter().any(|(_, ok, _)| !ok);
    if has_failures {
        content.push_str("\n## Failed Blocks\n\n");
        for (i, (line, ok, err)) in results.iter().enumerate() {
            if !ok {
                content.push_str(&format!(
                    "### Block {} (line {})\n\n```\n{err}\n```\n\n",
                    i + 1,
                    line
                ));
            }
        }
    }

    std::fs::write(path, &content).unwrap_or_else(|e| {
        eprintln!("Warning: failed to write {path:?}: {e}");
    });

    println_cargo_style!("Report: written to {}", path.display());
}
