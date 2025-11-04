use crate::{
    components::{Checkbox, InputType, Section},
    t,
    types::FormType,
    FieldError, Form, FormError, FormMessage, Name,
};
use leptos::prelude::*;
use uuid::Uuid;

macro_rules! impl_form_for_int {
    ($($type:ty),*) => {
        $(
            impl FormType for $type {
                const INPUT_TYPE: InputType = InputType::Number;
                const MIN: Option<Self> = Some(<$type>::MIN);
                const MAX: Option<Self> = Some(<$type>::MAX);
                const STEP: Option<Self> = Some(1 as $type);
                const REQUIRED: Option<bool> = Some(true);
            }
        )*
    };
}

// Implement Form for all standard integer types
impl_form_for_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

macro_rules! impl_form_for_float {
    ($($type:ty),*) => {
        $(
            impl FormType for $type {
                const INPUT_TYPE: InputType = InputType::Number;
                const REQUIRED: Option<bool> = Some(true);
            }
        )*
    };
}

// Implement Form for all standard integer types
impl_form_for_float!(f32, f64);

impl FormType for String {
    const INPUT_TYPE: InputType = InputType::Text;
}

impl Form for bool {
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        view! {
            <Checkbox<bool>
                label=field.label.expect("No label provided")
                description=field.description
                name=name
                value=value
                callback={callback.map(|callback| Callback::new(move |v: Result<Self, FieldError>| {
                    callback.run(v.map_err(FormError::from));
                }))}
            />
        }
    }
}

impl<T> Form for Vec<T>
where
    T: Form + Clone + Send + Sync + 'static,
{
    fn view(
        field: crate::FieldConfiguration,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
    ) -> impl IntoView {
        #[derive(Clone)]
        struct Child<T> {
            id: Uuid,
            value: Option<Result<T, FormError>>,
        }

        impl<T> Child<T> {
            fn new() -> Self {
                Self {
                    id: Uuid::new_v4(),
                    value: None,
                }
            }
        }

        impl<T> From<T> for Child<T> {
            fn from(value: T) -> Self {
                Self {
                    id: Uuid::new_v4(),
                    value: Some(Ok(value)),
                }
            }
        }

        let children: RwSignal<Vec<Child<T>>> = RwSignal::new(
            value
                .map(|v| v.into_iter().map(Child::from).collect())
                .unwrap_or_default(),
        );

        if let Some(callback) = callback {
            Effect::new(move |_| {
                #[allow(clippy::manual_try_fold)]
                let results: Result<Vec<T>, FormError> =
                    children
                        .get()
                        .iter()
                        .cloned()
                        .fold(Ok(Vec::new()), |acc, c| match (acc, c.value) {
                            (val, None) => val,
                            (Ok(mut vec), Some(Ok(ref val))) => {
                                vec.push(val.clone());
                                Ok(vec)
                            }
                            (Ok(_), Some(Err(e))) => Err(e),
                            (Err(e), Some(Err(e2))) => {
                                let mut combined = e;
                                combined.extend(e2);
                                Err(combined)
                            }
                            (Err(e), _) => Err(e),
                        });

                callback.run(results);
            });
        }

        view! {
            <Section name=name heading={field.label.expect("No label provided").clone()}>
                <For
                    each={move || children.get().into_iter().enumerate()}
                    key={move |(_, child)| child.id}
                    children={move |(index, child)| {

                        view! {
                            <div class={format!("array-item item-{}", child.id)}>
                                {T::view(crate::FieldConfiguration {
                                    label: None,
                                    description: None,
                                }, name.push_index(index), child.value.and_then(|v| v.ok()), Some(Callback::new(move |v: Result<T, FormError>| {
                                    let mut children = children.write();
                                    if let Some(pos) = children.iter().position(|c| c.id == child.id) {
                                        children[pos].value = Some(v);
                                    }
                                })))}
                                <button
                                    type="button"
                                    class="array-remove-button"
                                    on:click={move |_| {
                                        children.update(move |children| {
                                            children.retain(|c| c.id != child.id);
                                        });
                                    }}
                                >{t(FormMessage::RemoveButton)}</button>
                            </div>
                        }
                    }}
                />
                <button
                    type="button"
                    class="array-add-button"
                    on:click={move |_| {
                        children.update(move |children| {
                            children.push(Child::new());
                        });
                    }}
                >{t(FormMessage::AddButton)}</button>
            </Section>
        }
    }
}

/*
impl<T> Form for Option<T>
where
    T: Form + Clone + Send + Sync + 'static,
{
    fn view(
        label: &'static str,
        name: Name,
        value: Option<Self>,
        callback: Option<Callback<Result<Self, FormError>>>,
        required: bool,
    ) -> impl IntoView {
        let inner_callback = callback.map(|callback| {
            Callback::new(move |v: Result<T, FormError>| {
                callback.run(Ok(v.ok()));
            })
        });

        let value = value.flatten();

        view! {
            {T::view(label, name, value, inner_callback, false)}
        }
    }
}
*/
