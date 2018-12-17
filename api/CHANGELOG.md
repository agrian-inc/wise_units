# Change Log

## [unreleased]

### Added

### Changed

* Updated `package.edition` to use `"2018"` in `Cargo.toml`.

### Deprecated

### Removed

### Fixed

### Security

## [0.5.1] - 2018-11-06

### Changed

- [DEV-3155] Reverted the change to the `Composable` trait definition to deal
  only with `self`; this caused `cannot move out of borrowed content` errors
  when trying to use the API normally.

## [0.5.0] - 2018-11-06

### Added

- [DEV-241] Add `AsFraction` and implement for `Unit`.
- [DEV-1013] Inlined most of the public API methods.
- [DEV-3155] Added `DefaultCompatibility` marker trait to mark types that should
  implement the default behavior of `IsCompatibleWith`, effectively paving the
  way to be able to define a different behavior for `Term`.
- [DEV-3327] Added `AnnotationComposable` and `AnnotationComposition` and
  implemented for `&[Term]`. Allows for pessimistic comparing of `Term`s with
  annotations.

### Changed

- [DEV-3155] Changed `Term`'s implementation of `Display` to include its
  `annotation`.
- [DEV-3155] Changed the `Composable` trait definition to deal only with `self`
  (which makes implementations cleaner).
- [DEV-3155] Refactored out `Composable::is_compatible_with()` into a new trait,
  `IsCompatibleWith`, and blanket-implemented that for all types that implement
  `Composable`. Not only does this clean up code, but also now lets you compare
  `Measurement`s and `Unit`s. Library consumers that call `is_compatible_with`
  will need to change `use wise_units::Composable` to
  `use wise_units::IsCompatibleWith`.
- [DEV-3155] Changed `Term::is_compatible_with()` to account for the
  `annotation` attribute of each `Term` being compared.

### Removed

- In preparation for the Rust 2018 edition release before the end of the year,
  updated to Rust 1.30.0 features, *making this the minimum supported version*.

### Fixed

- [DEV-3300] Parsing annotations now excludes curly braces in the resulting
  string capture.

## [0.4.0] - 2018-10-02

### Added

- Implemented `Composable` for `Measurement` to allow checking compatibility
  of `Measurement`s.
- `Unit::into_reduced()`
- `unit::term_reducing` for reducing a `Unit`'s `Term`s. This, in conjunction
  with the still-existing `SimpleDecomposer`, now replaces the
  `ReductionDecomposer`.
- `Term::has_value()`

### Changed

- [DEV-236] `Unit`s are now reduced after being multiplied or divided.
- A `Unit` that's supposed to represent the unity ("1") used to be a `Unit` with
  a `Vec<Term>` with 1 `Term` whose attributes were all `None`; now that same
  `Term` has a `factor` of `Some(1)`. It's possible that a `Unit` with 0 `Term`s
  may still be interpreted similarly (haven't looked into this yet).

## [0.3.0] - 2018-08-27

### Added

- `Decomposable` is now public.
- `UcumSymbol` is now public.
- Added `Term::factor_as_u32()`.
- `Term` `factor` and `exponent` are now wrapped in an `Option` since many
  `Term`s don't need these values.

### Changed

- Refactored trait implementations for `Measurement` and `Unit` to sub-modules.
  The files were getting too long.
- `Term::factor` and `Term::exponent` are now wrapped in an `Option` (the same
  goes for their related `parser::terms::mapper` components) to save unnecessary
  allocations.
- Renamed `Decomposable::expression()` to `Decomposable::decompose()` to be more
  conventional.
- `Decomposable::decompose()` now takes a value to let the caller decide what to
  pass in.
- `Decomposable` now defines associated types for more `impl` flexibility.
- Extracted some of `Decomposable::decompose()` to a new method,
  `Decomposable::format_output()`, which let's consumers customize the formatted
  output.
- `Decomposable::numerator()` and `Decomposable::denominator()` now return
  `Option<String>` instead of `String`.
- Refactored a number of `Decomposable` implementation methods.

### Removed

- *Removed* `stdweb` support. This was overkill from the start.

### Fixed

- [DEV-331] Fixed a number of conversion bugs. Redesigned `Composition` to get
  there.
- To match SI displaying, added a space for the implementation of `Display` for
  `Measurement` in between the value and the unit.

## [0.2.0] - 2018-06-26

### Added

- Added implementations of `Add`, `Sub`, `Mul`, `Div` for `&'a Measurement` and
  `&'a Unit` where the right-hand side is an owned version of the same type.
- Added implementations of `Mul`, `Div` for `&'a Measurement` where the
  right-hand side is an `f64`.

### Changed

- `Convertible` trait now uses associated types for the output type and the
  error type, letting definers use their own.

### Removed

- Removed implementations of `Add`, `Sub`, `Mul`, `Div` for `&'a mut Measurement`
  and `&'a mut Unit`. Those seem like edge-cases and thus code bloat at this
  point.

## [0.2.0] - 2018-06-26

### Added

- [AGDEV-30315] Add serde support using "with_serde" feature flag.
- [AGDEV-30253] Add stdweb support using "with_stdweb" feature flag. Just adds
  `js_serializable!` and `js_deserializable!` for both `Measurement` and `Unit`.
- The `UcumUnit` trait now handles some of the core attributes defined by the
  UCUM spec. `Atom`, `Term`, `Unit`, and `Measurement` implement this.
- The `Convertible` trait allows for converting to different objects using
  `convert_to()`. `Measurement` implements this for `Unit` and `&str`.
- The `FieldEq` trait lets you define how to compare objects using `field_eq()`.
  Normally this would be handled by `PartialEq`, but since it seems more often
  that library users would want to check that `1km == 1000m` than check
  `1km == 1km && 1km != 1000m`.  If they need to check the latter, that's what
  `field_eq()` is for. Both `Measurement` and `Unit` implement this.
- The `Reducible` trait provides methods for reducing an object down to its
  associated base unit(s). `Atom`, `Term`, `Vec<Term>`, `Definition`, `Unit`,
  and `Measurement` implement this.
- `Unit` can now `Deref` to `&[Term]`.
- `Unit` can now `From` into `Vec<Term>`.
- Probably some other things...

### Changed

- Lots of refactoring.
- The `PartialEq` implementation for `Unit` checked that its attributes were
  equal to each other--which isn't what we want here. It now checks that the
  two `Unit`s are compatible, then compares the result of their `scalar()` calls.
- To allow for defining custom units, parsing is now done in two stages:
    1. Parsing the outer tokens (`.`, `/`, `{` and `}`, etc.)
    1. Parsing the unit symbols (`m`, `[acr_us]`, `10^`, etc.)

### Removed

- `Measurement::mul_scalar()` and `Measurement::div_scalar()` are now solved
  by implementing [`Mul<f64>`](https://doc.rust-lang.org/std/ops/trait.Mul.html)
  and [`Div<f64>`](https://doc.rust-lang.org/std/ops/trait.Div.html).
- `Unit::mul_u32()` and `Unit::div_u32()` have been removed since dividing a
  `Unit` by a value would end up making the `Unit`'s factor a fraction, but
  UCUM doesn't allow that.
- `Unit::is_valid` has been removed. `Unit`s will normally be instantiated
  by using `from_str()`, which will fail if the tokens represent an invalid
  unit. It's still possible to make an invalid `Unit` by manually
  instantiating each `Term` then instantiating the `Unit`. You're on your
  own if you do that.
- `calculate_scalar()` has been moved into a trait and renamed
  `Reducible::reduce_value()`.
- The `Measurable` trait has been removed, but its methods are still
  implemented accordingly.
- `Measurement::unit_string()` has been removed. Depending on your need, you
  can use `measurement.unit.to_string()` or `measurement.unit.expression()`,
  or `measurement.unit.expression_reduced()`.
- `Measurement::to_f64()` has been removed. This just returned `self.value`,
  so there wasn't much value in it.
- Split into two subcrates:
    - `wise_units` (under `api/`): The main API for dealing with units via `Measurement`
      and `Unit` types.
    - `wise_units-atom_generator` (under `atom_generator/`): Code used for generating
      the list of allowed atoms during build time.
- Removed `Atom::TheUnity` in favor of dealing with this as a `Term` with a `factor` of 1.
- `Composable::composition()` now returns a `Composition` instead of `Option<Composition>`.

## [0.1.0] - 2018-01-22

### Added

- Initial release!
