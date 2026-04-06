use size::Size;

use crate::parser::{Argument, Pickable};

impl Pickable for String {
    type Output = String;

    fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
        args.pick_argument(flag)
    }
}

impl Pickable for Vec<String> {
    type Output = Vec<String>;

    fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
        Some(args.pick_arguments(flag))
    }
}

macro_rules! impl_pickable_for_number {
    ($($t:ty),*) => {
        $(
            impl Pickable for $t {
                type Output = $t;

                fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
                    let picked = args.pick_argument(flag)?;
                    picked.parse().ok()
                }
            }

            impl Pickable for Vec<$t> {
                type Output = Vec<$t>;

                fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
                    let picked_vec = args.pick_arguments(flag);
                    let mut result = Vec::new();
                    for picked in picked_vec {
                        if let Ok(parsed) = picked.parse() {
                            result.push(parsed);
                        } else {
                            return None;
                        }
                    }
                    Some(result)
                }
            }
        )*
    };
}

impl_pickable_for_number!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);

impl Pickable for bool {
    type Output = bool;

    fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
        Some(args.pick_flag(flag))
    }
}

impl Pickable for usize {
    type Output = usize;

    fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
        let picked = args.pick_argument(flag)?;
        let size_parse = Size::from_str(picked.as_str());
        match size_parse {
            Ok(size) => Some(size.bytes() as usize),
            Err(_) => None,
        }
    }
}

impl Pickable for Vec<usize> {
    type Output = Vec<usize>;

    fn pick(args: &mut crate::parser::Argument, flag: mingling_core::Flag) -> Option<Self::Output> {
        let picked_vec = args.pick_arguments(flag);
        let mut result = Vec::new();
        for picked in picked_vec {
            let size_parse = Size::from_str(picked.as_str());
            match size_parse {
                Ok(size) => result.push(size.bytes() as usize),
                Err(_) => return None,
            }
        }
        Some(result)
    }
}

impl Pickable for Argument {
    type Output = Argument;

    fn pick(
        args: &mut crate::parser::Argument,
        _flag: mingling_core::Flag,
    ) -> Option<Self::Output> {
        Some(args.dump_remains().into())
    }
}
