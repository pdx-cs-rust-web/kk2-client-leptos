mod joke;

use std::collections::HashSet;

use leptos::prelude::*;

fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

fn fetch_joke() -> impl IntoView {
    let joke = LocalResource::new(move || joke::fetch("random-joke"));
    view! {
        <Transition fallback=|| view! { <div>"Loading..."</div> }> {
            move || Suspend::new( async move {
                joke.map(|joke| view! {
                    <div class="joke">
                        <span class="teller">{"Knock-Knock!"}</span><br/>
                        <span class="tellee">{"Who's there?"}</span><br/>
                        <span class="teller">{joke.whos_there.clone()}</span><br/>
                        <span class="tellee">{format!("{} who?", &joke.whos_there)}</span><br/>
                        <span class="teller">{joke.answer_who.clone()}</span>
                    </div>
                    <span class="annotation">
                        {format!("[id: {}", &joke.id)}
                        {format!("; tags: {}", &format_tags(&joke.tags))}
                        {format!("; source: {}", &joke.source)}
                        {"]"}
                    </span>
                })
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
