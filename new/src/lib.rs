use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="page">
            <header class="hero">
                <p class="eyebrow">"Alexey Belyakov — Engineering Manager"</p>
                <h1>"Next-generation CV Experience"</h1>
                <p class="lede">
                    "This WebAssembly preview experiments with an interactive, client-rendered version of the CV."
                </p>
            </header>

            <section class="grid">
                <article class="card">
                    <h2>"Performance-ready"
                    </h2>
                    <p>"Leptos and WebAssembly deliver a fast, static bundle ready for GitHub Pages under `/new`."</p>
                </article>

                <article class="card">
                    <h2>"Offline-friendly"
                    </h2>
                    <p>"Static assets and a compact wasm module keep the experience available even on flaky networks."</p>
                </article>

                <article class="card">
                    <h2>"Stay in sync"
                    </h2>
                    <p>
                        "Content remains aligned with the canonical Markdown sources while we iterate on richer interactions."
                    </p>
                </article>
            </section>
        </main>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| view! { <App/> });
}
