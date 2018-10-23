use as_fraction::AsFraction;
use parser::Term;
use unit::Unit;

impl AsFraction for Unit {
    type Numerator = Unit;
    type Denominator = Unit;

    fn numerator(&self) -> Self::Numerator {
        let positive_terms: Vec<Term> = self.terms.iter()
            .filter(|term| term.exponent.unwrap_or(1).is_positive())
            .map(|term| term.clone())
            .collect();

        Unit::from(positive_terms)
    }

    fn denominator(&self) -> Option<Self::Denominator> {
        let negative_terms: Vec<Term> = self.terms.iter()
            .filter(|term| term.exponent.unwrap_or(1).is_negative())
            .map(|term| {
                let mut new_term = term.clone();
                // Flip the sign
                new_term.exponent = term.exponent.map(|e| -e);
                new_term
            })
            .collect();

        if negative_terms.is_empty() {
            None
        } else {
            Some(Unit::from(negative_terms))
        }
    }
}

#[cfg(test)]
mod tests {
    use as_fraction::AsFraction;
    use unit::Unit;
    use std::str::FromStr;

    lazy_static! {
        static ref METER: Unit = Unit::from_str("m").unwrap();
        static ref SECOND: Unit = Unit::from_str("s").unwrap();

        static ref GRAM_METER: Unit = Unit::from_str("g.m").unwrap();
        static ref METER_PER_SECOND: Unit = Unit::from_str("m/s").unwrap();

        static ref PER_SECOND: Unit = Unit::from_str("/s").unwrap();
        static ref PER_GRAM_METER: Unit = Unit::from_str("/g.m").unwrap();
    }

    #[test]
    fn validate_numerator_one_numerator_term() {
        let numerator = METER.numerator();
        assert_eq!(&numerator, &*METER);
    }

    #[test]
    fn validate_numerator_two_numerator_terms() {
        let numerator = GRAM_METER.numerator();
        assert_eq!(&numerator, &*GRAM_METER);
    }

    #[test]
    fn validate_numerator_one_numerator_term_one_denominator_term() {
        let numerator = METER_PER_SECOND.numerator();
        assert_eq!(&numerator, &*METER);
    }

    #[test]
    fn validate_numerator_one_denominator_term() {
        let numerator = PER_SECOND.numerator();
        let unity = Unit::new_unity();
        assert_eq!(&numerator, &unity);
    }

    #[test]
    fn validate_denominator_one_numerator_term() {
        let denominator = METER.denominator();
        assert!(denominator.is_none());
    }

    #[test]
    fn validate_denominator_two_numerator_terms() {
        let denominator = GRAM_METER.denominator();
        assert!(denominator.is_none());
    }

    #[test]
    fn validate_denominator_one_numerator_term_one_denominator_term() {
        let denominator = METER_PER_SECOND.denominator().unwrap();
        assert_eq!(&denominator, &*SECOND);
    }

    #[test]
    fn validate_denominator_one_denominator_term() {
        let denominator = PER_SECOND.denominator().unwrap();
        assert_eq!(&denominator, &*SECOND);
    }

    #[test]
    fn validate_denominator_two_denominator_terms() {
        let denominator = PER_GRAM_METER.denominator().unwrap();
        assert_eq!(&denominator, &*GRAM_METER);
    }
}