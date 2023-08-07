#![allow(non_camel_case_types)]

mod a;
mod div;
mod h1;

use crate::library::{Cows, IntoCows};


pub(crate) struct BaseElement {
    pub(crate) class: Option<Cows>,
    pub(crate) id:    Option<Cows>,
    pub(crate) style: Option<Cows>,
} impl BaseElement {
    pub(crate) fn new() -> Self {
        Self { class: None, id: None, style: None }
    }
    pub(crate) fn render_to(self, buf: &mut String) {
        let Self { class, id, style } = self;
        if let Some(c) = class {  
            buf.push_str(" class=");
            c.render_to(buf)
        }
        if let Some(i) = id {
            buf.push_str(" id=");
            i.render_to(buf)
        }
        if let Some(s) = style {
            buf.push_str(" style=");
            s.render_to(buf)
        }
    }
}

/*
pub enum Dir {
    auto,
    ltr,
    rtl,
} impl Dir {
    pub(crate) fn render(&self) -> &'static str {
        match self {
            Self::auto => "\"auto\"",
            Self::ltr  => "\"ltr\"",
            Self::rtl  => "\"rtl\"",
        }
    }
}
*/

/*
pub enum Hidden {
    hidden,
    until_found,
} impl Hidden {
    pub(crate) fn render_to(self, buf: &mut String) {
        buf.push_str(match self {
            Self::hidden      => "\"hidden\"",
            Self::until_found => "\"until-found\"",
        })
    }
}
*/

/*
pub enum AriaRole {
    /*==== ROLES ====*/

    /*---- widget role ----*/
    button,
    checkbox,
    gridcell,
    link,
    menuitem,
    menuitemcheckbox,
    menuitemradio,
    option,
    progressbar,
    radio,
    scrollbar,
    searchbox,
    separator,
    slider,
    spinbutton,
    switch,
    tab,
    tabpanel,
    textbox,
    treeitem,

    /*---- composit role ----*/
    combobox,
    grid,
    listbox,
    menu,
    menubar,
    radiogroup,
    tablist,
    tree,
    treegrid,

    /*---- document structure role ----*/
    application,
    article,
    cell,
    columnheader,
    definition,
    directory,
    document,
    feed,
    figure,
    group,
    heading,
    img,
    list,
    listitem,
    math,
    none,
    note,
    presentation,
    row,
    rowgroup,
    rowheader,
    // separator,
    table,
    term,
    toolbar,
    tooltip,

    /*---- landmark role ----*/
    banner,
    complementary,
    contentinfo,
    form,
    main,
    navigation,
    region,
    search,
    
    /*---- live region role ----*/
    alert,
    log,
    marquee,
    status,
    timer,

    /*---- window role ----*/
    alertdialog,
    dialog,


    /*==== STATES & PROPERTIES ====*/

    /*---- widget attribute ----*/
    aria_autocomplete,
    aria_checked,
    aria_current,
    aria_disabled,
    aria_errormessage,
    aria_expanded,
    aria_haspopup,
    aria_hidden,
    aria_invalid,
    aria_label,
    aria_level,
    aria_modal,
    aria_multiline,
    arai_multiselectable,
    aria_orientation,
    aria_placeholder,
    aria_pressed,
    aria_readonly,
    aria_required,
    aria_selected,
    aria_sort,
    aria_valumax,
    aria_valumin,
    aria_valuenow,
    aria_valuetext,

    /*---- live region attribute ----*/
    aria_live,
    aria_relevant,
    aria_atomic,
    aria_busy,

    /*---- relationship attribute ----*/
    aria_activedescendant,
    aria_colcount,
    aria_colindex,
    aria_colspan,
    aria_controls,
    aria_describedby,
    aria_detials,
    // aria_errormessage,
    aria_flowto,
    aria_labelledby,
    aria_owns,
    aria_posinset,
    aria_rowcount,
    aria_rowindex,
    aria_rowspan,
    aria_setsize,
}
*/
