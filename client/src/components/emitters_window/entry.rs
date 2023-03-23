use std::fmt::Display;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, Node, Document, window};
use yew::{
    classes, function_component, html, use_effect_with_deps, use_node_ref, use_state_eq, Callback,
    Html, Properties,
};

use crate::utils::console_log;

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
        let div_ref = div_ref.clone();

        use_effect_with_deps(
            move |_| {
                let div = div_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                let document = window().unwrap().document().unwrap();

                let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(|_| {
                    web_sys::console::log_1(&"Clicked!".into());
                }));

                let handle_click_outside = Closure::<dyn Fn(Event)>::wrap(Box::new(|e| {
                    let target = e.target();

                    console_log!("target: {:#?}", target);
            
            
                    // if let Some(g) = div_ref.get() {
                    //     if g.contains(target) {
            
                    //     }
                    // };
                }));

                document.add_event_listener_with_callback("click", handle_click_outside.as_ref().unchecked_ref()).unwrap();

                // div.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
                //     .unwrap();

                move || {
                    document.remove_event_listener_with_callback(
                        "click",
                        handle_click_outside.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                }
            },
            (),
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
            <div {ondblclick}>{ value }</div>
        </>
        },
        EntryMode::Write => {
            let form_type = match value {
                EntryValue::String(_) => "text",
                EntryValue::OptionString(_) => "text",
                EntryValue::I32(_) => "number",
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
        <div class={classes!("entry")} {style} ref={div_ref}>
            { m }
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
}

impl Display for EntryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::OptionString(os) => write!(f, "{}", os.as_ref().map_or_else(|| "", |s| s)),
            Self::I32(i) => write!(f, "{}", i),
        }
    }
}
