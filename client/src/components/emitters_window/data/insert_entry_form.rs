use crate::models::mutations::manual_mutation::delete_entry::DeleteEntryVariant;
use crate::models::mutations::manual_mutation::insert_entry::{
    InsertEntryInput, Variables as VariablesInsertEntry,
};
use yew::{function_component, html, use_state_eq, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub row_num: usize,
    pub insert_entry_variant: DeleteEntryVariant,
    pub handle_insert_entry: Callback<VariablesInsertEntry>,
}

#[function_component(ObjectRowComponent)]
pub fn object_row_component(
    Props {
        row_num,
        insert_entry_variant,
        handle_insert_entry,
    }: &Props,
) -> Html {
    let input_entry_input_handle = use_state_eq(|| InsertEntryInput {
        controller: None,
        compressor: None,
    });

    let insert_entry_input = (*input_entry_input_handle).clone();

    let view = match insert_entry_variant {
        DeleteEntryVariant::CONTROLLER => {
            html! {
                <form>
                    <input style="grid-row: 2; grid-column: 3;"/>
                    <input style="grid-row: 2; grid-column: 4;"/>
                    <input style="grid-row: 2; grid-column: 5;"/>
                    <input style="grid-row: 2; grid-column: 6;"/>
                    <input style="grid-row: 2; grid-column: 7;"/>
                </form>
            }
        }
        _ => todo!(),
    };

    view
}
