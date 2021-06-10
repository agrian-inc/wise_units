use crate::reducible::Reducible;
use crate::ucum_unit::UcumUnit;
use crate::unit::Unit;

#[cfg_attr(feature = "cffi", ffi_derive::expose_impl)]
impl UcumUnit for Unit {
    /// The UCUM defines "special units" as:
    ///
    /// > units that imply a measurement on a scale other than a ratio scale
    ///
    /// Each atom in `Atoms.toml` has the `isSpecial` attribute; a `Unit` is
    /// special if any of its `Term`s has an `Atom` that is special.
    ///
    fn is_special(&self) -> bool {
        self.terms.iter().any(UcumUnit::is_special)
    }

    /// The UCUM defines "metric units" using four points. First:
    ///
    /// > Only metric unit atoms may be combined with a prefix.
    ///
    /// Second:
    ///
    /// > To be metric or not to be metric is a predicate assigned to each unit
    /// atom where that unit > atom is defined.
    ///
    /// Third:
    ///
    /// > All base units are metric. No non-metric unit can be part of the
    /// basis.
    ///
    /// Fourth:
    ///
    /// > A unit must be a quantity on a ratio scale in order to be metric.
    ///
    /// This library laxes the first rule and allows any atom/unit to use
    /// `Prefix`es. Also this method only returns `true` when *all* of its
    /// `Term`s are metric.
    ///
    fn is_metric(&self) -> bool {
        self.terms.iter().all(UcumUnit::is_metric)
    }

    fn is_arbitrary(&self) -> bool {
        self.terms.iter().all(UcumUnit::is_arbitrary)
    }

    /// This gives the scalar value of `self` in terms of `self`'s
    /// base-unit(s). It takes account for each of `self`'s `Term`'s
    /// `factor` and `exponent`.
    ///
    /// ```rust
    /// use wise_units::{UcumUnit, Unit};
    /// use std::str::FromStr;
    ///
    /// // A "km" is 1000 meters.
    /// let unit = Unit::from_str("km").unwrap();
    /// assert_eq!(unit.scalar(), 1000.0);
    ///
    /// // A "10km" is 10_000 meters.
    /// let unit = Unit::from_str("10km").unwrap();
    /// assert_eq!(unit.scalar(), 10_000.0);
    ///
    /// // A "km-1" is 0.001 meters.
    /// let unit = Unit::from_str("km-1").unwrap();
    /// assert_eq!(unit.scalar(), 0.001);
    ///
    /// // A "10km-1" is 0.000_1 meters.
    /// let unit = Unit::from_str("10km-1").unwrap();
    /// assert_eq!(unit.scalar(), 0.000_1);
    ///
    /// // A "km/h" is 0.2777777777777778 meters/second.
    /// let unit = Unit::from_str("km/h").unwrap();
    /// assert_eq!(unit.scalar(), 0.277_777_777_777_777_8);
    ///
    #[inline]
    fn scalar(&self) -> f64 {
        self.reduce_value(1.0)
    }

    /// The scalar value of `self` in terms of `self`'s actual unit(s).
    ///
    /// ```rust
    /// use wise_units::{UcumUnit, Unit};
    /// use std::str::FromStr;
    ///
    /// // A "km" is 1000 meters.
    /// let unit = Unit::from_str("km").unwrap();
    /// assert_eq!(unit.magnitude(), 1000.0);
    ///
    /// // A "10km" is 10_000 meters.
    /// let unit = Unit::from_str("10km").unwrap();
    /// assert_eq!(unit.magnitude(), 10_000.0);
    ///
    /// // A "km-1" is 0.001 meters.
    /// let unit = Unit::from_str("km-1").unwrap();
    /// assert_eq!(unit.magnitude(), 0.001);
    ///
    /// // A "10km-1" is 0.000_1 meters.
    /// let unit = Unit::from_str("10km-1").unwrap();
    /// assert_eq!(unit.magnitude(), 0.000_1);
    ///
    /// // A "km/h" is 1000 meters/hour.
    /// let unit = Unit::from_str("km/h").unwrap();
    /// assert_eq!(unit.magnitude(), 1000.0);
    ///
    /// // A "m3" is 1 cubic meters.
    /// let unit = Unit::from_str("m3").unwrap();
    /// assert_eq!(unit.magnitude(), 1.0);
    ///
    /// // A "L" is 1 liter.
    /// let unit = Unit::from_str("L").unwrap();
    /// assert_eq!(unit.magnitude(), 1.0);
    ///
    /// // A "10m/5s" is 2 "meters per second".
    /// let unit = Unit::from_str("10m/5s").unwrap();
    /// assert_eq!(unit.magnitude(), 2.0);
    ///
    /// // A "10m/5s2" is 0.4 "meters per second".
    /// let unit = Unit::from_str("10m/5s2").unwrap();
    /// assert_eq!(unit.magnitude(), 0.4);
    ///
    #[inline]
    fn magnitude(&self) -> f64 {
        self.calculate_magnitude(self.scalar())
    }
}

#[cfg(test)]
mod tests {
    use crate::ucum_unit::UcumUnit;
    use crate::unit::Unit;
    use approx::{assert_relative_eq, assert_ulps_eq};
    use std::str::FromStr;

    macro_rules! validate_scalar {
        ($test_name:ident, $input_string:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                assert_relative_eq!(unit.scalar(), $expected_value);
                assert_ulps_eq!(unit.scalar(), $expected_value);
            }
        };
    }

    macro_rules! validate_scalars {
        ($($test_name: ident, $input_string: expr, $expected_value: expr);+ $(;)*) => {
            $(
                validate_scalar!($test_name, $input_string, $expected_value);
            )+
        };
    }

    macro_rules! validate_magnitude {
        ($test_name:ident, $input_string:expr, $expected_value:expr) => {
            #[test]
            fn $test_name() {
                let unit = Unit::from_str($input_string).unwrap();
                assert_relative_eq!(unit.magnitude(), $expected_value);
                assert_ulps_eq!(unit.magnitude(), $expected_value);
            }
        };
    }

    macro_rules! validate_magnitudes {
        ($($test_name: ident, $input_string: expr, $expected_value: expr);+ $(;)*) => {
            $(
                validate_magnitude!($test_name, $input_string, $expected_value);
            )+
        };
    }

    #[test]
    fn validate_is_special() {
        let unit = Unit::from_str("m").unwrap();
        assert!(!unit.is_special());
    }

    validate_scalars!(
        validate_scalar_m, "m", 1.0;
        validate_scalar_km, "km", 1000.0;
        validate_scalar_m_minus_1, "m-1", 1.0;
        validate_scalar_10m, "10m", 10.0;
        validate_scalar_10km, "10km", 10_000.0;
        validate_scalar_10km_minus_1, "10km-1", 0.000_1;
        validate_scalar_10km_minus_1_m2, "10km-1.m2", 0.000_1;
        validate_scalar_km_slash_m2_dot_cm, "km/m2.cm", 100_000.0;
        validate_scalar_km_minus_1_slash_m2_dot_cm, "km-1/m2.cm", 0.1;
        validate_scalar_m_slash_s2, "m/s2", 1.0;
        validate_scalar_slash_1, "/1", 1.0;
        validate_scalar_slash_m, "/m", 1.0;
        validate_scalar_slash_annotation, "/{tot}", 1.0;

        validate_scalar_liter, "l", 0.001;
        validate_scalar_liter_caps, "L", 0.001;
        validate_scalar_pi, "[pi]", ::std::f64::consts::PI;
        validate_scalar_ten_pi, "10[pi]", 10.0 * ::std::f64::consts::PI;
        validate_scalar_hectare, "har", 10_000.0;
        validate_scalar_week, "wk", 604_800.0;
        validate_scalar_kilogram, "kg", 1000.0;
        validate_scalar_fahrenheit, "[degF]", 255.927_777_777_777_8;
    );

    validate_magnitudes!(
        validate_magnitude_m, "m", 1.0;
        validate_magnitude_km, "km", 1000.0;
        validate_magnitude_m_minus_1, "m-1", 1.0;
        validate_magnitude_10m, "10m", 10.0;
        validate_magnitude_10km, "10km", 10_000.0;
        validate_magnitude_10km_minus_1, "10km-1", 0.000_1;
        validate_magnitude_10km_minus_1_m2, "10km-1.m2", 0.000_1;
        validate_magnitude_km_slash_m2_dot_cm, "km/m2.cm", 100_000.0;
        validate_magnitude_km_minus_1_slash_m2_dot_cm, "km-1/m2.cm", 0.1;
        validate_magnitude_m_slash_s2, "m/s2", 1.0;
        validate_magnitude_slash_1, "/1", 1.0;
        validate_magnitude_slash_m, "/m", 1.0;
        validate_magnitude_slash_annotation, "/{tot}", 1.0;

        validate_magnitude_m2, "m2", 1.0;
        validate_magnitude_m3, "m3", 1.0;
        validate_magnitude_liter, "l", 1.0;
        validate_magnitude_liter_caps, "L", 1.0;
        validate_magnitude_8m_slash_4s, "8m/4s", 2.0;
        validate_magnitude_8m_slash_4s2, "8m/4s2", 0.5;

        validate_magnitude_pi, "[pi]", 1.0;
        validate_magnitude_ten_pi, "10[pi]", 10.0;

        // TODO: This doesn't parse (AGDEV-33099)
        // validate_magnitude_decare, "dar", 0.1;
        validate_magnitude_dekare, "daar", 10.0;
        validate_magnitude_hectare, "har", 100.0;
        validate_magnitude_kilare, "kar", 1000.0;

        validate_magnitude_week, "wk", 1.0;
        validate_magnitude_kilogram, "kg", 1000.0;
        validate_magnitude_fahrenheit, "[degF]", 1.000_000_000_000_056_8;
    );
}
