use classification::Classification;
use composition::Composition;
use definition::Definition;
use measurable::Measurable;
use property::Property;

pub trait UcumSymbol {
    fn classification(&self) -> Classification;
    fn composition(&self) -> Option<Composition>;
    fn definition(&self) -> Definition;
    fn primary_code(&self) -> &'static str;
    fn print_symbol(&self) -> Option<&'static str>;
    fn property(&self) -> Property;
    fn names(&self) -> Vec<&'static str>;
    fn secondary_code(&self) -> &'static str;

    fn is_arbitrary(&self) -> bool;
    fn is_metric(&self) -> bool;
    fn is_special(&self) -> bool;

    fn scalar(&self) -> f64 {
        self.definition().scalar()
    }

    fn magnitude(&self) -> f64 {
        self.definition().magnitude()
    }

    // TODO: is ok?
    fn calculate_scalar(&self, magnitude: f64) -> f64 {
        self.definition().calculate_scalar(magnitude)
    }

    // TODO: is ok?
    fn calculate_magnitude(&self, scalar: f64) -> f64 {
        self.definition().calculate_magnitude(scalar)
    }
}
