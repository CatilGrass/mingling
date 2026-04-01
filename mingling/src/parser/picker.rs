use crate::parser::Argument;
use mingling_core::Flag;

#[doc(hidden)]
pub mod builtin;

/// A builder for extracting values from command-line arguments.
///
/// The `Picker` struct holds parsed arguments and provides a fluent interface
/// to extract values associated with specific flags.
pub struct Picker {
    /// The parsed command-line arguments.
    pub args: Argument,
}

impl Picker {
    /// Creates a new `Picker` from a value that can be converted into `Argument`.
    pub fn new(args: impl Into<Argument>) -> Picker {
        Picker { args: args.into() }
    }

    /// Extracts a value for the given flag and returns a `Pick1` builder.
    ///
    /// The extracted type `TNext` must implement `Pickable` and `Default`.
    /// If the flag is not present, the default value for `TNext` is used.
    pub fn pick<TNext>(mut self, val: impl Into<Flag>) -> Pick1<TNext>
    where
        TNext: Pickable<Output = TNext> + Default,
    {
        let v = TNext::pick(&mut self.args, val.into()).unwrap_or_default();
        Pick1 {
            args: self.args,
            val_1: v,
        }
    }
}

impl<T: Into<Argument>> From<T> for Picker {
    fn from(value: T) -> Self {
        Picker::new(value)
    }
}

/// Extracts values from command-line arguments
///
/// The `Pickable` trait defines how to extract the value of a specific flag from parsed arguments
pub trait Pickable {
    /// The output type produced by the extraction operation, must implement the `Default` trait
    type Output: Default;

    /// Extracts the value associated with the given flag from the provided arguments
    ///
    /// If the flag exists and the value can be successfully extracted, returns `Some(Output)`;
    /// otherwise returns `None`
    fn pick(args: &mut Argument, flag: Flag) -> Option<Self::Output>;
}

#[doc(hidden)]
macro_rules! define_pick_structs {
    ($n:tt $($T:ident $val:ident),+) => {
        #[doc(hidden)]
        pub struct $n<$($T),+>
        where
            $($T: Pickable,)+
        {
            #[allow(dead_code)]
            args: Argument,
            $(pub $val: $T,)+
        }

        impl<$($T),+> From<$n<$($T),+>> for ($($T,)+)
        where
            $($T: Pickable,)+
        {
            fn from(pick: $n<$($T),+>) -> Self {
                ($(pick.$val,)+)
            }
        }

        impl<$($T),+> $n<$($T),+>
        where
            $($T: Pickable,)+
        {
            /// Unpacks into the corresponding tuple
            pub fn unpack(self) -> ($($T,)+) {
                ($(self.$val,)+)
            }
        }
    };
}

#[doc(hidden)]
macro_rules! impl_pick_structs {
    ($n:ident $next:ident $next_val:ident $($T:ident $val:ident),+) => {
        impl<$($T),+> $n<$($T),+>
        where
            $($T: Pickable,)+
        {
            /// Extracts a value for the given flag and returns a `PickN` builder.
            pub fn pick<TNext>(mut self, val: impl Into<mingling_core::Flag>) -> $next<$($T,)+ TNext>
            where
                TNext: Pickable<Output = TNext> + Default,
            {
                let v = TNext::pick(&mut self.args, val.into()).unwrap_or_default();
                $next {
                    args: self.args,
                    $($val: self.$val,)+
                    $next_val: v,
                }
            }
        }
    };
}

define_pick_structs! { Pick1 T1 val_1 }
define_pick_structs! { Pick2 T1 val_1, T2 val_2 }
define_pick_structs! { Pick3 T1 val_1, T2 val_2, T3 val_3 }
define_pick_structs! { Pick4 T1 val_1, T2 val_2, T3 val_3, T4 val_4 }
define_pick_structs! { Pick5 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5 }
define_pick_structs! { Pick6 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6 }
define_pick_structs! { Pick7 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7 }
define_pick_structs! { Pick8 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8 }
define_pick_structs! { Pick9 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9 }
define_pick_structs! { Pick10 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10 }
define_pick_structs! { Pick11 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10, T11 val_11 }
define_pick_structs! { Pick12 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10, T11 val_11, T12 val_12 }

impl_pick_structs! { Pick1 Pick2 val_2 T1 val_1 }
impl_pick_structs! { Pick2 Pick3 val_3 T1 val_1, T2 val_2 }
impl_pick_structs! { Pick3 Pick4 val_4 T1 val_1, T2 val_2, T3 val_3 }
impl_pick_structs! { Pick4 Pick5 val_5 T1 val_1, T2 val_2, T3 val_3, T4 val_4 }
impl_pick_structs! { Pick5 Pick6 val_6 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5 }
impl_pick_structs! { Pick6 Pick7 val_7 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6 }
impl_pick_structs! { Pick7 Pick8 val_8 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7 }
impl_pick_structs! { Pick8 Pick9 val_9 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8 }
impl_pick_structs! { Pick9 Pick10 val_10 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9 }
impl_pick_structs! { Pick10 Pick11 val_11 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10 }
impl_pick_structs! { Pick11 Pick12 val_12 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10, T11 val_11 }
