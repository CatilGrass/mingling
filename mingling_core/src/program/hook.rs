#![allow(dead_code)]

use std::any::Any;

use crate::{AnyOutput, Program, ProgramCollect, RenderResult};

#[cfg(not(feature = "async"))]
use crate::error::ProgramPanic;

#[derive(Default)]
pub struct ProgramHook<C>
where
    C: ProgramCollect<Enum = C>,
{
    /// Executes when the program starts running
    pub begin: Option<fn()>,

    /// Executes before the program dispatches
    pub pre_dispatch: Option<fn(args: &[String])>,

    /// Executes after the program dispatches
    pub post_dispatch: Option<fn(entry: &C)>,

    /// Executes before the type enters the chain
    pub pre_chain: Option<fn(input: &C, raw: &dyn Any)>,

    /// Executes after the chain processing for the type ends
    pub post_chain: Option<fn(output: &AnyOutput<C>)>,

    /// Executes before the type enters the renderer
    pub pre_render: Option<fn(input: &C, raw: &dyn Any)>,

    /// Executes after the type enters the renderer
    pub post_render: Option<fn(result: &RenderResult)>,

    /// Executes before the program ends
    pub finish: Option<fn() -> i32>,

    /// Executes when the program panics
    #[cfg(not(feature = "async"))]
    pub exec_panic: Option<fn(&ProgramPanic)>,

    /// Executes when the REPL starts (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_on_begin: Option<fn()>,

    /// Executes before reading the next REPL line (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_pre_readline: Option<fn()>,

    /// Custom REPL line reader (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_readline: Option<fn() -> Option<String>>,

    /// Executes after reading a REPL line (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_post_readline: Option<fn(line: &mut String)>,

    /// Executes before executing a REPL command (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_pre_exec: Option<fn(args: &[String])>,

    /// Executes after executing a REPL command (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_post_exec: Option<fn()>,

    /// Executes when the REPL receives a render result (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_on_receive_result: Option<fn(&RenderResult)>,

    /// Executes when the REPL panics (only available with `repl` feature)
    #[cfg(all(feature = "repl", not(feature = "async")))]
    pub repl_on_panic: Option<fn(&ProgramPanic)>,

    /// Executes when the REPL exits (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_exit: Option<fn()>,

    /// Executes after each REPL loop iteration (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub repl_loop_once: Option<fn()>,
}

impl<C> Program<C>
where
    C: ProgramCollect<Enum = C>,
{
    /// Adds a typed hook to the program. The hook will be called at the appropriate
    /// lifecycle events.
    pub fn with_hook(&mut self, hook: ProgramHook<C>) {
        self.hooks.push(hook);
    }

    pub(crate) fn run_hook_on_begin(&self) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(begin) = hook.begin {
                begin();
            }
        }
    }

    pub(crate) fn run_hook_pre_dispatch(&self, args: &[String]) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(pre_dispatch) = hook.pre_dispatch {
                pre_dispatch(args);
            }
        }
    }

    pub(crate) fn run_hook_post_dispatch(&self, entry: &C) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(post_dispatch) = hook.post_dispatch {
                post_dispatch(entry);
            }
        }
    }

    pub(crate) fn run_hook_pre_chain(&self, input: &C, raw: &dyn Any) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(pre_chain) = hook.pre_chain {
                pre_chain(input, raw);
            }
        }
    }

    pub(crate) fn run_hook_post_chain(&self, output: &AnyOutput<C>) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(post_chain) = hook.post_chain {
                post_chain(output);
            }
        }
    }

    pub(crate) fn run_hook_pre_render(&self, input: &C, raw: &dyn Any) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(pre_render) = hook.pre_render {
                pre_render(input, raw);
            }
        }
    }

    pub(crate) fn run_hook_post_render(&self, result: &RenderResult) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(post_render) = hook.post_render {
                post_render(result);
            }
        }
    }

    #[allow(dead_code)]
    #[cfg(not(feature = "async"))]
    pub(crate) fn run_hook_exec_panic(&self, panic_info: &ProgramPanic) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(exec_panic) = hook.exec_panic {
                exec_panic(panic_info);
            }
        }
    }

    pub(crate) fn run_hook_finish(&self) -> i32 {
        if !self.user_context.run_hook {
            return 0;
        }

        let mut exit_code = 0;
        for hook in &self.hooks {
            if let Some(finish) = hook.finish {
                exit_code = finish();
                if exit_code != 0 {
                    return exit_code;
                }
            }
        }
        exit_code
    }

    /// Runs the REPL begin hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_on_begin(&self) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_on_begin) = hook.repl_on_begin {
                repl_on_begin()
            }
        }
    }

    /// Runs the REPL pre-readline hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_pre_readline(&self) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_pre_readline) = hook.repl_pre_readline {
                repl_pre_readline()
            }
        }
    }

    /// Runs the custom REPL readline hook (only available with `repl` feature)
    /// Returns `Some(line)` if a hook was set and returned Some, otherwise `None`.
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_readline(&self) -> Option<String> {
        if !self.user_context.run_hook {
            return None;
        }

        for hook in &self.hooks {
            if let Some(repl_readline) = hook.repl_readline {
                return repl_readline();
            }
        }
        None
    }

    /// Runs the REPL post-readline hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_post_readline(&self, line: &mut String) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_post_readline) = hook.repl_post_readline {
                repl_post_readline(line)
            }
        }
    }

    /// Runs the REPL pre-exec hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_pre_exec(&self, args: &[String]) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_pre_exec) = hook.repl_pre_exec {
                repl_pre_exec(args)
            }
        }
    }

    /// Runs the REPL post-exec hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_post_exec(&self) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_post_exec) = hook.repl_post_exec {
                repl_post_exec()
            }
        }
    }

    /// Runs the REPL receive result hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_on_receive_result(&self, result: &RenderResult) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_on_receive_result) = hook.repl_on_receive_result {
                repl_on_receive_result(result)
            }
        }
    }

    /// Runs the REPL panic hooks (only available with `repl` feature)
    #[cfg(all(feature = "repl", not(feature = "async")))]
    pub(crate) fn run_hook_repl_on_panic(&self, panic_info: &ProgramPanic) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_on_panic) = hook.repl_on_panic {
                repl_on_panic(panic_info)
            }
        }
    }

    /// Runs the REPL exit hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_exit(&self) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_exit) = hook.repl_exit {
                repl_exit()
            }
        }
    }

    /// Runs the REPL loop_once hooks (only available with `repl` feature)
    #[cfg(feature = "repl")]
    pub(crate) fn run_hook_repl_loop_once(&self) {
        if !self.user_context.run_hook {
            return;
        }

        for hook in &self.hooks {
            if let Some(repl_loop_once) = hook.repl_loop_once {
                repl_loop_once()
            }
        }
    }
}

impl<C> ProgramHook<C>
where
    C: ProgramCollect<Enum = C>,
{
    /// Creates a new empty hook set with no handlers.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            begin: None,
            pre_dispatch: None,
            post_dispatch: None,
            pre_chain: None,
            post_chain: None,
            pre_render: None,
            post_render: None,
            finish: None,
            #[cfg(not(feature = "async"))]
            exec_panic: None,
            #[cfg(feature = "repl")]
            repl_on_begin: None,
            #[cfg(feature = "repl")]
            repl_pre_readline: None,
            #[cfg(feature = "repl")]
            repl_readline: None,
            #[cfg(feature = "repl")]
            repl_post_readline: None,
            #[cfg(feature = "repl")]
            repl_pre_exec: None,
            #[cfg(feature = "repl")]
            repl_post_exec: None,
            #[cfg(feature = "repl")]
            repl_on_receive_result: None,
            #[cfg(all(feature = "repl", not(feature = "async")))]
            repl_on_panic: None,
            #[cfg(feature = "repl")]
            repl_exit: None,
            #[cfg(feature = "repl")]
            repl_loop_once: None,
        }
    }

    /// Sets the handler for the `begin` event.
    #[must_use]
    pub fn on_begin(mut self, handler: fn()) -> Self {
        let _ = self.begin.insert(handler);
        self
    }

    /// Sets the handler for the `pre_dispatch` event.
    #[must_use]
    pub fn on_pre_dispatch(mut self, handler: fn(args: &[String])) -> Self {
        let _ = self.pre_dispatch.insert(handler);
        self
    }

    /// Sets the handler for the `post_dispatch` event.
    #[must_use]
    pub fn on_post_dispatch(mut self, handler: fn(entry: &C)) -> Self {
        let _ = self.post_dispatch.insert(handler);
        self
    }

    /// Sets the handler for the `pre_chain` event.
    #[must_use]
    pub fn on_pre_chain(mut self, handler: fn(input: &C, raw: &dyn Any)) -> Self {
        let _ = self.pre_chain.insert(handler);
        self
    }

    /// Sets the handler for the `post_chain` event.
    #[must_use]
    pub fn on_post_chain(mut self, handler: fn(output: &AnyOutput<C>)) -> Self {
        let _ = self.post_chain.insert(handler);
        self
    }

    /// Sets the handler for the `pre_render` event.
    #[must_use]
    pub fn on_pre_render(mut self, handler: fn(input: &C, raw: &dyn Any)) -> Self {
        let _ = self.pre_render.insert(handler);
        self
    }

    /// Sets the handler for the `post_render` event.
    #[must_use]
    pub fn on_post_render(mut self, handler: fn(result: &RenderResult)) -> Self {
        let _ = self.post_render.insert(handler);
        self
    }

    /// Sets the handler for the `finish` event.
    #[must_use]
    pub fn on_finish(mut self, handler: fn() -> i32) -> Self {
        let _ = self.finish.insert(handler);
        self
    }

    /// Sets the handler for the `exec_panic` event.
    #[cfg(not(feature = "async"))]
    #[must_use]
    pub fn on_exec_panic(mut self, handler: fn(&ProgramPanic)) -> Self {
        let _ = self.exec_panic.insert(handler);
        self
    }

    /// Sets the handler for the REPL begin event (only available with `repl` feature).
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_begin(mut self, handler: fn()) -> Self {
        let _ = self.repl_on_begin.insert(handler);
        self
    }

    /// Sets the handler for the REPL pre-readline event (only available with `repl` feature).
    /// This hook runs after `on_repl_begin` but before reading the next input line.
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_pre_readline(mut self, handler: fn()) -> Self {
        let _ = self.repl_pre_readline.insert(handler);
        self
    }

    /// Sets the custom REPL line reader (only available with `repl` feature).
    /// If set, this function will be called to read a line instead of the default mechanism.
    /// Returning `None` signals that there is no input (e.g., EOF).
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_readline(mut self, handler: fn() -> Option<String>) -> Self {
        let _ = self.repl_readline.insert(handler);
        self
    }

    /// Sets the handler for the REPL post-readline event (only available with `repl` feature).
    /// This hook runs after reading a line of input and receives a mutable reference to the line.
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_post_readline(mut self, handler: fn(line: &mut String)) -> Self {
        let _ = self.repl_post_readline.insert(handler);
        self
    }

    /// Sets the handler for the REPL pre-exec event (only available with `repl` feature).
    /// This hook runs before executing a REPL command, receiving the parsed arguments.
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_pre_exec(mut self, handler: fn(args: &[String])) -> Self {
        let _ = self.repl_pre_exec.insert(handler);
        self
    }

    /// Sets the handler for the REPL post-exec event (only available with `repl` feature).
    /// This hook runs after executing a REPL command.
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_post_exec(mut self, handler: fn()) -> Self {
        let _ = self.repl_post_exec.insert(handler);
        self
    }

    /// Sets the handler for the REPL receive result event (only available with `repl` feature).
    /// This hook runs after a command is executed, receiving the render result on success.
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_receive_result(mut self, handler: fn(result: &RenderResult)) -> Self {
        let _ = self.repl_on_receive_result.insert(handler);
        self
    }

    /// Sets the handler for the REPL panic event (only available with `repl` feature).
    #[cfg(all(feature = "repl", not(feature = "async")))]
    #[must_use]
    pub fn on_repl_panic(mut self, handler: fn(panic: &ProgramPanic)) -> Self {
        let _ = self.repl_on_panic.insert(handler);
        self
    }

    /// Sets the handler for the REPL exit event (only available with `repl` feature).
    /// This hook runs when the REPL is about to exit.
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_exit(mut self, handler: fn()) -> Self {
        let _ = self.repl_exit.insert(handler);
        self
    }

    /// Sets the handler for the REPL loop_once event (only available with `repl` feature).
    /// This hook runs after each REPL loop iteration.
    #[cfg(feature = "repl")]
    #[must_use]
    pub fn on_repl_loop_once(mut self, handler: fn()) -> Self {
        let _ = self.repl_loop_once.insert(handler);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Groupped;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "general_renderer", derive(serde::Serialize))]
    enum MockHookEnum {
        A,
        B,
    }

    impl std::fmt::Display for MockHookEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Groupped<MockHookEnum> for MockHookEnum {
        fn member_id() -> MockHookEnum {
            MockHookEnum::A
        }
    }

    impl ProgramCollect for MockHookEnum {
        type Enum = MockHookEnum;
        type ErrorDispatcherNotFound = MockHookEnum;
        type ErrorRendererNotFound = MockHookEnum;
        type ResultEmpty = MockHookEnum;

        fn build_renderer_not_found(_member_id: MockHookEnum) -> AnyOutput<MockHookEnum> {
            unreachable!()
        }

        fn build_dispatcher_not_found(_args: Vec<String>) -> AnyOutput<MockHookEnum> {
            unreachable!()
        }

        fn build_empty_result() -> AnyOutput<MockHookEnum> {
            unreachable!()
        }

        fn render(_any: AnyOutput<MockHookEnum>, _r: &mut RenderResult) {
            unreachable!()
        }

        fn render_help(_any: AnyOutput<MockHookEnum>, _r: &mut RenderResult) {
            unreachable!()
        }

        fn do_chain(_any: AnyOutput<MockHookEnum>) -> crate::ChainProcess<MockHookEnum> {
            unreachable!()
        }

        fn has_renderer(_any: &AnyOutput<MockHookEnum>) -> bool {
            unreachable!()
        }

        fn has_chain(_any: &AnyOutput<MockHookEnum>) -> bool {
            unreachable!()
        }

        #[cfg(feature = "comp")]
        fn do_comp(_any: &AnyOutput<MockHookEnum>, _ctx: &crate::ShellContext) -> crate::Suggest {
            unreachable!()
        }

        #[cfg(feature = "general_renderer")]
        fn general_render(
            _any: AnyOutput<MockHookEnum>,
            _setting: &crate::GeneralRendererSetting,
        ) -> Result<crate::RenderResult, crate::error::GeneralRendererSerializeError> {
            unreachable!()
        }
    }

    #[test]
    fn test_hook_empty() {
        let hook = ProgramHook::<MockHookEnum>::empty();
        assert!(hook.begin.is_none());
        assert!(hook.pre_dispatch.is_none());
        assert!(hook.post_dispatch.is_none());
        assert!(hook.pre_chain.is_none());
        assert!(hook.post_chain.is_none());
        assert!(hook.pre_render.is_none());
        assert!(hook.post_render.is_none());
        assert!(hook.finish.is_none());
    }

    #[test]
    fn test_hook_on_begin() {
        static CALLED: AtomicBool = AtomicBool::new(false);
        let hook = ProgramHook::<MockHookEnum>::empty().on_begin(|| {
            CALLED.store(true, Ordering::SeqCst);
        });
        assert!(hook.begin.is_some());
        (hook.begin.unwrap())();
        assert!(CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hook_on_pre_dispatch() {
        static CALLED: AtomicBool = AtomicBool::new(false);
        let hook = ProgramHook::<MockHookEnum>::empty().on_pre_dispatch(|args| {
            assert_eq!(args, &["a", "b"]);
            CALLED.store(true, Ordering::SeqCst);
        });
        assert!(hook.pre_dispatch.is_some());
        (hook.pre_dispatch.unwrap())(&["a".to_string(), "b".to_string()]);
        assert!(CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hook_on_post_dispatch() {
        static CALLED: AtomicBool = AtomicBool::new(false);
        let hook = ProgramHook::<MockHookEnum>::empty().on_post_dispatch(|entry| {
            assert_eq!(*entry, MockHookEnum::A);
            CALLED.store(true, Ordering::SeqCst);
        });
        assert!(hook.post_dispatch.is_some());
        (hook.post_dispatch.unwrap())(&MockHookEnum::A);
        assert!(CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hook_on_pre_chain() {
        static CALLED: AtomicBool = AtomicBool::new(false);
        let hook = ProgramHook::<MockHookEnum>::empty().on_pre_chain(
            |input: &MockHookEnum, _raw: &dyn Any| {
                assert_eq!(*input, MockHookEnum::A);
                CALLED.store(true, Ordering::SeqCst);
            },
        );
        assert!(hook.pre_chain.is_some());
        (hook.pre_chain.unwrap())(&MockHookEnum::A, &42);
        assert!(CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hook_on_post_chain() {
        static CALLED: AtomicBool = AtomicBool::new(false);
        let hook = ProgramHook::<MockHookEnum>::empty().on_post_chain(
            |_output: &AnyOutput<MockHookEnum>| {
                CALLED.store(true, Ordering::SeqCst);
            },
        );
        assert!(hook.post_chain.is_some());
        let output = AnyOutput::new(MockHookEnum::A);
        (hook.post_chain.unwrap())(&output);
        assert!(CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hook_on_pre_render() {
        static CALLED: AtomicBool = AtomicBool::new(false);
        let hook = ProgramHook::<MockHookEnum>::empty().on_pre_render(
            |input: &MockHookEnum, _raw: &dyn Any| {
                assert_eq!(*input, MockHookEnum::A);
                CALLED.store(true, Ordering::SeqCst);
            },
        );
        assert!(hook.pre_render.is_some());
        (hook.pre_render.unwrap())(&MockHookEnum::A, &42);
        assert!(CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hook_on_post_render() {
        static CALLED: AtomicBool = AtomicBool::new(false);
        let hook = ProgramHook::<MockHookEnum>::empty().on_post_render(|_result: &RenderResult| {
            CALLED.store(true, Ordering::SeqCst);
        });
        assert!(hook.post_render.is_some());
        let result = RenderResult::default();
        (hook.post_render.unwrap())(&result);
        assert!(CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hook_on_finish() {
        let hook = ProgramHook::<MockHookEnum>::empty().on_finish(|| 42);
        assert!(hook.finish.is_some());
        assert_eq!((hook.finish.unwrap())(), 42);
    }

    #[test]
    fn test_hook_builder_chaining() {
        let hook = ProgramHook::<MockHookEnum>::empty()
            .on_begin(|| {})
            .on_pre_dispatch(|_| {})
            .on_post_dispatch(|_| {})
            .on_pre_chain(|_, _| {})
            .on_post_chain(|_| {})
            .on_pre_render(|_, _| {})
            .on_post_render(|_| {})
            .on_finish(|| 0);
        assert!(hook.begin.is_some());
        assert!(hook.pre_dispatch.is_some());
        assert!(hook.post_dispatch.is_some());
        assert!(hook.pre_chain.is_some());
        assert!(hook.post_chain.is_some());
        assert!(hook.pre_render.is_some());
        assert!(hook.post_render.is_some());
        assert!(hook.finish.is_some());
    }
}
