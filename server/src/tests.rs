//! Tests for the Rocket code in main.rs.

use super::dotenv;
use super::rocket;

use server::langpack::{LangInfo, Language};
use server::opldb::OplDb;

use rocket::http::{Cookie, Header, Status};
use rocket::local::Client;

use std::sync::{Once, ONCE_INIT};

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = ONCE_INIT;

fn db() -> &'static OplDb {
    const LIFTERS_CSV: &str = "../build/lifters.csv";
    const MEETS_CSV: &str = "../build/meets.csv";
    const ENTRIES_CSV: &str = "../build/entries.csv";

    unsafe {
        OPLDB_INIT.call_once(|| {
            // This isn't really the place for it, but preload the environment.
            dotenv::from_filename("server.env").unwrap();

            OPLDB_GLOBAL =
                Some(OplDb::from_csv(LIFTERS_CSV, MEETS_CSV, ENTRIES_CSV).unwrap());
        });

        OPLDB_GLOBAL.as_ref().unwrap()
    }
}

static mut LANGINFO_GLOBAL: Option<LangInfo> = None;
static LANGINFO_INIT: Once = ONCE_INIT;

fn langinfo() -> &'static LangInfo {
    unsafe {
        LANGINFO_INIT.call_once(|| {
            LANGINFO_GLOBAL = Some(super::load_langinfo().unwrap());
        });
        LANGINFO_GLOBAL.as_ref().unwrap()
    }
}

/// Returns a client's view into the Rocket server, suitable for making
/// requests.
fn client() -> Client {
    Client::new(rocket(db(), langinfo())).expect("valid rocket instance")
}

#[test]
fn test_db_loads() {
    db();
    langinfo();
}

#[test]
fn test_pages_load() {
    let client = client();
    assert_eq!(client.get("/").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/rankings/uspa").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/records").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/records/uspa").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/u/seanstangl").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/mlist").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/m/uspa/0485").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/status").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/data").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/faq").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/contact").dispatch().status(), Status::Ok);
}

/// Some rankings pages that contain only a few entries have
/// historically produced crashes, when the context-generating
/// code assumes a minimum entry count.
#[test]
fn test_small_rankings_pages() {
    let client = client();
    // The BB federation is small and defunct, therefore good for testing.
    assert_eq!(
        client.get("/rankings/44/bb").dispatch().status(),
        Status::Ok
    );
}

/// Test that meet pages load with different sorts.
#[test]
fn test_meet_pages_with_explicit_sorts() {
    let client = client();
    assert_eq!(client.get("/m/wrpf/bob4").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/m/wrpf/bob4/by-glossbrenner").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/m/wrpf/bob4/by-division").dispatch().status(), Status::Ok);
}

#[test]
fn test_username_redirects() {
    let client = client();
    let response = client.get("/u/TrystanOakley").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert!(response.headers().contains("location"));
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/u/trystanoakley"
    );
}

/// Test that URL patterns from the old web/ implementation are redirected
/// to their proper server/ equivalents.
#[test]
fn test_old_redirects() {
    let client = client();

    let response = client.get("/lifters.html?q=Sean Stangl").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/u/seanstangl"
    );

    let response = client.get("/lifters.html?q=Sean%20Stangl").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/u/seanstangl"
    );

    let response = client.get("/meet.html?m=rps/1617").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/m/rps/1617"
    );

    let response = client.get("/?fed=USPA").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/rankings/uspa"
    );

    let response = client.get("/?fed=365Strong").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/rankings/365strong"
    );

    let response = client.get("/index.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/");

    let response = client.get("/meetlist.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/mlist");

    let response = client.get("/data.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/data");

    let response = client.get("/faq.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/faq");

    let response = client.get("/contact.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/contact");
}

#[test]
fn test_no_server_header() {
    // By default, the Rocket server serves a response header
    // "Server: Rocket". But it's unnecessary and an information leak.
    let client = client();
    let response = client.get("/").dispatch();
    assert!(!response.headers().contains("Server"));
}

/// Files served from "/static" should be served with the "Cache-Control"
/// header, to prevent them from being constantly reloaded.
#[test]
fn test_static_cache_control() {
    let client = client();
    let response = client.get("/static/images/favicon.ico").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.headers().contains("Cache-Control"));
    let cache_control = response.headers().get_one("Cache-Control").unwrap();
    assert!(cache_control.contains("max-age="));
}

/// Tests that the Accept-Language HTTP header can determine the language.
#[test]
fn test_accept_language_header() {
    // Iterate through all languages and ensure they are handled.
    for language in Language::string_list() {
        let content = format!("<html lang=\"{}\"", &language);
        let client = client();
        let mut res = client
            .get("/")
            .header(Header::new("Accept-Language", language))
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert!(res.body_string().unwrap().contains(&content));
    }

    // The "lang" cookie should override Accept-Language.
    let client = client();
    let mut res = client
        .get("/")
        .header(Header::new("Accept-Language", "ru"))
        .cookie(Cookie::new("lang", "eo"))
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.body_string().unwrap().contains("<html lang=\"eo\""));
}

/// Setting the "lang" cookie should change the text language,
/// via the HTML5 html "lang" tag.
#[test]
fn test_language_cookie() {
    let client = client();
    let lang_cookie = Cookie::new("lang", "ru");
    let mut res = client.get("/").cookie(lang_cookie).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.body_string().unwrap().contains("<html lang=\"ru\""));
}

/// A nonsense "lang" cookie value should still render OK (with the English
/// default).
#[test]
fn test_language_cookie_nonsense() {
    let client = client();
    let lang_cookie = Cookie::new("lang", "fgsfds");
    let mut res = client.get("/").cookie(lang_cookie).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.body_string().unwrap().contains("<html lang=\"en\""));
}

/// Test that some nonsensical rankings options don't crash the server.
#[test]
fn test_rankings_nonsense() {
    let client = client();
    assert_eq!(
        client.get("/rankings/push-pull/by-squat").dispatch().status(),
        Status::Ok
    );
}
