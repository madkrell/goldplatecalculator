use leptos::prelude::*;

use crate::components::checklist::Checklist;
use crate::state::use_app_state;

#[component]
pub fn StepFinish() -> impl IntoView {
    let state = use_app_state();

    let on_restart = move |_| {
        state.reset();
    };

    view! {
        <div class="step step-finish">
            <h2>"Step 8: Finish"</h2>
            <p class="step-description">
                "Your item is now plated! Complete the finishing steps below."
            </p>
            <Checklist items=vec![
                "Turn off the machine",
                "Remove item from gold tank",
                "Spray rinse with deionised water",
                "Dry the item",
                "(Optional) For small pieces: dip in Windowlene for 30 seconds, drain, pat dry with tissue",
            ] />

            <div class="finish-actions">
                <button class="btn btn-primary" on:click=on_restart>
                    "Start New Session"
                </button>
            </div>
        </div>
    }
}
