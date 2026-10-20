use leptos::{ev, mount::mount_to_body, prelude::*};

#[derive(Clone, Copy)]
struct Dot {
    id: usize,
    position: RwSignal<Option<(f64, f64)>>,
}

#[component]
fn App() -> impl IntoView {
    let dots = RwSignal::new(vec![Dot {
        id: 0,
        position: RwSignal::new(None),
    }]);
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
                    dot.position.set(Some(position));
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
        dots.update(|dots| {
            dots.push(Dot {
                id,
                position: RwSignal::new(None),
            });
        });
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

#[component]
fn WorldDot(
    dot: Dot,
    active_dot: RwSignal<Option<usize>>,
    grab_offset: RwSignal<(f64, f64)>,
) -> impl IntoView {
    let id = dot.id;
    let position = dot.position;

    view! {
        <button
            class="world-dot"
            class:dragging=move || active_dot.get() == Some(id)
            type="button"
            aria-label="World marker"
            style:left=move || position.get().map_or_else(
                || format!("calc(50% + {}px - 13px)", id * 28),
                |(x, _)| format!("{x}px"),
            )
            style:top=move || position.get().map_or_else(
                || "calc(50% - 13px)".to_owned(),
                |(_, y)| format!("{y}px"),
            )
            on:pointerdown=move |event: ev::PointerEvent| {
                event.prevent_default();
                grab_offset.set((
                    f64::from(event.offset_x()),
                    f64::from(event.offset_y()),
                ));
                active_dot.set(Some(id));
            }
        />
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
