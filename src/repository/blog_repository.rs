use crate::model::blog_post::Post;
// use std::sync::Arc;
#[cfg(feature = "ssr")]
use crate::shared::moonbound::ssr::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
use leptos::*;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
use prelude::ServerFnError;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "ssr")]
use uuid::Uuid;


#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String
) -> Result<String, ServerFnError> {
    // let pool: Arc<Pool<Sqlite>> = extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;
    let mut conn = db().await?;

    use uuid::Uuid;
    let id = id.unwrap_or(Uuid::new_v4().to_string());
    sqlx::query("INSERT INTO post VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (id) DO UPDATE SET dt=excluded.dt,
                    image_url=excluded.image_url,
                    title=excluded.title,
                    text=excluded.text
    ")
    .bind(&id)
    .bind(&dt)
    .bind(&image_url)
    .bind(&title)
    .bind(&text)
    .execute(&mut conn)
    .await.expect("Error".into());
    Ok(id)
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    // let pool: Arc<Pool<Sqlite>> = extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;
    let mut conn = db().await?;
    let res: Post = sqlx::query_as("SELECT * FROM post WHERE id = ?")
    .bind(id)
    .fetch_one(&mut  conn)
    .await.expect("Error".into());
    // .map_err(|_| "error getting post".to_owned())?;
    
    Ok(res)
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8
) -> Result<Vec<Post>, ServerFnError> {
    Ok(
        vec![
            // Post {
            //     id: "1".to_string(),
            //     dt: Local::now().naive_local(),
            //     title: "Ocean".to_string(),
            //     image_url: "https://bit.ly/adw34".to_string(),
            //     text: "I spent some time at the beach today".to_string(),
            // },
            // Post {
            //     id: "2".to_string(),
            //     dt: Local::now().naive_local(),
            //     title: "Desert".to_string(),
            //     image_url: "https://bit.ly/adw34".to_string(),
            //     text: "I spent some time in the desert today".to_string(),
            // }
            // Post {
            //     id: "3".to_string(),
            //     dt: Local::now().naive_local(),
            //     title: "Garden".to_string(),
            //     image_url: "https://bit.ly/adw34".to_string(),
            //     text: "I spent some time in the garden today".to_string(),
            // }
        ]
    )
}
