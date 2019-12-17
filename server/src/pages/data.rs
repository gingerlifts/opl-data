//! Logic for the data page.

use crate::langpack::{self, Language, Locale};
use opltypes;

/// The context object passed to `templates/data.html.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub urlprefix: &'static str,
    pub page_title: &'a str,
    pub page_description: &'a str,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: opltypes::WeightUnits,
}

impl<'a> Context<'a> {
    pub fn new(locale: &'a Locale) -> Context<'a> {
        Context {
            urlprefix: "/",
            page_title: &locale.strings.header.data,
            page_description: &locale.strings.html_header.description,
            strings: locale.strings,
            language: locale.language,
            units: locale.units,
        }
    }
}
