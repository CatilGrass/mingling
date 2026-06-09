use crate::{ProgramCollect, program::Program};

pub trait ProgramSetup<C>
where
    C: ProgramCollect<Enum = C>,
{
    fn setup(self, program: &mut Program<C>);
}

impl<C> Program<C>
where
    C: ProgramCollect<Enum = C>,
{
    /// Load and execute init logic
    pub fn with_setup<S: ProgramSetup<C> + 'static>(&mut self, setup: S) {
        S::setup(setup, self);
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

    struct TestSetup {
        called: std::rc::Rc<std::cell::Cell<bool>>,
    }

    impl ProgramSetup<MockCollect> for TestSetup {
        fn setup(self, _program: &mut Program<MockCollect>) {
            self.called.set(true);
        }
    }

    #[test]
    fn test_with_setup_calls_setup() {
        let called = std::rc::Rc::new(std::cell::Cell::new(false));
        let setup = TestSetup {
            called: std::rc::Rc::clone(&called),
        };
        let mut program: Program<MockCollect> = Program::new_with_args(["test"]);
        program.with_setup(setup);
        assert!(called.get());
    }
}
