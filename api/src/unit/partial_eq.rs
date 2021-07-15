use crate::is_compatible_with::IsCompatibleWith;
use crate::ucum_unit::UcumUnit;
use crate::unit::Unit;
use approx::ulps_eq;

//-----------------------------------------------------------------------------
// impl PartialEq
//-----------------------------------------------------------------------------
/// `Unit`s are `PartialEq` if
///
/// a) they are compatible
/// b) their `scalar()` values are equal
///
/// ```rust
/// use std::str::FromStr;
/// use wise_units::Unit;
///
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("m").unwrap();
/// assert!(unit == other);
///
/// let unit = Unit::from_str("m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(unit != other);
///
/// let unit = Unit::from_str("1000m").unwrap();
/// let other = Unit::from_str("km").unwrap();
/// assert!(unit == other);
/// ```
///
#[cfg_attr(feature = "cffi", ffi_derive::expose_impl)]
impl PartialEq for Unit {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if !self.is_compatible_with(other) {
            return false;
        }

        ulps_eq!(self.scalar(), other.scalar())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn validate_eq_with_different_precision() {
        let u1 = Unit::from_str("100[ft_i]").unwrap();
        let u2 = Unit::from_str("[in_i]2/5har").unwrap();
        let u3 = Unit::from_str("100[ft_i].[in_i]2/5har").unwrap();
        assert!((&u1 * &u2) == u3);
        assert!((&u2 * &u1) == u3);
    }
}
