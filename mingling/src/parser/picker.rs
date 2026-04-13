use std::marker::PhantomData;

use crate::parser::Argument;
use mingling_core::{EnumTag, Flag};

#[doc(hidden)]
pub mod builtin;

#[doc(hidden)]
pub mod bools;

/// A builder for extracting values from command-line arguments.
///
/// The `Picker` struct holds parsed arguments and provides a fluent interface
/// to extract values associated with specific flags.
pub struct Picker<G> {
    /// The parsed command-line arguments.
    pub args: Argument,

    _phantom: PhantomData<G>,
}

impl<R> Picker<R> {
    /// Creates a new `Picker` from a value that can be converted into `Argument`.
    pub fn new(args: impl Into<Argument>) -> Picker<R> {
        Picker {
            args: args.into(),
            _phantom: PhantomData,
        }
    }

    /// Extracts a value for the given flag and returns a `Pick1` builder.
    ///
    /// The extracted type `TNext` must implement `Pickable` and `Default`.
    /// If the flag is not present, the default value for `TNext` is used.
    pub fn pick<TNext>(mut self, val: impl Into<Flag>) -> Pick1<TNext, R>
    where
        TNext: Pickable<Output = TNext> + Default,
    {
        let v = TNext::pick(&mut self.args, val.into()).unwrap_or_default();
        Pick1 {
            args: self.args,
            val_1: v,
            route: None,
        }
    }

    /// Extracts a value for the given flag, returning the provided default value if not present,
    /// and returns a `Pick1` builder.
    ///
    /// The extracted type `TNext` must implement `Pickable`.
    /// If the flag is not present, the provided `or` value is used.
    pub fn pick_or<TNext>(mut self, val: impl Into<Flag>, or: TNext) -> Pick1<TNext, R>
    where
        TNext: Pickable<Output = TNext>,
    {
        let v = TNext::pick(&mut self.args, val.into()).unwrap_or(or);
        Pick1 {
            args: self.args,
            val_1: v,
            route: None,
        }
    }

    /// Extracts a value for the given flag, storing the provided route if the flag is not present,
    /// and returns a `Pick1` builder.
    ///
    /// The extracted type `TNext` must implement `Pickable` and `Default`.
    /// If the flag is not present, the default value for `TNext` is used and the provided `route`
    /// is stored in the returned builder for later error handling.
    pub fn pick_or_route<TNext>(mut self, val: impl Into<Flag>, route: R) -> Pick1<TNext, R>
    where
        TNext: Pickable<Output = TNext> + Default,
    {
        let v = match TNext::pick(&mut self.args, val.into()) {
            Some(value) => value,
            None => {
                return Pick1 {
                    args: self.args,
                    val_1: TNext::default(),
                    route: Some(route),
                };
            }
        };
        Pick1 {
            args: self.args,
            val_1: v,
            route: None,
        }
    }

    /// Extracts a value for the given flag, returning `None` if the flag is not present,
    /// and returns an `Option<Pick1<TNext>>` builder.
    ///
    /// The extracted type `TNext` must implement `Pickable`.
    /// If the flag is not present, `None` is returned.
    pub fn require<TNext>(mut self, val: impl Into<Flag>) -> Option<Pick1<TNext, R>>
    where
        TNext: Pickable<Output = TNext>,
    {
        let v = TNext::pick(&mut self.args, val.into());
        match v {
            Some(s) => Some(Pick1 {
                args: self.args,
                val_1: s,
                route: None,
            }),
            None => None,
        }
    }

    /// Applies an operation to the parsed arguments and returns the modified `Picker`.
    ///
    /// Takes a closure that receives the current `Argument` and returns a new `Argument`.
    /// The returned `Argument` replaces the original arguments in the builder.
    /// This method can be used to modify or transform the parsed arguments before extracting values.
    pub fn operate_args<F: FnOnce(Argument) -> Argument>(mut self, operation: F) -> Self {
        self.args = operation(self.args);
        self
    }
}

impl<T: Into<Argument>, G> From<T> for Picker<G> {
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
    ($n:tt $final:ident $final_val:ident $($T:ident $val:ident),+) => {
        #[doc(hidden)]
        pub struct $n<$($T,)+ R>
        where
            $($T: Pickable,)+
        {
            #[allow(dead_code)]
            args: Argument,
            $(pub $val: $T,)+
            route: Option<R>,
        }

        impl<$($T,)+ R> From<$n<$($T,)+ R>> for ($($T,)+)
        where
            $($T: Pickable,)+
        {
            fn from(pick: $n<$($T,)+ R>) -> Self {
                ($(pick.$val,)+)
            }
        }

        impl<$($T,)+ R> $n<$($T,)+ R>
        where
            $($T: Pickable,)+
        {
            /// Unpacks the builder into a tuple of extracted values.
            ///
            /// Returns `Ok((T1, T2, ...))` if all required flags were present.
            /// Returns `Err(R)` if a required flag was missing and a route was provided via `pick_or_route`.
            pub fn unpack(self) -> Result<($($T,)+), R> {
                match self.route {
                    Some(route) => Err(route),
                    None => Ok(($(self.$val,)+)),
                }
            }

            /// Unpacks the builder into a tuple of extracted values.
            ///
            /// Returns the tuple of extracted values regardless of whether any required flags were missing.
            /// If a required flag was missing and a route was provided via `pick_or_route`, the default value
            /// for that type is included in the tuple.
            pub fn unpack_directly(self) -> ($($T,)+) {
                ($(self.$val,)+)
            }

            /// Applies a transformation to the last extracted value.
            ///
            /// Takes a closure that receives the last extracted value and returns a new value of the same type.
            /// The transformed value replaces the original value in the builder.
            /// This method can be used to modify or validate the extracted value before final unpacking.
            pub fn after<F>(mut self, edit: F) -> Self
            where
                F: Fn($final) -> $final,
            {
                self.$final_val = edit(self.$final_val);
                self
            }

            /// Applies an operation to the parsed arguments and returns the modified `Picker`.
            ///
            /// Takes a closure that receives the current `Argument` and returns a new `Argument`.
            /// The returned `Argument` replaces the original arguments in the builder.
            /// This method can be used to modify or transform the parsed arguments before extracting values.
            pub fn operate_args<F: FnOnce(Argument) -> Argument>(mut self, operation: F) -> Self {
                self.args = operation(self.args);
                self
            }
        }
    };
}

#[doc(hidden)]
macro_rules! impl_pick_structs {
    ($n:ident $next:ident $next_val:ident $($T:ident $val:ident),+) => {
        impl<$($T,)+ R> $n<$($T,)+ R>
        where
            $($T: Pickable,)+
        {
            /// Extracts a value for the given flag and returns a `PickN` builder.
            pub fn pick<TNext>(mut self, val: impl Into<mingling_core::Flag>) -> $next<$($T,)+ TNext, R>
            where
                TNext: Pickable<Output = TNext> + Default,
            {
                let v = TNext::pick(&mut self.args, val.into()).unwrap_or_default();
                $next {
                    args: self.args,
                    $($val: self.$val,)+
                    $next_val: v,
                    route: self.route,
                }
            }

            /// Extracts a value for the given flag, returning the provided default value if not present,
            /// and returns a `PickN` builder.
            ///
            /// The extracted type `TNext` must implement `Pickable`.
            /// If the flag is not present, the provided `or` value is used.
            pub fn pick_or<TNext>(mut self, val: impl Into<mingling_core::Flag>, or: TNext) -> $next<$($T,)+ TNext, R>
            where
                TNext: Pickable<Output = TNext>,
            {
                let v = TNext::pick(&mut self.args, val.into()).unwrap_or(or);
                $next {
                    args: self.args,
                    $($val: self.$val,)+
                    $next_val: v,
                    route: self.route,
                }
            }

            /// Extracts a value for the given flag, storing the provided route if the flag is not present,
            /// and returns a `PickN` builder.
            ///
            /// The extracted type `TNext` must implement `Pickable` and `Default`.
            /// If the flag is not present, the default value for `TNext` is used and the provided `route`
            /// is stored in the returned builder for later error handling.
            ///
            /// If a route was already stored from a previous `pick_or_route` call (i.e., `self.route` is `Some`),
            /// the existing route is preserved and the new `route` parameter is ignored.
            pub fn pick_or_route<TNext>(mut self, val: impl Into<mingling_core::Flag>, route: R) -> $next<$($T,)+ TNext, R>
            where
                TNext: Pickable<Output = TNext> + Default,
            {
                let v = match TNext::pick(&mut self.args, val.into()) {
                    Some(value) => value,
                    None => {
                        let new_route = match self.route {
                            Some(existing_route) => Some(existing_route),
                            None => Some(route),
                        };
                        return $next {
                            args: self.args,
                            $($val: self.$val,)+
                            $next_val: TNext::default(),
                            route: new_route,
                        };
                    }
                };
                $next {
                    args: self.args,
                    $($val: self.$val,)+
                    $next_val: v,
                    route: self.route,
                }
            }

            /// Extracts a value for the given flag, returning `None` if the flag is not present,
            /// and returns an `Option<PickN<TNext>>` builder.
            ///
            /// The extracted type `TNext` must implement `Pickable`.
            /// If the flag is not present, `None` is returned.
            pub fn require<TNext>(mut self, val: impl Into<mingling_core::Flag>) -> Option<$next<$($T,)+ TNext, R>>
            where
                TNext: Pickable<Output = TNext>,
            {
                let v = TNext::pick(&mut self.args, val.into());
                match v {
                    Some(s) => Some($next {
                        args: self.args,
                        $($val: self.$val,)+
                        $next_val: s,
                        route: self.route,
                    }),
                    None => None,
                }
            }
        }
    };
}

define_pick_structs! { Pick1 T1 val_1 T1 val_1 }
define_pick_structs! { Pick2 T2 val_2 T1 val_1, T2 val_2 }
define_pick_structs! { Pick3 T3 val_3 T1 val_1, T2 val_2, T3 val_3 }
define_pick_structs! { Pick4 T4 val_4 T1 val_1, T2 val_2, T3 val_3, T4 val_4 }
define_pick_structs! { Pick5 T5 val_5 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5 }
define_pick_structs! { Pick6 T6 val_6 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6 }
define_pick_structs! { Pick7 T7 val_7 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7 }
define_pick_structs! { Pick8 T8 val_8 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8 }
define_pick_structs! { Pick9 T9 val_9 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9 }
define_pick_structs! { Pick10 T10 val_10 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10 }
define_pick_structs! { Pick11 T11 val_11 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10, T11 val_11 }
define_pick_structs! { Pick12 T12 val_12 T1 val_1, T2 val_2, T3 val_3, T4 val_4, T5 val_5, T6 val_6, T7 val_7, T8 val_8, T9 val_9, T10 val_10, T11 val_11, T12 val_12 }

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

pub trait PickableEnum: EnumTag + Default {}

impl<T> Pickable for T
where
    T: PickableEnum,
{
    type Output = T;

    fn pick(args: &mut Argument, flag: Flag) -> Option<Self::Output> {
        let name = args.pick_argument(flag)?;
        T::build_enum(name)
    }
}
