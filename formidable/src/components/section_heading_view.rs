use crate::{HeadingLevel, SectionHeading};
use leptos::prelude::*;

#[component]
pub fn SectionHeadingView(
    heading: Option<TextProp>,
    section_label: SectionHeading,
    name_len: usize,
) -> impl IntoView {
    match heading {
        Some(heading) => match section_label {
            SectionHeading::LeveledHeading(level) => match name_len - 1 + level.to_usize() {
                1 => view! { <h1>{heading.get()}</h1> }.into_any(),
                2 => view! { <h2>{heading.get()}</h2> }.into_any(),
                3 => view! { <h3>{heading.get()}</h3> }.into_any(),
                4 => view! { <h4>{heading.get()}</h4> }.into_any(),
                5 => view! { <h5>{heading.get()}</h5> }.into_any(),
                _ => view! { <h6>{heading.get()}</h6> }.into_any(),
            },
            SectionHeading::SameHeading(level) => match level {
                HeadingLevel::H1 => view! { <h1>{heading.get()}</h1> }.into_any(),
                HeadingLevel::H2 => view! { <h2>{heading.get()}</h2> }.into_any(),
                HeadingLevel::H3 => view! { <h3>{heading.get()}</h3> }.into_any(),
                HeadingLevel::H4 => view! { <h4>{heading.get()}</h4> }.into_any(),
                HeadingLevel::H5 => view! { <h5>{heading.get()}</h5> }.into_any(),
                HeadingLevel::H6 => view! { <h6>{heading.get()}</h6> }.into_any(),
            },
            SectionHeading::PlainText => {
                view! { <div class="section-label">{heading.get()}</div> }.into_any()
            }
        },
        None => ().into_any(),
    }
}
