mod as_fraction;
mod composable;
mod deref;
mod display;
mod field_eq;
mod from;
mod from_str;
mod invert;
mod is_compatible_with;
mod ops;
mod partial_eq;
mod partial_ord;
mod reducible;
mod term_reducing;
mod to_reduced;

#[allow(clippy::module_name_repetitions)]
mod ucum_unit;

#[cfg(feature = "cffi")]
pub mod custom_ffi;
#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "cffi")]
use ffi_derive::FFI;

use crate::parser::Term;

#[cfg_attr(
    feature = "cffi",
     derive(FFI), 
     ffi(custom = "src/unit/custom_ffi.rs", 
     failable_init,
     failable_fns(custom_ffi::get_unit_expression, custom_ffi::get_unit_expression)))]
#[derive(Clone, Debug)]
pub struct Unit {
    pub terms: Vec<Term>,
}

/// A `Unit` is the piece of data that represents a *valid* UCUM unit or
/// custom-defined unit. A `Unit` is defined as a number of `Term`s and thus
/// all methods defined on `Unit` rely on values from its `Terms`.
///
/// The easiest way to create a new `Unit` is via its implementation of
/// `std::str::FromStr`. This parses the given `&str` and returns a
/// `wise_units::error::Error` if it fails to parse:
///
/// ```rust
/// use std::str::FromStr;
/// use wise_units::{UcumUnit, Unit};
///
/// let m = Unit::from_str("m2").unwrap();
/// assert_eq!(m.scalar(), 1.0);
///
/// let bad_unit = Unit::from_str("not_a_unit");
/// assert!(bad_unit.is_err());
/// ```
///
impl Unit {
    /// Creates a new `Unit` that's equivalent to "1".
    ///
    #[must_use]
    pub fn new_unity() -> Self {
        let unity_term = Term::new_unity();

        Self {
            terms: vec![unity_term],
        }
    }

    /// A `Unit` is a unity `Unit` if represents "1", which technically means
    /// here:
    ///
    /// * it has 1 `Term`...
    ///     * whose `factor` is 1
    ///     * has no `exponent`
    ///     * has no `Atom`
    ///     * has no `Prefix`
    ///
    #[must_use]
    pub fn is_unity(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].is_unity()
    }

    /// Turns the Unit's Terms into Strings and combines them accordingly.
    /// This always returns a String that is parsable back into the same Unit.
    ///
    /// Ex. terms that would normally render `[acr_us].[in_i]/[acr_us]` would
    /// render the same result.
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use wise_units::Unit;
    ///
    /// let u = Unit::from_str("[acr_us].[in_i]/[acr_us]").unwrap();
    /// assert_eq!(u.expression().as_str(), "[acr_us].[in_i]/[acr_us]");
    /// ```
    ///
    #[inline]
    #[must_use]
    pub fn expression(&self) -> String {
        self.to_string()
    }

    /// If the unit terms are a fraction and can be reduced, this returns those
    /// as a string. Ex. terms that would normally render
    /// `[acr_us].[in_i]/[acr_us]` would simply render `[in_i]`.
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use wise_units::Unit;
    ///
    /// let u = Unit::from_str("[acr_us].[in_i]/[acr_us]").unwrap();
    /// assert_eq!(u.expression_reduced().as_str(), "[in_i]");
    /// ```
    ///
    #[inline]
    #[must_use]
    pub fn expression_reduced(&self) -> String {
        let reduced = term_reducing::reduce_terms(&self.terms);

        Self::from(reduced).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn validate_is_unity() {
        let unit = Unit::new_unity();
        assert!(unit.is_unity());

        let unit: Unit = Vec::new().into();
        assert!(!unit.is_unity());

        let unit = Unit::from_str("1").unwrap();
        assert!(unit.is_unity());

        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_unity());

        let unit = Unit::from_str("m/m").unwrap();
        assert!(!unit.is_unity());
    }

    #[test]
    fn validate_expression_reduced() {
        let unit = Unit::from_str("m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("M").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m");

        let unit = Unit::from_str("km/10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/10m");

        let unit = Unit::from_str("m-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/m");

        let unit = Unit::from_str("10m").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10m");

        let unit = Unit::from_str("10km").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "10km");

        let unit = Unit::from_str("10km-1").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/10km");

        let unit = Unit::from_str("km-1/m2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/m2.km");

        let unit = Unit::from_str("km/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km/m2.cm");

        let unit = Unit::from_str("km-1/m2.cm").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "/m2.cm.km");

        let unit = Unit::from_str("m/s2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "m/s2");

        let unit = Unit::from_str("km3/nm2").unwrap();
        assert_eq!(unit.expression_reduced().as_str(), "km3/nm2");
    }

    #[cfg(feature = "cffi")]
    mod cffi {
        use super::*;

        #[test]
        fn test_custom_and_derived_ffi() {
            let expression = "kg/[lb_av]";
            let unit = custom_ffi::unit_init(ffi_common::ffi_string!(expression));
            let c_expression = custom_ffi::get_unit_expression(unit);
            assert_eq!(expression, ffi_common::string::string_from_c(c_expression));
            unsafe { unit_ffi::unit_free(unit) };
        }
    }
}
