use leptos::{ev, prelude::*};

#[derive(Clone, Copy)]
pub struct Line {
    pub id: usize,
    start: RwSignal<(f64, f64)>,
    end: RwSignal<(f64, f64)>,
}

impl Line {
    pub fn new(id: usize) -> Self {
        let y = 32.0 + id as f64 * 28.0;
        Self {
            id,
            start: RwSignal::new((-48.0, y)),
            end: RwSignal::new((48.0, y)),
        }
    }

    pub fn set_endpoint(&self, is_start: bool, position: (f64, f64)) {
        if is_start {
            self.start.set(position);
        } else {
            self.end.set(position);
        }
    }
}

#[component]
pub fn WorldLine(
    line: Line,
    active_endpoint: RwSignal<Option<(usize, bool)>>,
    endpoint_drag_start: RwSignal<Option<(f64, f64, f64, f64)>>,
    scene_offset: RwSignal<(f64, f64)>,
    zoom: RwSignal<f64>,
) -> impl IntoView {
    let id = line.id;
    let start = line.start;
    let end = line.end;

    let begin_drag = move |event: ev::PointerEvent, is_start: bool| {
        event.prevent_default();
        event.stop_propagation();
        let (x, y) = if is_start {
            start.get_untracked()
        } else {
            end.get_untracked()
        };
        endpoint_drag_start.set(Some((
            f64::from(event.client_x()),
            f64::from(event.client_y()),
            x,
            y,
        )));
        active_endpoint.set(Some((id, is_start)));
    };

    let line_geometry = move || {
        let (start_x, start_y) = start.get();
        let (end_x, end_y) = end.get();
        let dx = end_x - start_x;
        let dy = end_y - start_y;
        let length = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx).to_degrees();
        (start_x, start_y, length, angle)
    };

    view! {
        <div class="line-layer">
            <div
                class="world-line"
                style:left=move || {
                    let (x, _, _, _) = line_geometry();
                    let (pan_x, _) = scene_offset.get();
                    format!("calc(50% + {}px)", pan_x + x * zoom.get())
                }
                style:top=move || {
                    let (_, y, _, _) = line_geometry();
                    let (_, pan_y) = scene_offset.get();
                    format!("calc(50% + {}px - 1px)", pan_y + y * zoom.get())
                }
                style:width=move || format!("{}px", line_geometry().2 * zoom.get())
                style:transform=move || format!("rotate({}deg)", line_geometry().3)
            />
            <button
                class="line-endpoint"
                class:dragging=move || active_endpoint.get() == Some((id, true))
                type="button"
                aria-label="Move line start"
                style:left=move || {
                    let (x, _, _, _) = line_geometry();
                    let (pan_x, _) = scene_offset.get();
                    format!("calc(50% + {}px - 7px)", pan_x + x * zoom.get())
                }
                style:top=move || {
                    let (_, y, _, _) = line_geometry();
                    let (_, pan_y) = scene_offset.get();
                    format!("calc(50% + {}px - 7px)", pan_y + y * zoom.get())
                }
                on:pointerdown=move |event| begin_drag(event, true)
            />
            <button
                class="line-endpoint"
                class:dragging=move || active_endpoint.get() == Some((id, false))
                type="button"
                aria-label="Move line end"
                style:left=move || {
                    let (_, _, _, _) = line_geometry();
                    let (x, _) = end.get();
                    let (pan_x, _) = scene_offset.get();
                    format!("calc(50% + {}px - 7px)", pan_x + x * zoom.get())
                }
                style:top=move || {
                    let (_, y) = end.get();
                    let (_, pan_y) = scene_offset.get();
                    format!("calc(50% + {}px - 7px)", pan_y + y * zoom.get())
                }
                on:pointerdown=move |event| begin_drag(event, false)
            />
        </div>
    }
}
