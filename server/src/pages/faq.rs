//! Logic for the faq page.

use crate::langpack;
use opltypes;

/// The context object passed to `templates/faq.html.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub urlprefix: &'static str,
    pub page_title: &'a str,
    pub page_description: &'a str,
    pub language: langpack::Language,
    pub strings: &'a langpack::Translations,
    pub units: opltypes::WeightUnits,
}

impl<'a> Context<'a> {
    pub fn new(locale: &'a langpack::Locale) -> Context<'a> {
        Context {
            urlprefix: "/",
            page_title: &locale.strings.header.faq,
            page_description: &locale.strings.html_header.description,
            strings: locale.strings,
            language: locale.language,
            units: locale.units,
        }
    }
}
