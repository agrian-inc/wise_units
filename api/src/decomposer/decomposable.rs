/// Defines an interface for deconstructing a Unit into a String that represents
/// the Terms that define it.
///
pub trait Decomposable<'a> {
    type Terms;
    type Collection;

    /// The main function clients should care about.
    ///
    fn decompose(&self, terms: Self::Terms) -> String {
        let collection = self.terms_to_collection(terms);
        let numerator = self.numerator(&collection);
        let denominator = self.denominator(&collection);

        self.format_output(numerator, denominator)
    }

    fn format_output(&self, numerator: Option<String>, denominator: Option<String>) -> String {
        let n = numerator.unwrap_or_default();

        if let Some(d) = denominator {
            format!("{}/{}", n, d)
        } else {
            n
        }
    }

    fn terms_to_collection(&self, terms: Self::Terms) -> Self::Collection;
    fn numerator(&self, collection: &Self::Collection) -> Option<String>;
    fn denominator(&self, collection: &Self::Collection) -> Option<String>;
}