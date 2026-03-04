use leptos::ev;
use leptos::prelude::*;

/// Gets the center coordinates of an element using getBoundingClientRect.
/// Only available on wasm32 targets where web_sys is functional.
#[cfg(target_arch = "wasm32")]
fn get_element_center(el: &web_sys::Element) -> (f64, f64) {
    let rect = el.get_bounding_client_rect();
    (rect.left() + rect.width() / 2.0, rect.top() + rect.height() / 2.0)
}

/// Convert pointer position relative to a center point into a knob value [0.0, 1.0].
/// Uses atan2 to compute the angle. The knob sweeps 270 degrees:
/// - 0.0 maps to 7 o'clock position (-135 degrees from 12 o'clock)
/// - 1.0 maps to 5 o'clock position (+135 degrees from 12 o'clock)
/// - The dead zone is the bottom 90 degrees (between 5 and 7 o'clock)
fn pointer_to_knob_value(px: f64, py: f64, cx: f64, cy: f64) -> f64 {
    let dx = px - cx;
    let dy = py - cy;

    // atan2 gives angle in radians: 0 = right, positive = downward
    // We want 12 o'clock (top) = 0 degrees, clockwise positive
    let angle_rad = dy.atan2(dx);
    let mut angle_deg = angle_rad.to_degrees() + 90.0; // shift so top = 0

    // Normalize to [-180, 180]
    if angle_deg > 180.0 {
        angle_deg -= 360.0;
    }

    // The knob range is [-135, 135] degrees.
    // Dead zone is the bottom arc from +135 to +180 and -180 to -135.
    // In the dead zone, snap to nearest end.
    if angle_deg > 135.0 {
        return 1.0;
    }
    if angle_deg < -135.0 {
        return 0.0;
    }

    // Map [-135, 135] -> [0.0, 1.0]
    (angle_deg + 135.0) / 270.0
}

#[component]
pub fn RotaryKnob(
    #[prop(into)] label: String,
    value: RwSignal<f64>,
    #[prop(into)] display_value: Signal<String>,
    #[prop(default = true)] enabled: bool,
) -> impl IntoView {
    let min_angle: f64 = -135.0;
    let max_angle: f64 = 135.0;

    let knob_ref = NodeRef::<leptos::html::Div>::new();
    let is_dragging = RwSignal::new(false);
    // Center of the knob element in viewport coordinates
    let center_x = RwSignal::new(0.0_f64);
    let center_y = RwSignal::new(0.0_f64);

    let rotation_deg = move || {
        let v = value.get();
        min_angle + v * (max_angle - min_angle)
    };

    let on_pointer_down = move |ev: ev::PointerEvent| {
        if !enabled {
            return;
        }
        ev.prevent_default();

        // Calculate center of the knob element
        #[cfg(target_arch = "wasm32")]
        if let Some(el) = knob_ref.get() {
            use wasm_bindgen::JsCast;
            if let Some(element) = el.dyn_ref::<web_sys::Element>() {
                let (cx, cy) = get_element_center(element);
                center_x.set(cx);
                center_y.set(cy);
            }
        }

        is_dragging.set(true);

        // Immediately set value from click position for responsive feel
        let new_val = pointer_to_knob_value(
            ev.client_x() as f64,
            ev.client_y() as f64,
            center_x.get(),
            center_y.get(),
        );
        value.set(new_val);
    };

    let on_pointer_move = move |ev: ev::PointerEvent| {
        if !is_dragging.get() {
            return;
        }
        ev.prevent_default();

        let new_val = pointer_to_knob_value(
            ev.client_x() as f64,
            ev.client_y() as f64,
            center_x.get(),
            center_y.get(),
        );
        value.set(new_val);
    };

    let on_pointer_up = move |_: ev::PointerEvent| {
        is_dragging.set(false);
    };

    view! {
        <div
            class="knob-container"
            class:disabled=!enabled
            on:pointermove=on_pointer_move
            on:pointerup=on_pointer_up
            on:pointerleave=on_pointer_up
        >
            <div class="knob-label">{label}</div>
            <div
                class="knob-body"
                node_ref=knob_ref
                on:pointerdown=on_pointer_down
                style:transform=move || format!("rotate({}deg)", rotation_deg())
            >
                <div class="knob-indicator"></div>
            </div>
            <div class="knob-value">{display_value}</div>
        </div>
    }
}
