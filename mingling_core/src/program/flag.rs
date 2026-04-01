use std::fmt::Display;

use crate::{Program, ProgramCollect};

/// A wrapper for a collection of static string slices representing command-line flags or arguments.
///
/// `Flag` is used to store one or more static string slices (e.g., `["-h", "--help"]`) that
/// represent command-line flags or arguments. It provides conversions from various input types
/// (like a single `&'static str`, a slice, or an array) and dereferences to a slice of strings
/// for easy iteration and access.
///
/// # Examples
///
/// ```
/// use mingling_core::Flag;
///
/// // Create a Flag from a single string slice
/// let flag1 = Flag::from("-h");
/// assert_eq!(flag1.as_ref(), &["-h"]);
///
/// // Create a Flag from a slice of string slices
/// let flag2 = Flag::from(&["-h", "--help"][..]);
/// assert_eq!(flag2.as_ref(), &["-h", "--help"]);
///
/// // Create a Flag from an array
/// let flag3 = Flag::from(["-v", "--verbose"]);
/// assert_eq!(flag3.as_ref(), &["-v", "--verbose"]);
///
/// // Create a Flag from a reference to an array
/// let arr = &["-f", "--file"];
/// let flag4 = Flag::from(arr);
/// assert_eq!(flag4.as_ref(), &["-f", "--file"]);
///
/// // Create an empty Flag from unit type
/// let flag5 = Flag::from(());
/// assert_eq!(flag5.as_ref(), &[] as &[&str]);
///
/// // Dereference to slice for iteration
/// let flag = Flag::from(["-a", "-b"]);
/// for arg in flag.iter() {
///     println!("Flag: {}", arg);
/// }
/// ```
pub struct Flag {
    vec: Vec<&'static str>,
}

impl From<()> for Flag {
    fn from(_: ()) -> Self {
        Flag { vec: vec![] }
    }
}

impl From<&'static str> for Flag {
    fn from(s: &'static str) -> Self {
        Flag { vec: vec![s] }
    }
}

impl From<&'static [&'static str]> for Flag {
    fn from(slice: &'static [&'static str]) -> Self {
        Flag {
            vec: slice.to_vec(),
        }
    }
}

impl<const N: usize> From<[&'static str; N]> for Flag {
    fn from(slice: [&'static str; N]) -> Self {
        Flag {
            vec: slice.to_vec(),
        }
    }
}

impl<const N: usize> From<&'static [&'static str; N]> for Flag {
    fn from(slice: &'static [&'static str; N]) -> Self {
        Flag {
            vec: slice.to_vec(),
        }
    }
}

impl AsRef<[&'static str]> for Flag {
    fn as_ref(&self) -> &[&'static str] {
        &self.vec
    }
}

impl std::ops::Deref for Flag {
    type Target = [&'static str];

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! special_flag {
    ($args:expr, $flag:expr) => {{
        let flag = $flag;
        let found = $args.iter().any(|arg| arg == flag);
        $args.retain(|arg| arg != flag);
        found
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! special_argument {
    ($args:expr, $flag:expr) => {{
        let flag = $flag;
        let mut value: Option<String> = None;
        let mut i = 0;
        while i < $args.len() {
            if &$args[i] == flag {
                if i + 1 < $args.len() {
                    value = Some($args[i + 1].clone());
                    $args.remove(i + 1);
                    $args.remove(i);
                } else {
                    value = None;
                    $args.remove(i);
                }
                break;
            }
            i += 1;
        }
        value
    }};
}

impl<C, G> Program<C, G>
where
    C: ProgramCollect,
    G: Display,
{
    /// Registers a global argument (with value) and its handler.
    pub fn global_argument<F, A>(&mut self, arguments: A, do_fn: F)
    where
        F: Fn(&mut Program<C, G>, String),
        A: Into<Flag>,
    {
        let flag = arguments.into();
        for argument in flag.iter() {
            let value = special_argument!(self.args, argument);
            if let Some(value) = value {
                do_fn(self, value);
                return;
            }
        }
    }

    /// Registers a global flag (boolean) and its handler.
    pub fn global_flag<F, A>(&mut self, flag: A, do_fn: F)
    where
        F: Fn(&mut Program<C, G>),
        A: Into<Flag>,
    {
        let flag = flag.into();
        for argument in flag.iter() {
            let enabled = special_flag!(self.args, argument);
            if enabled {
                do_fn(self);
                return;
            }
        }
    }

    /// Extracts a global argument (with value) from arguments
    pub fn pick_global_argument<F>(&mut self, flag: F) -> Option<String>
    where
        F: Into<Flag>,
    {
        let flag: Flag = flag.into();
        for argument in flag.iter() {
            let value = special_argument!(self.args, argument);
            if value.is_some() {
                return value;
            }
        }
        None
    }

    /// Extracts global flags from arguments
    pub fn pick_global_flag<F>(&mut self, flag: F) -> bool
    where
        F: Into<Flag>,
    {
        let flag: Flag = flag.into();
        for argument in flag.iter() {
            let enabled = special_flag!(self.args, argument);
            if enabled {
                return enabled;
            }
        }
        false
    }
}
