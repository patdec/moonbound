use chrono::Local;
use leptos::*;

use crate::model::blog_post::Post;

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String
) -> Result<String, ServerFnError> {
    Ok(String::from("placeholder"))
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    Ok(
        Post {
            id: "1".to_string(),
            dt: Local::now().naive_local(),
            title: "Ocean".to_string(),
            image_url: "https://bit.ly/adw34".to_string(),
            text: "I spent some time at the beach today".to_string(),
        }
    )
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8
) -> Result<Vec<Post, ServerFnError> {
    Ok(
        vec!(
            Post {
                id: "1".to_string(),
                dt: Local::now().naive_local(),
                title: "Ocean".to_string(),
                image_url: "https://bit.ly/adw34".to_string(),
                text: "I spent some time at the beach today".to_string(),
            },
            Post {
                id: "2".to_string(),
                dt: Local::now().naive_local(),
                title: "Desert".to_string(),
                image_url: "https://bit.ly/adw34".to_string(),
                text: "I spent some time in the desert today".to_string(),
            }
            Post {
                id: "3".to_string(),
                dt: Local::now().naive_local(),
                title: "Garden".to_string(),
                image_url: "https://bit.ly/adw34".to_string(),
                text: "I spent some time in the garden today".to_string(),
            }
        )
    )
}
