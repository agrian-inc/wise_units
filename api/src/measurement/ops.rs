use crate::convertible::Convertible;
use crate::error::Error;
use crate::measurement::Measurement;
use std::ops::{Add, Div, Mul, Sub};

//-----------------------------------------------------------------------------
// impl Add
//-----------------------------------------------------------------------------
#[cfg_attr(feature = "cffi",ffi_derive::expose_fn)]
fn add_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value + rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Add for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        add_measurements(&self, &other)
    }
}

impl<'a> Add<&'a Measurement> for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn add(self, other: &'a Self) -> Self::Output {
        add_measurements(&self, other)
    }
}

impl<'a> Add for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn add(self, other: &'a Measurement) -> Self::Output {
        add_measurements(self, other)
    }
}

impl<'a> Add<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn add(self, other: Measurement) -> Self::Output {
        add_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Sub
//-----------------------------------------------------------------------------
#[cfg_attr(feature = "cffi",ffi_derive::expose_fn)]
fn sub_measurements(lhs: &Measurement, rhs: &Measurement) -> Result<Measurement, Error> {
    let rhs_converted = rhs.convert_to(&lhs.unit)?;
    let new_value = lhs.value - rhs_converted.value;

    Ok(Measurement {
        value: new_value,
        unit: lhs.unit.clone(),
    })
}

impl Sub for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        sub_measurements(&self, &other)
    }
}

impl<'a> Sub<&'a Measurement> for Measurement {
    type Output = Result<Self, Error>;

    #[inline]
    fn sub(self, other: &'a Self) -> Self::Output {
        sub_measurements(&self, other)
    }
}

impl<'a> Sub for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn sub(self, other: &'a Measurement) -> Self::Output {
        sub_measurements(self, other)
    }
}

impl<'a> Sub<Measurement> for &'a Measurement {
    type Output = Result<Measurement, Error>;

    #[inline]
    fn sub(self, other: Measurement) -> Self::Output {
        sub_measurements(self, &other)
    }
}

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
#[cfg_attr(feature = "cffi",ffi_derive::expose_fn)]
fn mul_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let converted_rhs = rhs.convert_to(&lhs.unit);
    let actual_rhs = converted_rhs.as_ref().unwrap_or(rhs);
    let new_value = lhs.value * actual_rhs.value;
    let new_unit = &lhs.unit * &actual_rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

impl Mul for Measurement {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        mul_measurements(&self, &other)
    }
}

impl<'a> Mul<&'a Measurement> for Measurement {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        mul_measurements(&self, other)
    }
}

impl<'a> Mul for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn mul(self, other: &'a Measurement) -> Self::Output {
        mul_measurements(self, other)
    }
}

impl<'a> Mul<Measurement> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn mul(self, other: Measurement) -> Self::Output {
        mul_measurements(self, &other)
    }
}

/// Multiplies the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl Mul<f64> for Measurement {
    type Output = Self;

    #[inline]
    fn mul(self, other: f64) -> Self::Output {
        let new_value = self.value * other;

        Self {
            value: new_value,
            unit: self.unit,
        }
    }
}

impl<'a> Mul<f64> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn mul(self, other: f64) -> Self::Output {
        let new_value = self.value * other;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
#[cfg_attr(feature = "cffi",ffi_derive::expose_fn)]
fn div_measurements(lhs: &Measurement, rhs: &Measurement) -> Measurement {
    let converted_rhs = rhs.convert_to(&lhs.unit);
    let actual_rhs = converted_rhs.as_ref().unwrap_or(rhs);
    let new_value = lhs.value / actual_rhs.value;
    let new_unit = &lhs.unit / &actual_rhs.unit;

    Measurement {
        value: new_value,
        unit: new_unit,
    }
}

impl Div for Measurement {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        div_measurements(&self, &other)
    }
}

impl<'a> Div for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn div(self, other: &'a Measurement) -> Self::Output {
        div_measurements(self, other)
    }
}

impl<'a> Div<&'a Measurement> for Measurement {
    type Output = Self;

    #[inline]
    fn div(self, other: &'a Self) -> Self::Output {
        div_measurements(&self, other)
    }
}

impl<'a> Div<Measurement> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn div(self, other: Measurement) -> Self::Output {
        div_measurements(self, &other)
    }
}

/// Divides the `Measurement`'s scalar by `other` and returns a new
/// `Measurement`.
///
impl Div<f64> for Measurement {
    type Output = Self;

    #[inline]
    fn div(self, other: f64) -> Self::Output {
        let new_value = self.value / other;

        Self {
            value: new_value,
            unit: self.unit,
        }
    }
}

impl<'a> Div<f64> for &'a Measurement {
    type Output = Measurement;

    #[inline]
    fn div(self, other: f64) -> Self::Output {
        let new_value = self.value / other;

        Measurement {
            value: new_value,
            unit: self.unit.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::measurement::Measurement;

    macro_rules! validate_op {
        ($result:expr, $expected:expr) => {
            let expected = $expected.unwrap();
            assert_eq!(
                $result,
                expected,
                "expected Measurements to be equal;\nresult: {:?};\nunit string: {}",
                $result,
                $result.unit.to_string()
            );
        };
    }

    macro_rules! validate_ownership_and_borrowing {
        ($lhs:expr, $method:ident, $rhs:expr, $expected:expr) => {
            fn validate_borrowed_borrowed(lhs: &Measurement, rhs: &Measurement) {
                let result = lhs.$method(rhs);
                validate_op!(result, $expected);
            }
            fn validate_borrowed_owned(lhs: &Measurement, rhs: Measurement) {
                let result = lhs.$method(rhs);
                validate_op!(result, $expected);
            }
            fn validate_owned_borrowed(lhs: Measurement, rhs: &Measurement) {
                let result = lhs.$method(rhs);
                validate_op!(result, $expected);
            }
            fn validate_owned_owned(lhs: Measurement, rhs: Measurement) {
                let result = lhs.$method(rhs);
                validate_op!(result, $expected);
            }

            validate_borrowed_borrowed(&$lhs, &$rhs);
            validate_borrowed_owned(&$lhs, $rhs.clone());
            validate_owned_borrowed($lhs.clone(), &$rhs);
            validate_owned_owned($lhs, $rhs);
        };
    }

    mod add {
        use super::*;

        #[test]
        fn validate_add_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((m1 + m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((&m1 + &m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_owned_and_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((m1 + &m2).unwrap(), expected);
        }

        #[test]
        fn validate_add_borrowed_and_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(3.0, "m").unwrap();

            assert_eq!((&m1 + m2).unwrap(), expected);
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn validate_sub_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((m1 - m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((&m1 - &m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_owned_and_borrowed() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((m1 - &m2).unwrap(), expected);
        }

        #[test]
        fn validate_sub_borrowed_and_owned() {
            let m1 = Measurement::new(1.0, "m").unwrap();
            let m2 = Measurement::new(2.0, "m").unwrap();
            let expected = Measurement::new(-1.0, "m").unwrap();

            assert_eq!((&m1 - m2).unwrap(), expected);
        }
    }

    mod mul {
        use super::*;
        use std::ops::Mul;

        #[test]
        fn validate_ownership_and_borrowing() {
            validate_ownership_and_borrowing!(
                Measurement::new(2.0, "m").unwrap(),
                mul,
                Measurement::new(3.0, "m").unwrap(),
                Measurement::new(6.0, "m2")
            );
        }

        #[test]
        fn validate_mul_reductions() {
            validate_op!(
                Measurement::new(2.0, "1").unwrap() * Measurement::new(3.0, "1").unwrap(),
                Measurement::new(6.0, "1")
            );

            validate_op!(
                Measurement::new(2.0, "m/s").unwrap() * Measurement::new(3.0, "s/m").unwrap(),
                Measurement::new(6.0, "1")
            );

            validate_op!(
                Measurement::new(2.0, "m/m/m/m").unwrap()
                    * Measurement::new(3.0, "m/m/m/m").unwrap(),
                Measurement::new(6.0, "1")
            );

            validate_op!(
                Measurement::new(2.0, "m.m").unwrap() * Measurement::new(3.0, "/m2").unwrap(),
                Measurement::new(6.0, "1")
            );

            validate_op!(
                Measurement::new(2.0, "kg").unwrap() * Measurement::new(3000.0, "g").unwrap(),
                Measurement::new(6.0, "kg2")
            );

            validate_op!(
                Measurement::new(3.0, "g").unwrap() * Measurement::new(0.002, "kg").unwrap(),
                Measurement::new(6.0, "g2")
            );

            validate_op!(
                Measurement::new(2.0, "1").unwrap() * Measurement::new(3.0, "m").unwrap(),
                Measurement::new(6.0, "m")
            );

            validate_op!(
                Measurement::new(2.0, "m").unwrap() * Measurement::new(3.0, "s").unwrap(),
                Measurement::new(6.0, "m.s")
            );

            validate_op!(
                Measurement::new(2.0, "m.m").unwrap() * Measurement::new(3.0, "m2").unwrap(),
                Measurement::new(6.0, "m4")
            );

            validate_op!(
                Measurement::new(2.0, "m.m").unwrap() * Measurement::new(3.0, "s2").unwrap(),
                Measurement::new(6.0, "m2.s2")
            );

            validate_op!(
                Measurement::new(2.0, "m").unwrap() * Measurement::new(3.0, "s/m").unwrap(),
                Measurement::new(6.0, "s")
            );

            validate_op!(
                Measurement::new(2.0, "10m2").unwrap() * Measurement::new(3.0, "10m").unwrap(),
                Measurement::new(6.0, "10m3")
            );

            validate_op!(
                Measurement::new(2.0, "10m2").unwrap() * Measurement::new(3.0, "5m").unwrap(),
                Measurement::new(6.0, "10m2.5m")
            );
        }

        #[test]
        fn validate_mul_f64() {
            let m = Measurement::new(10.0, "m").unwrap();
            let expected = Measurement::new(200.0, "m").unwrap();

            assert_eq!(m.mul(20.0), expected);
        }
    }

    mod div {
        use super::*;
        use std::ops::Div;

        #[test]
        fn validate_ownership_and_borrowing() {
            validate_ownership_and_borrowing!(
                Measurement::new(10.0, "m").unwrap(),
                div,
                Measurement::new(2.0, "m").unwrap(),
                Measurement::new(5.0, "1")
            );
        }

        #[test]
        fn validate_div_reductions() {
            validate_op!(
                Measurement::new(10.0, "1").unwrap() / Measurement::new(2.0, "1").unwrap(),
                Measurement::new(5.0, "1")
            );
            validate_op!(
                Measurement::new(10.0, "m.m").unwrap() / Measurement::new(2.0, "m2").unwrap(),
                Measurement::new(5.0, "1")
            );
            validate_op!(
                Measurement::new(10.0, "kg").unwrap() / Measurement::new(2000.0, "g").unwrap(),
                Measurement::new(5.0, "1")
            );
            validate_op!(
                Measurement::new(10.0, "m.m.m.m").unwrap()
                    / Measurement::new(2.0, "m.m.m.m").unwrap(),
                Measurement::new(5.0, "1")
            );

            validate_op!(
                Measurement::new(10.0, "m2").unwrap() / Measurement::new(2.0, "m").unwrap(),
                Measurement::new(5.0, "m")
            );
            validate_op!(
                Measurement::new(10.0, "m").unwrap() / Measurement::new(2.0, "s").unwrap(),
                Measurement::new(5.0, "m/s")
            );
            validate_op!(
                Measurement::new(10.0, "m2/s").unwrap() / Measurement::new(2.0, "m").unwrap(),
                Measurement::new(5.0, "m/s")
            );

            validate_op!(
                Measurement::new(10.0, "1").unwrap() / Measurement::new(2.0, "m").unwrap(),
                Measurement::new(5.0, "/m")
            );
            validate_op!(
                Measurement::new(10.0, "m").unwrap() / Measurement::new(2.0, "/m").unwrap(),
                Measurement::new(5.0, "m2")
            );

            validate_op!(
                Measurement::new(10.0, "10m2").unwrap() / Measurement::new(2.0, "10m").unwrap(),
                Measurement::new(5.0, "10m")
            );
            validate_op!(
                Measurement::new(10.0, "10m2").unwrap() / Measurement::new(2.0, "5m").unwrap(),
                Measurement::new(5.0, "10m2/5m")
            );
        }

        #[test]
        fn validate_div_f64() {
            let m = Measurement::new(10.0, "m").unwrap();
            let expected = Measurement::new(2.0, "m").unwrap();

            assert_eq!(m.div(5.0), expected);
        }
    }
}
