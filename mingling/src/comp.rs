use std::collections::HashSet;

use mingling_core::{Flag, ShellContext, Suggest};

pub struct ShellContextHelper {
    ctx: ShellContext,
}

impl ShellContextHelper {
    /// Checks if a flag appears exactly once in the command line arguments.
    ///
    /// This method is useful for determining whether a flag should be processed
    /// when it should only be applied once, even if it appears multiple times
    /// in the command line. It returns `true` if the flag is present and
    /// appears exactly once among all words in the shell context.
    ///
    /// # Example
    ///
    /// ```
    /// # use mingling_core::ShellContext;
    /// # use mingling_macros::suggest;
    /// # use mingling::comp_tools::ShellContextHelper;
    ///
    /// let ctx = ShellContext::default();
    /// let helper = ShellContextHelper::from(ctx);
    ///
    /// // Check if either "--insert" or "-i" appears exactly once
    /// if helper.filling_argument_first(["--insert", "-i"]) {
    ///     // Perform action that should only happen once, example:
    ///     // return suggest! {
    ///     //     "A", "B", "C"
    ///     // }
    /// }
    /// ```
    pub fn filling_argument_first(&self, flag: impl Into<Flag>) -> bool {
        let flag = flag.into();
        if self.filling_argument(&flag) {
            let mut flag_appears = 0;
            for w in self.ctx.all_words.iter() {
                for f in flag.iter() {
                    if *f == w {
                        flag_appears += 1;
                    }
                }
            }
            if flag_appears < 2 {
                return true;
            }
        }
        return false;
    }

    /// Checks if the previous word in the command line arguments matches any of the given flags.
    ///
    /// This method determines whether a flag is currently being processed
    /// by checking the word immediately before the cursor position. It returns
    /// `true` if the previous word matches any of the provided flag strings.
    ///
    /// # Example
    ///
    /// ```
    /// # use mingling_core::ShellContext;
    /// # use mingling_macros::suggest;
    /// # use mingling::comp_tools::ShellContextHelper;
    ///
    /// let ctx = ShellContext::default();
    /// let helper = ShellContextHelper::from(ctx);
    ///
    /// // Check if the previous word is either "--file" or "-f"
    /// if helper.filling_argument(["--file", "-f"]) {
    ///     // The user is likely expecting a file argument next, example:
    ///     // return suggest! {
    ///     //     "src/main.rs", "Cargo.toml", "README.md"
    ///     // }
    /// }
    /// ```
    pub fn filling_argument(&self, flag: impl Into<Flag>) -> bool {
        for f in flag.into().iter() {
            if self.ctx.previous_word == **f {
                return true;
            }
        }
        return false;
    }

    /// Checks if the user is currently typing a flag argument.
    ///
    /// This method determines whether the current word being typed starts with
    /// a dash (`-`), indicating that the user is likely in the process of
    /// entering a command-line flag. It returns `true` if the current word
    /// begins with a dash character.
    ///
    /// # Example
    ///
    /// ```
    /// # use mingling_core::ShellContext;
    /// # use mingling_macros::suggest;
    /// # use mingling::comp_tools::ShellContextHelper;
    ///
    /// let ctx = ShellContext::default();
    /// let helper = ShellContextHelper::from(ctx);
    ///
    /// // Check if the user is typing a flag
    /// if helper.typing_argument() {
    ///     // The user is likely entering a flag, example:
    ///     // return suggest! {
    ///     //     "--help", "--version", "--verbose"
    ///     // }
    /// }
    /// ```
    pub fn typing_argument(&self) -> bool {
        self.ctx.current_word.starts_with("-")
    }

    /// Filters out already typed flag arguments from suggestion results.
    ///
    /// This method removes any suggestions that match flag arguments already present
    /// in the command line. It is useful for preventing duplicate flag suggestions
    /// when the user has already typed certain flags. The method processes both
    /// regular suggestion sets and file completion suggestions differently.
    pub fn strip_typed_argument(&self, suggest: Suggest) -> Suggest {
        let typed = Self::get_typed_arguments(&self);
        match suggest {
            Suggest::Suggest(mut set) => {
                set.retain(|item| !typed.contains(item.suggest()));
                Suggest::Suggest(set)
            }
            Suggest::FileCompletion => Suggest::FileCompletion,
        }
    }

    /// Retrieves all flag arguments from the command line.
    ///
    /// This method collects all words in the shell context that start with a dash (`-`),
    /// which typically represent command-line flags or options. It returns a vector
    /// containing these flag strings, converted to owned `String` values.
    pub fn get_typed_arguments(&self) -> HashSet<String> {
        self.ctx
            .all_words
            .iter()
            .filter(|word| word.starts_with("-"))
            .map(|word| word.to_string())
            .collect()
    }
}

impl From<ShellContext> for ShellContextHelper {
    fn from(ctx: ShellContext) -> Self {
        Self { ctx }
    }
}

impl From<ShellContextHelper> for ShellContext {
    fn from(helper: ShellContextHelper) -> Self {
        helper.ctx
    }
}
