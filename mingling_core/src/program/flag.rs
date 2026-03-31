use crate::{Program, ProgramCollect};

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
macro_rules! special_flag {
    ($args:expr, $flag:expr) => {{
        let flag = $flag;
        let found = $args.iter().any(|arg| arg == flag);
        $args.retain(|arg| arg != flag);
        found
    }};
}

#[macro_export]
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

impl<C> Program<C>
where
    C: ProgramCollect,
{
    /// Registers a global argument (with value) and its handler.
    pub fn global_argument<F, A>(&mut self, arguments: A, do_fn: F)
    where
        F: Fn(&mut Program<C>, String),
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
        F: Fn(&mut Program<C>),
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
