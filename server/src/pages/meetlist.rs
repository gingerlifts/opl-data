//! Logic for the page that lists meets.

use opldb::query::direct::*;
use opldb::{self, Meet, MetaFederation};
use opltypes::*;

use std::ffi::OsStr;
use std::path;

use crate::langpack::{self, Language, Locale};

/// Query selection descriptor, corresponding to HTML widgets.
///
/// For code reuse, this is a subset of the Query struct
/// used by the rankings page. It needs to serialize to a structure
/// that has the same fields, so the templates can share code.
#[derive(Copy, Clone, PartialEq, Serialize)]
pub struct MeetListQuery {
    pub federation: FederationFilter,
    pub year: YearFilter,
}

impl Default for MeetListQuery {
    fn default() -> MeetListQuery {
        MeetListQuery {
            federation: FederationFilter::AllFederations,
            year: YearFilter::AllYears,
        }
    }
}

impl MeetListQuery {
    pub fn from_path(p: &path::Path, defaults: MeetListQuery) -> Result<Self, ()> {
        let mut ret = defaults;

        // Disallow empty path components.
        if let Some(s) = p.to_str() {
            if s.contains("//") {
                return Err(());
            }
        } else {
            // Failed parsing UTF-8.
            return Err(());
        }

        // Prevent fields from being overwritten or redundant.
        let mut parsed_federation: bool = false;
        let mut parsed_year: bool = false;

        // Iterate over each path component, attempting to determine
        // what kind of data it is.
        for segment in p
            .ancestors()
            .filter_map(|a| a.file_name().and_then(OsStr::to_str))
        {
            // Check whether this is federation information.
            if let Ok(f) = FederationFilter::from_str_preferring(
                segment,
                FedPreference::PreferFederation,
            ) {
                if parsed_federation {
                    return Err(());
                }

                // Even though we requested PreferFederation, in the case of
                // British Powerlifting, forcibly assign the MetaFederation.
                //
                // BP is a special case here because it's a country-level federation
                // that includes other countries. Lifters in, e.g., the EPA call
                // their federation "BP" and therefore expect results to show up there.
                if f == FederationFilter::One(Federation::BP) {
                    ret.federation = FederationFilter::Meta(MetaFederation::BP);
                } else {
                    ret.federation = f;
                }

                parsed_federation = true;
            // Check whether this is year information.
            } else if let Ok(y) = segment.parse::<YearFilter>() {
                if parsed_year {
                    return Err(());
                }
                ret.year = y;
                parsed_year = true;
            // Unknown string, therefore malformed URL.
            } else {
                return Err(());
            }
        }

        Ok(ret)
    }
}

// TODO: Share with pages::meet::MeetInfo.
#[derive(Serialize)]
pub struct MeetInfo<'db> {
    pub path: &'db str,
    pub federation: Federation,
    pub date: String,
    pub country: &'db str,
    pub state: Option<&'db str>,
    pub town: Option<&'db str>,
    pub name: &'db str,
    pub num_lifters: u32,
}

impl<'db> MeetInfo<'db> {
    pub fn from(
        meet: &'db opldb::Meet,
        strings: &'db langpack::Translations,
    ) -> MeetInfo<'db> {
        MeetInfo {
            path: &meet.path,
            federation: meet.federation,
            date: format!("{}", &meet.date),
            country: strings.translate_country(meet.country),
            state: match meet.state {
                None => None,
                Some(ref s) => Some(&s),
            },
            town: match meet.town {
                None => None,
                Some(ref s) => Some(&s),
            },
            name: &meet.name,
            num_lifters: meet.num_unique_lifters,
        }
    }
}

/// The context object passed to `templates/meet.html.tera`
#[derive(Serialize)]
pub struct Context<'db> {
    pub urlprefix: &'static str,
    pub page_title: &'db str,
    pub page_description: &'db str,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: WeightUnits,

    pub selection: &'db MeetListQuery,
    pub meets: Vec<MeetInfo<'db>>,

    /// Temporary crutch until we figure out how to show
    /// more meets on a single page.
    pub theres_more: bool,
}

impl<'db> Context<'db> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        locale: &'db Locale,
        mselection: &'db MeetListQuery,
    ) -> Context<'db> {
        // Maximum number of meets to send at once. ~200kb HTML.
        const PAGE_SIZE: usize = 1000;

        let year: Option<u32> = mselection.year.as_u32();

        // TODO: Move this selection logic into the opldb.
        let mut meets: Vec<&Meet> = match mselection.federation {
            FederationFilter::AllFederations => opldb
                .get_meets()
                .iter()
                .filter(|m| match year {
                    Some(year) => m.date.year() == year,
                    None => true,
                })
                .collect(),
            FederationFilter::One(fed) => {
                opldb
                    .get_meets()
                    .iter()
                    .filter(|m| {
                        // Filter by year.
                        if let Some(year) = year {
                            if m.date.year() != year {
                                return false;
                            }
                        }

                        m.federation == fed
                    })
                    .collect()
            }
            FederationFilter::Meta(meta) => opldb
                .get_metafed_cache()
                .get_meet_ids_for(meta)
                .iter()
                .map(|&i| opldb.get_meet(i))
                .filter(|m| match year {
                    Some(year) => m.date.year() == year,
                    None => true,
                })
                .collect(),
        };

        meets.sort_unstable_by(|a, b|
            // First sort by date, latest first.
            a.date.cmp(&b.date).reverse()
                // If equal, sort by federation name.
                .then(a.federation.cmp(&b.federation)));

        let total_meets = meets.len();

        Context {
            urlprefix: "/",
            page_title: &locale.strings.page_titles.meets,
            page_description: &locale.strings.html_header.description,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            selection: mselection,
            meets: meets
                .into_iter()
                .take(PAGE_SIZE)
                .map(|m| MeetInfo::from(m, &locale.strings))
                .collect(),
            theres_more: total_meets > PAGE_SIZE,
        }
    }
}
