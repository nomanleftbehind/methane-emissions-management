use std::fmt::Display;
use wasm_bindgen::JsCast;
use wasm_bindgen::{prelude::Closure, UnwrapThrowExt};
use web_sys::{window, Event, Node};
use yew::{
    classes, function_component, html, use_effect_with_deps, use_node_ref, use_state_eq, Callback,
    Html, Properties,
};

use crate::models::NaiveDateTime;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub row_num: usize,
    pub col_num: usize,
    pub value: EntryValue,
}

#[function_component(Entry)]
pub fn entry(
    Props {
        row_num,
        col_num,
        value,
    }: &Props,
) -> Html {
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

    let mode_state = mode_state.clone();
    let ondblclick = Callback::from(move |_| {
        mode_state.set(EntryMode::Write);
    });

    let style = format!("grid-row: {}; grid-column: {};", row_num, col_num);
    let m = match mode {
        EntryMode::ReadOnly => html! {
        <>
            <div class={classes!("entry-read-only")} {ondblclick}>{ value }</div>
        </>
        },
        EntryMode::Write => {
            let form_type = match value {
                EntryValue::String(_) => "text",
                EntryValue::OptionString(_) => "text",
                EntryValue::I32(_) => "number",
                EntryValue::NaiveDateTime(_) => "date",
            };

            html! {
                <>
                    <form>
                        <input type={form_type} value={value.to_string()} />
                        <button type="submit">{ "S" }</button>
                    </form>
                </>
            }
        }
    };

    html! {
        <div class={classes!("emitter-cell")} {style}>
            <div class={classes!("entry")} ref={div_ref}>
                { m }
            </div>
        </div>
    }
}

#[derive(PartialEq)]
pub enum EntryMode {
    ReadOnly,
    Write,
}

#[derive(PartialEq)]
pub enum EntryValue {
    String(String),
    OptionString(Option<String>),
    I32(i32),
    NaiveDateTime(NaiveDateTime),
}

impl Display for EntryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::OptionString(os) => write!(f, "{}", os.as_ref().map_or_else(|| "", |s| s)),
            Self::I32(i) => write!(f, "{}", i),
            Self::NaiveDateTime(ndt) => write!(f, "{}", ndt),
        }
    }
}
