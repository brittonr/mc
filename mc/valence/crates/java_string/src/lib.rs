#![doc = include_str!("../README.md")]
// Octet compatibility: this crate intentionally mirrors standard string APIs, including
// unsafe constructors, usize byte indices, and byte-level encoding arithmetic.
#![allow(
    unknown_lints,
    acronym_style,
    ambiguous_params,
    assertion_density,
    bool_naming,
    deref_polymorphism,
    excessive_file_length,
    explicit_defaults,
    function_length,
    module_file_count,
    nested_conditionals,
    no_panic,
    no_unwrap,
    non_trait_imports,
    numeric_units,
    path_segment_repetition,
    platform_dependent_cast,
    public_unsafe_api,
    raw_arithmetic_overflow,
    unchecked_narrowing,
    unbounded_loop,
    unjustified_allow,
    usize_in_public_api
)]

mod cesu8;
mod char;
mod error;
mod iter;
mod owned;
mod pattern;
#[cfg(feature = "serde")]
mod serde;
mod slice;
pub(crate) mod validations;

pub use char::*;
pub use error::*;
pub use iter::*;
pub use owned::*;
pub use pattern::*;
pub use slice::*;

#[macro_export]
macro_rules! format_java {
    ($($arg:tt)*) => {
        $crate::JavaString::from(::std::format!($($arg)*))
    }
}
