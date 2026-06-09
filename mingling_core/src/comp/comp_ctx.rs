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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnyOutput, ChainProcess, Groupped, RenderResult};

    /// Minimal mock collector that satisfies `C: ProgramCollect<Enum = C>`
    /// by setting `Enum = Self`.
    #[derive(Debug, Clone, PartialEq)]
    struct MockCollect;

    impl Groupped<MockCollect> for MockCollect {
        fn member_id() -> MockCollect {
            MockCollect
        }
    }

    impl ProgramCollect for MockCollect {
        type Enum = MockCollect;
        type ErrorDispatcherNotFound = MockCollect;
        type ErrorRendererNotFound = MockCollect;
        type ResultEmpty = MockCollect;

        fn build_renderer_not_found(_member_id: MockCollect) -> AnyOutput<MockCollect> {
            unimplemented!()
        }
        fn build_dispatcher_not_found(_args: Vec<String>) -> AnyOutput<MockCollect> {
            unimplemented!()
        }
        fn build_empty_result() -> AnyOutput<MockCollect> {
            unimplemented!()
        }
        fn render(_any: AnyOutput<MockCollect>, _r: &mut RenderResult) {
            unimplemented!()
        }
        fn render_help(_any: AnyOutput<MockCollect>, _r: &mut RenderResult) {
            unimplemented!()
        }
        fn do_chain(_any: AnyOutput<MockCollect>) -> ChainProcess<MockCollect> {
            unimplemented!()
        }
        #[cfg(feature = "comp")]
        fn do_comp(_any: &AnyOutput<MockCollect>, _ctx: &crate::ShellContext) -> crate::Suggest {
            unimplemented!()
        }
        fn has_renderer(_any: &AnyOutput<MockCollect>) -> bool {
            unimplemented!()
        }
        fn has_chain(_any: &AnyOutput<MockCollect>) -> bool {
            unimplemented!()
        }

        #[cfg(feature = "general_renderer")]
        fn general_render(
            _any: AnyOutput<MockCollect>,
            _setting: &crate::GeneralRendererSetting,
        ) -> Result<RenderResult, crate::error::GeneralRendererSerializeError> {
            unimplemented!()
        }
    }

    #[test]
    fn test_is_completing_with_comp_subcommand() {
        let program: Program<MockCollect> =
            Program::new_with_args(["program", "__comp", "some", "args"]);
        assert!(program.is_completing());
    }

    #[test]
    fn test_is_completing_with_normal_subcommand() {
        let program: Program<MockCollect> = Program::new_with_args(["program", "normal", "cmd"]);
        assert!(!program.is_completing());
    }

    #[test]
    fn test_is_completing_with_no_args() {
        let program: Program<MockCollect> = Program::new_with_args(["program"]);
        assert!(!program.is_completing());
    }
}
