use leptos::prelude::*;

use crate::models::wizard::WizardStep;
use crate::state::use_app_state;

#[component]
pub fn StepRinse() -> impl IntoView {
    let state = use_app_state();
    let water_beads = RwSignal::new(Option::<bool>::None);

    let on_sheets = move |_| water_beads.set(Some(false));
    let on_beads = move |_| water_beads.set(Some(true));
    let on_repeat = move |_| {
        state.current_step.set(WizardStep::ElectrocleanVoltage);
    };

    view! {
        <div class="step step-rinse">
            <h2>"Step 5: Rinse & Water Break Test"</h2>
            <div class="instructions">
                <ol>
                    <li>"Holding only the wire, remove the item from the electrocleaner tank"</li>
                    <li>"Spray rinse with deionised water"</li>
                    <li>"Observe how the water behaves on the surface"</li>
                </ol>
            </div>

            <div class="water-test">
                <h3>"Water Break Test"</h3>
                <p>"Does the water sheet off smoothly, or does it bead up on the surface?"</p>
                <div class="test-buttons">
                    <button class="btn btn-success" on:click=on_sheets>"Water Sheets Off (Good)"</button>
                    <button class="btn btn-danger" on:click=on_beads>"Water Beads Up (Bad)"</button>
                </div>

                {move || match water_beads.get() {
                    Some(true) => view! {
                        <div class="test-result fail">
                            <p>"Water beading indicates the surface is not clean enough. You need to repeat the electrocleaning process."</p>
                            <button class="btn btn-warning" on:click=on_repeat>
                                "Return to Electrocleaner (Step 3)"
                            </button>
                        </div>
                    }.into_any(),
                    Some(false) => view! {
                        <div class="test-result pass">
                            <p>"Water sheeting off confirms the surface is clean. You may proceed to the Palladium tank."</p>
                        </div>
                    }.into_any(),
                    None => view! {
                        <div class="test-result pending">
                            <p>"Perform the test above to continue."</p>
                        </div>
                    }.into_any(),
                }}
            </div>
        </div>
    }
}
