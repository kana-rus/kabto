#![allow(incomplete_features)]
#![feature(
    return_position_impl_trait_in_trait,
)]

pub mod tag;
pub mod html;
pub mod string;
pub mod component;

mod error;
mod config;

pub type Result<T> = std::result::Result<T, error::Error>;

pub mod macros {
    pub use kabto_macros::{Parent};
}
pub(crate) mod internal_macros {
    pub use kabto_macros::{ParentTag, html_escape};
}
