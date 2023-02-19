#![allow(non_camel_case_types)]

macro_rules! default {
    {$enum_name:ident : $tag:ident} => {
        impl Default for $enum_name {
            fn default() -> Self {
                Self::$tag
            }
        }
    };
}

default!{AlignItems: normal}
pub enum AlignItems {
    normal,

    stretch,
    center,
    start,
    end,
    flex_start,
    flex_end,
    baseline,
    first_baseline,
    last_baseline,
    safe_center,
    unsafe_center,

    inherit,
    initial,
    revert,
    unset,
}

pub enum Color {

}

default!{Display: none}
pub enum Display {
    none,

    block,
    inline,
    inline_block,
    flex,
    inline_flex,
    grid,
    inline_grid,
    flow_root,
    contents,
    table,
    table_row,
    list_item,

    inherit,
    initial,
    revert,
    unset,
}

