use leptos::prelude::*;

use crate::{
    components::{section_heading_view::SectionHeadingView, Description},
    name::Name,
    FormConfiguration,
};

#[component]
pub fn Section(
    #[prop(into)] name: Name,
    #[prop(into, default = None)] heading: Option<TextProp>,
    #[prop(into, default = None)] description: Option<TextProp>,
    #[prop(into, default = None)] class: Option<String>,
    #[prop(into, default = None)] columns: Option<u32>,
    #[prop(into, default = None)] colspan: Option<u32>,
    children: Children,
) -> impl IntoView {
    let form_configuration = use_context::<FormConfiguration>().unwrap_or_default();

    view! {
        <div
            class={format!("form-section{}", class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default())}
            id=name.to_string()
            style={colspan.map(|cols| format!("grid-column: span {};", cols))}
        >
            <div class="form-section-heading">
                <SectionHeadingView
                    heading={heading}
                    section_label={form_configuration.section_label}
                    name_len={name.len()}
                />
            </div>
            <Description description={description} />
            <div
                class="form-section-content"
                style={columns.map(|cols| format!("display: grid; grid-template-columns: repeat({}, 1fr);", cols))}
            >
                { children() }
            </div>
        </div>
    }
}
