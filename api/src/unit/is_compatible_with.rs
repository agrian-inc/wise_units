use crate::is_compatible_with::IsCompatibleWith;
use crate::measurement::Measurement;
use crate::parser::Term;
use crate::unit::Unit;

impl<'a, 'b> IsCompatibleWith<&'b [Term]> for &'a Unit {
    fn is_compatible_with(self, rhs: &'b [Term]) -> bool {
        let lhs_terms: &'a [Term] = &*self;

        lhs_terms.is_compatible_with(rhs)
    }
}

#[cfg_attr(feature = "cffi", ffi_derive::expose_impl)]
impl<'a, 'b> IsCompatibleWith<&'b Unit> for &'a Unit {
    #[inline]
    fn is_compatible_with(self, rhs: &'b Unit) -> bool {
        let rhs_terms: &'b [Term] = &*rhs;

        self.is_compatible_with(rhs_terms)
    }
}

impl<'a, 'b> IsCompatibleWith<&'b Measurement> for &'a Unit {
    #[inline]
    fn is_compatible_with(self, rhs: &'b Measurement) -> bool {
        let rhs_terms: &'b [Term] = &*rhs.unit;

        (&*self).is_compatible_with(rhs_terms)
    }
}

#[cfg(test)]
mod tests {
    use crate::measurement::Measurement;
    use crate::unit::Unit;
    use crate::IsCompatibleWith;
    use std::str::FromStr;

    #[test]
    fn validate_is_compatible_with_unit() {
        let meter = Unit::from_str("m").unwrap();
        let km = Unit::from_str("km").unwrap();
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = Unit::from_str("km/10m").unwrap();
        assert!(!meter.is_compatible_with(&km_per_10m));

        let per_meter = Unit::from_str("m-1").unwrap();
        assert!(!meter.is_compatible_with(&per_meter));

        let ten_meter = Unit::from_str("10m").unwrap();
        assert!(meter.is_compatible_with(&ten_meter));

        let ten_km = Unit::from_str("10km").unwrap();
        assert!(meter.is_compatible_with(&ten_km));

        let per_ten_km = Unit::from_str("10km-1").unwrap();
        assert!(!meter.is_compatible_with(&per_ten_km));

        let per_meter_cubed = Unit::from_str("km-1/m2").unwrap();
        assert!(!meter.is_compatible_with(&per_meter_cubed));

        let km_per_length_cubed = Unit::from_str("km/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_cubed));

        let km_per_length_fourthed = Unit::from_str("km-1/m2.cm").unwrap();
        assert!(!meter.is_compatible_with(&km_per_length_fourthed));

        let meter_per_second_squared = Unit::from_str("m/s2").unwrap();
        assert!(!meter.is_compatible_with(&meter_per_second_squared));

        let km_cubed_per_nanometer_squared = Unit::from_str("km3/nm2").unwrap();
        assert!(meter.is_compatible_with(&km_cubed_per_nanometer_squared));
    }

    #[test]
    #[allow(clippy::blacklisted_name)]
    fn validate_is_compatible_with_units_with_annotations() {
        fn verify_compatible(lhs: &Unit, rhs: &Unit) {
            assert!(lhs.is_compatible_with(rhs));
        }
        fn verify_incompatible(lhs: &Unit, rhs: &Unit) {
            assert!(!lhs.is_compatible_with(rhs));
        }

        let foo = Unit::from_str("{foo}").unwrap();
        verify_compatible(&foo, &foo);

        let bar = Unit::from_str("{bar}").unwrap();
        verify_incompatible(&foo, &bar);

        let unity = Unit::from_str("1").unwrap();
        verify_incompatible(&foo, &unity);

        let mfoo = Unit::from_str("m{foo}").unwrap();
        let mfoo2_div_mfoo = Unit::from_str("m2{foo}/m{foo}").unwrap();
        verify_compatible(&mfoo, &mfoo2_div_mfoo);

        let mfoo2_div_mbar = Unit::from_str("m2{foo}/m{bar}").unwrap();
        verify_incompatible(&mfoo, &mfoo2_div_mbar);
    }

    #[test]
    fn validate_is_compatible_with_measurement() {
        let meter = Unit::from_str("m").unwrap();
        let km = Measurement::new(1.0, "km").unwrap();
        assert!(meter.is_compatible_with(&km));

        let km_per_10m = Measurement::new(1.0, "km/10m").unwrap();
        assert!(!meter.is_compatible_with(&km_per_10m));
    }
}
