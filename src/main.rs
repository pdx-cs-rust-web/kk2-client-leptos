mod fetch_api;

use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| set_count.set(3)
        >
            "Get Joke"
            {count}
        </button>
        <p>
        {move || fetch_api::fetch_api("http://localhost:3000/api/v1/random-joke")}
        </p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
