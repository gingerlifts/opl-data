//! Defines Rocket handlers for the OpenIPF distribution.
//!
//! On openpowerlifting.org, these handlers are mounted under /dist/openipf/.
//! The openipf.org site works by using the same server as openpowerlifting.org,
//! with Nginx rewriting URLs based on domain.

use opltypes::*;

use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

use server::langpack::{Language, Locale};
use server::opldb::{self, Entry, MetaFederation};
use server::pages;

use std::path::PathBuf;

use crate::common::*;

/// URL prefix used when accessing OpenIPF through OpenPowerlifting.org or
/// localhost.
pub const LOCAL_PREFIX: &'static str = "/dist/openipf/";

/// Default selections used in the OpenIPF rankings.
///
/// This information is also hardcoded in the rankings template.
fn default_openipf_selection() -> pages::selection::Selection {
    use pages::selection::*;
    Selection {
        equipment: EquipmentSelection::Raw,
        federation: FederationSelection::Meta(MetaFederation::IPFAndAffiliates),
        weightclasses: WeightClassSelection::AllClasses,
        sex: SexSelection::AllSexes,
        ageclass: AgeClassSelection::AllAges,
        year: YearSelection::AllYears,
        event: EventSelection::FullPower,
        sort: SortSelection::ByIPFPoints,
        state: None,
    }
}

/// Defines the default rankings used on the site homepage, suitable for the
/// IPF.
#[get("/?<lang>")]
pub fn index(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let selection = default_openipf_selection();
    let mut context = pages::rankings::Context::new(&opldb, &locale, &selection)?;
    context.urlprefix = LOCAL_PREFIX;
    Some(Template::render("openipf/rankings", &context))
}

/// Defines a Rankings sub-page.
///
/// The intention is to reuse as much backend code as possible with
/// OpenPowerlifting, and just swap out the frontend to be IPF-specific so it
/// looks like its own thing.
#[get("/rankings/<selections..>?<lang>")]
pub fn rankings(
    selections: PathBuf,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let default = default_openipf_selection();
    let selection = pages::selection::Selection::from_path(&selections, &default).ok()?;
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut context = pages::rankings::Context::new(&opldb, &locale, &selection)?;
    context.urlprefix = LOCAL_PREFIX;
    Some(Template::render("openipf/rankings", &context))
}

/// API endpoint for fetching a slice of rankings data as JSON.
#[get("/api/rankings/<selections..>?<query..>")]
pub fn rankings_api(
    selections: Option<PathBuf>,
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    let default = default_openipf_selection();
    let selection = match selections {
        None => default,
        Some(path) => pages::selection::Selection::from_path(&path, &default).ok()?,
    };

    let language = query.lang.parse::<Language>().ok()?;
    let units = query.units.parse::<WeightUnits>().ok()?;
    let locale = Locale::new(&langinfo, language, units);

    let slice = pages::api_rankings::get_slice(
        &opldb,
        &locale,
        &selection,
        query.start,
        query.end,
    );

    // TODO: Maybe we can use rocket_contrib::Json, but the lifetimes
    // of the values in `slice` outlive this function, which doesn't work.
    Some(JsonString(serde_json::to_string(&slice).ok()?))
}

#[get("/api/rankings?<query..>")]
pub fn default_rankings_api(
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    rankings_api(None, query, opldb, langinfo)
}

/// API endpoint for rankings search.
#[get("/api/search/rankings/<selections..>?<query..>")]
pub fn search_rankings_api<'db>(
    selections: Option<PathBuf>,
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    let default = default_openipf_selection();
    let selection = match selections {
        None => default,
        Some(path) => pages::selection::Selection::from_path(&path, &default).ok()?,
    };

    let result =
        pages::api_search::search_rankings(&opldb, &selection, query.start, &query.q);

    Some(JsonString(serde_json::to_string(&result).ok()?))
}

#[get("/api/search/rankings?<query..>")]
pub fn default_search_rankings_api(
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    search_rankings_api(None, query, opldb)
}

#[get("/records/<selections..>?<lang>")]
pub fn records(
    selections: Option<PathBuf>,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let default_rankings = default_openipf_selection();
    let default = pages::records::RecordsSelection {
        equipment: default_rankings.equipment,
        federation: default_rankings.federation,
        sex: pages::selection::SexSelection::Men,
        classkind: pages::records::ClassKindSelection::IPF,
        ageclass: default_rankings.ageclass,
        year: default_rankings.year,
        state: None,
    };

    let selection = if let Some(sel) = selections {
        pages::records::RecordsSelection::from_path(&sel, &default).ok()?
    } else {
        default
    };
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut context = pages::records::Context::new(
        &opldb,
        &locale,
        &selection,
        &default_openipf_selection(),
    );
    context.urlprefix = LOCAL_PREFIX;
    Some(Template::render("openipf/records", &context))
}

#[get("/records?<lang>")]
pub fn records_default(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    records(None, lang, opldb, langinfo, languages, cookies)
}

/// Used to show only IPF-sanctioned meets.
fn ipf_only_filter(opldb: &opldb::OplDb, e: &Entry) -> bool {
    let meet = opldb.get_meet(e.meet_id);
    meet.federation.sanctioning_body(meet.date) == Some(Federation::IPF)
}

#[get("/u/<username>?<lang>")]
pub fn lifter(
    username: String,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Result<Template, Redirect>> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);

    // Disambiguations end with a digit.
    // Some lifters may have failed to be merged with their disambiguated username.
    // Therefore, for usernames without a digit, it cannot be assumed that they are
    // *not* a disambiguation.
    let is_definitely_disambiguation: bool = username
        .chars()
        .last()
        .map_or(false, |c| c.is_ascii_digit());

    let lifter_ids: Vec<u32> = if is_definitely_disambiguation {
        if let Some(id) = opldb.get_lifter_id(&username) {
            vec![id]
        } else {
            vec![]
        }
    } else {
        opldb.get_lifters_under_username(&username)
    };

    match lifter_ids.len() {
        // If no LifterID was found, maybe the name just needs to be lowercased.
        0 => {
            let lowercase = username.to_ascii_lowercase();
            let _guard = opldb.get_lifter_id(&lowercase)?;
            Some(Err(Redirect::permanent(format!("/u/{}", lowercase))))
        }

        // If a specific lifter was referenced, return the lifter's unique page.
        1 => {
            let mut context = pages::lifter::Context::new(
                &opldb,
                &locale,
                lifter_ids[0],
                Some(ipf_only_filter),
            );
            context.urlprefix = LOCAL_PREFIX;

            // Change the equipment terminology to be IPF-specific.
            for best in &mut context.bests {
                if best.equipment == &locale.strings.equipment.raw {
                    best.equipment = &locale.strings.equipment.classic;
                }
                if best.equipment == &locale.strings.equipment.single {
                    best.equipment = &locale.strings.equipment.equipped;
                }
            }
            for result in &mut context.meet_results {
                if result.equipment == &locale.strings.equipment.raw {
                    result.equipment = &locale.strings.equipment.classic;
                }
                if result.equipment == &locale.strings.equipment.single {
                    result.equipment = &locale.strings.equipment.equipped;
                }
            }

            Some(Ok(Template::render("openipf/lifter", &context)))
        }

        // If multiple lifters were referenced, return a disambiguation page.
        _ => {
            let mut context = pages::disambiguation::Context::new(
                &opldb,
                &locale,
                &username,
                &lifter_ids,
            );
            context.urlprefix = LOCAL_PREFIX;
            Some(Ok(Template::render("openipf/disambiguation", &context)))
        }
    }
}

#[get("/u/<username>/csv")]
pub fn lifter_csv(username: String, opldb: State<ManagedOplDb>) -> Option<String> {
    let lifter_id = opldb.get_lifter_id(&username)?;
    pages::lifter_csv::export_csv(&opldb, lifter_id, Some(ipf_only_filter)).ok()
}

#[get("/m/<meetpath..>?<lang>")]
pub fn meet(
    meetpath: PathBuf,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let mut meetpath_str: &str = meetpath.to_str()?;
    let mut sort = pages::meet::MeetSortSelection::ByFederationDefault;

    // The meetpath may contain an optional sorting directive.
    // If present, detect and remove that component from the path.
    let component = meetpath.as_path().file_name()?.to_str()?;
    if let Ok(sortselection) = component.parse::<pages::meet::MeetSortSelection>() {
        sort = sortselection;
        meetpath_str = meetpath.as_path().parent()?.to_str()?;
    }

    let meet_id = opldb.get_meet_id(meetpath_str)?;
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut context = pages::meet::Context::new(&opldb, &locale, meet_id, sort);

    // Change the equipment terminology to be IPF-specific.
    for table in &mut context.tables {
        for row in &mut table.rows {
            if row.equipment == &locale.strings.equipment.raw {
                row.equipment = &locale.strings.equipment.classic;
            }
            if row.equipment == &locale.strings.equipment.single {
                row.equipment = &locale.strings.equipment.equipped;
            }
        }
    }

    context.urlprefix = LOCAL_PREFIX;
    Some(Template::render("openipf/meet", &context))
}

#[get("/faq?<lang>")]
pub fn faq(
    lang: Option<String>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut context = pages::faq::Context::new(&locale);
    context.urlprefix = LOCAL_PREFIX;
    Some(Template::render("openipf/faq", &context))
}
