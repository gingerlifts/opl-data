#![feature(proc_macro_hygiene, decl_macro)]

extern crate accept_language;
extern crate dotenv;
extern crate opltypes;
use opltypes::{Federation, WeightUnits};
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate strum;

#[cfg(test)]
mod tests;

use rocket::fairing::AdHoc;
use rocket::http::{ContentType, Cookies, Status};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::{self, content, NamedFile, Redirect, Responder, Response};
use rocket::{Outcome, State};
use rocket_contrib::templates::Template;

use strum::IntoEnumIterator;

use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

extern crate server;
use server::langpack::{self, LangInfo, Language, Locale};
use server::opldb;
use server::pages;

/// Request guard for reading the "Accept-Encoding" HTTP header.
struct AcceptEncoding(pub Option<String>);

impl AcceptEncoding {
    pub fn supports_gzip(&self) -> bool {
        match &self.0 {
            None => false,
            Some(s) => s.contains("gzip"),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptEncoding {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AcceptEncoding, ()> {
        let keys: Vec<_> = request.headers().get("Accept-Encoding").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptEncoding(None)),
            1 => Outcome::Success(AcceptEncoding(Some(keys[0].to_string()))),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

/// Request guard for reading the "Accept-Language" HTTP header.
struct AcceptLanguage(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AcceptLanguage, ()> {
        let keys: Vec<_> = request.headers().get("Accept-Language").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptLanguage(None)),
            1 => Outcome::Success(AcceptLanguage(Some(keys[0].to_string()))),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

fn select_display_language(languages: AcceptLanguage, cookies: &Cookies) -> Language {
    let default = Language::en;

    // The user may explicitly override the language choice by using
    // a cookie named "lang".
    if let Some(cookie) = cookies.get("lang") {
        if let Ok(lang) = cookie.value().parse::<Language>() {
            return lang;
        }
    }

    // If a language was not explicitly selected, the Accept-Language HTTP
    // header is consulted, defaulting to English.
    match languages.0 {
        Some(s) => {
            // TODO: It would be better if this vector was static.
            let known_languages: Vec<String> = Language::string_list();
            let borrowed: Vec<&str> =
                known_languages.iter().map(|s| s.as_ref()).collect();
            let valid_languages = accept_language::intersection(&s, borrowed);

            if valid_languages.is_empty() {
                default
            } else {
                valid_languages[0].parse::<Language>().unwrap_or(default)
            }
        }
        None => default,
    }
}

fn select_weight_units(language: Language, cookies: &Cookies) -> WeightUnits {
    // The user may explicitly override the weight unit choice by using
    // a cookie named "units".
    if let Some(cookie) = cookies.get("units") {
        if let Ok(units) = cookie.value().parse::<WeightUnits>() {
            return units;
        }
    }

    // TODO: Check Accept-Language header for regional variants of English,
    // for example Australia, to select Kg.

    // Otherwise, infer based on the language.
    language.default_units()
}

fn make_locale<'db>(
    langinfo: &'db LangInfo,
    languages: AcceptLanguage,
    cookies: &Cookies,
) -> Locale<'db> {
    let lang = select_display_language(languages, &cookies);
    let units = select_weight_units(lang, &cookies);
    Locale::new(&langinfo, lang, units)
}

/// A file served from /static.
enum StaticFile {
    /// PathBuf is the path to the non-gz version of the file.
    Gzipped(PathBuf, File),
    Plain(NamedFile),
}

impl Responder<'static> for StaticFile {
    fn respond_to(self, req: &Request) -> Result<Response<'static>, Status> {
        let mut response = match self {
            StaticFile::Gzipped(p, f) => {
                let mut r = f.respond_to(req)?;
                r.set_raw_header("Content-Encoding", "gzip");
                if let Some(ext) = p.extension() {
                    if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy())
                    {
                        r.set_header(ct);
                    }
                }
                r
            }
            StaticFile::Plain(f) => f.respond_to(req)?,
        };
        // Set to 1 year -- effectively forever.
        response.set_raw_header("Cache-Control", "public, max-age=31556926");
        Ok(response)
    }
}

#[get("/static/<file..>")]
fn statics(file: PathBuf, encoding: AcceptEncoding) -> Option<StaticFile> {
    let staticdir = env::var("STATICDIR").ok()?;
    let filepath = Path::new(&staticdir).join(&file);

    // Prefer returning a compressed variant (same filename plus ".gz").
    if encoding.supports_gzip() {
        let gzfilename = format!("{}.gz", file.file_name()?.to_str()?);
        let gzfilepath = filepath.with_file_name(gzfilename);
        if let Ok(gzfile) = File::open(gzfilepath) {
            return Some(StaticFile::Gzipped(filepath, gzfile));
        }
    }

    let namedfile = NamedFile::open(filepath).ok()?;
    Some(StaticFile::Plain(namedfile))
}

/// Actually store the favicon in static/images/,
/// but allow serving from the root.
#[get("/favicon.ico")]
fn root_favicon(encoding: AcceptEncoding) -> Option<StaticFile> {
    statics(PathBuf::from("images/favicon.ico"), encoding)
}

#[get("/apple-touch-icon.png")]
fn root_apple_touch_icon(encoding: AcceptEncoding) -> Option<StaticFile> {
    statics(PathBuf::from("images/apple-touch-icon.png"), encoding)
}

#[get("/rankings/<selections..>")]
fn rankings(
    selections: PathBuf,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let selection = pages::selection::Selection::from_path(&selections).ok()?;
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::rankings::Context::new(&opldb, &locale, &selection)?;
    Some(Template::render("rankings", &context))
}

#[get("/rankings")]
fn rankings_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/records/<selections..>")]
fn records(
    selections: Option<PathBuf>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let selection = if let Some(sel) = selections {
        pages::records::RecordsSelection::from_path(&sel).ok()?
    } else {
        pages::records::RecordsSelection::default()
    };
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::records::Context::new(&opldb, &locale, &selection);
    Some(Template::render("records", &context))
}

#[get("/records")]
fn records_default(
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    records(None, opldb, langinfo, languages, cookies)
}

#[get("/u/<username>")]
fn lifter(
    username: String,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Result<Template, Redirect>> {
    let lifter_id = match opldb.get_lifter_id(&username) {
        None => {
            // If the name just needs to be lowercased, redirect to that page.
            let lowercase = username.to_ascii_lowercase();
            let _guard = opldb.get_lifter_id(&lowercase)?;
            return Some(Err(Redirect::permanent(format!("/u/{}", lowercase))));
        }
        Some(id) => id,
    };

    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::lifter::Context::new(&opldb, &locale, lifter_id);
    Some(Ok(Template::render("lifter", &context)))
}

#[get("/mlist/<mselections..>")]
fn meetlist(
    mselections: Option<PathBuf>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let mselection = match mselections {
        None => pages::meetlist::MeetListSelection::default(),
        Some(p) => pages::meetlist::MeetListSelection::from_path(&p).ok()?,
    };
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::meetlist::Context::new(&opldb, &locale, &mselection);
    Some(Template::render("meetlist", &context))
}

#[get("/mlist")]
fn meetlist_default(
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    meetlist(None, opldb, langinfo, languages, cookies)
}

#[get("/m/<meetpath..>")]
fn meet(
    meetpath: PathBuf,
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
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::meet::Context::new(&opldb, &locale, meet_id, sort);
    Some(Template::render("meet", &context))
}

#[get("/status")]
fn status(
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::status::Context::new(&opldb, &locale);
    Some(Template::render("status", &context))
}

#[get("/data")]
fn data(
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::data::Context::new(&locale);
    Some(Template::render("data", &context))
}

#[get("/faq")]
fn faq(
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::faq::Context::new(&locale);
    Some(Template::render("faq", &context))
}

#[get("/contact")]
fn contact(
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::contact::Context::new(&locale);
    Some(Template::render("contact", &context))
}

#[get("/")]
fn index(
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let selection = pages::selection::Selection::default();
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::rankings::Context::new(&opldb, &locale, &selection);
    Some(Template::render("rankings", &context))
}

/// Return type for pre-rendered Json strings.
#[derive(Debug)]
struct JsonString(pub String);

impl Responder<'static> for JsonString {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        content::Json(self.0).respond_to(req)
    }
}

#[derive(FromForm)]
struct RankingsApiQuery {
    start: usize,
    end: usize,
    lang: String,
    units: String,
}

/// API endpoint for fetching a slice of rankings data as JSON.
#[get("/api/rankings/<selections..>?<query..>")]
fn rankings_api(
    selections: Option<PathBuf>,
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    let selection = match selections {
        None => pages::selection::Selection::default(),
        Some(path) => pages::selection::Selection::from_path(&path).ok()?,
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
fn default_rankings_api<'db>(
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    rankings_api(None, query, opldb, langinfo)
}

// TODO: Version / magicValue / etc.
#[derive(FromForm)]
struct SearchRankingsApiQuery {
    q: String,
    start: usize,
}

/// API endpoint for rankings search.
#[get("/api/search/rankings/<selections..>?<query..>")]
fn search_rankings_api<'db>(
    selections: Option<PathBuf>,
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    let selection = match selections {
        None => pages::selection::Selection::default(),
        Some(path) => pages::selection::Selection::from_path(&path).ok()?,
    };

    let result =
        pages::api_search::search_rankings(&opldb, &selection, query.start, &query.q);

    Some(JsonString(serde_json::to_string(&result).ok()?))
}

#[get("/api/search/rankings?<query..>")]
fn default_search_rankings_api(
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    search_rankings_api(None, query, opldb)
}

#[derive(FromForm)]
struct OldIndexQuery {
    fed: String,
}

#[get("/?<query..>")]
fn old_index_query(query: Form<OldIndexQuery>) -> Option<Redirect> {
    let fed = query.fed.parse::<Federation>().ok()?;
    let target = format!("/rankings/{}", fed.to_string().to_ascii_lowercase());
    Some(Redirect::permanent(target))
}

#[derive(FromForm)]
struct OldLiftersQuery {
    q: String,
}

#[get("/lifters.html?<query..>")]
fn old_lifters(
    opldb: State<ManagedOplDb>,
    query: Form<OldLiftersQuery>,
) -> Option<Redirect> {
    let name = &query.q;
    let id = opldb.get_lifter_id_by_name(name)?;
    let username = &opldb.get_lifter(id).username;
    Some(Redirect::permanent(format!("/u/{}", username)))
}

#[derive(FromForm)]
struct OldMeetQuery {
    m: String,
}

#[get("/meetlist.html")]
fn old_meetlist() -> Redirect {
    Redirect::permanent("/mlist")
}

#[get("/meet.html?<query..>")]
fn old_meet(opldb: State<ManagedOplDb>, query: Form<OldMeetQuery>) -> Option<Redirect> {
    let meetpath = &query.m;
    let id = opldb.get_meet_id(meetpath)?;
    let pathstr = &opldb.get_meet(id).path;
    Some(Redirect::permanent(format!("/m/{}", pathstr)))
}

#[get("/index.html")]
fn old_index() -> Redirect {
    Redirect::permanent("/")
}

#[get("/data.html")]
fn old_data() -> Redirect {
    Redirect::permanent("/data")
}

#[get("/faq.html")]
fn old_faq() -> Redirect {
    Redirect::permanent("/faq")
}

#[get("/contact.html")]
fn old_contact() -> Redirect {
    Redirect::permanent("/contact")
}

#[get("/robots.txt")]
fn robots_txt() -> &'static str {
    // Allow robots full site access except for JSON endpoints.
    "User-agent: *\nDisallow: /api/"
}

#[catch(404)]
fn not_found() -> &'static str {
    "404"
}

#[catch(500)]
fn internal_error() -> &'static str {
    "500"
}

// Tests want to load the data only once.
#[cfg(not(test))]
type ManagedOplDb = opldb::OplDb;
#[cfg(test)]
type ManagedOplDb = &'static opldb::OplDb;

#[cfg(not(test))]
type ManagedLangInfo = langpack::LangInfo;
#[cfg(test)]
type ManagedLangInfo = &'static langpack::LangInfo;

fn rocket(opldb: ManagedOplDb, langinfo: ManagedLangInfo) -> rocket::Rocket {
    // Initialize the server.
    rocket::ignite()
        .manage(opldb)
        .manage(langinfo)
        .mount(
            "/",
            routes![
                index,
                rankings,
                rankings_redirect,
                records,
                records_default,
                lifter,
                meetlist,
                meetlist_default,
                meet,
                statics,
                root_favicon,
                root_apple_touch_icon,
                status,
                data,
                faq,
                contact,
                robots_txt,
            ],
        )
        .mount(
            "/",
            routes![
                rankings_api,
                default_rankings_api,
                search_rankings_api,
                default_search_rankings_api
            ],
        )
        .mount(
            "/",
            routes![
                old_lifters,
                old_meetlist,
                old_meet,
                old_index,
                old_index_query,
                old_data,
                old_faq,
                old_contact,
            ],
        )
        .register(catchers![not_found, internal_error])
        .attach(Template::fairing())
        .attach(AdHoc::on_response(
            "Delete Server Header",
            |_request, response| {
                response.remove_header("Server");
            },
        ))
}

fn load_langinfo() -> Result<LangInfo, Box<Error>> {
    let mut langinfo = langpack::LangInfo::default();
    for language in Language::iter() {
        let path = format!("translations/{}.json", language);
        langinfo.load_translations(language, &path)?;
    }
    Ok(langinfo)
}

fn main() -> Result<(), Box<Error>> {
    // Accept an optional "--set-cwd" argument to manually specify the
    // current working directory. This allows the binary and the data
    // to be separated on a production server.
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "--set-cwd" {
        let fileroot = Path::new(&args[2]);
        env::set_current_dir(&fileroot).expect("Invalid --set-cwd argument");
    }

    // Populate std::env with the contents of any .env file.
    dotenv::from_filename("server.env").expect("Couldn't find server.env");

    // Ensure that "STATICDIR" is set.
    env::var("STATICDIR").expect("STATICDIR envvar not set");

    // Load the OplDb.
    let lifters_csv = env::var("LIFTERS_CSV").expect("LIFTERS_CSV not set");
    let meets_csv = env::var("MEETS_CSV").expect("MEETS_CSV not set");
    let entries_csv = env::var("ENTRIES_CSV").expect("ENTRIES_CSV not set");
    let opldb = opldb::OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv)?;
    println!("OplDb loaded in {}MB.", opldb.size_bytes() / 1024 / 1024);

    #[allow(unused_variables)]
    let langinfo = load_langinfo()?;

    #[cfg(not(test))]
    rocket(opldb, langinfo).launch();
    Ok(())
}
