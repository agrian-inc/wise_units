use atom::Classification;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Prefix {
    pub classification: Classification,
    pub names: &'static [&'static str],
    pub primary_code: &'static str,
    pub print_symbol: Option<&'static str>,
    pub scalar: f64,
    pub secondary_code: &'static str,
}

pub static PREFIXES: [Prefix; 24] = [
    Prefix {
        classification: Classification::SI,
        names: &["yotta"],
        primary_code: "Y",
        print_symbol: Some("Y"),
        scalar: 1e24,
        secondary_code: "YA",
    },
    Prefix {
        classification: Classification::SI,
        names: &["zetta"],
        primary_code: "Z",
        print_symbol: Some("Z"),
        scalar: 1e21,
        secondary_code: "ZA",
    },
    Prefix {
        classification: Classification::SI,
        names: &["exa"],
        primary_code: "E",
        print_symbol: Some("E"),
        scalar: 1e18,
        secondary_code: "EX",
    },
    Prefix {
        classification: Classification::SI,
        names: &["peta"],
        primary_code: "P",
        print_symbol: Some("P"),
        scalar: 1e15,
        secondary_code: "PT",
    },
    Prefix {
        classification: Classification::SI,
        names: &["tera"],
        primary_code: "T",
        print_symbol: Some("T"),
        scalar: 1e12,
        secondary_code: "TR",
    },
    Prefix {
        classification: Classification::SI,
        names: &["giga"],
        primary_code: "G",
        print_symbol: Some("G"),
        scalar: 1e9,
        secondary_code: "GA",
    },
    Prefix {
        classification: Classification::SI,
        names: &["mega"],
        primary_code: "M",
        print_symbol: Some("M"),
        scalar: 1e6,
        secondary_code: "MA",
    },
    Prefix {
        classification: Classification::SI,
        names: &["kilo"],
        primary_code: "k",
        print_symbol: Some("k"),
        scalar: 1e3,
        secondary_code: "K",
    },
    Prefix {
        classification: Classification::SI,
        names: &["hecto"],
        primary_code: "h",
        print_symbol: Some("h"),
        scalar: 1e2,
        secondary_code: "H",
    },
    Prefix {
        classification: Classification::SI,
        names: &["deka"],
        primary_code: "da",
        print_symbol: Some("da"),
        scalar: 1e1,
        secondary_code: "DA",
    },
    Prefix {
        classification: Classification::SI,
        names: &["deci"],
        primary_code: "d",
        print_symbol: Some("d"),
        scalar: 1e-1,
        secondary_code: "D",
    },
    Prefix {
        classification: Classification::SI,
        names: &["centi"],
        primary_code: "c",
        print_symbol: Some("c"),
        scalar: 1e-2,
        secondary_code: "C",
    },
    Prefix {
        classification: Classification::SI,
        names: &["milli"],
        primary_code: "m",
        print_symbol: Some("m"),
        scalar: 1e-3,
        secondary_code: "M",
    },
    Prefix {
        classification: Classification::SI,
        names: &["micro"],
        primary_code: "u",
        print_symbol: Some("μ"),
        scalar: 1e-6,
        secondary_code: "U",
    },
    Prefix {
        classification: Classification::SI,
        names: &["nano"],
        primary_code: "n",
        print_symbol: Some("n"),
        scalar: 1e-9,
        secondary_code: "N",
    },
    Prefix {
        classification: Classification::SI,
        names: &["pico"],
        primary_code: "p",
        print_symbol: Some("p"),
        scalar: 1e-12,
        secondary_code: "P",
    },
    Prefix {
        classification: Classification::SI,
        names: &["femto"],
        primary_code: "f",
        print_symbol: Some("f"),
        scalar: 1e-15,
        secondary_code: "F",
    },
    Prefix {
        classification: Classification::SI,
        names: &["atto"],
        primary_code: "a",
        print_symbol: Some("a"),
        scalar: 1e-18,
        secondary_code: "A",
    },
    Prefix {
        classification: Classification::SI,
        names: &["zepto"],
        primary_code: "z",
        print_symbol: Some("z"),
        scalar: 1e-21,
        secondary_code: "ZO",
    },
    Prefix {
        classification: Classification::SI,
        names: &["yocto"],
        primary_code: "y",
        print_symbol: Some("y"),
        scalar: 1e-24,
        secondary_code: "YO",
    },
    Prefix {
        classification: Classification::SI,
        names: &["kibi"],
        primary_code: "Ki",
        print_symbol: Some("Ki"),
        scalar: 1024.0,
        secondary_code: "KIB",
    },
    Prefix {
        classification: Classification::SI,
        names: &["mebi"],
        primary_code: "Mi",
        print_symbol: Some("Mi"),
        scalar: 1048576.0,
        secondary_code: "MIB",
    },
    Prefix {
        classification: Classification::SI,
        names: &["gibi"],
        primary_code: "Gi",
        print_symbol: Some("Gi"),
        scalar: 1073741824.0,
        secondary_code: "GIB",
    },
    Prefix {
        classification: Classification::SI,
        names: &["tebi"],
        primary_code: "Ti",
        print_symbol: Some("Ti"),
        scalar: 1099511627776.0,
        secondary_code: "TIB",
    },
    ];
