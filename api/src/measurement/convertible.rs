use crate::{Convertible, Error, FieldEq, IsCompatibleWith, Measurement, Unit};
use std::str::FromStr;

/// This implementation of `Convertible` lets you pass in a `&str` for the
/// `Unit`, which will parse the chars and convert accordingly. If `expression`
/// is invalid, you'll get an `Error`. If `self`'s `Unit` and `other_unit` are
/// incompatible, you'll get an `Error`.
///
#[cfg_attr(feature = "cffi", ffi_derive::expose_impl)]
impl<'a> Convertible<&'a str> for Measurement {
    type Output = Self;
    type ConversionError = Error;

    #[inline]
    fn convert_to(&self, expression: &'a str) -> Result<Self, Self::ConversionError> {
        let other_unit = Unit::from_str(expression)?;

        convert_measurement(self, &other_unit)
    }
}

pub unsafe extern "C" fn convertible_measurement_ffi_convert_to2(
    measurement: *const Measurement,
    expression: *const std::os::raw::c_char,
) -> *const Measurement {
    let data = (*measurement).clone();
    let expression: String = ffi_common::string::string_from_c(expression);
    let return_value = data.convert_to(&*expression);
    match return_value.map(|r| Box::into_raw(Box::new(r.clone()))) {
        Ok(val) => val,
        Err(error) => {
            ::ffi_common::error::set_last_err_msg(error.to_string().as_str());
            std::ptr::null()
        }
    }
}

/// This implementation of `Convertible` skips any string parsing and gets
/// right to converting to `other_unit`. If `self`'s `Unit` and `other_unit`
/// are incompatible, you'll get an `Error`.
///
impl<'a> Convertible<&'a Unit> for Measurement {
    type Output = Self;
    type ConversionError = Error;

    #[inline]
    fn convert_to(&self, other_unit: &'a Unit) -> Result<Self, Self::ConversionError> {
        convert_measurement(self, other_unit)
    }
}

fn convert_measurement(lhs: &Measurement, dest_unit: &Unit) -> Result<Measurement, Error> {
    // Short-circuit if `dest_unit` is the same as the Measurement's Unit.
    if lhs.unit.field_eq(dest_unit) {
        return Ok(lhs.clone());
    }

    let source_unit = &lhs.unit;

    if !source_unit.is_compatible_with(dest_unit) {
        let e = Error::IncompatibleUnitTypes {
            lhs: source_unit.expression(),
            rhs: dest_unit.expression(),
        };
        return Err(e);
    }

    let new_measurement = Measurement {
        value: lhs.converted_scalar(dest_unit),
        unit: dest_unit.clone(),
    };

    Ok(new_measurement)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, assert_ulps_eq};

    #[test]
    fn validate_convert_to_meter_to_meter_str() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let converted = meter.convert_to("m").unwrap();
        assert_eq!(converted, meter);
        assert_relative_eq!(converted.value, 1.0);
        assert_ulps_eq!(converted.value, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2meter_str() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let converted = meter.convert_to("2m").unwrap();
        assert_relative_eq!(converted.value, 0.5);
        assert_ulps_eq!(converted.value, 0.5);
    }

    #[test]
    fn validate_convert_to_meter_to_km_str() {
        let meter = Measurement::new(1000.0, "m").unwrap();
        let converted = meter.convert_to("km").unwrap();
        assert_relative_eq!(converted.value, 1.0);
        assert_ulps_eq!(converted.value, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2km_str() {
        let meter = Measurement::new(1000.0, "m").unwrap();
        let converted = meter.convert_to("2km").unwrap();
        assert_relative_eq!(converted.value, 0.5);
        assert_ulps_eq!(converted.value, 0.5);
    }

    #[test]
    fn validate_convert_to_meter_to_meter_unit() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let unit = Unit::from_str("m").unwrap();
        let converted = meter.convert_to(&unit).unwrap();
        assert_eq!(converted, meter);
        assert_relative_eq!(converted.value, 1.0);
        assert_ulps_eq!(converted.value, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2meter_unit() {
        let meter = Measurement::new(1.0, "m").unwrap();
        let unit = Unit::from_str("2m").unwrap();
        let converted = meter.convert_to(&unit).unwrap();
        assert_relative_eq!(converted.value, 0.5);
        assert_ulps_eq!(converted.value, 0.5);
    }

    #[test]
    fn validate_convert_to_meter_to_km_unit() {
        let meter = Measurement::new(1000.0, "m").unwrap();
        let unit = Unit::from_str("km").unwrap();
        let converted = meter.convert_to(&unit).unwrap();
        assert_relative_eq!(converted.value, 1.0);
        assert_ulps_eq!(converted.value, 1.0);
    }

    #[test]
    fn validate_convert_to_meter_to_2km_unit() {
        let meter = Measurement::new(1000.0, "m").unwrap();
        let unit = Unit::from_str("2km").unwrap();
        let converted = meter.convert_to(&unit).unwrap();
        assert_relative_eq!(converted.value, 0.5);
        assert_ulps_eq!(converted.value, 0.5);
    }
}
