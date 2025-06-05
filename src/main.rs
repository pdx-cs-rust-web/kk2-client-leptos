mod joke;

use std::collections::HashSet;
use leptos::prelude::*;

fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

fn fetch_joke() -> impl IntoView {
    let (endpoint, set_endpoint) = signal::<String>("random-joke".to_string());
    let joke = LocalResource::new(move || joke::fetch(endpoint.get()));

    let error_fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect::<Vec<_>>()
            })
        };

        view! {
            <div>
                <h2>"Error"</h2>
                <span class="error">{error_list}</span>
            </div>
        }
    };

    view! {
        <div><Transition fallback=|| view! { <div>"Loading..."</div> }>
            <ErrorBoundary fallback=error_fallback>
                {move || Suspend::new( async move {
                    joke.map(|j| {
                        // XXX Don't know how to fix this unwrap() yet.
                        let j = j.as_ref().unwrap();
                        view! {
                            <div class="joke">
                                <span class="teller">{"Knock-Knock!"}</span><br/>
                                <span class="tellee">{"Who's there?"}</span><br/>
                                <span class="teller">{j.whos_there.clone()}</span><br/>
                                <span class="tellee">{format!("{} who?", j.whos_there)}</span><br/>
                                <span class="teller">{j.answer_who.clone()}</span>
                            </div>
                            <span class="annotation">
                                {format!(
                                    "[id: {}; tags: {}; source: {}]",
                                    j.id,
                                    format_tags(&j.tags),
                                    j.source,
                                )}
                            </span>
                        }
                    })
                })}
            </ErrorBoundary>
        </Transition></div>
        <div>
            <button on:click=move |_| {
                let ep = "random-joke".to_string();
                set_endpoint.set(ep)
            }>Tell me another!</button>
            // XXX here
            <input foo=move || {
                let ep = format!("jokes/{}", joke_id.target().value());
                eprintln!("{}", ep);
                set_endpoint.set(ep);
            } />
        </div>
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
