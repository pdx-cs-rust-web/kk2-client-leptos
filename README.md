# kk2-client-leptos: A Leptos/WASM Client for knock-knock-2
Bart Massey 2025

This is a browser-based frontend for the
[knock-knock-2](../knock-knock-2) joke server. It is built with
[Leptos](https://leptos.dev) in client-side rendering (CSR)
mode and compiled to WebAssembly.

It connects to the `knock-knock-2` REST API at
`http://localhost:3000/api/v1/` to fetch and display
knock-knock jokes.

## Prerequisites

Install [trunk](https://trunkrs.dev) and the WASM target:

    cargo install trunk
    rustup target add wasm32-unknown-unknown

## Build and Run

Start the `knock-knock-2` backend first (see its README),
then in this directory:

    trunk serve

Open `http://localhost:8080` in your browser. The page
displays a random joke on load. Use **Tell me another!** for
a new random joke, or enter a joke ID in the search box and
press Enter to look up a specific one.

## How It Fits Together

```
Browser  <-->  kk2-client-leptos (localhost:8080)
                        |
                        | HTTP/JSON
                        v
              knock-knock-2 (localhost:3000)
                        |
                        | SQLite
                        v
                  knock-knock.db
```

`kk2-client-leptos` is a static WASM app served by trunk's
development server. It makes fetch requests directly from
the browser to the `knock-knock-2` Axum backend. No
server-side rendering is involved.

## License

This work is made available under the "Apache 2.0 or MIT
License". See the file `LICENSE.txt` in this distribution
for license terms.
