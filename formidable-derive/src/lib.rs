use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Meta};

#[proc_macro_derive(Form, attributes(form))]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_form_macro(&ast)
}

fn parse_field_label(attrs: &[Attribute], field_name: &str) -> String {
    for attr in attrs {
        if attr.path().is_ident("form") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens = meta_list.tokens.to_string();
                // Parse tokens looking for label="value"
                if let Some(start) = tokens.find("label = \"") {
                    let start_idx = start + 9; // length of "label = \""
                    if let Some(end_idx) = tokens[start_idx..].find("\"") {
                        return tokens[start_idx..start_idx + end_idx].to_string();
                    }
                }
            }
        }
    }
    // Fallback to field name if no label attribute found
    field_name.to_string()
}

fn impl_form_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    match &ast.data {
        syn::Data::Struct(data_struct) => impl_form_for_struct(name, data_struct),
        syn::Data::Enum(_data_enum) => impl_form_for_enum(name),
        _ => panic!("Form can only be derived for structs and enums"),
    }
}

fn impl_form_for_enum(name: &syn::Ident) -> TokenStream {
    let generated = quote! {
        impl Form for #name {
            fn view(
                label: &'static str,
                name: formidable::Name,
                value: Option<Self>,
                callback: Option<leptos::prelude::Callback<Result<Self, formidable::FormError>>>,
            ) -> impl leptos::prelude::IntoView {
                use leptos::prelude::*;
                use formidable::components;

                let value = RwSignal::new(value.unwrap_or_default());

                if let Some(callback) = callback {
                    Effect::new(move |_| {
                        callback.run(Ok(value.get()));
                    });
                }

                if Self::VARIANTS.len() > 5 {
                    view! { <components::Select<#name> label=label name=name value=value /> }.into_any()
                } else {
                    view! { <components::Radio<#name> label=label name=name value=value /> }.into_any()
                }
            }
        }
    };

    generated.into()
}

fn impl_form_for_struct(name: &syn::Ident, data_struct: &syn::DataStruct) -> TokenStream {
    // Parse the struct data
    let fields = match &data_struct.fields {
        syn::Fields::Named(fields_named) => &fields_named.named,
        _ => panic!("Form can only be derived for structs with named fields"),
    };

    // Generate field signals - one for each field to track its state
    let field_signal_declarations: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let signal_name = quote::format_ident!("{}_signal", field_name);
            
            quote! {
                let #signal_name: leptos::prelude::RwSignal<Option<Result<#field_type, formidable::FormError>>> = 
                    leptos::prelude::RwSignal::new(value.as_ref().map(|v| Ok(v.#field_name.clone())));
            }
        })
        .collect();

    // Generate field names for checking all are OK
    let field_signal_names: Vec<_> = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote::format_ident!("{}_signal", field_name)
    }).collect();

    // Generate the merge logic to construct the struct from all field results
    // Only construct when ALL fields are valid
    let field_constructor: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let signal_name = quote::format_ident!("{}_signal", field_name);
            
            quote! {
                #field_name: #signal_name.get_untracked().and_then(|r| r.ok()).expect("Field should be valid when all_ok is true")
            }
        })
        .collect();

    // Generate field inputs
    let field_inputs: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_name_str = field_name.to_string();
            let field_type = &field.ty;
            let field_label = parse_field_label(&field.attrs, &field_name_str);
            let signal_name = quote::format_ident!("{}_signal", field_name);

            quote! {
                {
                    let field_name_as_name = name.push_key(#field_name_str);
                    let field_value = #signal_name.get_untracked().and_then(|r| r.ok());
                    let field_callback = Some(leptos::prelude::Callback::new(move |result: Result<#field_type, formidable::FormError>| {
                        #signal_name.set(Some(result));
                    }));
                    
                    <#field_type as Form>::view(#field_label, field_name_as_name, field_value, field_callback)
                }
            }
        })
        .collect();

    let generated = quote! {
        impl Form for #name {
            fn view(
                label: &'static str,
                name: formidable::Name,
                value: Option<Self>,
                callback: Option<leptos::prelude::Callback<Result<Self, formidable::FormError>>>,
            ) -> impl leptos::prelude::IntoView {
                use leptos::prelude::*;

                // Create signals for each field to track their state
                #(#field_signal_declarations)*

                // Create an effect to monitor all field signals and call parent callback when appropriate
                if let Some(parent_callback) = callback {
                    leptos::prelude::Effect::new(move || {
                        // Check if all fields have some value (either Ok or Err)
                        let all_fields_have_values = #(#field_signal_names.get().is_some())&&*;
                        
                        if all_fields_have_values {
                            // All fields have been touched, now check if all are valid
                            let all_ok = #(#field_signal_names.get().map(|r| r.is_ok()).unwrap_or(false))&&*;
                            
                            if all_ok {
                                // All fields are valid, construct the struct
                                let merged_struct = #name {
                                    #(#field_constructor),*
                                };
                                parent_callback.run(Ok(merged_struct));
                            } else {
                                // Some fields have errors, collect and merge them
                                let mut merged_error = formidable::FormError::from(vec![]);
                                #(
                                    if let Some(Err(err)) = #field_signal_names.get() {
                                        merged_error.extend(err);
                                    }
                                )*
                                parent_callback.run(Err(merged_error));
                            }
                        }
                    });
                }

                view! {
                    <fieldset>
                        <legend>{label}</legend>
                        #(#field_inputs)*
                    </fieldset>
                }.into_any()
            }
        }
    };

    generated.into()
}
