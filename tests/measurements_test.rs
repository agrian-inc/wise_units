extern crate wu;
use wu::Measurement;

#[test]
fn validate_conversions() {
    let subject = Measurement::new(500.0, "1");
    let converted = subject.convert_to("10^").unwrap();
    assert_eq!(converted.value, 50.0);

    let subject = Measurement::new(500.0, "1");
    let converted = subject.convert_to("%").unwrap();
    assert_eq!(converted.value, 50_000.0);

    let subject = Measurement::new(1.0, "m");
    let converted = subject.convert_to("km").unwrap();
    assert_eq!(converted.value, 0.001);

    let subject = Measurement::new(1.0, "km");
    let converted = subject.convert_to("m").unwrap();
    assert_eq!(converted.value, 1_000.0);

    let subject = Measurement::new(1.0, "m2");
    let converted = subject.convert_to("km2").unwrap();
    assert_eq!(converted.value, 0.000_001);

    let subject = Measurement::new(1.0, "km2");
    let converted = subject.convert_to("m2").unwrap();
    assert_eq!(converted.value, 1_000_000.0);

    let subject = Measurement::new(1.0, "m2/s");
    let converted = subject.convert_to("km2/s").unwrap();
    assert_eq!(converted.value, 0.000_001);

    let subject = Measurement::new(1.0, "km2/s");
    let converted = subject.convert_to("m2/s").unwrap();
    assert_eq!(converted.value, 1_000_000.0);

    let subject = Measurement::new(1.0, "s/m2");
    let converted = subject.convert_to("s/km2").unwrap();
    assert_eq!(converted.value, 1_000_000.0);

    let subject = Measurement::new(1.0, "s/km2");
    let converted = subject.convert_to("s/m2").unwrap();
    assert_eq!(converted.value, 0.000_001);

    let subject = Measurement::new(5.0, "[pi].m2");
    let converted = subject.convert_to("m2").unwrap();
    assert_eq!((converted.value * 10_000.0).round() / 10_000.0, 15.708);

    let subject = Measurement::new(500.0, "%");
    let converted = subject.convert_to("10^").unwrap();
    assert_eq!(converted.value, 0.5);

    let subject = Measurement::new(1.0, "[pi]");
    let converted = subject.convert_to("[ppth]").unwrap();
    assert_eq!(round_value(converted.value), 3141.5927);

    let subject = Measurement::new(25.0, "Cel");
    let converted = subject.convert_to("K").unwrap();
    assert_eq!(round_value(converted.value), 298.15);

    let subject = Measurement::new(298.15, "K");
    let converted = subject.convert_to("Cel").unwrap();
    assert_eq!(round_value(converted.value), 25.0);
}

fn round_value(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}
