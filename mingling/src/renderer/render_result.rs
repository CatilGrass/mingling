use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Default, Debug, PartialEq)]
pub struct RenderResult {
    render_text: String,
}

impl Display for RenderResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.render_text.trim())
    }
}

impl Deref for RenderResult {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.render_text
    }
}

impl RenderResult {
    pub fn print(&mut self, text: &str) {
        self.render_text.push_str(text);
    }

    pub fn println(&mut self, text: &str) {
        self.render_text.push_str(text);
        self.render_text.push('\n');
    }

    pub fn clear(&mut self) {
        self.render_text.clear();
    }
}
