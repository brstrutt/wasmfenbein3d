mod app;
mod dot;
mod line;

use leptos::{mount::mount_to_body, prelude::*};

fn main() {
    mount_to_body(|| view! { <app::App/> });
}
