/// Defining insert form component for every table is a repetitive and unscalable task.
/// Scalability is not achievable with generic components because every table has varying number of fields of differing types and nullability.
/// This macro manages to accomplish scalability by getting rid of boilerplate and repetitiveness, while at the same time offering flexibility with fields of different types and nullability.
macro_rules! insert_form {
    (component_name: $component_name:ident,
    id: $id:ident,
    id_is_in_input: $id_is_in_input:ident,
    has_modal: $has_modal_callback:ident,
    form_input: ($variable:ident, $input:ident, $input_type:ident),
    class: $class:literal,
    state_handles: $(($state:ident, mandatory: $mandatory_callback:ident, button_disable: $button_disable_callback:ident, $onchange_callback:ident($onchange:ident, $handle_state:ident), $input_html_callback:ident(grid_column_num: $grid_column:literal))),*
) => {
        use std::rc::Rc;
        use uuid::Uuid;
        use yew::{classes, function_component, use_state_eq, TargetCast, Callback, Event, html, Html, Properties, SubmitEvent};
        use web_sys::HtmlInputElement;
        use wasm_bindgen::UnwrapThrowExt;

        #[derive(PartialEq, Properties)]
        pub struct Props {
            pub handle_insert: Callback<$variable>,
            pub close_insert_form: Callback<()>,
            pub $id: Rc<Uuid>,
            pub modal_variant_handle: $has_modal_callback!(),
        }

        // modal_variant_handle sometimes doesn't get used, depending on if the form has any dropdown selection fields.
        #[allow(unused_variables)]
        #[function_component($component_name)]
        pub fn insert_form(
            Props {
                handle_insert,
                close_insert_form,
                $id,
                modal_variant_handle,
            }: &Props,
        ) -> Html {
            $(let $handle_state = use_state_eq(|| None);)*

            $(let $state = (*$handle_state).clone();)*

            $($onchange_callback!($onchange, $handle_state);)*

            let disabled = !($($button_disable_callback!($state))&&*);

            let onsubmit = {
                let handle_insert = handle_insert.clone();
                let close_insert_form = close_insert_form.clone();
                let $id = $id.clone();

                Callback::from(move |e: SubmitEvent| {
                    e.prevent_default();
                    $(let $state = $state.clone();)*
                    if let ($($mandatory_callback!($state)),*) = ($($state),*) {
                        let variables = $variable {
                            $input: $id_is_in_input!($input_type, $id, $($state),*),
                        };
                        handle_insert.emit(variables);
                        close_insert_form.emit(());
                    }
                })
            };

            html! {
                <form {onsubmit} class={classes!("insert-form", "emitter-cell")}>
                    <fieldset class={classes!($class, "center")}>
                        <button class={classes!("entry-button")} style="grid-row: 1; grid-column: 1;" type="submit" {disabled}>{"✓"}</button>
                        // { [$($input_html_callback, $id, $onchange, modal_variant_handle),*].into_iter().enumerate().map(|(col_num, (input_html_callback, id, onchange, modal_variant_handle))| {
                        //     input_html_callback(id, col_num, onchange, modal_variant_handle)
                        // }).collect::<Html>() }
                        { [$($input_html_callback!($id, $grid_column, $onchange, modal_variant_handle)),*].into_iter().collect::<Html>() }
                    </fieldset>
                </form>
            }
        }
    };
}

macro_rules! id_is_in_input {
    ($input_type:ident, $id:ident, $($state:ident),*) => {
        $input_type {
            $id: *$id,
            $($state),*
        }
    };
}

macro_rules! id_is_not_in_input {
    ($input_type:ident, $id:ident, $($state:ident),*) => {
        $input_type {
            $($state),*
        }
    };
}

macro_rules! has_modal {
    () => {
        Callback<Option<ModalVariant>>
    }
}

macro_rules! no_modal {
    () => {
        Option<()>
    }
}

macro_rules! disable {
    ($state:ident) => {
        $state.is_some()
    };
}

macro_rules! enable {
    ($state:ident) => {
        true
    };
}

macro_rules! yes {
    ($state:ident) => {
        Some($state)
    };
}

macro_rules! no {
    ($state:ident) => {
        $state
    };
}

macro_rules! onchange_string_callback {
    ($onchange:ident, $handle_state:ident) => {
        let $onchange = Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input_value = input.value();
            $handle_state.set((!input_value.is_empty()).then(|| input_value));
        });
    };
}

macro_rules! onchange_date_callback {
    ($onchange:ident, $handle_state:ident) => {
        let $onchange = Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input_value = input.value_as_number();
            $handle_state.set((!input_value.is_nan()).then(|| {
                NaiveDateTime::from_timestamp_millis(input_value as i64)
                    .expect_throw("Unable to convert i64 to NaiveDateTime.")
                    .date()
            }));
        });
    };
}

macro_rules! onchange_float_callback {
    ($onchange:ident, $handle_state:ident) => {
        use yew::InputEvent;
        let $onchange = Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input_value = input.value_as_number();
            $handle_state.set((!input_value.is_nan()).then(|| input_value));
        });
    };
}

macro_rules! onchange_uuid_callback {
    ($onchange:ident, $handle_state:ident) => {
        let $onchange = Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input_value = input.value();
            $handle_state.set(
                (!input_value.is_empty())
                    .then(|| Uuid::parse_str(input_value.as_str()).ok())
                    .flatten(),
            );
        });
    };
}

macro_rules! onchange_pneumatic_instrument_type_callback {
    ($onchange:ident, $handle_state:ident) => {
        let $onchange = Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let input_value = input.value();
            use std::str::FromStr;
            $handle_state.set(
                (!input_value.is_empty())
                    .then(|| PneumaticInstrumentType::from_str(input.value().as_str()).ok())
                    .flatten(),
            );
        });
    };
}

macro_rules! string_input_html_callback {
    ($dropdown_id:ident, $grid_column:literal, $onchange:ident, $modal_variant_handle:ident) => {
        html! { <input type="text" style={concat!("grid-row: 1; grid-column: ", $grid_column, ';')} onchange={$onchange}/> }
    };
}

macro_rules! date_input_html_callback {
    ($dropdown_id:ident, $grid_column:literal, $onchange:ident, $modal_variant_handle:ident) => {
        html! { <input type="date" style={concat!("grid-row: 1; grid-column: ", $grid_column, ';')} onchange={$onchange}/> }
    };
}

macro_rules! float_input_html_callback {
    ($dropdown_id:ident, $grid_column:literal, $onchange:ident, $modal_variant_handle:ident) => {
        // oninput event occurs immediately after the content has been changed, while onchange occurs when the element loses focus.
        // Using oninput here because we want button to stop being disabled as soon as all mandatory fields are entered, and not having to click outide for onchange to occur.
        html! { <input type="number" step="any" style={concat!("grid-row: 1; grid-column: ", $grid_column, ';')} oninput={$onchange}/> }
    };
}

macro_rules! device_manufacturer_id_input_html_callback {
    ($dropdown_id:ident, $grid_column:literal, $onchange:ident, $modal_variant_handle:ident) => {
        html! {
            <DropdownSelectionComponent
                dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::DEVICE_MANUFACTURER_ID, id: None, modal_variant_handle: $modal_variant_handle.clone()}}
                onchange={$onchange}
                col_num={$grid_column}
            />
        }
    };
}

macro_rules! site_id_input_html_callback {
    ($dropdown_id:ident, $grid_column:literal, $onchange:ident, $modal_variant_handle:ident) => {
        html! {
            <DropdownSelectionComponent
                dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::SITE_ID, id: Some(**$dropdown_id), modal_variant_handle: $modal_variant_handle.clone()}}
                onchange={$onchange}
                col_num={$grid_column}
            />
        }
    };
}

macro_rules! pneumatic_instrument_type_input_html_callback {
    ($dropdown_id:ident, $grid_column:literal, $onchange:ident, $modal_variant_handle:ident) => {
        html! {
            <DropdownSelectionComponent
                dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::PNEUMATIC_INSTRUMENT_TYPE, id: None, modal_variant_handle: $modal_variant_handle.clone()}}
                onchange={$onchange}
                col_num={$grid_column}
            />
        }
    };
}

pub(crate) use insert_form;

pub(crate) use id_is_in_input;
pub(crate) use id_is_not_in_input;

pub(crate) use no;
pub(crate) use yes;

pub(crate) use disable;
pub(crate) use enable;

pub(crate) use has_modal;
pub(crate) use no_modal;

pub(crate) use onchange_string_callback;
pub(crate) use string_input_html_callback;

pub(crate) use date_input_html_callback;
pub(crate) use onchange_date_callback;

pub(crate) use float_input_html_callback;
pub(crate) use onchange_float_callback;

pub(crate) use device_manufacturer_id_input_html_callback;
pub(crate) use onchange_uuid_callback;
pub(crate) use site_id_input_html_callback;

pub(crate) use onchange_pneumatic_instrument_type_callback;
pub(crate) use pneumatic_instrument_type_input_html_callback;
