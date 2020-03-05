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

/// Assigns the local prefix based on the Host HTTP header.
///
/// If served from openipf.org, we want it to pretend to be at the root,
/// since Nginx has a rewrite rule that always prepends /dist/openipf.
///
/// If served from elsewhere (localhost or openpowerlifting.org), we want
/// to prepend /dist/openipf/ to allow it to use the same common server.
fn get_local_prefix(host: &Host) -> &'static str {
    if host.served_from_openipf_org() {
        "/"
    } else {
        LOCAL_PREFIX
    }
}

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
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let default = default_openipf_selection();
    let mut cx =
        pages::rankings::Context::new(&opldb, &locale, &default, &default, true)?;
    cx.urlprefix = get_local_prefix(&host);

    // FIXME: Hack for launch day.
    cx.page_title = format!("IPF {}", locale.strings.header.rankings);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/rankings", &cx),
        Device::Mobile => Template::render("openipf/mobile/rankings", &cx),
    })
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
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    let default = default_openipf_selection();
    let selection = pages::selection::Selection::from_path(&selections, &default).ok()?;
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut cx =
        pages::rankings::Context::new(&opldb, &locale, &selection, &default, true)?;
    cx.urlprefix = get_local_prefix(&host);

    // FIXME: Hack for launch day.
    cx.page_title = format!("IPF {}", locale.strings.header.rankings);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/rankings", &cx),
        Device::Mobile => Template::render("openipf/mobile/rankings", &cx),
    })
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

    let mut slice = pages::api_rankings::get_slice(
        &opldb,
        &locale,
        &selection,
        &default,
        query.start,
        query.end,
    );

    for row in &mut slice.rows {
        if row.equipment == &locale.strings.equipment.raw {
            row.equipment = &locale.strings.equipment.classic;
        }
        if row.equipment == &locale.strings.equipment.single {
            row.equipment = &locale.strings.equipment.equipped;
        }
    }

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
    host: Host,
    device: Device,
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
    let mut cx = pages::records::Context::new(
        &opldb,
        &locale,
        &selection,
        &default_openipf_selection(),
    );
    cx.urlprefix = get_local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/records", &cx),
        Device::Mobile => Template::render("openipf/mobile/records", &cx),
    })
}

#[get("/records?<lang>")]
pub fn records_default(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    records(
        None, lang, opldb, langinfo, languages, host, device, cookies,
    )
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
    host: Host,
    device: Device,
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
            let mut cx = pages::lifter::Context::new(
                &opldb,
                &locale,
                lifter_ids[0],
                Some(ipf_only_filter),
            );
            cx.urlprefix = get_local_prefix(&host);

            // Change the equipment terminology to be IPF-specific.
            for best in &mut cx.bests {
                if best.equipment == &locale.strings.equipment.raw {
                    best.equipment = &locale.strings.equipment.classic;
                }
                if best.equipment == &locale.strings.equipment.single {
                    best.equipment = &locale.strings.equipment.equipped;
                }
            }
            for result in &mut cx.meet_results {
                if result.equipment == &locale.strings.equipment.raw {
                    result.equipment = &locale.strings.equipment.classic;
                }
                if result.equipment == &locale.strings.equipment.single {
                    result.equipment = &locale.strings.equipment.equipped;
                }
            }

            Some(Ok(match device {
                Device::Desktop => Template::render("openipf/desktop/lifter", cx),
                Device::Mobile => Template::render("openipf/mobile/lifter", cx),
            }))
        }

        // If multiple lifters were referenced, return a disambiguation page.
        _ => {
            let mut cx = pages::disambiguation::Context::new(
                &opldb,
                &locale,
                &username,
                &lifter_ids,
            );
            cx.urlprefix = get_local_prefix(&host);

            Some(Ok(match device {
                Device::Desktop => Template::render("openipf/desktop/disambiguation", cx),
                Device::Mobile => Template::render("openipf/mobile/disambiguation", cx),
            }))
        }
    }
}

#[get("/u/<username>/csv")]
pub fn lifter_csv(username: String, opldb: State<ManagedOplDb>) -> Option<String> {
    let lifter_id = opldb.get_lifter_id(&username)?;
    pages::lifter_csv::export_csv(&opldb, lifter_id, Some(ipf_only_filter)).ok()
}

#[get("/mlist/<mselections..>?<lang>")]
pub fn meetlist(
    mselections: Option<PathBuf>,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    let openipf_defaults = default_openipf_selection();
    let defaults = pages::meetlist::MeetListSelection {
        federation: openipf_defaults.federation,
        year: openipf_defaults.year,
    };

    let mselection = match mselections {
        None => defaults,
        Some(p) => pages::meetlist::MeetListSelection::from_path(&p, defaults).ok()?,
    };
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut cx = pages::meetlist::Context::new(&opldb, &locale, &mselection);
    cx.urlprefix = get_local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/meetlist", &cx),
        Device::Mobile => Template::render("openipf/mobile/meetlist", &cx),
    })
}

#[get("/mlist?<lang>")]
pub fn meetlist_default(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    meetlist(
        None, lang, opldb, langinfo, languages, host, device, cookies,
    )
}

#[get("/m/<meetpath..>?<lang>")]
pub fn meet(
    meetpath: PathBuf,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
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
    let mut cx = pages::meet::Context::new(&opldb, &locale, meet_id, sort);
    cx.urlprefix = get_local_prefix(&host);

    // Change the equipment terminology to be IPF-specific.
    for table in &mut cx.tables {
        for row in &mut table.rows {
            if row.equipment == &locale.strings.equipment.raw {
                row.equipment = &locale.strings.equipment.classic;
            }
            if row.equipment == &locale.strings.equipment.single {
                row.equipment = &locale.strings.equipment.equipped;
            }
        }
    }

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/meet", &cx),
        Device::Mobile => Template::render("openipf/mobile/meet", &cx),
    })
}

/// Used to show only IPF-sanctioned federations.
fn ipf_fed_filter(fed: Federation) -> bool {
    // Using a maximum date causes the sanctioning_body() logic to return the most
    // current sanctioning information.
    let latest = Date::from_parts(9999, 01, 01);
    fed.sanctioning_body(latest) == Some(Federation::IPF)
}

#[get("/status?<lang>")]
pub fn status(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut cx = pages::status::Context::new(&opldb, &locale, Some(ipf_fed_filter));
    cx.urlprefix = get_local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/status", &cx),
        Device::Mobile => Template::render("openipf/mobile/status", &cx),
    })
}

#[get("/data?<lang>")]
pub fn data(
    lang: Option<String>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut cx = pages::data::Context::new(&locale);
    cx.urlprefix = get_local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/data", &cx),
        Device::Mobile => Template::render("openipf/mobile/data", &cx),
    })
}

#[get("/faq?<lang>")]
pub fn faq(
    lang: Option<String>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut cx = pages::faq::Context::new(&locale);
    cx.urlprefix = get_local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/faq", &cx),
        Device::Mobile => Template::render("openipf/mobile/faq", &cx),
    })
}

#[get("/contact?<lang>")]
pub fn contact(
    lang: Option<String>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut cx = pages::contact::Context::new(&locale);
    cx.urlprefix = get_local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/contact", &cx),
        Device::Mobile => Template::render("openipf/mobile/contact", &cx),
    })
}
