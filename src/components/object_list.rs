use leptos::prelude::*;

use crate::models::shape::target_amperage;
use crate::state::use_app_state;

#[component]
pub fn ObjectList() -> impl IntoView {
    let state = use_app_state();

    let objects = move || state.objects.get();
    let has_objects = move || !objects().is_empty();

    let total_sa = move || {
        objects()
            .iter()
            .map(|o| o.surface_area())
            .sum::<f64>()
    };

    let target_amps = move || target_amperage(total_sa());

    let remove_object = move |id: usize| {
        state.objects.update(|list| list.retain(|o| o.id != id));
    };

    view! {
        <div class="object-list">
            {move || if has_objects() {
                view! {
                    <div>
                        {move || objects().iter().map(|obj| {
                            let id = obj.id;
                            let name = obj.name.clone();
                            let shape_label = obj.shape.label().to_string();
                            let sa = obj.surface_area();
                            view! {
                                <div class="object-card">
                                    <div class="object-info">
                                        <div class="object-name">{name}</div>
                                        <div class="object-details">{shape_label}</div>
                                    </div>
                                    <div class="object-sa">
                                        {format!("{:.2} cm\u{00B2}", sa)}
                                    </div>
                                    <button
                                        class="object-remove"
                                        on:click=move |_| remove_object(id)
                                        title="Remove"
                                    >
                                        "x"
                                    </button>
                                </div>
                            }
                        }).collect::<Vec<_>>()}

                        <div class="sa-summary">
                            <div class="summary-row">
                                <span class="summary-label">"Total Surface Area"</span>
                                <span class="summary-value">{move || format!("{:.2} cm\u{00B2}", total_sa())}</span>
                            </div>
                            <div class="summary-row">
                                <span class="summary-label">"Target Amperage (x 0.008)"</span>
                                <span class="summary-value">{move || format!("{:.4} A", target_amps())}</span>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <p class="empty-message" style="color: #718096; font-style: italic; text-align: center;">
                        "No objects added yet. Add objects above to calculate the target amperage."
                    </p>
                }.into_any()
            }}
        </div>
    }
}
