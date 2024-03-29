use crate::{
    components::emitters_window::data::entry::{DropdownSelectionComponent, DropdownSelectionProp},
    models::{
        mutations::manual_mutation::update_field::{
            UpdateFieldInput, UpdateFieldValue, UpdateFieldVariant,
            Variables as VariablesUpdateField,
        },
        NaiveDateTime,
    },
    utils::console_log,
};
use common::{
    PneumaticInstrumentType,
    UpdateFieldValueEnum::{
        self, FloatValue, IntegerValue, NaiveDateTimeValue, NaiveDateValue, OptionFloatValue,
        OptionIntegerValue, OptionNaiveDateTimeValue, OptionNaiveDateValue,
        OptionPneumaticInstrumentTypeValue, OptionStringValue, OptionUuidValue,
        PneumaticInstrumentTypeValue, StringValue, UuidValue,
    },
};
use std::str::FromStr;
use uuid::Uuid;
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::{window, Event, HtmlInputElement, Node, SubmitEvent};
use yew::{
    classes, function_component, html, use_effect_with_deps, use_node_ref, use_state_eq, Callback,
    Html, Properties, TargetCast,
};

#[derive(PartialEq, Clone, Debug)]
pub struct EditFieldProp {
    pub update_field_variant: UpdateFieldVariant,
    pub handle_update_field: Callback<VariablesUpdateField>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
    pub row_num: usize,
    pub col_num: usize,
    pub value: UpdateFieldValueEnum,
    pub display_value: Option<UpdateFieldValueEnum>,
    pub edit_field: Option<EditFieldProp>,
    pub dropdown_selection: Option<DropdownSelectionProp>,
}

#[function_component(Entry)]
pub fn entry(
    Props {
        id,
        row_num,
        col_num,
        value,
        display_value,
        edit_field,
        dropdown_selection,
    }: &Props,
) -> Html {
    let option_input_value_handle = use_state_eq(|| None);
    let option_input_value = (*option_input_value_handle).clone();

    let mode_handle = use_state_eq(|| EntryMode::ReadOnly);
    let mode = &*mode_handle;
    let div_ref = use_node_ref();

    let editable = edit_field.is_some();

    {
        let mode_handle = mode_handle.clone();
        let option_input_value_handle = option_input_value_handle.clone();
        let div_ref = div_ref.clone();

        use_effect_with_deps(
            move |div_ref| {
                let document = window()
                    .expect_throw("window is undefined")
                    .document()
                    .expect_throw("document is undefined");

                let div_ref = div_ref.clone();

                let handle_click_outside = Closure::<dyn Fn(Event)>::wrap(Box::new(move |e| {
                    let target = e.target();
                    let target = target.as_ref().map(|t| t.dyn_ref::<Node>()).flatten();

                    if let Some(node) = div_ref.get() {
                        if !node.contains(target) {
                            mode_handle.set(EntryMode::ReadOnly);
                            option_input_value_handle.set(None);
                        }
                    };
                }));

                document
                    .add_event_listener_with_callback(
                        "click",
                        handle_click_outside.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                move || {
                    document
                        .remove_event_listener_with_callback(
                            "click",
                            handle_click_outside.as_ref().unchecked_ref(),
                        )
                        .unwrap();
                }
            },
            div_ref,
        );
    }

    // use_effect_with_deps(
    //     move |u| {
    //         console_log!("input value: {:#?}", u);
    //     },
    //     option_input_value.clone(),
    // );

    let onchange = {
        let value = value.clone();
        let option_input_value_handle = option_input_value_handle.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            // let num = input.value_as_number().is_nan();
            console_log!("onchange input value: {}", input.value_as_number());
            let changed_value = match value {
                StringValue(_) | OptionStringValue(_) => {
                    let input_value = input.value();
                    OptionStringValue((!input_value.is_empty()).then(|| input_value))
                }
                IntegerValue(_) | OptionIntegerValue(_) => {
                    let input_value = input.value_as_number();
                    OptionIntegerValue((!input_value.is_nan()).then(|| input_value as i64))
                }
                FloatValue(_) | OptionFloatValue(_) => {
                    let input_value = input.value_as_number();
                    OptionFloatValue((!input_value.is_nan()).then(|| input_value))
                }
                UuidValue(_) | OptionUuidValue(_) => {
                    let input_value = input.value();
                    OptionUuidValue(
                        (!input_value.is_empty())
                            .then(|| Uuid::parse_str(input_value.as_str()).ok())
                            .flatten(),
                    )
                }
                NaiveDateValue(_) | OptionNaiveDateValue(_) => {
                    let input_value = input.value_as_number();

                    OptionNaiveDateValue((!input_value.is_nan()).then(|| {
                        NaiveDateTime::from_timestamp_millis(input_value as i64)
                            .expect_throw("Unable to convert i64 to NaiveDateTime.")
                            .date()
                    }))
                }
                NaiveDateTimeValue(_) | OptionNaiveDateTimeValue(_) => {
                    let input_value = input.value_as_number();
                    OptionNaiveDateTimeValue((!input_value.is_nan()).then(|| {
                        NaiveDateTime::from_timestamp_millis(input_value as i64)
                            .expect_throw("Unable to convert i64 to NaiveDateTime.")
                    }))
                }
                PneumaticInstrumentTypeValue(_) | OptionPneumaticInstrumentTypeValue(_) => {
                    let input_value = input.value();
                    OptionPneumaticInstrumentTypeValue(
                        (!input_value.is_empty())
                            .then(|| PneumaticInstrumentType::from_str(input.value().as_str()).ok())
                            .flatten(),
                    )
                }
                // BoolValue(_) | OptionBoolValue(_) => {
                //     let input_value = input.value_as_number();
                //     OptionBoolValue((!input_value.is_nan()).then(|| input_value as i64))
                // }
                _ => todo!(),
            };

            option_input_value_handle.set(Some(changed_value));
        })
    };

    let onsubmit = {
        let edit_field = edit_field.clone();
        let option_input_value = option_input_value.clone();
        let mode_handle = mode_handle.clone();
        let option_input_value_handle = option_input_value_handle.clone();
        let id = id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let edit_field = edit_field.clone();
            let option_input_value = option_input_value.clone();

            if let Some(EditFieldProp {
                handle_update_field,
                update_field_variant,
            }) = edit_field
            {
                if let Some(input_value) = option_input_value {
                    let value = match input_value {
                        StringValue(string_value) => UpdateFieldValue {
                            string_value: Some(string_value),
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        OptionStringValue(option_string_value) => UpdateFieldValue {
                            string_value: option_string_value,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        IntegerValue(integer_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: Some(integer_value),
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        OptionIntegerValue(option_integer_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: option_integer_value,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        FloatValue(float_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: Some(float_value),
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        OptionFloatValue(option_float_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: option_float_value,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        UuidValue(uuid_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: Some(uuid_value),
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        OptionUuidValue(option_uuid_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: option_uuid_value,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        NaiveDateValue(naive_date_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: Some(naive_date_value),
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        OptionNaiveDateValue(option_naive_date_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: option_naive_date_value,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        NaiveDateTimeValue(naive_date_time_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: Some(naive_date_time_value),
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: None,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        OptionNaiveDateTimeValue(option_naive_date_time_value) => {
                            UpdateFieldValue {
                                string_value: None,
                                integer_value: None,
                                float_value: None,
                                uuid_value: None,
                                naive_date_value: None,
                                naive_date_time_value: option_naive_date_time_value,
                                bool_value: None,
                                calculation_method_value: None,
                                compressor_seal_testing_point_value: None,
                                compressor_type_value: None,
                                control_device_inactivity_reason_value: None,
                                control_device_value: None,
                                facility_type_value: None,
                                pneumatic_instrument_type_value: None,
                                seal_type_value: None,
                                site_type_value: None,
                            }
                        }
                        PneumaticInstrumentTypeValue(pneumatic_instrument_type_value) => {
                            UpdateFieldValue {
                                string_value: None,
                                integer_value: None,
                                float_value: None,
                                uuid_value: None,
                                naive_date_value: None,
                                naive_date_time_value: None,
                                bool_value: None,
                                calculation_method_value: None,
                                compressor_seal_testing_point_value: None,
                                compressor_type_value: None,
                                control_device_inactivity_reason_value: None,
                                control_device_value: None,
                                facility_type_value: None,
                                pneumatic_instrument_type_value: Some(
                                    pneumatic_instrument_type_value,
                                ),
                                seal_type_value: None,
                                site_type_value: None,
                            }
                        }
                        OptionPneumaticInstrumentTypeValue(
                            option_pneumatic_instrument_type_value,
                        ) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                            bool_value: None,
                            calculation_method_value: None,
                            compressor_seal_testing_point_value: None,
                            compressor_type_value: None,
                            control_device_inactivity_reason_value: None,
                            control_device_value: None,
                            facility_type_value: None,
                            pneumatic_instrument_type_value: option_pneumatic_instrument_type_value,
                            seal_type_value: None,
                            site_type_value: None,
                        },
                        _ => todo!(),
                    };

                    let variables = VariablesUpdateField {
                        update_field_input: UpdateFieldInput {
                            id,
                            value,
                            update_field_variant: update_field_variant.clone(),
                        },
                    };

                    handle_update_field.emit(variables);
                };
            };
            mode_handle.set(EntryMode::ReadOnly);
            option_input_value_handle.set(None);
        })
    };

    let onclick = {
        let mode_handle = mode_handle.clone();
        let option_input_value_handle = option_input_value_handle.clone();
        let value = value.clone();

        Callback::from(move |_| {
            if editable {
                mode_handle.set(EntryMode::Write);
                option_input_value_handle.set(Some(value.clone()));
            }
        })
    };

    let style = format!("grid-row: {}; grid-column: {};", row_num, col_num);

    let form_type = match value {
        IntegerValue(_) | OptionIntegerValue(_) | FloatValue(_) | OptionFloatValue(_) => "number",
        NaiveDateValue(_) | OptionNaiveDateValue(_) => "date",
        NaiveDateTimeValue(_) | OptionNaiveDateTimeValue(_) => "datetime-local",
        _ => "text",
        // StringValue(_) | OptionStringValue(_) | UuidValue(_) | OptionUuidValue(_) => "text",
    };

    let form_step = match value {
        FloatValue(_) | OptionFloatValue(_) => Some("any"),
        _ => None,
    };

    let offer_null = match value {
        OptionStringValue(_)
        | OptionIntegerValue(_)
        | OptionFloatValue(_)
        | OptionUuidValue(_)
        | OptionNaiveDateValue(_)
        | OptionNaiveDateTimeValue(_) => true,
        OptionPneumaticInstrumentTypeValue(_) => true,
        _ => false,
    };

    html! {
        <div class={classes!("emitter-cell")} {style}>
            <div class={classes!("entry", editable.then(|| "editable"), (mode == &EntryMode::Write).then(|| "write"))} ref={div_ref}>
                // Child <div> of div_ref element cannot be conditionally rendered like child <form> because div_ref wouldn't register it as a descendant node of itself when clicking on div_ref.
                // Consequentially `EntryMode` would be flash set to `Write` and immediately turned back to `ReadOnly` when clicking on div_ref.
                <div style={(mode == &EntryMode::Write).then(|| "display: none;")} class={classes!("entry-read-only")} {onclick}>{ if let Some(value) = display_value { value } else { value } }</div>
                if mode == &EntryMode::Write {
                    <form {onsubmit}>
                        <fieldset>
                            <div class={classes!("input")}>
                                <button type="submit" class={classes!("form-button")}>{ "✓" }</button>
                                if let Some(dropdown_selection) = dropdown_selection {
                                    <DropdownSelectionComponent dropdown_selection={dropdown_selection.clone()} {onchange} {offer_null} value={value.to_string()}/>
                                } else {
                                    <input type={form_type} step={form_step} value={option_input_value.map(|input_value| input_value.to_string())} {onchange} />
                                }
                            </div>
                        </fieldset>
                    </form>
                }
            </div>
        </div>
    }
}

#[derive(PartialEq, Debug)]
pub enum EntryMode {
    ReadOnly,
    Write,
}
