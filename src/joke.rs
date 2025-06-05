// This code originally borrowed from the leptos crate
// examples, where variants appear throughout.

use leptos::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Joke {
    pub id: String,
    pub whos_there: String,
    pub answer_who: String,
    pub tags: HashSet<String>,
    pub source: String,
}


pub async fn fetch(endpoint: &str) -> Result<Joke, Error> {
    let result = reqwasm::http::Request::get(&format!(
        "http://localhost:3000/api/v1/{}",
        endpoint,
    ))
        .send()
        .await?
        // convert it to JSON
        .json()
        .await?;
    Ok(result)
}
