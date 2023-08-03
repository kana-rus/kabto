use std::borrow::Cow;


pub(crate) type Cows = Cow<'static, str>;

pub(crate) trait IntoCows {
    fn into_cows(self) -> Cows;
} const _: () = {
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
};
