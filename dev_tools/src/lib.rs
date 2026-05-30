use colored::Colorize;

#[macro_export]
macro_rules! run_cmd {
    ($fmt:literal, $($arg:tt)*) => {
        $crate::run_cmd(format!($fmt, $($arg)*))
    };
    ($cmd:expr) => {
        $crate::run_cmd($cmd)
    };
}

#[macro_export]
macro_rules! println_cargo_style {
    ($fmt:literal, $($arg:tt)*) => {
        $crate::println_cargo_style(format!($fmt, $($arg)*))
    };
    ($cmd:expr) => {
        $crate::println_cargo_style($cmd)
    };
}

#[macro_export]
macro_rules! eprintln_cargo_style {
    ($fmt:literal, $($arg:tt)*) => {
        $crate::eprintln_cargo_style(format!($fmt, $($arg)*))
    };
    ($cmd:expr) => {
        $crate::eprintln_cargo_style($cmd)
    };
}

/// Print a message in cargo style format, with bold green prefix.
///
/// # Panics
///
/// Panics if the prefix (text before the first `:`) exceeds 12 characters.
pub fn println_cargo_style(str: impl Into<String>) {
    let s = str.into();
    let (prefix, content) = if let Some(pos) = s.find(':') {
        (
            s[..pos].trim().to_string(),
            s[pos + 1..].trim_start().to_string(),
        )
    } else {
        (String::new(), s.trim().to_string())
    };

    assert!(
        prefix.len() <= 12,
        "prefix length exceeds 12: '{}' has length {}",
        prefix,
        prefix.len()
    );

    let padding = " ".repeat(12 - prefix.len());

    println!(
        "{}{} {}",
        padding,
        prefix.bold().bright_green(),
        content.trim()
    );
}

pub fn eprintln_cargo_style(str: impl Into<String>) {
    println!("{}: {}", "error".bold().bright_red(), str.into());
}

/// Run a shell command and return its exit status.
///
/// # Panics
///
/// Panics if the shell command cannot be spawned (e.g. the shell binary is not found).
///
/// # Errors
///
/// Returns `Err` with the exit code if the command finishes with a non-zero exit code.
pub fn run_cmd(cmd: impl Into<String>) -> Result<(), i32> {
    let shell = if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "sh"
    };
    let status = std::process::Command::new(shell)
        .arg("-c")
        .arg(cmd.into())
        .current_dir(std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")))
        .status()
        .expect("failed to execute command");

    let exit_code = status.code().unwrap_or(1);
    if exit_code == 0 {
        Ok(())
    } else {
        Err(exit_code)
    }
}

#[must_use]
pub fn cargo_tomls() -> Vec<std::path::PathBuf> {
    let mut cargo_tomls = Vec::new();
    let mut dirs = vec![std::path::PathBuf::from(".")];
    while let Some(dir) = dirs.pop() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Skip the dev_tools directory
                    if path.file_name().and_then(|n| n.to_str()) == Some("dev_tools") {
                        continue;
                    }
                    dirs.push(path);
                } else if path.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml") {
                    cargo_tomls.push(path);
                }
            }
        }
    }
    cargo_tomls
}
