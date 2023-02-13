#![allow(incomplete_features)]
#![feature(
    return_position_impl_trait_in_trait,
    trait_alias,
)]

pub mod tag;
pub mod component;
pub mod string;

mod error;
mod config;

pub type Result<T> = std::result::Result<T, error::Error>;
