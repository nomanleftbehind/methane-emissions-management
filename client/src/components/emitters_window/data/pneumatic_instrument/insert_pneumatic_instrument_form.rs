use crate::models::{
    mutations::pneumatic_instrument::insert_pneumatic_instrument::{
        InsertPneumaticInstrumentInput, Variables as VariablesInsertPneumaticInstrument,
    },
    NaiveDateTime,
};
use common::PneumaticInstrumentType;
use std::rc::Rc;
use std::str::FromStr;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state_eq, Callback, Event, Html, Properties,
    SubmitEvent, TargetCast,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub site_id: Rc<Uuid>,
    pub handle_insert_pneumatic_instrument: Callback<VariablesInsertPneumaticInstrument>,
    pub close_insert_form: Callback<()>,
}

#[function_component(InsertPneumaticInstrumentForm)]
pub fn insert_pneumatic_instrument_form(
    Props {
        site_id,
        handle_insert_pneumatic_instrument,
        close_insert_form,
    }: &Props,
) -> Html {
    let input_type_handle = use_state_eq(|| None);
    let input_manufacturer_id_handle = use_state_eq(|| None);
    let input_model_handle = use_state_eq(|| None);
    let input_serial_number_handle = use_state_eq(|| None);
    let input_start_date_handle = use_state_eq(|| None);
    let input_end_date_handle = use_state_eq(|| None);

    let type_ = *input_type_handle;
    let manufacturer_id = *input_manufacturer_id_handle;
    let model = (*input_model_handle).clone();
    let serial_number = (*input_serial_number_handle).clone();
    let start_date = *input_start_date_handle;
    let end_date = *input_end_date_handle;

    let onchange_type = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let Ok(type_) = PneumaticInstrumentType::from_str(input.value().as_str()) else {
            return
        };
        input_type_handle.set(Some(type_));
    });

    let onchange_manufacturer_id = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let Ok(uuid) = Uuid::parse_str(input.value().as_str()) else {
            return
        };
        input_manufacturer_id_handle.set(Some(uuid));
    });

    let onchange_model = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input_model_handle.set(Some(input.value()));
    });

    let onchange_serial_number = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input_serial_number_handle.set(Some(input.value()));
    });

    let onchange_start_date = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value_as_number();
        let start_date = (!input_value.is_nan()).then(|| {
            NaiveDateTime::from_timestamp_millis(input_value as i64)
                .expect_throw("Unable to convert i64 to NaiveDateTime.")
                .date()
        });
        input_start_date_handle.set(start_date);
    });

    let onchange_end_date = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value_as_number();
        let end_date = (!input_value.is_nan()).then(|| {
            NaiveDateTime::from_timestamp_millis(input_value as i64)
                .expect_throw("Unable to convert i64 to NaiveDateTime.")
                .date()
        });
        input_end_date_handle.set(end_date);
    });

    let onsubmit = {
        let handle_insert_pneumatic_instrument = handle_insert_pneumatic_instrument.clone();
        let close_insert_form = close_insert_form.clone();
        let site_id = site_id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let end_date = end_date.clone();
            let model = model.clone();
            let serial_number = serial_number.clone();

            if let (Some(type_), Some(manufacturer_id), Some(start_date)) =
                (type_, manufacturer_id, start_date)
            {
                let variables = VariablesInsertPneumaticInstrument {
                    insert_pneumatic_instrument_input: InsertPneumaticInstrumentInput {
                        site_id: *site_id,
                        type_,
                        manufacturer_id,
                        model,
                        serial_number,
                        start_date,
                        end_date,
                    },
                };
                handle_insert_pneumatic_instrument.emit(variables);
                close_insert_form.emit(());
            }
        })
    };

    html! {
        <form {onsubmit} class={classes!("insert-form", "emitter-cell")}>
            <fieldset class={classes!("controller-form", "center")}>
                <button class={classes!("entry-button")} style="grid-row: 1; grid-column: 1;" type="submit">{"✓"}</button>
                <input type="text" style="grid-row: 1; grid-column: 3;" onchange={onchange_type}/>
                <input type="text" style="grid-row: 1; grid-column: 4;" onchange={onchange_manufacturer_id}/>
                <input type="text" style="grid-row: 1; grid-column: 5;" onchange={onchange_model}/>
                <input type="text" style="grid-row: 1; grid-column: 6;" onchange={onchange_serial_number}/>
                <input type="text" style="grid-row: 1; grid-column: 7;" onchange={onchange_start_date}/>
                <input type="text" style="grid-row: 1; grid-column: 8;" onchange={onchange_end_date}/>
            </fieldset>
        </form>
    }
}
