use leptos::{ev, prelude::*};

#[derive(Clone, Copy)]
pub struct Dot {
    pub id: usize,
    position: RwSignal<Option<(f64, f64)>>,
}

impl Dot {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            position: RwSignal::new(None),
        }
    }

    pub fn set_position(&self, position: (f64, f64)) {
        self.position.set(Some(position));
    }
}

#[component]
pub fn WorldDot(
    dot: Dot,
    active_dot: RwSignal<Option<usize>>,
    grab_offset: RwSignal<(f64, f64)>,
    scene_offset: RwSignal<(f64, f64)>,
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
                position.get().map_or_else(
                    || format!("calc(50% + {}px + {pan_x}px - 13px)", id * 28),
                    |(x, _)| format!("{}px", x + pan_x),
                )
            }
            style:top=move || {
                let (_, pan_y) = scene_offset.get();
                position.get().map_or_else(
                    || format!("calc(50% + {pan_y}px - 13px)"),
                    |(_, y)| format!("{}px", y + pan_y),
                )
            }
            on:pointerdown=move |event: ev::PointerEvent| {
                event.prevent_default();
                event.stop_propagation();
                grab_offset.set((
                    f64::from(event.offset_x()),
                    f64::from(event.offset_y()),
                ));
                active_dot.set(Some(id));
            }
        />
    }
}
