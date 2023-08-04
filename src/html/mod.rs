mod tag;

use crate::{Cows, IntoCows};


pub trait HTML {
    fn render(self) -> Cows;
}


const _: () = {
    impl HTML for () {
        fn render(self) -> Cows {
            Cows::Borrowed("")
        }
    }

    impl<IC: IntoCows> HTML for IC {
        fn render(self) -> Cows {
            self.into_cows()
        }
    }
    impl<IC: IntoCows> HTML for (IC,) {
        fn render(self) -> Cows {
            self.0.into_cows()
        }
    }

    macro_rules! impl_for_tuple_of_multi_values {
        ($( $name:ident ),+ $(,)?) => {
            #[allow(non_snake_case)]
            impl<$( $name:IntoCows, )+> HTML for ( $( $name, )+ ) {
                fn render(self) -> Cows {
                    let ( $( $name, )+ ) = self;
                    Cows::Owned([$(
                        $name.into_cows(),
                    )+].concat())
                }
            }
        };
    } const _: () = {
        impl_for_tuple_of_multi_values!(IC1, IC2);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9, IC10);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9, IC10, IC11);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9, IC10, IC11, IC12);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9, IC10, IC11, IC12, IC13);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9, IC10, IC11, IC12, IC13, IC14);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9, IC10, IC11, IC12, IC13, IC14, IC15);
        impl_for_tuple_of_multi_values!(IC1, IC2, IC3, IC4, IC5, IC6, IC7, IC8, IC9, IC10, IC11, IC12, IC13, IC14, IC15, IC16);
    };
};
