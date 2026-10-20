use leptos::{ev, prelude::*};

#[derive(Clone, Copy)]
pub struct Dot {
    pub id: usize,
    position: RwSignal<(f64, f64)>,
}

impl Dot {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            position: RwSignal::new((id as f64 * 28.0, 0.0)),
        }
    }

    pub fn set_position(&self, position: (f64, f64)) {
        self.position.set(position);
    }
}

#[component]
pub fn WorldDot(
    dot: Dot,
    active_dot: RwSignal<Option<usize>>,
    dot_drag_start: RwSignal<Option<(f64, f64, f64, f64)>>,
    scene_offset: RwSignal<(f64, f64)>,
    zoom: RwSignal<f64>,
) -> impl IntoView {
    let id = dot.id;
    let position = dot.position;

    view! {
        <button
            class="world-dot"
            class:dragging=move || active_dot.get() == Some(id)
            type="button"
            aria-label="World marker"
            style:left=move || {
                let (pan_x, _) = scene_offset.get();
                let (x, _) = position.get();
                format!("calc(50% + {}px - 13px)", pan_x + x * zoom.get())
            }
            style:top=move || {
                let (_, pan_y) = scene_offset.get();
                let (_, y) = position.get();
                format!("calc(50% + {}px - 13px)", pan_y + y * zoom.get())
            }
            on:pointerdown=move |event: ev::PointerEvent| {
                event.prevent_default();
                event.stop_propagation();
                let (x, y) = position.get_untracked();
                dot_drag_start.set(Some((
                    f64::from(event.client_x()),
                    f64::from(event.client_y()),
                    x,
                    y,
                )));
                active_dot.set(Some(id));
            }
        />
    }
}
