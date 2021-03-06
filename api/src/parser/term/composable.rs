use super::Term;
use crate::parser::{Composable, Composition};

impl Composable for Term {
    /// Combines the `Composition` from the `Term`'s `Atom` with its own `exponent` to build a
    /// `Composition`. If the `Term` has no `Atom`, it has no dimension, thus will have an empty
    /// `Composition`.
    ///
    // TODO: https://agrian.atlassian.net/browse/DEV-971
    //
    fn composition(&self) -> Composition {
        match self.atom {
            Some(atom) => {
                let atom_composition = atom.composition();

                match self.exponent {
                    Some(term_exponent) => atom_composition * term_exponent,
                    None => atom_composition,
                }
            }
            // If there's no Atom in the Term, there's no dimension--even if there's an exponent on
            // the Term.
            None => Composition::default(),
        }
    }
}

impl Composable for Vec<Term> {
    fn composition(&self) -> Composition {
        self.iter()
            .fold(Composition::default(), |acc, term| acc * term.composition())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Atom, Composable, Composition, Dimension, Prefix, Term};

    macro_rules! validate_composition {
        ($test_name:ident, $term:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = $term;
                assert_eq!(term.composition(), $expected_value);
            }
        };

        ($test_name:ident, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let term = term!();
                assert_eq!(term.composition(), $expected_value);
            }
        };
    }

    // Composition tests
    validate_composition!(validate_composition_blank, Composition::default());
    validate_composition!(
        validate_composition_meter,
        term!(Meter),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_kilometer,
        term!(Kilo, Meter),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_meter_positive_non1_exponent,
        term!(Meter, exponent: 2),
        Composition::new(Dimension::Length, 2)
    );
    validate_composition!(
        validate_composition_meter_negative_exponent,
        term!(Meter, exponent: -1),
        Composition::new(Dimension::Length, -1)
    );
    validate_composition!(
        validate_composition_meter_negative_exponent2,
        term!(Meter, exponent: -2),
        Composition::new(Dimension::Length, -2)
    );
    validate_composition!(
        validate_composition_meter_factor,
        term!(Meter, factor: 10),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_kilometer_factor,
        term!(Kilo, Meter, factor: 10),
        Composition::new(Dimension::Length, 1)
    );
    validate_composition!(
        validate_composition_kilometer_factor_negative_exponent,
        term!(Kilo, Meter, factor: 10, exponent: -1),
        Composition::new(Dimension::Length, -1)
    );
}
