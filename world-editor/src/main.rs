use leptos::{ev, mount::mount_to_body, prelude::*};

#[component]
fn App() -> impl IntoView {
    let (is_dragging, set_is_dragging) = signal(false);
    let (position, set_position) = signal(None::<(f64, f64)>);
    let (grab_offset, set_grab_offset) = signal((0.0, 0.0));

    let move_listener = window_event_listener(ev::pointermove, move |event| {
        if is_dragging.get_untracked() {
            let (grab_x, grab_y) = grab_offset.get_untracked();
            set_position.set(Some((
                f64::from(event.client_x()) - grab_x,
                f64::from(event.client_y()) - grab_y,
            )));
        }
    });
    let stop_drag = move |_| set_is_dragging.set(false);
    let up_listener = window_event_listener(ev::pointerup, stop_drag);
    let cancel_listener = window_event_listener(ev::pointercancel, stop_drag);

    on_cleanup(move || {
        move_listener.remove();
        up_listener.remove();
        cancel_listener.remove();
    });

    view! {
        <main class="workspace">
            <header class="workspace-heading">
                <h1>"World Editor"</h1>
            </header>
            <button
                class="world-dot"
                class:dragging=is_dragging
                type="button"
                aria-label="World marker"
                style:left=move || position.get().map_or_else(
                    || "calc(50% - 13px)".to_owned(),
                    |(x, _)| format!("{x}px"),
                )
                style:top=move || position.get().map_or_else(
                    || "calc(50% - 13px)".to_owned(),
                    |(_, y)| format!("{y}px"),
                )
                on:pointerdown=move |event: ev::PointerEvent| {
                    event.prevent_default();
                    set_is_dragging.set(true);
                    set_grab_offset((
                        f64::from(event.offset_x()),
                        f64::from(event.offset_y()),
                    ));
                }
            />
        </main>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
