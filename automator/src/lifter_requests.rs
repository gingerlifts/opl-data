use axum::extract::Json;
use chrono::naive::NaiveDate;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LifterRequest {
    name: String,
    instagram: Option<String>,
    date_of_birth: Option<NaiveDate>,
}

pub async fn handler(Json(request): Json<LifterRequest>) -> &'static str {
    tracing::info!("Got a lifter request: {:?}", request);

    let LifterRequest {
        name,
        instagram,
        date_of_birth,
    } = request;

    // Validate that we have either an Instagram or a date of birth at least
    if instagram.is_none() && date_of_birth.is_none() {
        return "That wasn't very nice of you.";
    }

    "Yeah, I'll sort that for you!"
}
