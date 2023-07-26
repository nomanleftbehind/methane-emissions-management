use crate::{
    components::emitters_window::data::entry::{DropdownSelectionComponent, DropdownSelectionProp},
    models::{
        mutations::pneumatic_instrument::insert_pneumatic_instrument::{
            InsertPneumaticInstrumentInput, Variables as VariablesInsertPneumaticInstrument,
        },
        queries::dropdown_selection::get_dropdown_selection::DropdownSelectionVariant,
        NaiveDateTime,
    },
    pages::ModalVariant,
    utils::console_log,
};
use common::PneumaticInstrumentType;
use std::{rc::Rc, str::FromStr};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state_eq, Callback, Event, Html,
    Properties, SubmitEvent, TargetCast,
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub handle_insert_pneumatic_instrument: Callback<VariablesInsertPneumaticInstrument>,
    pub close_insert_form: Callback<()>,
    pub modal_variant_handle: Callback<Option<ModalVariant>>,
    pub facility_id: Rc<Uuid>,
}

#[function_component(InsertPneumaticInstrumentForm)]
pub fn insert_pneumatic_instrument_form(
    Props {
        handle_insert_pneumatic_instrument,
        close_insert_form,
        modal_variant_handle,
        facility_id,
    }: &Props,
) -> Html {
    let input_site_id_handle = use_state_eq(|| None);
    let input_type_handle = use_state_eq(|| None);
    let input_manufacturer_id_handle = use_state_eq(|| None);
    let input_model_handle = use_state_eq(|| None);
    let input_serial_number_handle = use_state_eq(|| None);
    let input_start_date_handle = use_state_eq(|| None);
    let input_end_date_handle = use_state_eq(|| None);

    let site_id = *input_site_id_handle;
    let type_ = *input_type_handle;
    let manufacturer_id = *input_manufacturer_id_handle;
    let model = (*input_model_handle).clone();
    let serial_number = (*input_serial_number_handle).clone();
    let start_date = *input_start_date_handle;
    let end_date = *input_end_date_handle;

    let onchange_site_id = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let uuid = Uuid::parse_str(input.value().as_str()).ok();
        input_site_id_handle.set(uuid);
    });

    let onchange_type = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let type_ = PneumaticInstrumentType::from_str(input.value().as_str()).ok();
        input_type_handle.set(type_);
    });

    let onchange_manufacturer_id = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let uuid = Uuid::parse_str(input.value().as_str()).ok();
        input_manufacturer_id_handle.set(uuid);
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

    use_effect_with_deps(
        move |u| {
            console_log!("number_of_updated_fields: {:#?}", u);
        },
        (site_id, type_, manufacturer_id, start_date),
    );

    let disabled = !(site_id.is_some()
        && type_.is_some()
        && manufacturer_id.is_some()
        && start_date.is_some());

    let onsubmit = {
        let handle_insert_pneumatic_instrument = handle_insert_pneumatic_instrument.clone();
        let close_insert_form = close_insert_form.clone();
        let model = model.clone();
        let serial_number = serial_number.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let model = model.clone();
            let serial_number = serial_number.clone();

            if let (Some(site_id), Some(type_), Some(manufacturer_id), Some(start_date)) =
                (site_id, type_, manufacturer_id, start_date)
            {
                console_log!(
                    "type {:?}, {:?}, {:?}, {:?}",
                    &site_id,
                    &type_,
                    &manufacturer_id,
                    &start_date
                );
                let variables = VariablesInsertPneumaticInstrument {
                    insert_pneumatic_instrument_input: InsertPneumaticInstrumentInput {
                        site_id,
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

    // When inserting new pneumatic instrument, server needs to offer only sites belonging to a selected facility.
    // This is achieved by passing optional `id`, which, in this case, is `facility_id`.
    let site_facility_id = Some(**facility_id);

    html! {
        <form {onsubmit} class={classes!("insert-form", "emitter-cell")}>
            <fieldset class={classes!("pneumatic-instrument-form", "center")}>
                <button class={classes!("entry-button")} style="grid-row: 1; grid-column: 1;" type="submit" {disabled}>{"✓"}</button>
                <DropdownSelectionComponent
                    dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::SITE_ID, id: site_facility_id, modal_variant_handle: modal_variant_handle.clone()}}
                    onchange={onchange_site_id}
                    col_num={3}
                />
                <DropdownSelectionComponent
                    dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::PNEUMATIC_INSTRUMENT_TYPE, id: None, modal_variant_handle: modal_variant_handle.clone()}}
                    onchange={onchange_type}
                    col_num={4}
                />
                <DropdownSelectionComponent
                    dropdown_selection={DropdownSelectionProp {variant: DropdownSelectionVariant::DEVICE_MANUFACTURER_ID, id: None, modal_variant_handle: modal_variant_handle.clone()}}
                    onchange={onchange_manufacturer_id}
                    col_num={5}
                />
                <input type="text" style="grid-row: 1; grid-column: 6;" onchange={onchange_model} value={model.map_or_else(|| "".to_string(), |model| model)}/>
                <input type="text" style="grid-row: 1; grid-column: 7;" onchange={onchange_serial_number} value={serial_number.map_or_else(|| "".to_string(), |serial_number| serial_number)}/>
                <input type="date" style="grid-row: 1; grid-column: 8;" onchange={onchange_start_date}/>
                <input type="date" style="grid-row: 1; grid-column: 9;" onchange={onchange_end_date}/>
            </fieldset>
        </form>
    }
}
