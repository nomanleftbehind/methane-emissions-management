use crate::models::mutations::manual_mutation::delete_entry::{
    DeleteEntryInput, DeleteEntryVariant, Variables as VariablesDeleteEntry,
};
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Uuid,
    pub row_num: usize,
    pub col_num: usize,
    pub delete_entry_variant: DeleteEntryVariant,
    pub handle_delete_entry: Callback<VariablesDeleteEntry>,
}

#[function_component(DeleteEntryComponent)]
pub fn delete_entry_component(
    Props {
        id,
        row_num,
        col_num,
        delete_entry_variant,
        handle_delete_entry,
    }: &Props,
) -> Html {
    let style = format!("grid-row: {}; grid-column: {};", row_num, col_num);

    let onclick = {
        let id = id.clone();
        let delete_entry_variant = delete_entry_variant.clone();
        let handle_delete_entry = handle_delete_entry.clone();
        Callback::from(move |_| {
            let delete_entry_variant = delete_entry_variant.clone();
            let variables = VariablesDeleteEntry {
                delete_entry_input: DeleteEntryInput {
                    id,
                    delete_entry_variant,
                },
            };
            handle_delete_entry.emit(variables);
        })
    };

    html! {
        <div class={classes!("emitter-cell")} {style}>
            <button class={classes!("form-button")} {onclick}>{ "🗑" }</button>
        </div>
    }
}
