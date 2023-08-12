#![allow(
    non_camel_case_types
)]

#![feature(
    unboxed_closures, fn_traits,
    tuple_trait
)]

#![allow(incomplete_features)]
#![feature(
    return_position_impl_trait_in_trait,
)]


pub(crate) mod library;
mod dom;
mod component;


pub use component::{tag::*, Component, HTML};
