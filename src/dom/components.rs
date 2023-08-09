pub enum AnkerTarget {
    _blank,
    _self,
    _top,
    _parent,
} impl AnkerTarget {
    #[inline] pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::_blank => "_blank",
            Self::_self => "_self",
            Self::_top => "_top",
            Self::_parent => "_parent",
        }
    }
}

pub enum AnkerRel {
    alternate,
    author,
    bookmark,
    external,
    help,
    license,
    next,
    nofollow,
    noopener,
    noreferrer,
    opener,
    prev,
    search,
    tag,
} impl AnkerRel {
    #[inline] pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::alternate => "alternate",
            Self::author => "author",
            Self::bookmark => "bookmark",
            Self::external => "external",
            Self::help => "help",
            Self::license => "license",
            Self::next => "next",
            Self::nofollow => "nofollow",
            Self::noopener => "noopener",
            Self::noreferrer => "noreferrer",
            Self::opener => "opener",
            Self::prev => "prev",
            Self::search => "search",
            Self::tag => "tag",
        }
    }
}
