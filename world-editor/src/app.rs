use crate::dot::{Dot, WorldDot};
use crate::line::{Line, WorldLine};
use leptos::{ev, prelude::*};

const MIN_ZOOM: f64 = 0.25;
const MAX_ZOOM: f64 = 4.0;
const ZOOM_STEP: f64 = 1.2;
const SNAP_DISTANCE: f64 = 14.0;

fn change_zoom(zoom: RwSignal<f64>, factor: f64) {
    zoom.update(|zoom| *zoom = (*zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM));
}

fn snap_delta(
    candidates: &[(f64, f64)],
    lines: &[Line],
    moving_line_id: usize,
    zoom: f64,
) -> Option<(f64, f64)> {
    let mut closest: Option<(f64, f64, f64)> = None;
    let max_distance_squared = SNAP_DISTANCE * SNAP_DISTANCE;

    for &(x, y) in candidates {
        for line in lines.iter().filter(|line| line.id != moving_line_id) {
            let (start, end) = line.endpoints();
            for (target_x, target_y) in [start, end] {
                let delta_x = target_x - x;
                let delta_y = target_y - y;
                let distance_squared = (delta_x * delta_x + delta_y * delta_y) * zoom * zoom;

                if distance_squared <= max_distance_squared
                    && closest.map_or(true, |(_, _, distance)| distance_squared < distance)
                {
                    closest = Some((delta_x, delta_y, distance_squared));
                }
            }
        }
    }

    closest.map(|(delta_x, delta_y, _)| (delta_x, delta_y))
}

#[component]
pub fn App() -> impl IntoView {
    let dots = RwSignal::new(vec![Dot::new(0)]);
    let lines = RwSignal::new(Vec::<Line>::new());
    let next_dot_id = RwSignal::new(1usize);
    let next_line_id = RwSignal::new(0usize);
    let active_dot = RwSignal::new(None::<usize>);
    let active_endpoint = RwSignal::new(None::<(usize, bool)>);
    let active_line = RwSignal::new(None::<usize>);
    let dot_drag_start = RwSignal::new(None::<(f64, f64, f64, f64)>);
    let endpoint_drag_start = RwSignal::new(None::<(f64, f64, f64, f64)>);
    let line_drag_start = RwSignal::new(None::<(f64, f64, f64, f64, f64, f64)>);
    let pan_start = RwSignal::new(None::<(f64, f64, f64, f64)>);
    let scene_offset = RwSignal::new((0.0, 0.0));
    let zoom = RwSignal::new(1.0_f64);

    let move_listener = window_event_listener(ev::pointermove, move |event| {
        if let Some(active_id) = active_dot.get_untracked() {
            if let Some((pointer_x, pointer_y, x, y)) = dot_drag_start.get_untracked() {
                let scale = zoom.get_untracked();
                let position = (
                    x + (f64::from(event.client_x()) - pointer_x) / scale,
                    y + (f64::from(event.client_y()) - pointer_y) / scale,
                );

                dots.with_untracked(|dots| {
                    if let Some(dot) = dots.iter().find(|dot| dot.id == active_id) {
                        dot.set_position(position);
                    }
                });
            }
        } else if let (Some((line_id, is_start)), Some((pointer_x, pointer_y, x, y))) = (
            active_endpoint.get_untracked(),
            endpoint_drag_start.get_untracked(),
        ) {
            let scale = zoom.get_untracked();
            let position = (
                x + (f64::from(event.client_x()) - pointer_x) / scale,
                y + (f64::from(event.client_y()) - pointer_y) / scale,
            );

            lines.with_untracked(|lines| {
                if let Some(line) = lines.iter().find(|line| line.id == line_id) {
                    let delta = snap_delta(&[position], lines, line_id, scale);
                    let position =
                        delta.map_or(position, |(x, y)| (position.0 + x, position.1 + y));
                    line.set_endpoint(is_start, position);
                }
            });
        } else if let (
            Some(line_id),
            Some((pointer_x, pointer_y, start_x, start_y, end_x, end_y)),
        ) = (active_line.get_untracked(), line_drag_start.get_untracked())
        {
            let scale = zoom.get_untracked();
            let delta_x = (f64::from(event.client_x()) - pointer_x) / scale;
            let delta_y = (f64::from(event.client_y()) - pointer_y) / scale;

            lines.with_untracked(|lines| {
                if let Some(line) = lines.iter().find(|line| line.id == line_id) {
                    let start = (start_x + delta_x, start_y + delta_y);
                    let end = (end_x + delta_x, end_y + delta_y);
                    let delta = snap_delta(&[start, end], lines, line_id, scale);
                    let (snap_x, snap_y) = delta.unwrap_or((0.0, 0.0));
                    line.set_endpoints(
                        (start.0 + snap_x, start.1 + snap_y),
                        (end.0 + snap_x, end.1 + snap_y),
                    );
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
        dot_drag_start.set(None);
        active_endpoint.set(None);
        endpoint_drag_start.set(None);
        active_line.set(None);
        line_drag_start.set(None);
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
    let zoom_in = move |_| change_zoom(zoom, ZOOM_STEP);
    let zoom_out = move |_| change_zoom(zoom, 1.0 / ZOOM_STEP);

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
                    <div class="zoom-controls">
                        <button
                            class="zoom-button"
                            type="button"
                            aria-label="Zoom out"
                            title="Zoom out"
                            on:click=zoom_out
                        >
                            "-"
                        </button>
                        <output class="zoom-level">
                            {move || format!("{:.0}%", zoom.get() * 100.0)}
                        </output>
                        <button
                            class="zoom-button"
                            type="button"
                            aria-label="Zoom in"
                            title="Zoom in"
                            on:click=zoom_in
                        >
                            "+"
                        </button>
                    </div>
                </div>
            </header>
            <div
                class="world-scene"
                class:panning=move || pan_start.get().is_some()
                style:background-position=move || {
                    let (x, y) = scene_offset.get();
                    format!("calc(50% + {x}px) calc(50% + {y}px)")
                }
                style:background-size=move || {
                    let grid_size = 32.0 * zoom.get();
                    format!("{grid_size}px {grid_size}px")
                }
                on:wheel=move |event: ev::WheelEvent| {
                    event.prevent_default();
                    change_zoom(zoom, if event.delta_y() < 0.0 { ZOOM_STEP } else { 1.0 / ZOOM_STEP });
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
                            dot_drag_start=dot_drag_start
                            scene_offset=scene_offset
                            zoom=zoom
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
                            active_line=active_line
                            line_drag_start=line_drag_start
                            scene_offset=scene_offset
                            zoom=zoom
                        />
                    }
                />
            </div>
        </main>
    }
}
