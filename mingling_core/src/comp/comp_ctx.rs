use crate::{COMPLETION_SUBCOMMAND, Program, ProgramCollect};

impl<C> Program<C>
where
    C: ProgramCollect<Enum = C>,
{
    /// Checks whether the program is currently in a completion mode.
    ///
    /// This is determined by checking if the special completion subcommand
    /// (defined by [`COMPLETION_SUBCOMMAND`]) appears among the parsed arguments.
    /// When `true`, the program should generate shell completions instead of
    /// running its normal execution path.
    pub fn is_completing(&self) -> bool {
        // Check if the first argument (args[1]) is the completion subcommand
        self.args
            .get(1)
            .is_some_and(|arg| arg == COMPLETION_SUBCOMMAND)
    }
}
