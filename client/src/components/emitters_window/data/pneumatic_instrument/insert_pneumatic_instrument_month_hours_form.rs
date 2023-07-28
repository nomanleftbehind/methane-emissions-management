use crate::{
    models::{
        mutations::pneumatic_instrument::insert_pneumatic_instrument_month_hours::{
            InsertPneumaticInstrumentMonthHoursInput,
            Variables as VariablesInsertPneumaticInstrumentMonthHours,
        },
        NaiveDateTime,
    },
    utils::console_log,
};
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state_eq, Callback, Event, Html, InputEvent, Properties,
    SubmitEvent, TargetCast,
};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub handle_insert_pneumatic_instrument_month_hours:
        Callback<VariablesInsertPneumaticInstrumentMonthHours>,
    pub close_insert_form: Callback<()>,
    pub pneumatic_instrument_id: Rc<Uuid>,
}

#[function_component(InsertPneumaticInstrumentMonthHoursForm)]
pub fn insert_pneumatic_instrument_month_hours_form(
    Props {
        handle_insert_pneumatic_instrument_month_hours,
        close_insert_form,
        pneumatic_instrument_id,
    }: &Props,
) -> Html {
    let input_month_handle = use_state_eq(|| None);
    let input_hours_on_handle = use_state_eq(|| None);

    let month = *input_month_handle;
    let hours_on = *input_hours_on_handle;

    let onchange_month = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value_as_number();
        let month = (!input_value.is_nan()).then(|| {
            NaiveDateTime::from_timestamp_millis(input_value as i64)
                .expect_throw("Unable to convert i64 to NaiveDateTime.")
                .date()
        });
        input_month_handle.set(month);
    });

    let oninput_hours_on = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let input_value = input.value_as_number();
        console_log!("value: {}", input_value);
        input_hours_on_handle.set(Some(input_value));
    });

    let disabled = !(month.is_some() && hours_on.is_some());

    let onsubmit = {
        let handle_insert_pneumatic_instrument_month_hours =
            handle_insert_pneumatic_instrument_month_hours.clone();
        let close_insert_form = close_insert_form.clone();
        let pneumatic_instrument_id = pneumatic_instrument_id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let (Some(hours_on), Some(month)) = (hours_on, month) {
                let variables = VariablesInsertPneumaticInstrumentMonthHours {
                    insert_pneumatic_instrument_month_hours_input:
                        InsertPneumaticInstrumentMonthHoursInput {
                            pneumatic_instrument_id: *pneumatic_instrument_id,
                            month,
                            hours_on,
                        },
                };
                handle_insert_pneumatic_instrument_month_hours.emit(variables);
                close_insert_form.emit(());
            }
        })
    };

    html! {
        <form {onsubmit} class={classes!("insert-form", "emitter-cell")}>
            <fieldset class={classes!("pneumatic-instrument-month-hours-form", "center")}>
                <button class={classes!("entry-button")} style="grid-row: 1; grid-column: 1;" type="submit" {disabled}>{"✓"}</button>
                <input type="date" style="grid-row: 1; grid-column: 2;" onchange={onchange_month}/>
                // oninput event occurs immediately after the content has been changed, while onchange occurs when the element loses focus.
                // Using oninput here because we want button to stop being disabled as soon as month and hours_on are entered, and not having to click outide for onchange to occur.
                <input type="number" step="any" style="grid-row: 1; grid-column: 3;" oninput={oninput_hours_on}/>
            </fieldset>
        </form>
    }
}
