use crate::models::mutations::controller::insert_controller::{
    InsertControllerInput, Variables as VariablesInsertController,
};
use std::rc::Rc;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state_eq, Callback, Event, Html, Properties,
    SubmitEvent, TargetCast,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub facility_id: Rc<Uuid>,
    pub handle_insert_controller: Callback<VariablesInsertController>,
}

#[function_component(InsertControllerForm)]
pub fn insert_controller_form(
    Props {
        facility_id,
        handle_insert_controller,
    }: &Props,
) -> Html {
    // let input_entry_input_handle = use_state_eq(|| InsertEntryInput {
    //     controller: InsertControllerInput {},
    //     compressor: None,
    // });
    // let insert_entry_input = (*input_entry_input_handle).clone();

    let input_model_handle = use_state_eq(|| None);
    let input_serial_number_handle = use_state_eq(|| None);
    let input_manufacturer_id_handle = use_state_eq(|| None);
    let input_application_id_handle = use_state_eq(|| None);
    let input_fdc_rec_id_handle = use_state_eq(|| None);

    let model = (*input_model_handle).clone();
    let serial_number = (*input_serial_number_handle).clone();
    let manufacturer_id = *input_manufacturer_id_handle;
    let application_id = *input_application_id_handle;
    let fdc_rec_id = (*input_fdc_rec_id_handle).clone();

    let onchange_model = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input_model_handle.set(Some(input.value()));
    });

    let onchange_serial_number = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input_serial_number_handle.set(Some(input.value()));
    });

    let onchange_manufacturer_id = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let Ok(uuid) = Uuid::parse_str(input.value().as_str()) else {
            return
        };
        input_manufacturer_id_handle.set(Some(uuid));
    });

    let onchange_application_id = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let Ok(uuid) = Uuid::parse_str(input.value().as_str()) else {
            return
        };
        input_application_id_handle.set(Some(uuid));
    });

    let onchange_fdc_rec_id = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        input_fdc_rec_id_handle.set(Some(input.value()));
    });

    let onsubmit = {
        let handle_insert_controller = handle_insert_controller.clone();
        let facility_id = facility_id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let fdc_rec_id = fdc_rec_id.clone();
            let model = model.clone();
            let serial_number = serial_number.clone();

            if let (Some(fdc_rec_id), Some(manufacturer_id)) = (fdc_rec_id, manufacturer_id) {
                let variables = VariablesInsertController {
                    insert_controller_input: InsertControllerInput {
                        facility_id: *facility_id,
                        model,
                        serial_number,
                        manufacturer_id,
                        application_id,
                        fdc_rec_id,
                    },
                };
                handle_insert_controller.emit(variables)
            }
        })
    };

    html! {
        <form {onsubmit} class={classes!("insert-form", "emitter-cell")}>
            <fieldset class={classes!("controller-form", "center")}>
                <button class={classes!("entry-button")} style="grid-row: 1; grid-column: 1;" type="submit">{"✓"}</button>
                <input type="text" style="grid-row: 1; grid-column: 3;" onchange={onchange_model}/>
                <input type="text" style="grid-row: 1; grid-column: 4;" onchange={onchange_serial_number}/>
                <input type="text" style="grid-row: 1; grid-column: 5;" onchange={onchange_manufacturer_id}/>
                <input type="text" style="grid-row: 1; grid-column: 6;" onchange={onchange_application_id}/>
                <input type="text" style="grid-row: 1; grid-column: 7;" onchange={onchange_fdc_rec_id}/>
            </fieldset>
        </form>
    }
}
