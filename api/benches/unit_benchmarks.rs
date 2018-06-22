#[macro_use]
extern crate criterion;
extern crate wise_units;

mod common;

use criterion::Criterion;
use std::str::FromStr;
use wise_units::{Composable, Unit};

macro_rules! bench_over_inputs_method {
    ($function_name:ident, $test_name:expr, $method_name:ident) => {
        fn $function_name(c: &mut Criterion) {
            c.bench_function_over_inputs($test_name, |b, &unit_string| {
                let unit = Unit::from_str(unit_string).unwrap();

                b.iter(|| unit.$method_name());
            }, &common::UNIT_STRINGS);
        }
    };
}

macro_rules! bench_over_inputs_math {
    ($function_name:ident, $test_name:expr, $method_name:tt) => {
        fn $function_name(c: &mut Criterion) {
            c.bench_function_over_inputs($test_name, |b, &(lhs_string, rhs_string)| {
                let lhs = &Unit::from_str(lhs_string).unwrap();
                let rhs = &Unit::from_str(rhs_string).unwrap();

                b.iter(|| lhs $method_name rhs);
            }, &common::UNIT_PAIRS);
        }
    };
}

bench_over_inputs_method!(is_special_group, "Unit::is_special()", is_special);
bench_over_inputs_method!(is_metric_group, "Unit::is_metric()", is_metric);
bench_over_inputs_method!(is_unity_group, "Unit::is_unity()", is_unity);

bench_over_inputs_method!(scalar_group, "Unit::scalar()", scalar);
bench_over_inputs_method!(magnitude_group, "Unit::magnitude()", magnitude);
bench_over_inputs_method!(expression_group, "Unit::expression()", expression);
bench_over_inputs_method!(expression_reduced_group, "Unit::expression_reduced()", expression_reduced);

//-----------------------------------------------------------------------------
// impl Composable
//-----------------------------------------------------------------------------
bench_over_inputs_method!(composition_group, "Unit::composition()", composition);

fn is_compatible_with_group(c: &mut Criterion) {
    c.bench_function_over_inputs("Unit::is_compatible_with", |b, &(lhs_string, rhs_string)| {
        let lhs = &Unit::from_str(lhs_string).unwrap();
        let rhs = &Unit::from_str(rhs_string).unwrap();

        b.iter(|| lhs.is_compatible_with(rhs));
    }, &common::UNIT_PAIRS);
}

//-----------------------------------------------------------------------------
// impl Display
//-----------------------------------------------------------------------------
bench_over_inputs_method!(display_group, "Unit::to_string()", to_string);

//-----------------------------------------------------------------------------
// impl FromStr
//-----------------------------------------------------------------------------
fn from_str_group(c: &mut Criterion) {
    c.bench_function_over_inputs("Unit::from_str", |b, &unit_string| {
        b.iter(|| Unit::from_str(unit_string));
    }, &common::UNIT_STRINGS);
}

//-----------------------------------------------------------------------------
// impl PartialEq
//-----------------------------------------------------------------------------
bench_over_inputs_math!(partial_eq_group, "Unit::partial_eq", ==);

//-----------------------------------------------------------------------------
// impl PartialOrd
//-----------------------------------------------------------------------------
bench_over_inputs_math!(partial_ord_gt_group, "Unit::partial_ord(>)", >);

//-----------------------------------------------------------------------------
// impl Mul
//-----------------------------------------------------------------------------
bench_over_inputs_math!(mul_group, "Unit::mul", *);

//-----------------------------------------------------------------------------
// impl Div
//-----------------------------------------------------------------------------
bench_over_inputs_math!(div_group, "Unit::div", /);

criterion_group!(
    unit_benches,

    is_special_group,
    is_metric_group,
    is_unity_group,

    scalar_group,
    magnitude_group,
    expression_group,
    expression_reduced_group,

    composition_group,
    is_compatible_with_group,
    display_group,
    from_str_group,
    partial_eq_group,
    mul_group,
    div_group,
    partial_ord_gt_group,
);
criterion_main!(unit_benches);
