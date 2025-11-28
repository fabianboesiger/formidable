use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Meta, MetaNameValue, Expr, Lit};

#[proc_macro_derive(Form, attributes(form))]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_form_macro(&ast)
}

// Helper function to create String from expression (for top-level generation)
fn create_string_from_expr(expr: &Expr) -> proc_macro2::TokenStream {
    match expr {
        Expr::Lit(expr_lit) => {
            if let Lit::Str(_) = &expr_lit.lit {
                quote! {
                    String::from(#expr)
                }
            } else {
                panic!("Only string literals are supported");
            }
        },
        #[cfg(feature = "leptos_i18n")]
        Expr::Path(_) => {
            quote! {
                {
                    let i18n = crate::app::i18n::use_i18n();
                    String::from(leptos_i18n::tu_string!(i18n, #expr))
                }
            }
        },
        _ => {
            panic!("Only string literals and i18n paths are supported");
        }
    }
}

// Unified form attribute configuration parsing
#[derive(Default)]
struct FieldConfigurationParser {
    label: Option<Expr>,
    description: Option<Expr>,
    render_as: Option<String>,
    class: Option<String>,
    columns: Option<u32>,
    colspan: Option<u32>,
    placeholder: Option<String>,
}

impl FieldConfigurationParser {
    fn parse_from_attributes(attrs: &[Attribute]) -> Self {
        let mut config = Self::default();
        
        for attr in attrs {
            if attr.path().is_ident("form") {
                match &attr.meta {
                    Meta::List(meta_list) => {
                        let parsed = meta_list.parse_args_with(|input: syn::parse::ParseStream| {
                            let mut pairs = Vec::new();
                            
                            while !input.is_empty() {
                                let name: syn::Ident = input.parse()?;
                                input.parse::<syn::Token![=]>()?;
                                let value: syn::Expr = input.parse()?;
                                pairs.push((name, value));
                                
                                // Handle optional comma
                                if input.peek(syn::Token![,]) {
                                    input.parse::<syn::Token![,]>()?;
                                }
                            }
                            
                            Ok(pairs)
                        });
                        
                        if let Ok(pairs) = parsed {
                            for (name, value) in pairs {
                                match name.to_string().as_str() {
                                    "label" => config.label = Some(value),
                                    "description" => config.description = Some(value),
                                    "render_as" => {
                                        if let Expr::Lit(expr_lit) = &value {
                                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                                config.render_as = Some(lit_str.value());
                                            }
                                        }
                                    },
                                    "class" => {
                                        if let Expr::Lit(expr_lit) = &value {
                                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                                config.class = Some(lit_str.value());
                                            }
                                        }
                                    },
                                    "columns" => {
                                        if let Expr::Lit(expr_lit) = &value {
                                            if let Lit::Int(lit_int) = &expr_lit.lit {
                                                if let Ok(columns) = lit_int.base10_parse::<u32>() {
                                                    config.columns = Some(columns);
                                                }
                                            }
                                        }
                                    },
                                    "colspan" => {
                                        if let Expr::Lit(expr_lit) = &value {
                                            if let Lit::Int(lit_int) = &expr_lit.lit {
                                                if let Ok(colspan) = lit_int.base10_parse::<u32>() {
                                                    config.colspan = Some(colspan);
                                                }
                                            }
                                        }
                                    },
                                    "placeholder" => {
                                        if let Expr::Lit(expr_lit) = &value {
                                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                                config.placeholder = Some(lit_str.value());
                                            }
                                        }
                                    },
                                    _ => {} // Ignore unknown attributes
                                }
                            }
                        }
                    },
                    Meta::NameValue(MetaNameValue { path, value, .. }) => {
                        // Handle single name-value pairs like #[form(label = "value")]
                        if path.is_ident("label") {
                            config.label = Some(value.clone());
                        } else if path.is_ident("description") {
                            config.description = Some(value.clone());
                        } else if path.is_ident("render_as") {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Str(lit_str) = &expr_lit.lit {
                                    config.render_as = Some(lit_str.value());
                                }
                            }
                        } else if path.is_ident("class") {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Str(lit_str) = &expr_lit.lit {
                                    config.class = Some(lit_str.value());
                                }
                            }
                        } else if path.is_ident("columns") {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Int(lit_int) = &expr_lit.lit {
                                    if let Ok(columns) = lit_int.base10_parse::<u32>() {
                                        config.columns = Some(columns);
                                    }
                                }
                            }
                        } else if path.is_ident("colspan") {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Int(lit_int) = &expr_lit.lit {
                                    if let Ok(colspan) = lit_int.base10_parse::<u32>() {
                                        config.colspan = Some(colspan);
                                    }
                                }
                            }
                        } else if path.is_ident("placeholder") {
                            if let Expr::Lit(expr_lit) = value {
                                if let Lit::Str(lit_str) = &expr_lit.lit {
                                    config.placeholder = Some(lit_str.value());
                                }
                            }
                        }
                    },
                    _ => {} // Ignore other meta types
                }
            }
        }
        
        config
    }
    

    fn to_field_configuration(&self) -> proc_macro2::TokenStream {       
        let label =  create_string_from_expr(self.label.as_ref().expect("Label is required"));
        let label = quote! { leptos::prelude::TextProp::from(#label) };

        let description = if let Some(desc_expr) = &self.description {
            let description = create_string_from_expr(desc_expr);
            quote! { Some(leptos::prelude::TextProp::from(#description)) }
        } else {
            quote! { None }
        };

        let class = if let Some(class_str) = &self.class {
            quote! { Some(String::from(#class_str)) }
        } else {
            quote! { None }
        };

        let colspan = if let Some(colspan_val) = &self.colspan {
            quote! { Some(#colspan_val) }
        } else {
            quote! { None }
        };

        let placeholder = if let Some(placeholder_str) = &self.placeholder {
            quote! { Some(String::from(#placeholder_str)) }
        } else {
            quote! { None }
        };

        quote! {
            formidable::FieldConfiguration {
                label: Some(#label),
                description: #description,
                class: #class,
                colspan: #colspan,
                placeholder: #placeholder,
            }
        }
    }

    fn label_string(&self) -> proc_macro2::TokenStream {
        let label = self.label.as_ref().expect("Label is required");
        create_string_from_expr(label)
    }
}

// Shared field processing logic to eliminate duplication
struct FieldProcessor;

impl FieldProcessor {
    /// Generate field signals for tracking field state
    fn generate_field_signals(
        fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
        enum_name: Option<&syn::Ident>,
        variant_name: Option<&syn::Ident>,
    ) -> Vec<proc_macro2::TokenStream> {
        fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let signal_name = quote::format_ident!("{}_signal", field_name);
            let field_type = &field.ty;
            
            let initial_value = if let (Some(enum_name), Some(variant_name)) = (enum_name, variant_name) {
                quote! {
                    match value.as_ref() {
                        Some(#enum_name::#variant_name { #field_name, .. }) => Some(Ok(#field_name.clone())),
                        _ => None,
                    }
                }
            } else {
                quote! { value.as_ref().map(|v| Ok(v.#field_name.clone())) }
            };
            
            quote! {
                let #signal_name: leptos::prelude::RwSignal<Option<Result<#field_type, formidable::FormError>>> = 
                    leptos::prelude::RwSignal::new(#initial_value);
            }
        }).collect()
    }
    
    /// Generate field signal names for validation logic
    fn generate_field_signal_names(
        fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    ) -> Vec<proc_macro2::Ident> {
        fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            quote::format_ident!("{}_signal", field_name)
        }).collect()
    }
    
    /// Generate field constructor expressions for building structs/enums
    fn generate_field_constructor(
        fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    ) -> Vec<proc_macro2::TokenStream> {
        fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let signal_name = quote::format_ident!("{}_signal", field_name);
            quote! {
                #field_name: #signal_name.get_untracked().and_then(|r| r.ok()).expect("Field should be valid when all_ok is true")
            }
        }).collect()
    }
    
    /// Generate field form UI elements
    fn generate_field_forms(
        fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    ) -> Vec<proc_macro2::TokenStream> {
        fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_name_str = field_name.to_string();
            let field_type = &field.ty;
            let form_config = FieldConfigurationParser::parse_from_attributes(&field.attrs);
            let signal_name = quote::format_ident!("{}_signal", field_name);
            let field_configuration = form_config.to_field_configuration();
            
            quote! {
                {
                    let field_name_as_name = name.push_key(#field_name_str);
                    let field_value = #signal_name.get_untracked().and_then(|r| r.ok());
                    let field_callback = Some(leptos::prelude::Callback::new(move |result: Result<#field_type, formidable::FormError>| {
                        #signal_name.set(Some(result));
                    }));
                    
                    <#field_type as Form>::view(
                        #field_configuration,
                        field_name_as_name, 
                        field_value, 
                        field_callback
                    )
                }
            }
        }).collect()
    }
    
    /// Generate unified callback effect for field validation and construction
    fn generate_callback_effect(
        field_signal_names: &[proc_macro2::Ident],
        constructor_type: ConstructorType,
    ) -> proc_macro2::TokenStream {
        let constructor = match constructor_type {
            ConstructorType::Struct { name, field_constructor } => {
                quote! {
                    let merged_struct = #name {
                        #(#field_constructor),*
                    };
                    parent_callback.run(Ok(merged_struct));
                }
            }
            ConstructorType::EnumVariant { enum_name, variant_name, field_constructor } => {
                quote! {
                    let new_enum_value = #enum_name::#variant_name {
                        #(#field_constructor),*
                    };
                    parent_callback.run(Ok(new_enum_value));
                }
            }
        };
        
        quote! {
            if let Some(parent_callback) = callback {
                leptos::prelude::Effect::new(move || {
                    // Check if all fields have some value (either Ok or Err)
                    let all_fields_have_values = #(#field_signal_names.get().is_some())&&*;
                    
                    if all_fields_have_values {
                        // All fields have been touched, now check if all are valid
                        let all_ok = #(#field_signal_names.get().map(|r| r.is_ok()).unwrap_or(false))&&*;
                        
                        if all_ok {
                            // All fields are valid, construct the result
                            #constructor
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
        }
    }
}

/// Constructor type for generating different construction logic
enum ConstructorType<'a> {
    Struct {
        name: &'a syn::Ident,
        field_constructor: &'a [proc_macro2::TokenStream],
    },
    EnumVariant {
        enum_name: &'a syn::Ident,
        variant_name: &'a syn::Ident,
        field_constructor: &'a [proc_macro2::TokenStream],
    },
}

fn generate_empty_struct_form(name: &syn::Ident) -> TokenStream {
    let generated = quote! {
        impl Form for #name {
            fn view(
                _field: formidable::FieldConfiguration,
                _name: formidable::Name,
                _value: Option<Self>,
                callback: Option<leptos::prelude::Callback<Result<Self, formidable::FormError>>>,
            ) -> impl leptos::prelude::IntoView {
                use leptos::prelude::*;
                
                // For empty structs, immediately call callback with the struct instance
                if let Some(callback) = callback {
                    Effect::new(move |_| {
                        callback.run(Ok(#name));
                    });
                }

                // Render nothing for unit structs
                view! {}.into_any()
            }
        }
    };

    generated.into()
}

fn impl_form_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    match &ast.data {
        syn::Data::Struct(data_struct) => impl_form_for_struct(name, data_struct, ast),
        syn::Data::Enum(data_enum) => impl_form_for_enum(name, data_enum, ast),
        _ => panic!("Form can only be derived for structs and enums"),
    }
}

fn impl_form_for_enum(name: &syn::Ident, data_enum: &syn::DataEnum, ast: &syn::DeriveInput) -> TokenStream {
    let variants = &data_enum.variants;
    
    // Create a discriminant enum for variant selection
    let discriminant_name = quote::format_ident!("{}Discriminant", name);
    
    // Generate discriminant enum variants with strum attributes
    let discriminant_variants: Vec<_> = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        
        quote! {
            #variant_name 
        }
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
        let form_config = FieldConfigurationParser::parse_from_attributes(&variant.attrs);
        let field_configuration = form_config.to_field_configuration();

        
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
                        
                        ().into_any()
                    }
                }
            },
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    // Single unnamed field - use variant's form attributes for field configuration
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
                                #field_configuration,
                                name, 
                                field_value, 
                                field_callback
                            ).into_any()
                        }
                    }
                } else {
                    // Multiple unnamed fields - simplified for now
                    panic!("Multiple unnamed fields in enum variants not supported");
                }
            },
            syn::Fields::Named(fields) => {
                // Named fields - use shared field processing logic
                let field_signals = FieldProcessor::generate_field_signals(&fields.named, Some(name), Some(variant_name));
                let field_signal_names = FieldProcessor::generate_field_signal_names(&fields.named);
                let field_constructor = FieldProcessor::generate_field_constructor(&fields.named);
                let field_forms = FieldProcessor::generate_field_forms(&fields.named);
           
                let callback_effect = FieldProcessor::generate_callback_effect(
                    &field_signal_names,
                    ConstructorType::EnumVariant {
                        enum_name: name,
                        variant_name,
                        field_constructor: &field_constructor,
                    },
                );
                
                quote! {
                    #discriminant_name::#variant_name => {                        
                        #(#field_signals)*
                        
                        #callback_effect

                        let field_configuration = #field_configuration;
                        
                        view! {
                            <formidable::components::Section name=name heading={field_configuration.label.clone()}>
                                #(#field_forms)*
                            </formidable::components::Section>
                        }.into_any()
                    }
                }
            }
        }
    }).collect();
    
    // Generate match arms for discriminant value_label (used in variant selector)
    let discriminant_value_label_match_arms: Vec<_> = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let form_config = FieldConfigurationParser::parse_from_attributes(&variant.attrs);
        let label_string = form_config.label_string();
        
        quote! { #discriminant_name::#variant_name => {
            write!(f, "{}", #label_string)
        } }
    }).collect();
    
    // Parse enum attributes to determine variant selection type and class
    let enum_config = FieldConfigurationParser::parse_from_attributes(&ast.attrs);
    let variant_selection_type = enum_config.render_as.as_deref().unwrap_or("radio");
    let enum_class = if let Some(class_str) = &enum_config.class {
        format!("enum {}", class_str)
    } else {
        "enum".to_string()
    };

    // Generate the variant selector component at compile time
    let variant_selector = if variant_selection_type == "select" {
        quote! {
            view! { <components::Select label=field.label.expect("No label provided") name=name.push_key("variant") value=selected_discriminant class=field.class colspan=field.colspan /> }.into_any()
        }
    } else if variant_selection_type == "radio" {
        quote! {
            view! { <components::Radio label=field.label.expect("No label provided") name=name.push_key("variant") value=selected_discriminant class=field.class colspan=field.colspan /> }.into_any()
        }
    } else {
        panic!("Unsupported render_as type: {}", variant_selection_type);
    };

    let generated = quote! {
        impl Form for #name {
            fn view(
                field: formidable::FieldConfiguration,
                name: formidable::Name,
                value: Option<Self>,
                callback: Option<leptos::prelude::Callback<Result<Self, formidable::FormError>>>,
            ) -> impl leptos::prelude::IntoView {
                use leptos::prelude::*;
                use formidable::components;
                
                // For now, create a simple discriminant enum inline
                #[derive(Clone, Copy, Debug, PartialEq, Eq, formidable::strum::IntoStaticStr, formidable::strum::VariantArray, Default)]
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

                impl std::fmt::Display for #discriminant_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#discriminant_value_label_match_arms)*
                        }
                    }
                }
                
                view! {
                    <div 
                        class={#enum_class}
                        style={field.colspan.map(|cols| format!("grid-column: span {};", cols))}
                    >
                        // Variant selector
                        <div class="enum-variant-selector">
                            { #variant_selector }
                        </div>
                        
                        // Variant-specific form
                        <div class="enum-variant">
                            {move || {
                                match selected_discriminant.get() {
                                    #(#variant_forms)*
                                }
                            }}
                        </div>
                    </div>
                }.into_any()
            }
        }
    };

    generated.into()
}

fn impl_form_for_struct(name: &syn::Ident, data_struct: &syn::DataStruct, ast: &syn::DeriveInput) -> TokenStream {
    // Parse the struct data
    let fields = match &data_struct.fields {
        syn::Fields::Named(fields_named) => &fields_named.named,
        syn::Fields::Unit => {
            // Handle unit structs (no fields) - they should render nothing
            return generate_empty_struct_form(name);
        }
        _ => panic!("Form can only be derived for structs with named fields or unit structs"),
    };

    // Parse struct attributes to get class, columns, and render_as
    let struct_config = FieldConfigurationParser::parse_from_attributes(&ast.attrs);
    let render_as_type = struct_config.render_as.as_deref().unwrap_or("section");
    let struct_class = if let Some(class_str) = &struct_config.class {
        quote! { Some(String::from(#class_str)) }
    } else {
        quote! { None }
    };
    let struct_columns = if let Some(columns_val) = &struct_config.columns {
        quote! { Some(#columns_val) }
    } else {
        quote! { None }
    };

    // Use shared field processing logic
    let field_signals = FieldProcessor::generate_field_signals(fields, None, None);
    let field_signal_names = FieldProcessor::generate_field_signal_names(fields);
    let field_constructor = FieldProcessor::generate_field_constructor(fields);
    let field_forms = FieldProcessor::generate_field_forms(fields);
    
    let callback_effect = FieldProcessor::generate_callback_effect(
        &field_signal_names,
        ConstructorType::Struct {
            name,
            field_constructor: &field_constructor,
        },
    );

    let generated = quote! {
        impl Form for #name {
            fn view(
                field: formidable::FieldConfiguration,
                name: formidable::Name,
                value: Option<Self>,
                callback: Option<leptos::prelude::Callback<Result<Self, formidable::FormError>>>,
            ) -> impl leptos::prelude::IntoView {
                use leptos::prelude::*;
                
                // Create signals for each field to track their state
                #(#field_signals)*

                #callback_effect

                // Choose between Section and PaginatedSection based on render_as attribute
                if #render_as_type == "paginate" {
                    let pages: Vec<Box<dyn Fn() -> leptos::prelude::AnyView + Send + Sync>> = vec![
                        #(Box::new(move || #field_forms.into_any())),*
                    ];
                    view! {
                        <formidable::components::PaginatedSection name=name heading={field.label} description={field.description} class=#struct_class columns=#struct_columns colspan={field.colspan} pages=pages />
                    }.into_any()
                } else if #render_as_type == "section" {
                    view! {
                        <formidable::components::Section name=name heading={field.label} description={field.description} class=#struct_class columns=#struct_columns colspan={field.colspan}>
                            #(#field_forms)*
                        </formidable::components::Section>
                    }.into_any()
                } else {
                    panic!("Unsupported render_as type for struct: {}. Supported values are 'section' (default) and 'paginate'", #render_as_type);
                }
            }
        }
    };

    generated.into()
}
