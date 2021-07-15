use crate::is_compatible_with::IsCompatibleWith;
use crate::measurement::Measurement;
use crate::parser::Term;
use crate::unit::Unit;

impl<'a, 'b> IsCompatibleWith<&'b Measurement> for &'a Measurement {
    #[inline]
    fn is_compatible_with(self, rhs: &'b Measurement) -> bool {
        self.is_compatible_with(&rhs.unit)
    }
}

#[cfg_attr(feature = "cffi", ffi_derive::expose_impl)]
impl<'a, 'b> IsCompatibleWith<&'b Unit> for &'a Measurement {
    #[inline]
    fn is_compatible_with(self, rhs: &'b Unit) -> bool {
        let self_terms: &'a [Term] = &*self.unit;
        let rhs_terms: &'b [Term] = &*rhs;

        self_terms.is_compatible_with(rhs_terms)
    }
}

#[cfg(test)]
mod tests {
    use crate::is_compatible_with::IsCompatibleWith;
    use crate::measurement::Measurement;
    use crate::unit::Unit;
    use std::str::FromStr;

    #[test]
    fn validate_is_compatible_with_measurement() {
        let subject = Measurement::new(1.0, "m").unwrap();
        let other = Measurement::new(1.0, "km").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Measurement::new(1.0, "kg").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let other = Measurement::new(1.0, "m{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let subject = Measurement::new(1.0, "m{foo}").unwrap();
        let other = Measurement::new(1.0, "km{foo}").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Measurement::new(1.0, "g{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));
    }

    #[test]
    fn validate_is_compatible_with_unit() {
        let subject = Measurement::new(1.0, "m").unwrap();
        let other = Unit::from_str("km").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Unit::from_str("kg").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let other = Unit::from_str("m{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));

        let subject = Measurement::new(1.0, "m{foo}").unwrap();
        let other = Unit::from_str("km{foo}").unwrap();
        assert!(subject.is_compatible_with(&other));

        let other = Unit::from_str("g{foo}").unwrap();
        assert!(!subject.is_compatible_with(&other));
    }
}
