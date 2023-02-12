#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]

pub mod tag;
pub mod component;

mod error;
mod config;

pub type Result<T> = std::result::Result<T, error::Error>;
