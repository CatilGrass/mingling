/// Program stdout settings
#[derive(Debug, Clone)]
pub struct ProgramStdoutSetting {
    /// Output error messages
    pub error_output: bool,

    /// Render results and output
    pub render_output: bool,
}

impl Default for ProgramStdoutSetting {
    fn default() -> Self {
        ProgramStdoutSetting {
            error_output: true,
            render_output: true,
        }
    }
}

/// Program stdout settings
#[derive(Debug, Clone, Default)]
pub struct ProgramUserContext {
    /// View help information instead of running the command
    pub help: bool,

    /// Skip user confirmation step
    pub confirm: bool,
}
