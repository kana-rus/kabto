mod tag;


pub trait HTML {
    fn render(self) -> crate::Cows;
}

impl<IC: crate::IntoCows> HTML for IC {
    fn render(self) -> crate::Cows {
        self.into_cows()
    }
}

// TODO: impl for (), (IC1,), (IC1,IC2), ...
