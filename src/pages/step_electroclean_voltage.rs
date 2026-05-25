use leptos::prelude::*;

use crate::components::machine_panel::MachinePanel;
use crate::state::use_app_state;

#[component]
pub fn StepElectrocleanVoltage() -> impl IntoView {
    let state = use_app_state();

    let voltage = Signal::derive(move || state.machine.get().voltage_reading());
    let amperage = Signal::derive(move || state.machine.get().amperage_reading());

    let voltage_ok = move || voltage.get() >= 6.0;
    let amperage_ok = move || amperage.get() >= 8.0;
    let target_reached = move || voltage_ok() || amperage_ok();

    // For this step: current dials enabled (set high), voltage dials interactive
    // enabled: [coarse_voltage, fine_voltage, coarse_current, fine_current]

    view! {
        <div class="step step-electroclean-voltage">
            <h2>"Step 3: Electrocleaner - Voltage Clean"</h2>
            <p class="step-description">
                "Set up the GSP 10A SmartAmp for voltage-based electrocleaning. Use the knobs below to practice."
            </p>
            <div class="instructions">
                <ol>
                    <li>"Turn the two "<strong>"current"</strong>" dials fully UP, and the two "<strong>"voltage"</strong>" dials fully DOWN"</li>
                    <li>"Plug red lead into red hole, black lead into black hole"</li>
                    <li>"Attach red lead to Electrocleaner tank anode, black lead to item wire"</li>
                    <li>"Lower item into tank (as close to waterline as possible without breaking it)"</li>
                    <li>"Switch on machine and slowly raise "<strong>"coarse voltage"</strong></li>
                    <li>"Stop when EITHER volts read "<strong>"6V"</strong>" OR amps read "<strong>"8.00A"</strong>" (whichever comes first)"</li>
                    <li>"Use "<strong>"fine voltage"</strong>" dial to adjust small amounts"</li>
                    <li>"Leave to fizz for "<span class="instruction-time">"3 minutes"</span></li>
                </ol>
            </div>

            {move || if target_reached() {
                view! {
                    <div class="test-result pass" style="margin: 1rem 0;">
                        <p>{move || if voltage_ok() {
                            "Target reached: Voltage is at 6V or above."
                        } else {
                            "Target reached: Amperage is at 8.00A or above."
                        }}</p>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="test-result pending" style="margin: 1rem 0;">
                        <p>"Raise coarse voltage until either volts = 6V or amps = 8.00A"</p>
                    </div>
                }.into_any()
            }}

            <MachinePanel enabled=[true, true, true, true] />
        </div>
    }
}
