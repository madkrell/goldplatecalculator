use leptos::prelude::*;

use crate::models::wizard::WizardStep;
use crate::state::use_app_state;

#[component]
pub fn ProgressBar() -> impl IntoView {
    let state = use_app_state();
    let current = move || state.current_step.get();

    view! {
        <div class="progress-bar">
            {WizardStep::all_steps()
                .iter()
                .map(|step| {
                    let step = *step;
                    let is_current = move || current() == step;
                    let is_completed = move || step.number() < current().number();
                    let step_num = step.number();
                    let title = step.short_title();
                    view! {
                        <div
                            class="progress-step"
                            class:active=is_current
                            class:completed=is_completed
                            on:click=move |_| state.current_step.set(step)
                        >
                            <div class="step-circle">{step_num}</div>
                            <div class="step-label">{title}</div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
