use leptos::prelude::*;

#[component]
pub fn JsonWindow() -> impl IntoView {
    let minimized = RwSignal::new(false);

    view! {
        <aside class="json-window" aria-labelledby="json-window-title">
            <header class="json-window-heading">
                <div>
                    <p>"Line data"</p>
                    <h2 id="json-window-title">"JSON"</h2>
                </div>
                <button
                    class="json-window-toggle"
                    type="button"
                    aria-label=move || {
                        if minimized.get() {
                            "Expand JSON window"
                        } else {
                            "Minimize JSON window"
                        }
                    }
                    title=move || {
                        if minimized.get() {
                            "Expand JSON window"
                        } else {
                            "Minimize JSON window"
                        }
                    }
                    aria-expanded=move || (!minimized.get()).to_string()
                    on:click=move |_| minimized.update(|minimized| *minimized = !*minimized)
                >
                    {move || if minimized.get() { "+" } else { "-" }}
                </button>
            </header>
            <textarea
                class="json-textarea"
                aria-label="Line positions JSON"
                placeholder="Paste a JSON object here..."
                spellcheck="false"
                hidden=move || minimized.get()
            />
        </aside>
    }
}
