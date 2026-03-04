use leptos::prelude::*;

use crate::state::use_app_state;

#[component]
pub fn StepNav() -> impl IntoView {
    let state = use_app_state();
    let current = move || state.current_step.get();

    let has_prev = move || current().prev().is_some();
    let has_next = move || current().next().is_some();

    let on_prev = move |_| {
        if let Some(prev) = current().prev() {
            state.current_step.set(prev);
        }
    };

    let on_next = move |_| {
        if let Some(next) = current().next() {
            state.current_step.set(next);
        }
    };

    view! {
        <div class="step-nav">
            <button
                class="btn btn-prev"
                on:click=on_prev
                disabled=move || !has_prev()
            >
                "Previous"
            </button>
            <button
                class="btn btn-next"
                on:click=on_next
                disabled=move || !has_next()
            >
                {move || if has_next() { "Next" } else { "Complete" }}
            </button>
        </div>
    }
}
