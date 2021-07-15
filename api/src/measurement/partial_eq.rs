use crate::is_compatible_with::IsCompatibleWith;
use crate::measurement::Measurement;
use crate::ucum_unit::UcumUnit;
use approx::ulps_eq;

/// `Measurement`s are `PartialEq` if
///
/// a) their `Unit`s are compatible
/// b) their `scalar()` values are equal
///
/// ```rust
/// use wise_units::Measurement;
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "m").unwrap();
/// assert!(measurement == other);
///
/// let measurement = Measurement::new(1.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement != other);
///
/// let measurement = Measurement::new(1.0, "1000m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement == other);
///
/// let measurement = Measurement::new(1000.0, "m").unwrap();
/// let other = Measurement::new(1.0, "km").unwrap();
/// assert!(measurement == other);
/// ```
///
#[cfg_attr(feature = "cffi", ffi_derive::expose_impl)]
impl PartialEq for Measurement {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if !self.unit.is_compatible_with(&other.unit) {
            return false;
        }

        ulps_eq!(self.scalar(), other.scalar())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_eq_same_unit() {
        let m1 = Measurement::new(1.0, "m").unwrap();
        let m2 = Measurement::new(1.0, "m").unwrap();
        assert!(m1 == m2);

        let m2 = Measurement::new(1.1, "m").unwrap();
        assert!(m1 != m2);
    }

    #[test]
    fn validate_eq_unit_with_prefix() {
        let m = Measurement::new(1000.0, "m").unwrap();
        let km = Measurement::new(1.0, "km").unwrap();
        assert!(m == km);

        let km = Measurement::new(1.1, "km").unwrap();
        assert!(m != km);
    }

    #[test]
    fn validate_eq_unit_with_derived() {
        let m2 = Measurement::new(10_000.0, "m2").unwrap();
        let har = Measurement::new(1.0, "har").unwrap();
        assert!(m2 == har);

        let har = Measurement::new(1.1, "har").unwrap();
        assert!(m2 != har);
    }

    #[test]
    fn validate_eq_incompatible_unit() {
        let m = Measurement::new(1.0, "m").unwrap();
        let s = Measurement::new(1.0, "s").unwrap();
        assert!(m != s);
    }

    #[test]
    fn validate_eq_with_different_precision() {
        let m1 = Measurement::new(1.0, "[ft_i]").unwrap();
        let m2 = Measurement::new(12.0, "[in_i]").unwrap();
        let m3 = Measurement::new(12.0, "[ft_i].[in_i]").unwrap();
        assert!((&m1 * &m2) == m3);
        assert!((&m2 * &m1) == m3);
    }
}
