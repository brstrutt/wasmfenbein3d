use crate::dot::{Dot, WorldDot};
use leptos::{ev, prelude::*};

#[component]
pub fn App() -> impl IntoView {
    let dots = RwSignal::new(vec![Dot::new(0)]);
    let next_dot_id = RwSignal::new(1usize);
    let active_dot = RwSignal::new(None::<usize>);
    let grab_offset = RwSignal::new((0.0, 0.0));

    let move_listener = window_event_listener(ev::pointermove, move |event| {
        if let Some(active_id) = active_dot.get_untracked() {
            let (grab_x, grab_y) = grab_offset.get_untracked();
            let position = (
                f64::from(event.client_x()) - grab_x,
                f64::from(event.client_y()) - grab_y,
            );

            dots.with_untracked(|dots| {
                if let Some(dot) = dots.iter().find(|dot| dot.id == active_id) {
                    dot.set_position(position);
                }
            });
        }
    });
    let up_listener = window_event_listener(ev::pointerup, move |_| active_dot.set(None));
    let cancel_listener = window_event_listener(ev::pointercancel, move |_| active_dot.set(None));

    on_cleanup(move || {
        move_listener.remove();
        up_listener.remove();
        cancel_listener.remove();
    });

    let add_dot = move |_| {
        let id = next_dot_id.get_untracked();
        next_dot_id.set(id + 1);
        dots.update(|dots| dots.push(Dot::new(id)));
    };

    view! {
        <main class="workspace">
            <header class="workspace-heading">
                <h1>"World Editor"</h1>
                <button class="add-dot" type="button" on:click=add_dot>
                    "+ Add dot"
                </button>
            </header>
            <For
                each=move || dots.get()
                key=|dot| dot.id
                children=move |dot| view! {
                    <WorldDot dot=dot active_dot=active_dot grab_offset=grab_offset />
                }
            />
        </main>
    }
}
