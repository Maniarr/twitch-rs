use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::responses::*;
use crate::{TwitchApi, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct HelixStream {
    pub id: String,
    pub user_id: String,
    pub user_name: String,
    pub game_id: String,
    pub r#type: String,
    pub title: String,
    pub viewer_count: i32,
    pub started_at: String,
    pub language: String,
    pub thumbnail_url: String
}

impl super::traits::HelixModel for HelixStream {}

#[derive(Debug)]
pub struct StreamFilters {
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<u64>,
    pub game_ids: Vec<String>,
    pub languages: Vec<String>,
    pub user_ids: Vec<String>,
    pub user_logins: Vec<String>,

}

pub async fn get(twitch_api: &TwitchApi, filters: StreamFilters) -> Result<HelixPaginatedResponse<HelixStream>> {
    let mut data: Vec<(&str, String)> = vec![];

    if let Some(value) = filters.after {
        data.push(("after", value));
    }

    if let Some(value) = filters.before {
        data.push(("before", value));
    }

    if let Some(value) = filters.first {
        data.push(("first", value.to_string()));
    }

    for game_id in filters.game_ids {
        data.push(("game_id", game_id));
    }

    for language in filters.languages {
        data.push(("language", language));
    }

    for user_id in filters.user_ids {
        data.push(("user_id", user_id));
    }

    for user_login in filters.user_logins {
        data.push(("user_login", user_login));
    }

    Ok(
        serde_json::from_str(
            &twitch_api.get(String::from("https://api.twitch.tv/helix/streams"), &data)
                .await?
                .text()
                .await?[..]
        )?
    )
}
