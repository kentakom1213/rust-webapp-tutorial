use crate::response;
use crate::views::{Home, Tweet};
use axum::{
    extract::Form,
    response::IntoResponse,
    routing,
    Router,
};
use serde::Deserialize;

pub fn tweets() -> Router {
    Router::new().route("/new", routing::post(post))
}

async fn post(form: Form<TweetForm>) -> impl IntoResponse {
    let tweets = vec![Tweet {
        name: "太郎".to_string(),
        message: form.message.clone(),
        posted_at: "2022-01-01".to_string(),
    }];
    let home = Home { tweets };
    response::from_template(home)
}

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}
