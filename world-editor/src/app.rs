use crate::dot::{Dot, WorldDot};
use crate::line::{Line, WorldLine};
use leptos::{ev, prelude::*};

#[component]
pub fn App() -> impl IntoView {
    let dots = RwSignal::new(vec![Dot::new(0)]);
    let lines = RwSignal::new(Vec::<Line>::new());
    let next_dot_id = RwSignal::new(1usize);
    let next_line_id = RwSignal::new(0usize);
    let active_dot = RwSignal::new(None::<usize>);
    let active_endpoint = RwSignal::new(None::<(usize, bool)>);
    let endpoint_drag_start = RwSignal::new(None::<(f64, f64, f64, f64)>);
    let grab_offset = RwSignal::new((0.0, 0.0));
    let pan_start = RwSignal::new(None::<(f64, f64, f64, f64)>);
    let scene_offset = RwSignal::new((0.0, 0.0));

    let move_listener = window_event_listener(ev::pointermove, move |event| {
        if let Some(active_id) = active_dot.get_untracked() {
            let (grab_x, grab_y) = grab_offset.get_untracked();
            let (pan_x, pan_y) = scene_offset.get_untracked();
            let position = (
                f64::from(event.client_x()) - grab_x - pan_x,
                f64::from(event.client_y()) - grab_y - pan_y,
            );

            dots.with_untracked(|dots| {
                if let Some(dot) = dots.iter().find(|dot| dot.id == active_id) {
                    dot.set_position(position);
                }
            });
        } else if let (Some((line_id, is_start)), Some((pointer_x, pointer_y, x, y))) = (
            active_endpoint.get_untracked(),
            endpoint_drag_start.get_untracked(),
        ) {
            let position = (
                x + f64::from(event.client_x()) - pointer_x,
                y + f64::from(event.client_y()) - pointer_y,
            );

            lines.with_untracked(|lines| {
                if let Some(line) = lines.iter().find(|line| line.id == line_id) {
                    line.set_endpoint(is_start, position);
                }
            });
        } else if let Some((start_x, start_y, offset_x, offset_y)) = pan_start.get_untracked() {
            scene_offset.set((
                offset_x + f64::from(event.client_x()) - start_x,
                offset_y + f64::from(event.client_y()) - start_y,
            ));
        }
    });
    let stop_drag = move |_| {
        active_dot.set(None);
        active_endpoint.set(None);
        endpoint_drag_start.set(None);
        pan_start.set(None);
    };
    let up_listener = window_event_listener(ev::pointerup, stop_drag);
    let cancel_listener = window_event_listener(ev::pointercancel, stop_drag);

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
    let add_line = move |_| {
        let id = next_line_id.get_untracked();
        next_line_id.set(id + 1);
        lines.update(|lines| lines.push(Line::new(id)));
    };

    view! {
        <main class="workspace">
            <header class="workspace-heading">
                <h1>"World Editor"</h1>
                <div class="workspace-actions">
                    <button class="add-action add-dot" type="button" on:click=add_dot>
                        "+ Add dot"
                    </button>
                    <button class="add-action add-line" type="button" on:click=add_line>
                        "+ Add line"
                    </button>
                </div>
            </header>
            <div
                class="world-scene"
                class:panning=move || pan_start.get().is_some()
                style:background-position=move || {
                    let (x, y) = scene_offset.get();
                    format!("calc(50% + {x}px) calc(50% + {y}px)")
                }
                on:pointerdown=move |event: ev::PointerEvent| {
                    event.prevent_default();
                    let (offset_x, offset_y) = scene_offset.get_untracked();
                    pan_start.set(Some((
                        f64::from(event.client_x()),
                        f64::from(event.client_y()),
                        offset_x,
                        offset_y,
                    )));
                }
            >
                <For
                    each=move || dots.get()
                    key=|dot| dot.id
                    children=move |dot| view! {
                        <WorldDot
                            dot=dot
                            active_dot=active_dot
                            grab_offset=grab_offset
                            scene_offset=scene_offset
                        />
                    }
                />
                <For
                    each=move || lines.get()
                    key=|line| line.id
                    children=move |line| view! {
                        <WorldLine
                            line=line
                            active_endpoint=active_endpoint
                            endpoint_drag_start=endpoint_drag_start
                            scene_offset=scene_offset
                        />
                    }
                />
            </div>
        </main>
    }
}
