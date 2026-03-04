use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::progress_bar::ProgressBar;
use crate::components::step_nav::StepNav;
use crate::models::wizard::WizardStep;
use crate::pages::step_electroclean_amperage::StepElectrocleanAmperage;
use crate::pages::step_electroclean_voltage::StepElectrocleanVoltage;
use crate::pages::step_finish::StepFinish;
use crate::pages::step_gold::StepGold;
use crate::pages::step_palladium::StepPalladium;
use crate::pages::step_preparation::StepPreparation;
use crate::pages::step_pretreatment::StepPretreatment;
use crate::pages::step_rinse::StepRinse;
use crate::state::{provide_app_state, use_app_state};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_app_state();

    view! {
        <Stylesheet id="leptos" href="/pkg/goldplatecalculator.css"/>
        <Title text="Gold Plating Calculator"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=WizardPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn WizardPage() -> impl IntoView {
    let state = use_app_state();
    let current = move || state.current_step.get();

    view! {
        <div class="wizard-container">
            <header class="app-header">
                <h1>"Gold Plating Calculator"</h1>
                <p class="subtitle">"Jewellery Tank Plating Guide"</p>
            </header>
            <ProgressBar />
            <div class="step-content">
                {move || match current() {
                    WizardStep::Pretreatment => view! { <StepPretreatment /> }.into_any(),
                    WizardStep::Preparation => view! { <StepPreparation /> }.into_any(),
                    WizardStep::ElectrocleanVoltage => view! { <StepElectrocleanVoltage /> }.into_any(),
                    WizardStep::ElectrocleanAmperage => view! { <StepElectrocleanAmperage /> }.into_any(),
                    WizardStep::Rinse => view! { <StepRinse /> }.into_any(),
                    WizardStep::Palladium => view! { <StepPalladium /> }.into_any(),
                    WizardStep::Gold => view! { <StepGold /> }.into_any(),
                    WizardStep::Finish => view! { <StepFinish /> }.into_any(),
                }}
            </div>
            <StepNav />
        </div>
    }
}
