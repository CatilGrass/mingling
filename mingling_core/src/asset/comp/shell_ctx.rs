use crate::ShellFlag;

/// Context passed from the shell to the completion system,
/// providing information about the current command line state
/// to guide how completions should be generated.
#[derive(Default, Debug)]
pub struct ShellContext {
    /// The full command line (-f / --command-line)
    pub command_line: String,

    /// Cursor position (-C / --cursor-position)
    pub cursor_position: usize,

    /// Current word (-w / --current-word)
    pub current_word: String,

    /// Previous word (-p / --previous-word)
    pub previous_word: String,

    /// Command name (-c / --command-name)
    pub command_name: String,

    /// Word index (-i / --word-index)
    pub word_index: usize,

    /// All words (-a / --all-words)
    pub all_words: Vec<String>,

    /// Flag to indicate completion context (-F / --shell-flag)
    pub shell_flag: ShellFlag,
}

impl TryFrom<Vec<String>> for ShellContext {
    type Error = String;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
        use std::collections::HashMap;

        // Parse arguments into a map for easy lookup
        let mut arg_map = HashMap::new();
        let mut i = 0;
        while i < args.len() {
            if args[i].starts_with('-') {
                let key = args[i].clone();
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    arg_map.insert(key, args[i + 1].clone());
                    i += 2;
                } else {
                    arg_map.insert(key, String::new());
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        // Extract values with defaults
        let command_line = arg_map.get("-f").cloned().unwrap_or_default();
        let cursor_position = arg_map
            .get("-C")
            .and_then(|s| s.parse().ok())
            .unwrap_or_default();
        let current_word = arg_map.get("-w").cloned().unwrap_or_default();
        let previous_word = arg_map.get("-p").cloned().unwrap_or_default();
        let command_name = arg_map.get("-c").cloned().unwrap_or_default();
        let word_index = arg_map
            .get("-i")
            .and_then(|s| s.parse().ok())
            .unwrap_or_default();
        let shell_flag = arg_map
            .get("-F")
            .cloned()
            .map(ShellFlag::from)
            .unwrap_or(ShellFlag::Other("unknown".to_string()));

        // Build all_words from command_line using basic whitespace splitting
        // Note: External input replaces '-' with '^' in arguments, so we need to restore them
        let all_words = command_line
            .split_whitespace()
            .map(|s| s.replace('^', "-"))
            .collect();

        // Also restore the original command_line with proper hyphens
        let command_line = command_line.replace('^', "-");

        Ok(ShellContext {
            command_line,
            cursor_position,
            current_word,
            previous_word,
            command_name,
            word_index,
            all_words,
            shell_flag,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_full_args() {
        let args = vec![
            "-f".to_string(),
            "git commit ^m 'test'".to_string(),
            "-C".to_string(),
            "12".to_string(),
            "-w".to_string(),
            "commit".to_string(),
            "-p".to_string(),
            "git".to_string(),
            "-c".to_string(),
            "git".to_string(),
            "-i".to_string(),
            "1".to_string(),
            "-F".to_string(),
            "bash".to_string(),
        ];

        let context = ShellContext::try_from(args).unwrap();
        assert_eq!(context.command_line, "git commit -m 'test'");
        assert_eq!(context.cursor_position, 12);
        assert_eq!(context.current_word, "commit");
        assert_eq!(context.previous_word, "git");
        assert_eq!(context.command_name, "git");
        assert_eq!(context.word_index, 1);
        assert_eq!(context.all_words, vec!["git", "commit", "-m", "'test'"]);
        assert!(matches!(context.shell_flag, ShellFlag::Bash));
    }

    #[test]
    fn test_try_from_partial_args() {
        let args = vec![
            "-f".to_string(),
            "ls ^la".to_string(),
            "-C".to_string(),
            "5".to_string(),
        ];

        let context = ShellContext::try_from(args).unwrap();
        assert_eq!(context.command_line, "ls -la");
        assert_eq!(context.cursor_position, 5);
        assert_eq!(context.current_word, "");
        assert_eq!(context.previous_word, "");
        assert_eq!(context.command_name, "");
        assert_eq!(context.word_index, 0);
        assert_eq!(context.all_words, vec!["ls", "-la"]);
        assert!(matches!(context.shell_flag, ShellFlag::Other(ref s) if s == "unknown"));
    }

    #[test]
    fn test_try_from_empty_args() {
        let args = vec![];
        let context = ShellContext::try_from(args).unwrap();
        assert_eq!(context.command_line, "");
        assert_eq!(context.cursor_position, 0);
        assert_eq!(context.current_word, "");
        assert_eq!(context.previous_word, "");
        assert_eq!(context.command_name, "");
        assert_eq!(context.word_index, 0);
        assert!(context.all_words.is_empty());
        assert!(matches!(context.shell_flag, ShellFlag::Other(ref s) if s == "unknown"));
    }

    #[test]
    fn test_try_from_flag_without_value() {
        let args = vec!["-F".to_string()];
        let context = ShellContext::try_from(args).unwrap();
        assert!(matches!(context.shell_flag, ShellFlag::Other(ref s) if s == ""));
    }

    #[test]
    fn test_all_words_splitting() {
        let args = vec!["-f".to_string(), "  cmd   arg1   arg2  ".to_string()];
        let context = ShellContext::try_from(args).unwrap();
        assert_eq!(context.all_words, vec!["cmd", "arg1", "arg2"]);
    }
}
