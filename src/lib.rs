#![feature(
    unboxed_closures, fn_traits,
    tuple_trait
)]

#![allow(incomplete_features)]
#![feature(
    return_position_impl_trait_in_trait,
)]


pub(crate) mod library;
mod html;
mod component;
mod dom;


pub(crate) use html::{HTML};
