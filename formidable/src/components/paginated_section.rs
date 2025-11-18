use leptos::prelude::*;

use crate::{
    components::{Description, SectionHeadingView},
    name::Name,
    FormConfiguration,
};

#[component]
pub fn PaginatedSection(
    #[prop(into)] name: Name,
    #[prop(into, default = None)] heading: Option<TextProp>,
    #[prop(into, default = None)] description: Option<TextProp>,
    #[prop(into, default = None)] class: Option<String>,
    #[prop(into, default = None)] columns: Option<u32>,
    #[prop(into, default = None)] colspan: Option<u32>,
    pages: Vec<Box<dyn Fn() -> AnyView + Send + Sync>>,
) -> impl IntoView {
    let form_configuration = use_context::<FormConfiguration>().unwrap_or_default();
    let total_pages = pages.len();
    let current_page = RwSignal::new(0);

    view! {
        <div
            class={format!("form-section paginated{}", class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default())}
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
            <div class="form-section-progress">
                <progress
                    value={move || current_page.get() + 1}
                    max={total_pages}
                    class="pagination-progress"
                />
            </div>
            <div
                class="form-section-content"
                style={columns.map(|cols| format!("display: grid; grid-template-columns: repeat({}, 1fr);", cols))}
            >
                { move || {
                    if let Some(page_fn) = pages.get(current_page.get()) {
                        page_fn()
                    } else {
                        ().into_any()
                    }
                }}
            </div>
            <div class="form-section-pagination-controls">
                <button
                    type="button"
                    on:click={move |_| current_page.update(|p| if *p > 0 { *p -= 1; })}
                    disabled={move || current_page.get() == 0}
                >{"Prev"}</button>
                <span>{move || format!("Page {} of {}", current_page.get() + 1, total_pages)}</span>
                <button
                    type="button"
                    on:click={move |_| current_page.update(|p| if *p + 1 < total_pages { *p += 1; })}
                    disabled={move || current_page.get() + 1 == total_pages}
                >{"Next"}</button>
            </div>
        </div>
    }
}
