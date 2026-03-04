use leptos::prelude::*;

use crate::components::checklist::Checklist;

#[component]
pub fn StepPretreatment() -> impl IntoView {
    view! {
        <div class="step step-pretreatment">
            <h2>"Step 1: Pre-treatment - Silver Deoxidising"</h2>
            <p class="step-description">
                "Deoxidise the silver before plating. This removes surface oxides and ensures proper adhesion."
            </p>
            <Checklist items=vec![
                "Line a bowl with aluminium foil (shiny side up)",
                "Sprinkle in a few tablespoons of baking powder (NOT baking soda or bicarbonate of soda)",
                "Pour freshly boiled water, deep enough to submerge the item",
                "Drop item in and sprinkle a couple more tablespoons of baking powder on top",
                "Leave for 30 minutes",
                "Remove item and rinse thoroughly",
            ] />
            <div class="warning-banner">
                "Important: You MUST use baking powder. Baking soda and bicarbonate of soda are both unsuitable."
            </div>
        </div>
    }
}
