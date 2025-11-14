use leptos::prelude::*;

use crate::{name::Name, FormConfiguration};

#[component]
pub fn Section(
    #[prop(into)] name: Name,
    #[prop(into, default = None)] heading: Option<TextProp>,
    #[prop(into, default = None)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let form_configuration = use_context::<FormConfiguration>().unwrap_or_default();

    view! {
        <div class={format!("form-section{}", class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default())} id=name.to_string()>
            <div class="form-section-heading">
            {
                heading.map(|heading| {
                    match form_configuration.section_label {
                        crate::SectionHeading::LeveledHeading(level) => {
                            match name.len() - 1 + level.to_usize() {
                                1 => view! { <h1>{heading.get()}</h1> }.into_any(),
                                2 => view! { <h2>{heading.get()}</h2> }.into_any(),
                                3 => view! { <h3>{heading.get()}</h3> }.into_any(),
                                4 => view! { <h4>{heading.get()}</h4> }.into_any(),
                                5 => view! { <h5>{heading.get()}</h5> }.into_any(),
                                _ => view! { <h6>{heading.get()}</h6> }.into_any(),
                            }
                        },
                        crate::SectionHeading::SameHeading(level) => {
                            match level {
                                crate::HeadingLevel::H1 => view! { <h1>{heading.get()}</h1> }.into_any(),
                                crate::HeadingLevel::H2 => view! { <h2>{heading.get()}</h2> }.into_any(),
                                crate::HeadingLevel::H3 => view! { <h3>{heading.get()}</h3> }.into_any(),
                                crate::HeadingLevel::H4 => view! { <h4>{heading.get()}</h4> }.into_any(),
                                crate::HeadingLevel::H5 => view! { <h5>{heading.get()}</h5> }.into_any(),
                                crate::HeadingLevel::H6 => view! { <h6>{heading.get()}</h6> }.into_any(),
                            }
                        },
                        crate::SectionHeading::PlainText => {
                            view! { <div class="section-label">{heading.get()}</div> }.into_any()
                        },
                    }
                })
            }
            </div>
            <div class="form-section-content">
                { children() }
            </div>
        </div>
    }
}
