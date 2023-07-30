#[macro_export]
macro_rules! insert_form {
    ($component_name:ident, $id:ident, ($variable:ident, $input:ident, $input_type:ident), $(($variant:path, $handle_inits:ident, $inits:ident, $onchange_inits:ident)),*) => {
        use std::rc::Rc;
        use uuid::Uuid;
        use yew::{function_component, use_state_eq, TargetCast, Callback, Event, Html, Properties, SubmitEvent};
        use web_sys::HtmlInputElement;
        use crate::models::NaiveDateTime;
        use wasm_bindgen::UnwrapThrowExt;

        #[derive(PartialEq, Properties)]
        pub struct Props {
            pub handle_insert: Callback<$variable>,
            pub close_insert_form: Callback<()>,
            pub $id: Rc<Uuid>,
        }

        #[function_component($component_name)]
        pub fn insert_form(
            Props {
                handle_insert,
                close_insert_form,
                $id,
            }: &Props,
        ) -> Html {
            $(let $handle_inits = use_state_eq(|| $variant(None));)*

            $(let $inits = (*$handle_inits).clone();)*

            $(let $onchange_inits = {

                // let input_date_handle = input_date_handle.clone();
                let $inits = $inits.clone();
                Callback::from(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    match $inits {
                        DateValue(_) => {
                            let input_value = input.value_as_number();
                            $handle_inits.set(DateValue((!input_value.is_nan()).then(|| {
                                NaiveDateTime::from_timestamp_millis(input_value as i64)
                                    .expect_throw("Unable to convert i64 to NaiveDateTime.")
                                    .date()
                            })));
                        }
                        FloatValue(_) => {
                            let input_value = input.value_as_number();
                            $handle_inits.set(FloatValue((!input_value.is_nan()).then(|| input_value)));
                        }
                    }
                })
            };)*

            let disabled = match ($(&$inits),*) {
                ($($variant(Some(_))),*) => false,
                _ => true,
            };

            let onsubmit = {
                let handle_insert = handle_insert.clone();
                let close_insert_form = close_insert_form.clone();
                let $id = $id.clone();

                Callback::from(move |e: SubmitEvent| {
                    e.prevent_default();
                    $(let $inits = $inits.clone();)*

                    if let ($($variant(Some($inits))),*) = ($($inits),*) {
                        let variables = $variable {
                            $input: $input_type {
                                    $id: *$id,
                                    $($inits),*
                                },
                        };
                        handle_insert.emit(variables);
                        close_insert_form.emit(());
                    }
                })
            };

            Html::default()
        }
    };
}

use crate::models::mutations::pneumatic_instrument::insert_pneumatic_instrument_emission_rate::{
    InsertPneumaticInstrumentEmissionRateInput, Variables,
};
use common::InsertFieldValueEnum::{DateValue, FloatValue};

insert_form!(
    InsertPneumaticInstrumentEmissionRateForm,
    pneumatic_instrument_id,
    (
        Variables,
        insert_pneumatic_instrument_emission_rate_input,
        InsertPneumaticInstrumentEmissionRateInput
    ),
    (DateValue, input_date_handle, date, onchange_date),
    (FloatValue, input_rate_handle, rate, onchange_rate)
);
