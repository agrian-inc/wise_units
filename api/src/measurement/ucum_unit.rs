use measurement::Measurement;
use num_rational::BigRational;
use reducible::Reducible;
use ucum_unit::UcumUnit;

impl UcumUnit for Measurement {
    /// Checks if the associated Unit is "special". "Special" units are ones
    /// that must be converted using a function in combination with some other
    /// non-special units. For example, Celsius is special since it must be
    /// first converted to Kelvin before converting to the requested unit.
    ///
    fn is_special(&self) -> bool {
        self.unit.is_special()
    }

    fn is_arbitrary(&self) -> bool {
        self.unit.is_arbitrary()
    }

    fn is_metric(&self) -> bool {
        self.unit.is_metric()
    }

    /// This scalar is the Measurement's value combined with any scalars that
    /// are part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::{Measurement, UcumUnit};
    ///
    /// let five_meters = Measurement::new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.scalar(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.scalar(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.scalar(), 15.707_963_267_948_966);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]").unwrap();
    /// assert!((sixty_five_f.scalar() - 291.483_333).abs() < 0.000_001);
    /// ```
    ///
    fn scalar(&self) -> BigRational {
        self.reduce_value(&self.value)
    }

    /// This magnitude is the Measurement's value combined with any magnitude
    /// that is part of the Unit's designation.
    ///
    /// # Examples
    ///
    /// ```
    /// use wise_units::{Measurement, UcumUnit};
    ///
    /// let five_meters = Measurement::new(5.0, "m").unwrap();
    /// assert_eq!(five_meters.magnitude(), 5.0);
    ///
    /// let five_meters_squared = Measurement::new(5.0, "m2").unwrap();
    /// assert_eq!(five_meters_squared.magnitude(), 5.0);
    ///
    /// let five_three_meters = Measurement::new(5.0, "[pi].m").unwrap();
    /// assert_eq!(five_three_meters.magnitude(), 5.0);
    ///
    /// let sixty_five_f = Measurement::new(65.0, "[degF]").unwrap();
    /// assert!((sixty_five_f.magnitude() - 65.0).abs() < 0.000_001);
    /// ```
    ///
    fn magnitude(&self) -> BigRational {
        self.calculate_magnitude(&self.value)
    }
}

#[cfg(test)]
mod tests {
    use num_help::BR_1;
    use measurement::Measurement;
    use ucum_unit::UcumUnit;

    #[test]
    fn validate_scalar() {
        let m = Measurement::new_try_unit(1, "m").unwrap();
        assert_eq!(m.scalar(), BR_1.clone());

        let m = Measurement::new_try_unit(big_rational_raw!(23, 10), "m").unwrap();
        assert_eq!(m.scalar(), big_rational_raw!(23, 10));

        let m = Measurement::new_try_unit(1, "km").unwrap();
        assert_eq!(m.scalar(), big_rational_raw!(1000));

        let m = Measurement::new_try_unit(big_rational_raw!(23, 10), "km").unwrap();
        assert_eq!(m.scalar(), big_rational_raw!(2300));

        let m = Measurement::new_try_unit(1, "g/L").unwrap();
        assert_eq!(m.scalar(), big_rational_raw!(1000));

        let m = Measurement::new_try_unit(big_rational_raw!(23, 10), "g/L").unwrap();
        assert_eq!(m.scalar(), big_rational_raw!(2300));

        let m = Measurement::new_try_unit(20, "Cel").unwrap();
        assert_eq!(m.scalar(), big_rational_raw!(29_315, 100));
    }

    #[test]
    fn validate_magnitude() {
        let m = Measurement::new_try_unit(1, "m").unwrap();
        assert_eq!(m.magnitude(), BR_1.clone());

        let m = Measurement::new_try_unit(big_rational_raw!(23, 10), "m").unwrap();
        assert_eq!(m.magnitude(), big_rational_raw!(23, 10));

        let m = Measurement::new_try_unit(1, "km").unwrap();
        assert_eq!(m.magnitude(), big_rational_raw!(1000));

        let m = Measurement::new_try_unit(big_rational_raw!(23, 10), "km").unwrap();
        assert_eq!(m.magnitude(), big_rational_raw!(2300));

        let m = Measurement::new_try_unit(1, "g/L").unwrap();
        assert_eq!(m.magnitude(), BR_1.clone());

        let m = Measurement::new_try_unit(big_rational_raw!(23, 10), "g/L").unwrap();
        assert_eq!(m.magnitude(), big_rational_raw!(23, 10));

        let m = Measurement::new_try_unit(20, "g/10L").unwrap();
        assert_eq!(m.magnitude(), big_rational_raw!(2));

        let m = Measurement::new_try_unit(20u8, "Cel").unwrap();
        assert_eq!(m.magnitude(), big_rational_raw!(20));
    }
}
