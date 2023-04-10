use crate::models::mutations::manual_mutation::insert_entry::{
    InsertEntryInput, Variables as VariablesInsertEntry,
};
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
    pub row_num: usize,
    pub col_num: usize,
    // pub input: InsertEntryInput,
    pub handle_insert_entry: Callback<VariablesInsertEntry>,
}

#[function_component(InsertEntryButton)]
pub fn insert_entry_button(
    Props {
        id,
        row_num,
        col_num,
        // input,
        handle_insert_entry,
    }: &Props,
) -> Html {
    let style = format!("grid-row: {}; grid-column: {};", row_num, col_num);

    // let onclick = {
    //     let id = id.clone();
    // let input = input.clone();
    // let handle_insert_entry = handle_insert_entry.clone();
    // Callback::from(move |_| {
    //     let input = input.clone();
    //     let variables = VariablesInsertEntry {
    //         insert_entry_input: input.clone(),
    //     };
    //     handle_insert_entry.emit(variables);
    // })
    // };

    html! {
        <div class={classes!("emitter-cell", "center")} {style}>
            // <button class={classes!("entry-button")} {onclick}>{ "+" }</button>
        </div>
    }
}
