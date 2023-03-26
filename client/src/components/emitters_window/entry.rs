use crate::{
    models::{
        mutations::update_field::update_field::{
            UpdateFieldInput, UpdateFieldValue, UpdateFieldVariant,
            Variables as VariablesUpdateField,
        },
        queries::controller::{
            get_controllers::{EmittersByInput, ResponseData, Variables},
            GetControllers,
        },
        NaiveDate, NaiveDateTime,
    },
    utils::console_log,
};
use common::UpdateFieldValue::{
    self as UpdateFieldValueEnum, FloatValue, IntegerValue, NaiveDateTimeValue, NaiveDateValue,
    OptionFloatValue, OptionIntegerValue, OptionStringValue, OptionUuidValue, StringValue,
    UuidValue,
};
use uuid::Uuid;
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::{window, Event, HtmlInputElement, Node};
use yew::{
    classes, function_component, html, use_effect_with_deps, use_node_ref, use_state_eq, Callback,
    Html, Properties, TargetCast,
};

#[derive(PartialEq)]
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
    pub edit_field: Option<EditFieldProp>,
}

#[function_component(Entry)]
pub fn entry(
    Props {
        id,
        row_num,
        col_num,
        value,
        edit_field,
    }: &Props,
) -> Html {
    let input_value_handle = use_state_eq(|| value.clone());
    let input_value = (*input_value_handle).clone();

    let mode_state = use_state_eq(|| EntryMode::ReadOnly);
    let mode = &*mode_state;

    let div_ref = use_node_ref();

    {
        let mode_state = mode_state.clone();
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
                            mode_state.set(EntryMode::ReadOnly);
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

    use_effect_with_deps(
        move |u| {
            console_log!("input changed: {:#?}", u);
        },
        input_value.clone(),
    );

    let value_for_onchange = value.clone();
    let onchange = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let changed_value = match value_for_onchange {
            StringValue(_) => StringValue(input.value()),
            OptionStringValue(_) => OptionStringValue(Some(input.value())),
            IntegerValue(_) => IntegerValue(input.value_as_number() as i64),
            OptionIntegerValue(_) => OptionIntegerValue(Some(input.value_as_number() as i64)),
            FloatValue(_) => FloatValue(input.value_as_number()),
            OptionFloatValue(_) => OptionFloatValue(Some(input.value_as_number())),
            UuidValue(_) => UuidValue(
                Uuid::parse_str(input.value().as_str())
                    .expect_throw("Unable to convert &str to Uuid."),
            ),
            OptionUuidValue(_) => OptionUuidValue(Some(
                Uuid::parse_str(input.value().as_str())
                    .expect_throw("Unable to convert &str to Uuid."),
            )),
            NaiveDateTimeValue(_) => NaiveDateTimeValue(
                NaiveDateTime::from_timestamp_millis(input.value_as_number() as i64)
                    .expect_throw("Unable to convert i64 to NaiveDateTime."),
            ),
            NaiveDateValue(_) => NaiveDateValue(
                NaiveDateTime::from_timestamp_millis(input.value_as_number() as i64)
                    .expect_throw("Unable to convert i64 to NaiveDateTime.")
                    .date(),
            ),
        };

        input_value_handle.set(changed_value);
    });

    if let Some(EditFieldProp {
        handle_update_field,
        update_field_variant,
    }) = edit_field
    {
        let value = match input_value.clone() {
            OptionStringValue(option_string_value) => UpdateFieldValue {
                string_value: option_string_value,
                integer_value: None,
                float_value: None,
                uuid_value: None,
                naive_date_value: None,
                naive_date_time_value: None,
            },
            StringValue(string_value) => UpdateFieldValue {
                string_value: Some(string_value),
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
            NaiveDateTimeValue(naive_date_time_value) => UpdateFieldValue {
                string_value: None,
                integer_value: None,
                float_value: None,
                uuid_value: None,
                naive_date_value: None,
                naive_date_time_value: Some(naive_date_time_value),
            },
        };
        let variables = VariablesUpdateField {
            input: UpdateFieldInput {
                id: *id,
                value,
                update_field_variant: update_field_variant.clone(),
            },
        };

        let y = handle_update_field.emit(variables);
    };

    let mode_state = mode_state.clone();
    let ondblclick = Callback::from(move |_| {
        mode_state.set(EntryMode::Write);
    });

    let style = format!("grid-row: {}; grid-column: {};", row_num, col_num);
    let view = match mode {
        EntryMode::ReadOnly => html! {
        <>
            <div class={classes!("entry-read-only")} {ondblclick}>{ value }</div>
        </>
        },
        EntryMode::Write => {
            let form_type = match value {
                IntegerValue(_) | FloatValue(_) => "number",
                NaiveDateValue(_) => "date",
                NaiveDateTimeValue(_) => "datetime-local",
                _ => "text",
            };

            html! {
                <>
                    <form>
                        <input type={form_type} value={input_value.to_string()} {onchange} />
                        <button type="submit">{ "S" }</button>
                    </form>
                </>
            }
        }
    };

    html! {
        <div class={classes!("emitter-cell")} {style}>
            <div class={classes!("entry")} ref={div_ref}>
                { view }
            </div>
        </div>
    }
}

#[derive(PartialEq)]
pub enum EntryMode {
    ReadOnly,
    Write,
}
