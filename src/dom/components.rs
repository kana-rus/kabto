pub enum AnkerTarget {
    _blank,
    _self,
    _top,
    _parent,
} impl AnkerTarget {
    #[inline] pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::_blank => "_blank",
            Self::_self => "_self",
            Self::_top => "_top",
            Self::_parent => "_parent",
        }
    }
    #[inline] pub(crate) fn from_str(s: &str) -> Self {
        match s {
            "_blank" => Self::_blank,
            "_self" => Self::_self,
            "_top" => Self::_top,
            "_parent" => Self::_parent,
            _ => (|| panic!("Unknown `target`: \"{s}\""))()
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
    #[inline] pub(crate) const fn as_str(&self) -> &'static str {
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
    #[inline] pub(crate) fn from_str(s: &str) -> Self {
        match s {
            "alternate" => Self::alternate,
            "author" => Self::author,
            "bookmark" => Self::bookmark,
            "external" => Self::external,
            "help" => Self::help,
            "license" => Self::license,
            "next" => Self::next,
            "nofollow" => Self::nofollow,
            "noopener" => Self::noopener,
            "noreferrer" => Self::noreferrer,
            "opener" => Self::opener,
            "prev" => Self::prev,
            "search" => Self::search,
            "tag" => Self::tag,
            _ => (|| panic!("Unknown `rel`: \"{s}\""))()
        }
    }
}
