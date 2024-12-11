#[cfg(feature = "ssr")]
use sqlx::types::chrono::{Local, NaiveDateTime};
#[cfg(feature = "ssr")]
use sqlx::FromRow;
#[cfg(feature = "hydrate")]
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(Clone, Serialize, Deserialize, Debug, FromRow))]
#[cfg_attr(feature = "hydrate", derive(Clone, Serialize, Deserialize, Debug))]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn new_empty() -> Self {
        Post {
            id: "".to_string(),
            dt: Local::now().naive_local(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
} 