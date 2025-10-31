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
        syn::Data::Enum(data_enum) => impl_form_for_enum(name, data_enum),
        _ => panic!("Form can only be derived for structs and enums"),
    }
}

fn impl_form_for_enum(name: &syn::Ident, data_enum: &syn::DataEnum) -> TokenStream {
    // All enums now use the complex implementation
    impl_form_for_complex_enum(name, data_enum)
}

fn impl_form_for_complex_enum(name: &syn::Ident, data_enum: &syn::DataEnum) -> TokenStream {
    let variants = &data_enum.variants;
    
    // Create a discriminant enum for variant selection
    let discriminant_name = quote::format_ident!("{}Discriminant", name);
    
    // Generate discriminant enum variants
    let discriminant_variants: Vec<_> = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! { #variant_name }
    }).collect();
    
    // Generate match arms for discriminant detection
    let discriminant_match_arms: Vec<_> = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Unit => {
                quote! { #name::#variant_name => #discriminant_name::#variant_name }
            },
            syn::Fields::Unnamed(_) => {
                quote! { #name::#variant_name(..) => #discriminant_name::#variant_name }
            },
            syn::Fields::Named(_) => {
                quote! { #name::#variant_name { .. } => #discriminant_name::#variant_name }
            }
        }
    }).collect();
    
    // Generate variant forms - simplified version for now
    let variant_forms: Vec<_> = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        
        match &variant.fields {
            syn::Fields::Unit => {
                // No fields, just the variant selection - call callback immediately when this variant is selected
                quote! {
                    #discriminant_name::#variant_name => {
                        // For unit variants, call the callback immediately with the variant
                        if let Some(parent_callback) = callback {
                            leptos::prelude::Effect::new(move || {
                                let new_enum_value = #name::#variant_name;
                                parent_callback.run(Ok(new_enum_value));
                            });
                        }
                        
                        view! { <span class="unit-variant-info">"No additional fields required"</span> }.into_any()
                    }
                }
            },
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    // Single unnamed field - don't add to name path
                    let field_type = &fields.unnamed.first().unwrap().ty;
                    quote! {
                        #discriminant_name::#variant_name => {
                            let field_value = match value.as_ref() {
                                Some(#name::#variant_name(inner)) => Some(inner.clone()),
                                _ => None,
                            };
                            let field_callback = callback.map(|cb| leptos::prelude::Callback::new(move |result: Result<#field_type, formidable::FormError>| {
                                match result {
                                    Ok(inner_value) => {
                                        let new_enum_value = #name::#variant_name(inner_value);
                                        cb.run(Ok(new_enum_value));
                                    },
                                    Err(err) => cb.run(Err(err)),
                                }
                            }));
                            
                            <#field_type as Form>::view(
                                stringify!(#variant_name), 
                                name, 
                                field_value, 
                                field_callback
                            ).into_any()
                        }
                    }
                } else {
                    // Multiple unnamed fields - simplified for now
                    quote! {
                        #discriminant_name::#variant_name => {
                            view! {
                                <div>
                                    <p>"Complex tuple variants not yet fully implemented"</p>
                                </div>
                            }.into_any()
                        }
                    }
                }
            },
            syn::Fields::Named(fields) => {
                // Named fields - treat like a struct
                let field_signals: Vec<_> = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let signal_name = quote::format_ident!("{}_signal", field_name);
                    let field_type = &field.ty;
                    quote! {
                        let #signal_name: leptos::prelude::RwSignal<Option<Result<#field_type, formidable::FormError>>> = 
                            leptos::prelude::RwSignal::new(
                                match value.as_ref() {
                                    Some(#name::#variant_name { #field_name, .. }) => Some(Ok(#field_name.clone())),
                                    _ => None,
                                }
                            );
                    }
                }).collect();
                
                let field_signal_names: Vec<_> = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    quote::format_ident!("{}_signal", field_name)
                }).collect();
                
                let field_constructor: Vec<_> = fields.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    let signal_name = quote::format_ident!("{}_signal", field_name);
                    quote! {
                        #field_name: #signal_name.get_untracked().and_then(|r| r.ok()).expect("Field should be valid when all_ok is true")
                    }
                }).collect();
                
                let field_forms: Vec<_> = fields.named.iter().map(|field| {
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
                            
                            <#field_type as Form>::view(
                                #field_label, 
                                field_name_as_name, 
                                field_value, 
                                field_callback
                            )
                        }
                    }
                }).collect();
                
                quote! {
                    #discriminant_name::#variant_name => {
                        #(#field_signals)*
                        
                        // Set up callback for parent when all fields are valid
                        if let Some(parent_callback) = callback {
                            leptos::prelude::Effect::new(move || {
                                // Check if all fields have some value (either Ok or Err)
                                let all_fields_have_values = #(#field_signal_names.get().is_some())&&*;
                                
                                if all_fields_have_values {
                                    // All fields have been touched, now check if all are valid
                                    let all_ok = #(#field_signal_names.get().map(|r| r.is_ok()).unwrap_or(false))&&*;
                                    
                                    if all_ok {
                                        // All fields are valid, construct the enum variant
                                        let new_enum_value = #name::#variant_name {
                                            #(#field_constructor),*
                                        };
                                        parent_callback.run(Ok(new_enum_value));
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
                                <legend>{stringify!(#variant_name)}</legend>
                                #(#field_forms)*
                            </fieldset>
                        }.into_any()
                    }
                }
            }
        }
    }).collect();
    
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
                
                // For now, create a simple discriminant enum inline
                #[derive(Clone, Copy, Debug, PartialEq, Eq, strum::Display, strum::IntoStaticStr, strum::VariantArray, Default)]
                enum #discriminant_name {
                    #[default]
                    #(#discriminant_variants),*
                }
                
                // Determine current discriminant from value
                let current_discriminant = value.as_ref().map(|v| {
                    match v {
                        #(#discriminant_match_arms,)*
                    }
                }).unwrap_or_default();
                
                let selected_discriminant = RwSignal::new(current_discriminant);
                
                // Handle variant changes - when discriminant changes, trigger the appropriate form
                // The individual variant forms will handle calling the callback with the appropriate values
                
                view! {
                    <fieldset>
                        <legend>{label}</legend>
                        
                        // Variant selector
                        <div class="variant-selector">
                            {
                                if #discriminant_name::VARIANTS.len() > 5 {
                                    view! { <components::Select<#discriminant_name> label="Variant" name=name.push_key("variant") value=selected_discriminant /> }.into_any()
                                } else {
                                    view! { <components::Radio<#discriminant_name> label="Variant" name=name.push_key("variant") value=selected_discriminant /> }.into_any()
                                }
                            }
                        </div>
                        
                        // Variant-specific form
                        <div class="variant-form">
                            {move || {
                                match selected_discriminant.get() {
                                    #(#variant_forms)*
                                }
                            }}
                        </div>
                    </fieldset>
                }.into_any()
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
