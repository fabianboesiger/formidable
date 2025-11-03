use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::types::FileError;
use crate::{types::File, FieldError, Name};
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

// Helper function to read file as bytes
async fn read_file_as_bytes(
    file: web_sys::File,
) -> Result<Vec<u8>, web_sys::wasm_bindgen::JsValue> {
    let array_buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await?;
    let uint8_array = web_sys::js_sys::Uint8Array::new(&array_buffer);
    Ok(uint8_array.to_vec())
}

#[component]
pub fn FileInput(
    #[prop(into)] label: TextProp,
    #[prop(into, default = None)] description: Option<TextProp>,
    #[prop(into)] name: Name,
    #[prop(into)] value: Option<File>,
    #[prop(into)] callback: Option<Callback<Result<File, FieldError>>>,
) -> impl IntoView {
    let node_ref = NodeRef::new();
    let touched = RwSignal::new(false);
    let current_file = RwSignal::new(value.ok_or(FileError::NoFileSelected));

    if let Some(callback) = callback {
        Effect::new(move |_| {
            let result = current_file.get().map_err(|err| FieldError::new(name, err));
            callback.run(result);
        });
    }

    let update_current_file = move |input: web_sys::HtmlInputElement| {
        if let Some(files) = input.files() {
            if files.length() > 0 {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();
                    let mime_type = file.type_();

                    let file_clone = file.clone();

                    spawn_local(async move {
                        match read_file_as_bytes(file_clone).await {
                            Ok(data) => {
                                let our_file = File::new(file_name, mime_type, data);
                                current_file.set(Ok(our_file));
                            }
                            Err(_) => {
                                current_file.set(Err(FileError::UnknownFileError));
                            }
                        }
                    });
                }
            } else {
                current_file.set(Err(FileError::NoFileSelected));
            }
        }
    };

    let handle_file_change = move |ev: web_sys::Event| {
        touched.set(true);

        if let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            update_current_file(input);
        }
    };

    node_ref.on_load(move |elem: leptos::web_sys::HtmlInputElement| {
        update_current_file(elem);
    });

    view! {
        <div class:error={move || touched.get() && current_file.get().is_err()} class="field file-input-field">
            <label for=name.to_string()>{label.get()}
                <span class="custom custom-file-input"></span>
                <input
                    node_ref=node_ref
                    type="file"
                    name=name.to_string()
                    id=name.to_string()
                    on:focus=move |_| {
                        touched.set(true);
                    }
                    on:change=handle_file_change
                />
            </label>
            { move || current_file.get().map(|file| view! {
                <span class="custom custom-file-input-filename">
                    { format!("{}", file) }
                </span>
            })}
            { move || {
                touched.get().then(move || current_file.get().err().map(|e| {
                    view! { <p class="error-message">{format!("{}", e)}</p> }
                }))
            }}
            {
                description.map(|desc| view! {
                    <p class="description">{desc.get()}</p>
                })
            }
        </div>
    }
}
