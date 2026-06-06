use colored::Colorize;

/// Formats a message in cargo-style format with a bold green prefix.
///
/// The message should be in the format `prefix: content`. The prefix will be
/// bold green and right-padded to 12 characters. If there is no colon in the
/// string, the entire string is printed as content with an empty prefix.
///
/// # Macros
///
/// - `format_cargo!("prefix: {}", arg)` — format-style invocation
/// - `format_cargo!(expr)` — direct expression invocation
///
/// # Panics
///
/// Panics if the prefix (text before the first `:`) exceeds 12 characters.
///
/// # Examples
///
/// ```
/// format_cargo!("Compiling: hello.rs");
/// // Output: "   Compiling hello.rs" (green bold "Compiling" padded to 12)
/// ```
#[macro_export]
macro_rules! format_cargo {
    ($fmt:literal, $($arg:tt)*) => {
        $crate::get_cargo_info_format(format!($fmt, $($arg)*))
    };
    ($cmd:expr) => {
        $crate::get_cargo_info_format($cmd)
    };
}

/// Formats an error message in cargo-style format with a bold red "error" prefix.
///
/// # Macros
///
/// - `eformat_cargo!("prefix: {}", arg)` — format-style invocation
/// - `eformat_cargo!(expr)` — direct expression invocation
///
/// # Examples
///
/// ```
/// eformat_cargo!("failed to parse input");
/// // Output: "error: failed to parse input" (red bold "error")
/// ```
#[macro_export]
macro_rules! eformat_cargo {
    ($fmt:literal, $($arg:tt)*) => {
        $crate::get_cargo_error_format(format!($fmt, $($arg)*))
    };
    ($cmd:expr) => {
        $crate::get_cargo_error_format($cmd)
    };
}

/// Print a message in cargo-style format with a bold green prefix.
///
/// # Macros
///
/// - `println_cargo!("prefix: {}", arg)` — format-style invocation
/// - `println_cargo!(expr)` — direct expression invocation
///
/// # Examples
///
/// ```
/// println_cargo!("Compiling: hello.rs");
/// ```
#[macro_export]
macro_rules! println_cargo {
    ($fmt:literal, $($arg:tt)*) => {
        println!("{}", $crate::get_cargo_info_format(format!($fmt, $($arg)*)))
    };
    ($cmd:expr) => {
        println!("{}", $crate::get_cargo_info_format($cmd))
    };
}

/// Print an error message in cargo-style format with a bold red "error" prefix.
///
/// # Macros
///
/// - `eprintln_cargo!("prefix: {}", arg)` — format-style invocation
/// - `eprintln_cargo!(expr)` — direct expression invocation
///
/// # Examples
///
/// ```
/// eprintln_cargo!("failed to parse input");
/// ```
#[macro_export]
macro_rules! eprintln_cargo {
    ($fmt:literal, $($arg:tt)*) => {
        eprintln!("{}", $crate::get_cargo_error_format(format!($fmt, $($arg)*)))
    };
    ($cmd:expr) => {
        eprintln!("{}", $crate::get_cargo_error_format($cmd))
    };
}

/// Format a message in cargo style format, with bold green prefix.
///
/// The input string is split at the first `:`. The part before the colon becomes
/// the prefix (bold green, right-padded to 12 characters), and the part after
/// becomes the content.
///
/// If no colon is found, the entire string is treated as content and no prefix
/// is shown.
///
/// # Panics
///
/// Panics if the prefix (text before the first `:`) exceeds 12 characters.
///
/// # Examples
///
/// ```ignore
/// get_cargo_info_format("Compiling: my_program.rs");
/// // returns "   Compiling my_program.rs"
/// ```
pub fn get_cargo_info_format(str: impl Into<String>) -> String {
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

    format!(
        "{}{} {}",
        padding,
        prefix.bold().bright_green(),
        content.trim()
    )
}

/// Format an error message in cargo style format, with bold red "error" prefix.
///
/// The input string is printed as the error content, prefixed by a bold red
/// `error:` label.
///
/// # Examples
///
/// ```ignore
/// get_cargo_error_format("something went wrong");
/// // returns "error: something went wrong"
/// ```
pub fn get_cargo_error_format(str: impl Into<String>) -> String {
    format!("{}: {}", "error".bold().bright_red(), str.into())
}
