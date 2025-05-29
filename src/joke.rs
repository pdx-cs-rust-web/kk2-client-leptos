// This code originally borrowed from the leptos crate
// examples, where variants appear throughout.

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

pub fn fetch(
    path: &str,
) -> impl std::future::Future<Output = Joke> + Send + '_ {
    use leptos::prelude::on_cleanup;
    use send_wrapper::SendWrapper;

    SendWrapper::new(async move {
        let abort_controller =
            SendWrapper::new(web_sys::AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        // abort in-flight requests if, e.g., we've navigated away from this page
        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort()
            }
        });

        let path = format!("http://localhost:3000/api/v1/{path}");
        gloo_net::http::Request::get(&path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    })
}
