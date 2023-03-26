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
        NaiveDateTime,
    },
    utils::console_log,
};
use common::UpdateFieldValue::{
    self as UpdateFieldValueEnum, IntegerValue, NaiveDateTimeValue, OptionStringValue, StringValue,
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
            UuidValue(_) => UuidValue(Uuid::parse_str(input.value().as_str()).unwrap_throw()),
            NaiveDateTimeValue(_) => IntegerValue(0), // _ => IntegerValue(0),
        };

        input_value_handle.set(changed_value);

        // console_log!("event: {:#?}", input_value);
    });

    if let Some(EditFieldProp {
        handle_update_field,
        update_field_variant,
    }) = edit_field
    {
        let value = match value.clone() {
            OptionStringValue(option_string_value) => UpdateFieldValue {
                string_value: option_string_value.clone(),
                integer_value: None,
                uuid_value: None,
                naive_date_time_value: None,
            },
            _ => UpdateFieldValue {
                string_value: Some("Hi".to_string()),
                integer_value: None,
                uuid_value: None,
                naive_date_time_value: None,
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
                IntegerValue(_) => "number",
                NaiveDateTimeValue(_) => "date",
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
