use leptos::prelude::*;

use crate::components::rotary_knob::RotaryKnob;
use crate::models::machine::LimitingFactor;
use crate::state::use_app_state;

#[component]
pub fn MachinePanel(
    #[prop(default = [true, true, true, true])] enabled: [bool; 4],
) -> impl IntoView {
    let state = use_app_state();

    // Use get_untracked() since these are just initial values for the local knob signals.
    // We don't need reactive tracking here - the Effect below handles syncing.
    let initial = state.machine.get_untracked();
    let coarse_voltage = RwSignal::new(initial.coarse_voltage);
    let fine_voltage = RwSignal::new(initial.fine_voltage);
    let coarse_current = RwSignal::new(initial.coarse_current);
    let fine_current = RwSignal::new(initial.fine_current);

    // Sync individual knob signals back to global MachineState
    Effect::new(move |_| {
        let cv = coarse_voltage.get();
        let fv = fine_voltage.get();
        let cc = coarse_current.get();
        let fc = fine_current.get();
        state.machine.update(|m| {
            m.coarse_voltage = cv;
            m.fine_voltage = fv;
            m.coarse_current = cc;
            m.fine_current = fc;
        });
    });

    let voltage_display = Signal::derive(move || {
        format!("{:.1} V", state.machine.get().voltage_reading())
    });
    let amperage_display = Signal::derive(move || {
        format!("{:.3} A", state.machine.get().amperage_reading())
    });

    let limiting = Signal::derive(move || state.machine.get().limiting_factor());

    let voltage_limiting_class = Signal::derive(move || {
        if limiting.get() == LimitingFactor::Voltage {
            "readout voltage limiting"
        } else {
            "readout voltage"
        }
    });
    let amperage_limiting_class = Signal::derive(move || {
        if limiting.get() == LimitingFactor::Current {
            "readout amperage limiting"
        } else {
            "readout amperage"
        }
    });

    let limiting_label = Signal::derive(move || match limiting.get() {
        LimitingFactor::Voltage => "Mode: Voltage Limited (Brush Plating)",
        LimitingFactor::Current => "Mode: Current Limited (Tank Plating)",
        LimitingFactor::Both => "Mode: Balanced",
    });

    let cv_display = Signal::derive(move || format!("{:.0}%", coarse_voltage.get() * 100.0));
    let fv_display = Signal::derive(move || format!("{:.0}%", fine_voltage.get() * 100.0));
    let cc_display = Signal::derive(move || format!("{:.0}%", coarse_current.get() * 100.0));
    let fc_display = Signal::derive(move || format!("{:.0}%", fine_current.get() * 100.0));

    view! {
        <div class="machine-panel">
            <h3 class="panel-title">"GSP 10A SmartAmp"</h3>
            <div class="machine-readouts">
                <div class=voltage_limiting_class>
                    <span class="readout-label">"Volts"</span>
                    <span class="readout-value">{voltage_display}</span>
                </div>
                <div class=amperage_limiting_class>
                    <span class="readout-label">"Amps"</span>
                    <span class="readout-value">{amperage_display}</span>
                </div>
            </div>
            <div class="limiting-mode">
                <span class="limiting-label">{limiting_label}</span>
            </div>
            <div class="knobs-row">
                <RotaryKnob label="Coarse Voltage".to_string() value=coarse_voltage display_value=cv_display enabled=enabled[0] />
                <RotaryKnob label="Fine Voltage".to_string() value=fine_voltage display_value=fv_display enabled=enabled[1] />
                <RotaryKnob label="Coarse Current".to_string() value=coarse_current display_value=cc_display enabled=enabled[2] />
                <RotaryKnob label="Fine Current".to_string() value=fine_current display_value=fc_display enabled=enabled[3] />
            </div>
        </div>
    }
}
