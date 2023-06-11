use axum::extract::Json;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "event_name")]
#[serde(rename_all = "lowercase")]
pub enum Webhook {
    Push(PushEvent),
}

#[derive(Clone, Debug, Deserialize)]
pub struct PushEvent {
    before: String,
    after: String,
    #[serde(rename = "ref")]
    reference: String,
    total_commits_count: u32,
}

pub async fn handler(Json(event): Json<Webhook>) -> &'static str {
    tracing::info!("Got a webhook event: {:?}", event);

    "Thanks"
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::Webhook;

    #[test]
    fn can_deserialize_from_gitlab_example() -> Result<()> {
        let raw_json = include_str!("../assets/example-gitlab-push-event.json");
        let deserialized: Webhook = serde_json::from_str(raw_json)?;
        let Webhook::Push(push_event) = deserialized;

        assert_eq!(
            push_event.before,
            "95790bf891e76fee5e1747ab589903a6a1f80f22"
        );

        assert_eq!(push_event.after, "da1560886d4f094c3e6c9ef40349f7d38b5d27d7");

        assert_eq!(push_event.reference, "refs/heads/master");
        assert_eq!(push_event.total_commits_count, 4);

        Ok(())
    }
}
