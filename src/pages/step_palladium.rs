use leptos::prelude::*;

use crate::components::machine_panel::MachinePanel;
use crate::state::use_app_state;

#[component]
pub fn StepPalladium() -> impl IntoView {
    let state = use_app_state();
    let target_amps = move || state.target_amperage();

    view! {
        <div class="step step-palladium">
            <h2>"Step 6: Palladium Tank"</h2>
            <p class="step-description">
                "The palladium layer acts as a barrier between the silver and gold, preventing migration."
            </p>

            <div class="target-display">
                <div class="target-value">
                    <span class="target-label">"Amperage (carried from Electrocleaner)"</span>
                    <span class="target-number">{move || format!("{:.4} A", target_amps())}</span>
                </div>
            </div>

            <div class="instructions">
                <ol>
                    <li>"Connect red lead to the anodes in your Palladium tank"</li>
                    <li>"Lower item into Palladium tank"</li>
                    <li>"Turn on the power at the machine (it already has the correct values set)"</li>
                    <li>"Sway item gently while it plates - just enough so it never sits still "<span class="instruction-time">"for 40 seconds"</span></li>
                    <li>"Item should come out a straw-tinted silver colour, darker but just as shiny"</li>
                    <li>"Turn off the machine but "<strong>"DO NOT"</strong>" touch the dials"</li>
                    <li>"Rinse the item with deionised water"</li>
                    <li>"Move the red lead from the palladium anode to the gold anode"</li>
                </ol>
            </div>

            <MachinePanel enabled=[false, false, false, false] />
        </div>
    }
}
