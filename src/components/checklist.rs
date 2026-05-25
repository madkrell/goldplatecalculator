use leptos::prelude::*;

/// Splits a checklist item into views, wrapping time values such as
/// "30 seconds" or "3 minutes" in a red, bold `.instruction-time` span
/// while leaving the surrounding text untouched.
fn highlight_times(text: &str) -> Vec<AnyView> {
    const UNITS: [&str; 10] = [
        "seconds", "second", "minutes", "minute", "hours", "hour", "secs", "sec", "mins", "min",
    ];

    let is_number = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());

    let words: Vec<&str> = text.split(' ').collect();
    let mut views: Vec<AnyView> = Vec::new();
    let mut plain = String::new();
    let mut i = 0;

    while i < words.len() {
        let word = words[i];
        // Leading alphabetic run of the following word is the unit candidate
        // (so trailing punctuation like the comma in "seconds," is excluded).
        let unit = words.get(i + 1).and_then(|next| {
            let alpha_len = next
                .chars()
                .take_while(|c| c.is_ascii_alphabetic())
                .map(char::len_utf8)
                .sum::<usize>();
            UNITS
                .contains(&&next[..alpha_len])
                .then_some((alpha_len, *next))
        });

        if is_number(word) && unit.is_some() {
            let (alpha_len, next_word) = unit.unwrap();
            if !plain.is_empty() {
                let segment = std::mem::take(&mut plain);
                views.push(view! { {segment} }.into_any());
            }
            let time_text = format!("{} {}", word, &next_word[..alpha_len]);
            views.push(view! { <span class="instruction-time">{time_text}</span> }.into_any());
            // Re-attach any trailing punctuation and the separating space.
            plain.push_str(&next_word[alpha_len..]);
            plain.push(' ');
            i += 2;
        } else {
            plain.push_str(word);
            plain.push(' ');
            i += 1;
        }
    }

    let trimmed = plain.trim_end().to_string();
    if !trimmed.is_empty() {
        views.push(view! { {trimmed} }.into_any());
    }

    views
}

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
                            <span>{highlight_times(text)}</span>
                        </label>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
