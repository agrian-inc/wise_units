use term::Term;
use terms::mapper::Finishable;

pub(super) struct Component {
    pub factor: Option<u32>,
    pub terms: Vec<Term>,
}

impl Finishable for Component {
    fn finish(mut self) -> Vec<Term> {
        if let Some(factor) = self.factor {
            if factor != 1 {
                if let Some(first_term) = self.terms.first_mut() {
                    first_term.factor = Some(factor);
                }
            }
        }

        self.terms
    }
}
