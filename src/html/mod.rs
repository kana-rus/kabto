mod tag;

use crate::{
    library::{IntoCows, Cows},
};


pub trait HTML {
    fn render(self) -> Cows;
} const _: () = {
    impl HTML for () {
        fn render(self) -> Cows {
            Cows::Borrowed("")
        }
    }

    impl<IC: IntoCows> HTML for IC {
        fn render(self) -> Cows {
            self.escaped()
        }
    }

    macro_rules! impl_for_tuple {
        ($( $name:ident ),*) => {
            #[allow(non_snake_case)]
            impl<$( $name:HTML, )*> HTML for ( $( $name, )* ) {
                fn render(self) -> Cows {
                    let ( $( $name, )* ) = self;
                    Cows::Owned([$(
                        $name.render(),
                    )*].concat())
                }
            }
        };
    } const _: () = {
        impl_for_tuple!(H1);
        impl_for_tuple!(H1, H2);
        impl_for_tuple!(H1, H2, H3);
        impl_for_tuple!(H1, H2, H3, H4);
        impl_for_tuple!(H1, H2, H3, H4, H5);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10, H11);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10, H11, H12);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10, H11, H12, H13);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10, H11, H12, H13, H14);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10, H11, H12, H13, H14, H15);
        impl_for_tuple!(H1, H2, H3, H4, H5, H6, H7, H8, H9, H10, H11, H12, H13, H14, H15, H16);
    };
};
