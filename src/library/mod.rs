use std::borrow::Cow;


pub(crate) type Cows = Cow<'static, str>;

pub trait IntoCows: Sized {
    fn into_cows(self) -> Cows;
    fn render_to(self, buf: &mut String) {
        buf.push('"');
        buf.push_str(&self.into_cows());
        buf.push('"')
    }
} const _: () = {
    impl IntoCows for Cows {
        fn into_cows(self) -> Cows {
            self
        }
    }

    impl IntoCows for &'static str {
        fn into_cows(self) -> Cows {
            Cow::Borrowed(self)
        }
    }

    impl IntoCows for String {
        fn into_cows(self) -> Cows {
            Cow::Owned(self)
        }
    }

    impl<'s> IntoCows for &'s String {
        fn into_cows(self) -> Cows {
            Cow::Owned(self.to_owned())
        }
    }

    impl IntoCows for usize {
        fn into_cows(self) -> Cows {
            match self {
                0 => Cow::Borrowed("0"),
                1 => Cow::Borrowed("1"),
                2 => Cow::Borrowed("2"),
                3 => Cow::Borrowed("3"),
                4 => Cow::Borrowed("4"),
                5 => Cow::Borrowed("5"),
                6 => Cow::Borrowed("6"),
                7 => Cow::Borrowed("7"),
                8 => Cow::Borrowed("8"),
                9 => Cow::Borrowed("9"),
                n => Cow::Owned(n.to_string()),
            }
        }
    }

    impl IntoCows for bool {
        fn into_cows(self) -> Cows {
            Cow::Borrowed(if self {"true"} else {"false"})
        }
    }
};


pub(crate) trait Optional<T> {
    fn into_option(self) -> Option<T>;
} const _: () = {
    impl<T> Optional<T> for T {
        fn into_option(self) -> Option<T> {
            Some(self)
        }
    }

    impl<T> Optional<T> for Option<T> {
        fn into_option(self) -> Option<T> {
            self
        }
    }
};
