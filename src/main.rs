mod joke;

use leptos::prelude::*;

#[component]
async fn App() -> impl IntoView {
    let joke = joke::fetch("random-joke").await;
    view! {
        <p>
        {joke.id}
        </p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
