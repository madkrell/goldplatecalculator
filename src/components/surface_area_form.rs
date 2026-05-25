use leptos::prelude::*;

use crate::db::{delete_saved_object, get_saved_objects, save_object_preset};
use crate::models::shape::{ObjectEntry, Shape};
use crate::state::use_app_state;

#[derive(Clone, Copy, PartialEq)]
enum ShapeType {
    Rectangle,
    Cylinder,
    Sphere,
}

#[component]
pub fn SurfaceAreaForm() -> impl IntoView {
    let state = use_app_state();

    let shape_type = RwSignal::new(ShapeType::Rectangle);
    let name = RwSignal::new(String::new());
    let length = RwSignal::new(String::new());
    let width = RwSignal::new(String::new());
    let height = RwSignal::new(String::new());
    let radius = RwSignal::new(String::new());

    // Load saved objects from the database on mount
    let saved_resource = Resource::new(|| (), |_| async {
        get_saved_objects().await.unwrap_or_default()
    });

    // When the resource loads, populate the state signal
    Effect::new(move |_| {
        if let Some(saved) = saved_resource.get() {
            state.saved_objects.set(saved);
        }
    });

    // Helper to populate form fields from a saved preset
    let load_preset = move |preset_shape: &Shape, preset_name: &str| {
        name.set(preset_name.to_string());
        match preset_shape {
            Shape::Rectangle { length: l, width: w, height: h } => {
                shape_type.set(ShapeType::Rectangle);
                length.set(format!("{}", l));
                width.set(format!("{}", w));
                height.set(format!("{}", h));
                radius.set(String::new());
            }
            Shape::Cylinder { radius: r, height: h } => {
                shape_type.set(ShapeType::Cylinder);
                radius.set(format!("{}", r));
                height.set(format!("{}", h));
                length.set(String::new());
                width.set(String::new());
            }
            Shape::Sphere { radius: r } => {
                shape_type.set(ShapeType::Sphere);
                radius.set(format!("{}", r));
                length.set(String::new());
                width.set(String::new());
                height.set(String::new());
            }
        }
    };

    let preview_sa = Signal::derive(move || -> Option<f64> {
        let shape = match shape_type.get() {
            ShapeType::Rectangle => {
                let l = length.get().parse::<f64>().ok()?;
                let w = width.get().parse::<f64>().ok()?;
                let h = height.get().parse::<f64>().ok()?;
                if l > 0.0 && w > 0.0 && h > 0.0 {
                    Shape::Rectangle {
                        length: l,
                        width: w,
                        height: h,
                    }
                } else {
                    return None;
                }
            }
            ShapeType::Cylinder => {
                let r = radius.get().parse::<f64>().ok()?;
                let h = height.get().parse::<f64>().ok()?;
                if r > 0.0 && h > 0.0 {
                    Shape::Cylinder { radius: r, height: h }
                } else {
                    return None;
                }
            }
            ShapeType::Sphere => {
                let r = radius.get().parse::<f64>().ok()?;
                if r > 0.0 {
                    Shape::Sphere { radius: r }
                } else {
                    return None;
                }
            }
        };
        Some(shape.surface_area())
    });

    let can_add = Signal::derive(move || preview_sa.get().is_some());

    let on_add = move |_| {
        let st = shape_type.get();
        let shape = match st {
            ShapeType::Rectangle => {
                let l = length.get().parse::<f64>().unwrap_or(0.0);
                let w = width.get().parse::<f64>().unwrap_or(0.0);
                let h = height.get().parse::<f64>().unwrap_or(0.0);
                Shape::Rectangle {
                    length: l,
                    width: w,
                    height: h,
                }
            }
            ShapeType::Cylinder => {
                let r = radius.get().parse::<f64>().unwrap_or(0.0);
                let h = height.get().parse::<f64>().unwrap_or(0.0);
                Shape::Cylinder { radius: r, height: h }
            }
            ShapeType::Sphere => {
                let r = radius.get().parse::<f64>().unwrap_or(0.0);
                Shape::Sphere { radius: r }
            }
        };

        let id = state.next_object_id.get();
        state.next_object_id.set(id + 1);

        let obj_name = if name.get().is_empty() {
            format!("{} {}", shape.label(), id + 1)
        } else {
            name.get()
        };

        // Save to in-memory state
        state.save_object_preset_local(&obj_name, &shape);

        // Persist to database via server function
        let db_name = obj_name.clone();
        let db_shape = shape.clone();
        leptos::task::spawn_local(async move {
            let shape_json = serde_json::to_string(&db_shape).unwrap_or_default();
            let _ = save_object_preset(db_name, shape_json).await;
        });

        let entry = ObjectEntry {
            id,
            name: obj_name,
            shape,
        };
        state.objects.update(|list| list.push(entry));

        // Reset form
        name.set(String::new());
        length.set(String::new());
        width.set(String::new());
        height.set(String::new());
        radius.set(String::new());
    };

    let has_saved = Signal::derive(move || !state.saved_objects.get().is_empty());

    view! {
        <div class="surface-area-form">

            // Saved objects preset selector
            {move || if has_saved.get() {
                let saved = state.saved_objects.get();
                view! {
                    <div class="saved-presets">
                        <label class="presets-label">"Saved Objects"</label>
                        <div class="preset-chips">
                            {saved.into_iter().map(|preset| {
                                let sa = preset.shape.surface_area();
                                let label_text = format!(
                                    "{} ({} - {:.1} cm\u{00B2})",
                                    preset.name,
                                    preset.shape.label(),
                                    sa
                                );
                                let p_shape = preset.shape.clone();
                                let p_name = preset.name.clone();
                                let del_shape = preset.shape.clone();
                                let del_name = preset.name.clone();
                                view! {
                                    <span class="preset-chip-wrapper">
                                        <button
                                            class="preset-chip"
                                            on:click=move |_| {
                                                load_preset(&p_shape, &p_name);
                                            }
                                        >
                                            {label_text}
                                        </button>
                                        <button
                                            class="preset-delete"
                                            title="Remove saved object"
                                            on:click=move |_| {
                                                let rm_name = del_name.clone();
                                                let rm_shape = del_shape.clone();
                                                // Remove from in-memory state
                                                state.saved_objects.update(|list| {
                                                    list.retain(|s| !(s.name == rm_name && s.shape == rm_shape));
                                                });
                                                // Remove from database
                                                let db_name = rm_name;
                                                let db_shape = rm_shape;
                                                leptos::task::spawn_local(async move {
                                                    let shape_json = serde_json::to_string(&db_shape).unwrap_or_default();
                                                    let _ = delete_saved_object(db_name, shape_json).await;
                                                });
                                            }
                                        >
                                            "\u{00D7}"
                                        </button>
                                    </span>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}

            <div class="shape-selector">
                <button
                    class="shape-btn"
                    class:active=move || shape_type.get() == ShapeType::Rectangle
                    on:click=move |_| shape_type.set(ShapeType::Rectangle)
                >
                    "Rectangle"
                </button>
                <button
                    class="shape-btn"
                    class:active=move || shape_type.get() == ShapeType::Cylinder
                    on:click=move |_| shape_type.set(ShapeType::Cylinder)
                >
                    "Cylinder"
                </button>
                <button
                    class="shape-btn"
                    class:active=move || shape_type.get() == ShapeType::Sphere
                    on:click=move |_| shape_type.set(ShapeType::Sphere)
                >
                    "Sphere"
                </button>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label>"Name (optional)"</label>
                    <input
                        type="text"
                        placeholder="e.g. Ring band"
                        prop:value=move || name.get()
                        on:input=move |ev| name.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <div class="dimension-inputs">
                {move || match shape_type.get() {
                    ShapeType::Rectangle => view! {
                        <div class="form-group">
                            <label>"Length (cm)"</label>
                            <input
                                type="text"
                                inputmode="decimal"
                                placeholder="0"
                                prop:value=move || length.get()
                                on:input=move |ev| length.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="form-group">
                            <label>"Width (cm)"</label>
                            <input
                                type="text"
                                inputmode="decimal"
                                placeholder="0"
                                prop:value=move || width.get()
                                on:input=move |ev| width.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="form-group">
                            <label>"Height (cm)"</label>
                            <input
                                type="text"
                                inputmode="decimal"
                                placeholder="0"
                                prop:value=move || height.get()
                                on:input=move |ev| height.set(event_target_value(&ev))
                            />
                        </div>
                    }.into_any(),
                    ShapeType::Cylinder => view! {
                        <div class="form-group">
                            <label>"Radius (cm)"</label>
                            <input
                                type="text"
                                inputmode="decimal"
                                placeholder="0"
                                prop:value=move || radius.get()
                                on:input=move |ev| radius.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="form-group">
                            <label>"Height (cm)"</label>
                            <input
                                type="text"
                                inputmode="decimal"
                                placeholder="0"
                                prop:value=move || height.get()
                                on:input=move |ev| height.set(event_target_value(&ev))
                            />
                        </div>
                    }.into_any(),
                    ShapeType::Sphere => view! {
                        <div class="form-group">
                            <label>"Radius (cm)"</label>
                            <input
                                type="text"
                                inputmode="decimal"
                                placeholder="0"
                                prop:value=move || radius.get()
                                on:input=move |ev| radius.set(event_target_value(&ev))
                            />
                        </div>
                    }.into_any(),
                }}
            </div>

            {move || preview_sa.get().map(|sa| view! {
                <div class="sa-preview">
                    {format!("Surface Area = {:.2} cm\u{00B2}", sa)}
                </div>
            })}

            <div class="form-row" style="margin-top: 1rem; justify-content: center;">
                <button
                    class="btn btn-primary"
                    on:click=on_add
                    disabled=move || !can_add.get()
                >
                    "Add Object"
                </button>
            </div>
        </div>
    }
}
