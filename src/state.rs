use leptos::prelude::*;

use crate::models::machine::MachineState;
use crate::models::plating::GoldType;
use crate::models::shape::{ObjectEntry, SavedObject};
use crate::models::wizard::WizardStep;

#[derive(Clone, Copy)]
pub struct AppState {
    pub current_step: RwSignal<WizardStep>,
    pub machine: RwSignal<MachineState>,
    pub objects: RwSignal<Vec<ObjectEntry>>,
    pub next_object_id: RwSignal<usize>,
    pub gold_type: RwSignal<GoldType>,
    pub microns: RwSignal<f64>,
    pub timer_running: RwSignal<bool>,
    pub timer_remaining_secs: RwSignal<f64>,
    /// Previously added objects that can be re-selected as presets
    pub saved_objects: RwSignal<Vec<SavedObject>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_step: RwSignal::new(WizardStep::Pretreatment),
            machine: RwSignal::new(MachineState::new()),
            objects: RwSignal::new(Vec::new()),
            next_object_id: RwSignal::new(0),
            gold_type: RwSignal::new(GoldType::Gold24K),
            microns: RwSignal::new(2.5),
            timer_running: RwSignal::new(false),
            timer_remaining_secs: RwSignal::new(0.0),
            saved_objects: RwSignal::new(Vec::new()),
        }
    }

    pub fn total_surface_area(&self) -> f64 {
        self.objects
            .get()
            .iter()
            .map(|o| o.surface_area())
            .sum()
    }

    pub fn target_amperage(&self) -> f64 {
        crate::models::shape::target_amperage(self.total_surface_area())
    }

    pub fn plating_time(&self) -> f64 {
        crate::models::plating::plating_time_seconds(self.gold_type.get(), self.microns.get())
    }

    /// Save an object as a reusable preset in memory (avoids duplicates by name+shape).
    pub fn save_object_preset_local(&self, name: &str, shape: &crate::models::shape::Shape) {
        let preset = SavedObject {
            name: name.to_string(),
            shape: shape.clone(),
        };
        self.saved_objects.update(|list| {
            // Don't add duplicates (same name and shape)
            if !list.iter().any(|s| s.name == preset.name && s.shape == preset.shape) {
                list.push(preset);
            }
        });
    }

    pub fn reset(&self) {
        self.current_step.set(WizardStep::Pretreatment);
        self.machine.set(MachineState::new());
        self.objects.set(Vec::new());
        self.next_object_id.set(0);
        self.gold_type.set(GoldType::Gold24K);
        self.microns.set(2.5);
        self.timer_running.set(false);
        self.timer_remaining_secs.set(0.0);
        // Note: saved_objects are NOT reset - they persist across sessions
    }
}

pub fn provide_app_state() {
    provide_context(AppState::new());
}

pub fn use_app_state() -> AppState {
    use_context::<AppState>().expect("AppState must be provided by a parent component")
}
