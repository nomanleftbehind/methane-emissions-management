use crate::{
    components::emitters_window::data::entry::{IdSelectionComponent, IdSelectionProp},
    models::{
        mutations::manual_mutation::update_field::{
            UpdateFieldInput, UpdateFieldValue, UpdateFieldVariant,
            Variables as VariablesUpdateField,
        },
        NaiveDateTime,
    },
};
use common::UpdateFieldValueEnum::{
    self, FloatValue, IntegerValue, NaiveDateTimeValue, NaiveDateValue, OptionFloatValue,
    OptionIntegerValue, OptionNaiveDateTimeValue, OptionNaiveDateValue, OptionStringValue,
    OptionUuidValue, StringValue, UuidValue,
};
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
    pub id_selection: Option<IdSelectionProp>,
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
        id_selection,
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
            let changed_value = match value {
                StringValue(_) => StringValue(input.value()),
                OptionStringValue(_) => {
                    let input_value = input.value();
                    OptionStringValue((!input_value.is_empty()).then(|| input_value))
                }
                IntegerValue(_) => IntegerValue(input.value_as_number() as i64),
                OptionIntegerValue(_) => OptionIntegerValue(Some(input.value_as_number() as i64)),
                FloatValue(_) => FloatValue(input.value_as_number()),
                OptionFloatValue(_) => OptionFloatValue(Some(input.value_as_number())),
                UuidValue(_) => {
                    let Ok(uuid_value) = Uuid::parse_str(input.value().as_str()) else {
                        option_input_value_handle.set(None);
                        return
                    };
                    UuidValue(uuid_value)
                }
                OptionUuidValue(_) => {
                    let Ok(uuid_value) = Uuid::parse_str(input.value().as_str()) else {
                        option_input_value_handle.set(None);
                        return
                    };
                    OptionUuidValue(Some(uuid_value))
                }
                NaiveDateValue(_) => NaiveDateValue(
                    NaiveDateTime::from_timestamp_millis(input.value_as_number() as i64)
                        .expect_throw("Unable to convert i64 to NaiveDateTime.")
                        .date(),
                ),
                OptionNaiveDateValue(_) => OptionNaiveDateValue(Some(
                    NaiveDateTime::from_timestamp_millis(input.value_as_number() as i64)
                        .expect_throw("Unable to convert i64 to NaiveDateTime.")
                        .date(),
                )),
                NaiveDateTimeValue(_) => NaiveDateTimeValue(
                    NaiveDateTime::from_timestamp_millis(input.value_as_number() as i64)
                        .expect_throw("Unable to convert i64 to NaiveDateTime."),
                ),
                OptionNaiveDateTimeValue(_) => OptionNaiveDateTimeValue(Some(
                    NaiveDateTime::from_timestamp_millis(input.value_as_number() as i64)
                        .expect_throw("Unable to convert i64 to NaiveDateTime."),
                )),
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
                        },
                        OptionStringValue(option_string_value) => UpdateFieldValue {
                            string_value: option_string_value,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                        },
                        IntegerValue(integer_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: Some(integer_value),
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                        },
                        OptionIntegerValue(option_integer_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: option_integer_value,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                        },
                        FloatValue(float_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: Some(float_value),
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                        },
                        OptionFloatValue(option_float_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: option_float_value,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: None,
                        },
                        UuidValue(uuid_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: Some(uuid_value),
                            naive_date_value: None,
                            naive_date_time_value: None,
                        },
                        OptionUuidValue(option_uuid_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: option_uuid_value,
                            naive_date_value: None,
                            naive_date_time_value: None,
                        },
                        NaiveDateValue(naive_date_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: Some(naive_date_value),
                            naive_date_time_value: None,
                        },
                        OptionNaiveDateValue(option_naive_date_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: option_naive_date_value,
                            naive_date_time_value: None,
                        },
                        NaiveDateTimeValue(naive_date_time_value) => UpdateFieldValue {
                            string_value: None,
                            integer_value: None,
                            float_value: None,
                            uuid_value: None,
                            naive_date_value: None,
                            naive_date_time_value: Some(naive_date_time_value),
                        },
                        OptionNaiveDateTimeValue(option_naive_date_time_value) => {
                            UpdateFieldValue {
                                string_value: None,
                                integer_value: None,
                                float_value: None,
                                uuid_value: None,
                                naive_date_value: None,
                                naive_date_time_value: option_naive_date_time_value,
                            }
                        }
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

    let ondblclick = {
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
    let view = match mode {
        EntryMode::ReadOnly => html! {
            <div class={classes!("entry-read-only")} {ondblclick}>{ if let Some(value) = display_value { value } else { value } }</div>
        },
        EntryMode::Write => {
            let form_type = match value {
                IntegerValue(_) | OptionIntegerValue(_) | FloatValue(_) | OptionFloatValue(_) => {
                    "number"
                }
                NaiveDateValue(_) | OptionNaiveDateValue(_) => "date",
                NaiveDateTimeValue(_) | OptionNaiveDateTimeValue(_) => "datetime-local",
                StringValue(_) | OptionStringValue(_) | UuidValue(_) | OptionUuidValue(_) => "text",
            };

            let form_step = match value {
                FloatValue(_) | OptionFloatValue(_) => Some("any"),
                _ => None,
            };

            html! {
                <form {onsubmit}>
                    <fieldset>
                        <div class={classes!("input")}>
                            <button type="submit" class={classes!("form-button")}>{ "✓" }</button>
                            if let Some(id_selection) = id_selection {
                                <IdSelectionComponent id_selection={id_selection.clone()} {onchange}/>
                            } else {
                                <input type={form_type} step={form_step} value={option_input_value.map_or_else(|| "".to_string(), |input_value| input_value.to_string())} {onchange} />
                            }
                        </div>
                    </fieldset>
                </form>
            }
        }
    };

    html! {
        <div class={classes!("emitter-cell")} {style}>
            <div class={classes!("entry", editable.then(|| "editable"), (mode == &EntryMode::Write).then(|| "write"))} ref={div_ref}>
                { view }
            </div>
        </div>
    }
}

#[derive(PartialEq, Debug)]
pub enum EntryMode {
    ReadOnly,
    Write,
}
