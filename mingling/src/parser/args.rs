use std::mem::replace;

use mingling_core::{Flag, special_argument, special_flag};

#[derive(Debug, Default)]
pub struct Argument {
    vec: Vec<String>,
}

impl From<&'static str> for Argument {
    fn from(s: &'static str) -> Self {
        Argument {
            vec: vec![s.to_string()],
        }
    }
}

impl From<&'static [&'static str]> for Argument {
    fn from(slice: &'static [&'static str]) -> Self {
        Argument {
            vec: slice.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

impl<const N: usize> From<[&'static str; N]> for Argument {
    fn from(slice: [&'static str; N]) -> Self {
        Argument {
            vec: slice.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

impl<const N: usize> From<&'static [&'static str; N]> for Argument {
    fn from(slice: &'static [&'static str; N]) -> Self {
        Argument {
            vec: slice.iter().map(|&s| s.to_string()).collect(),
        }
    }
}

impl From<Vec<String>> for Argument {
    fn from(vec: Vec<String>) -> Self {
        Argument { vec }
    }
}

impl AsRef<[String]> for Argument {
    fn as_ref(&self) -> &[String] {
        &self.vec
    }
}

impl std::ops::Deref for Argument {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl std::ops::DerefMut for Argument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl Argument {
    /// Extracts argument (with value) from arguments
    pub fn pick_argument<F>(&mut self, flag: F) -> Option<String>
    where
        F: Into<Flag>,
    {
        let flag: Flag = flag.into();
        for argument in flag.iter() {
            let value = special_argument!(self.vec, argument);
            if value.is_some() {
                return value;
            }
        }
        None
    }

    /// Extracts flags from arguments
    pub fn pick_flag<F>(&mut self, flag: F) -> bool
    where
        F: Into<Flag>,
    {
        let flag: Flag = flag.into();
        for argument in flag.iter() {
            let enabled = special_flag!(self.vec, argument);
            if enabled {
                return enabled;
            }
        }
        false
    }

    /// Dump all remaining arguments
    pub fn dump_remains(&mut self) -> Vec<String> {
        let new = Vec::new();
        replace(&mut self.vec, new)
    }
}
