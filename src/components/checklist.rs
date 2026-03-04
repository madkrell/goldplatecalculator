use leptos::prelude::*;

#[component]
pub fn Checklist(items: Vec<&'static str>) -> impl IntoView {
    let checked = RwSignal::new(vec![false; items.len()]);

    view! {
        <div class="checklist">
            {items
                .into_iter()
                .enumerate()
                .map(|(i, text)| {
                    let is_checked = move || checked.get()[i];
                    let toggle = move |_| {
                        checked.update(|c| c[i] = !c[i]);
                    };
                    view! {
                        <label class="checklist-item" class:checked=is_checked>
                            <input
                                type="checkbox"
                                prop:checked=is_checked
                                on:change=toggle
                            />
                            <span>{text}</span>
                        </label>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
