use leptos::prelude::*;

use crate::models::plating::{format_duration, plating_time_seconds, GoldType, VERMEIL_MICRONS};
use crate::state::use_app_state;

#[component]
pub fn PlatingTimer() -> impl IntoView {
    let state = use_app_state();

    let micron_input = RwSignal::new(format!("{}", state.microns.get()));

    // Sync micron input to state
    Effect::new(move |_| {
        if let Ok(v) = micron_input.get().parse::<f64>() {
            if v > 0.0 {
                state.microns.set(v);
            }
        }
    });

    let calculated_time = Signal::derive(move || {
        plating_time_seconds(state.gold_type.get(), state.microns.get())
    });

    let time_display = Signal::derive(move || format_duration(calculated_time.get()));

    let countdown_display = Signal::derive(move || {
        let remaining = state.timer_remaining_secs.get();
        let mins = (remaining / 60.0).floor() as u64;
        let secs = (remaining % 60.0).floor() as u64;
        format!("{:02}:{:02}", mins, secs)
    });

    let is_finished = Signal::derive(move || {
        state.timer_running.get() && state.timer_remaining_secs.get() <= 0.0
    });

    let on_start = move |_| {
        state.timer_remaining_secs.set(calculated_time.get());
        state.timer_running.set(true);

        // Set up interval timer using leptos set_interval
        #[cfg(feature = "hydrate")]
        {
            let timer_running = state.timer_running;
            let timer_remaining = state.timer_remaining_secs;
            leptos::task::spawn_local(async move {
                loop {
                    gloo_timers::future::TimeoutFuture::new(1000).await;
                    if !timer_running.get() {
                        break;
                    }
                    let remaining = timer_remaining.get() - 1.0;
                    if remaining <= 0.0 {
                        timer_remaining.set(0.0);
                        break;
                    }
                    timer_remaining.set(remaining);
                }
            });
        }
    };

    let on_stop = move |_| {
        state.timer_running.set(false);
    };

    let on_reset = move |_| {
        state.timer_running.set(false);
        state.timer_remaining_secs.set(0.0);
    };

    let set_vermeil = move |_| {
        micron_input.set(format!("{}", VERMEIL_MICRONS));
        state.microns.set(VERMEIL_MICRONS);
    };

    view! {
        <div class="plating-timer">
            <h3>"Plating Timer"</h3>

            <div class="gold-type-selector">
                {GoldType::all().iter().map(|gt| {
                    let gt = *gt;
                    let is_active = move || state.gold_type.get() == gt;
                    view! {
                        <button
                            class="gold-type-btn"
                            class:active=is_active
                            on:click=move |_| state.gold_type.set(gt)
                        >
                            {gt.label()}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="timer-config">
                <div class="micron-input">
                    <div class="form-group">
                        <label>"Thickness"</label>
                        <input
                            type="number"
                            step="0.1"
                            min="0.1"
                            prop:value=move || micron_input.get()
                            on:input=move |ev| micron_input.set(event_target_value(&ev))
                        />
                    </div>
                    <span class="unit-label">"\u{03BC}m"</span>
                </div>
                <div class="vermeil-preset">
                    <button class="btn btn-small btn-primary" on:click=set_vermeil>
                        "Vermeil (2.5\u{03BC}m)"
                    </button>
                </div>
            </div>

            <div class="timer-display">
                <div class="calculated-time">
                    "Calculated plating time: " <strong>{time_display}</strong>
                </div>

                {move || if state.timer_running.get() || state.timer_remaining_secs.get() > 0.0 {
                    view! {
                        <div
                            class="countdown"
                            class:running=move || state.timer_running.get() && !is_finished.get()
                            class:finished=is_finished
                        >
                            {countdown_display}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

                {move || if is_finished.get() {
                    view! {
                        <div class="timer-finished-msg" style="color: #48bb78; font-weight: 600; margin-top: 0.5rem;">
                            "Plating complete! Remove item from tank."
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>

            <div class="timer-controls">
                {move || if !state.timer_running.get() {
                    view! {
                        <button class="btn btn-success" on:click=on_start>
                            "Start Timer"
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <button class="btn btn-danger" on:click=on_stop>
                            "Stop"
                        </button>
                    }.into_any()
                }}
                <button class="btn btn-prev" on:click=on_reset>
                    "Reset"
                </button>
            </div>
        </div>
    }
}
