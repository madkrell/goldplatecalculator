use leptos::prelude::*;

use crate::components::checklist::Checklist;
use crate::components::object_list::ObjectList;
use crate::components::surface_area_form::SurfaceAreaForm;

#[component]
pub fn StepPreparation() -> impl IntoView {
    view! {
        <div class="step step-preparation">
            <h2>"Step 2: Preparation - Copper Wire & Surface Area"</h2>
            <p class="step-description">
                "Rig your item to copper wire and measure the surface area of all objects to be plated."
            </p>
            <Checklist items=vec![
                "Rig item to copper wire (1mm gauge recommended)",
                "Strip oxide off the copper wire by pulling firmly through steel wire wool",
                "Ensure connection to item is tight and not loose",
            ] />

            <h3>"Surface Area Calculator"</h3>
            <p>"Add each object you will be plating. The formula for a rectangular object is: SA = 2 \u{00D7} (L\u{00D7}W + L\u{00D7}H + W\u{00D7}H)"</p>

            <div class="surface-area-section">
                <SurfaceAreaForm />
                <ObjectList />
            </div>
        </div>
    }
}
