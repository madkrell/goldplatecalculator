use leptos::prelude::*;

use crate::components::machine_panel::MachinePanel;
use crate::state::use_app_state;

#[component]
pub fn StepElectrocleanAmperage() -> impl IntoView {
    let state = use_app_state();

    let target_amps = move || state.target_amperage();
    let total_sa = move || state.total_surface_area();
    let current_amps = Signal::derive(move || state.machine.get().amperage_reading());

    let amps_match = move || {
        let target = target_amps();
        let current = current_amps.get();
        target > 0.0 && (current - target).abs() < 0.05
    };

    let no_objects = move || state.objects.get().is_empty();

    view! {
        <div class="step step-electroclean-amperage">
            <h2>"Step 4: Electrocleaner - Amperage Setting"</h2>
            <p class="step-description">
                "Set the correct amperage based on the surface area of your items."
            </p>

            {move || if no_objects() {
                view! {
                    <div class="warning-banner">
                        "No objects have been added yet. Go back to Step 2 to add objects and calculate the surface area, or the target amperage will be 0."
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}

            <div class="target-display">
                <div class="target-value">
                    <span class="target-label">"Total Surface Area"</span>
                    <span class="target-number">{move || format!("{:.2} cm\u{00B2}", total_sa())}</span>
                </div>
                <div class="target-value">
                    <span class="target-label">"Target Amperage (SA \u{00D7} 0.008)"</span>
                    <span class="target-number highlight">{move || format!("{:.4} A", target_amps())}</span>
                </div>
                <div class="target-value">
                    <span class="target-label">"Current Machine Amperage"</span>
                    <span class="target-number" style=move || if amps_match() { "color: #48bb78;" } else { "" }>
                        {move || format!("{:.4} A", current_amps.get())}
                    </span>
                </div>
            </div>

            <div class="test-result" style="margin: 1rem 0; min-height: 3rem;"
                class:pass=amps_match
                style:visibility=move || if amps_match() { "visible" } else { "hidden" }
            >
                <p>"Amperage matches the target! Turn off the machine - it will remember these settings."</p>
            </div>

            <div class="instructions">
                <ol>
                    <li>"Leave item in the electrocleaner tank"</li>
                    <li>"Turn all 4 dials on the machine "<strong>"DOWN"</strong></li>
                    <li>"Turn both coarse and fine "<strong>"VOLTAGE"</strong>" dials fully "<strong>"UP"</strong></li>
                    <li>"Turn the "<strong>"FINE amp"</strong>" dial fully "<strong>"UP"</strong></li>
                    <li><strong>"Do NOT"</strong>" turn the COARSE amp dial yet"</li>
                    <li>"Slowly turn the "<strong>"COARSE amp"</strong>" dial until you approach the target amperage"</li>
                    <li>"Use the "<strong>"FINE amp"</strong>" dial to arrive at the exact target value"</li>
                    <li>"Once target is reached, turn off machine (it will remember the settings)"</li>
                </ol>
            </div>

            <MachinePanel enabled=[true, true, true, true] />
        </div>
    }
}
