mod joke;

use leptos::prelude::*;

fn fetch_joke() -> impl IntoView {
    let joke = LocalResource::new(move || joke::fetch("random-joke"));
    view! {
        <Transition fallback=|| view! { <div>"Loading..."</div> }> {
            move || Suspend::new( async move {
                joke.map(|joke| view! { <p> {joke.id.to_string()} </p> })
            })}
        </Transition>
    }
}

pub fn main() {
    use tracing_subscriber::fmt;
    use tracing_subscriber_wasm::MakeConsoleWriter;

    fmt()
        .with_writer(
            // To avoid trace events in the browser from showing their
            // JS backtrace, which is very annoying, in my opinion
            MakeConsoleWriter::default()
                .map_trace_level_to(tracing::Level::DEBUG),
        )
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .init();
    console_error_panic_hook::set_once();
    mount_to_body(fetch_joke)
}
