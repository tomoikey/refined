pub mod composer;
mod empty;
mod for_all;
mod non_empty;
mod number;
mod string;

use crate::result::Error;
pub use empty::*;
pub use for_all::*;
pub use non_empty::*;
pub use number::*;
pub use string::*;

/// This is a `trait` that specifies the conditions a type `T` should satisfy
pub trait Rule {
    type Item;
    fn validate(target: &Self::Item) -> Result<(), Error>;
}
