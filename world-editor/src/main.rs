use leptos::mount::mount_to_body;
use leptos::prelude::ElementChild;
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"World Editor"</h1>
            <p>"Your world editing workspace is ready."</p>
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
