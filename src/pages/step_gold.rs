use leptos::prelude::*;

use crate::components::machine_panel::MachinePanel;
use crate::components::plating_timer::PlatingTimer;
use crate::state::use_app_state;

#[component]
pub fn StepGold() -> impl IntoView {
    let state = use_app_state();
    let target_amps = move || state.target_amperage();

    view! {
        <div class="step step-gold">
            <h2>"Step 7: Gold Tank"</h2>
            <p class="step-description">
                "The final plating stage. Select your gold type and desired thickness below."
            </p>

            <div class="target-display">
                <div class="target-value">
                    <span class="target-label">"Amperage (carried from Electrocleaner)"</span>
                    <span class="target-number">{move || format!("{:.4} A", target_amps())}</span>
                </div>
            </div>

            <div class="instructions">
                <ol>
                    <li>"Turn on the "<strong>"air agitation FIRST"</strong></li>
                    <li><strong>"THEN"</strong>" turn on the power to the machine"</li>
                    <li>"Ensure agitation is directly under the part (engulfed in bubbles)"</li>
                    <li>"Plate for the calculated time shown below"</li>
                </ol>
            </div>

            <div class="warning-banner">
                "Important: Always turn on air agitation BEFORE turning on the power. Ensure bubbles are directly under the part to prevent burning/cloudiness."
            </div>

            <div class="info-table">
                <h3>"Plating Times Reference"</h3>
                <table>
                    <thead>
                        <tr>
                            <th>"Gold Type"</th>
                            <th>"Time per Micron"</th>
                            <th>"Vermeil (2.5\u{03BC}m)"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"24K Gold"</td>
                            <td>"3 min 10 sec"</td>
                            <td>"7 min 25 sec"</td>
                        </tr>
                        <tr>
                            <td>"18K / Rose / 14K Gold"</td>
                            <td>"5 min 0 sec"</td>
                            <td>"12 min 30 sec"</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <PlatingTimer />

            <MachinePanel enabled=[false, false, false, false] />
        </div>
    }
}
